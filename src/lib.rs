use std::marker::PhantomData;

use bevy::{
    asset::{Assets, Handle, HandleUntyped},
    core_pipeline::Opaque3d,
    ecs::system::SystemParam,
    pbr::{
        DrawMesh, MeshPipeline, MeshPipelineKey, MeshUniform, SetMeshBindGroup,
        SetMeshViewBindGroup,
    },
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::{Indices, Mesh, VertexAttributeValues},
        render_phase::{AddRenderCommand, DrawFunctions, RenderPhase, SetItemPipeline},
        render_resource::{
            PrimitiveTopology, RenderPipelineCache, RenderPipelineDescriptor, Shader,
            SpecializedPipeline, SpecializedPipelines, VertexAttribute, VertexBufferLayout,
            VertexFormat, VertexStepMode,
        },
        view::{ExtractedView, Msaa},
        RenderApp, RenderStage,
    },
};

const DEBUG_LINES_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 17477439189930443325);

/// Bevy plugin, for initializing stuff.
///
/// # Usage
///
/// ```
/// use bevy::prelude::*;
/// use bevy_prototype_debug_lines::*;
///
/// App::new()
///     .add_plugins(DefaultPlugins)
///     .add_plugin(DebugLinesPlugin)
///     .run();
/// ```
#[derive(Debug, Default, Clone)]
pub struct DebugLinesPlugin {
    always_on_top: bool,
}
impl DebugLinesPlugin {
    pub fn draw_on_top(always_on_top: bool) -> Self {
        DebugLinesPlugin { always_on_top }
    }
}
impl Plugin for DebugLinesPlugin {
    fn build(&self, app: &mut App) {
        let mut shaders = app.world.get_resource_mut::<Assets<Shader>>().unwrap();
        shaders.set_untracked(
            DEBUG_LINES_SHADER_HANDLE,
            Shader::from_wgsl(include_str!("debuglines.wgsl")),
        );
        app.init_resource::<ImmLinesStorage>();
        app.init_resource::<RetLinesStorage>();
        app.add_startup_system(setup_system)
            .add_system_to_stage(CoreStage::Last, update_debug_lines_mesh.label("draw_lines"));
        app.sub_app_mut(RenderApp)
            .insert_resource(self.clone())
            .add_render_command::<Opaque3d, DrawDebugLines>()
            .init_resource::<DebugLinePipeline>()
            .init_resource::<SpecializedPipelines<DebugLinePipeline>>()
            .add_system_to_stage(RenderStage::Extract, extract_debug_lines)
            .add_system_to_stage(RenderStage::Queue, queue_debug_lines);
        info!("Loaded debug lines plugin.");
    }
}

const DEBUG_LINES_MESH_COUNT: usize = 4;

const MAX_POINTS_PER_MESH: usize = 2_usize.pow(16);

/// Maximum number of unique lines to draw at once.
pub const MAX_LINES: usize = MAX_POINTS / 2;

/// Maximum number of points.
pub const MAX_POINTS: usize = MAX_POINTS_PER_MESH * DEBUG_LINES_MESH_COUNT;

fn spawn_debug_lines_mesh(meshes: &mut Assets<Mesh>, retain: DebugLinesMesh) -> impl Bundle {
    let is_immediate = matches!(retain, DebugLinesMesh::Immediate(_));
    (
        meshes.add(debug_lines_mesh(is_immediate)),
        Transform::default(),
        GlobalTransform::default(),
        Visibility::default(),
        ComputedVisibility::default(),
        retain,
    )
}
fn setup_system(mut cmds: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    use DebugLinesMesh::{Immediate, Retained};
    for i in 0..DEBUG_LINES_MESH_COUNT {
        cmds.spawn_bundle(spawn_debug_lines_mesh(&mut meshes, Immediate(i)));
    }
    for i in 0..DEBUG_LINES_MESH_COUNT {
        cmds.spawn_bundle(spawn_debug_lines_mesh(&mut meshes, Retained(i)));
    }
}

fn update_debug_lines_mesh(
    debug_line_meshes: Query<(&Handle<Mesh>, &DebugLinesMesh)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut lines: DebugLines,
) {
    use DebugLinesMesh::{Immediate, Retained};
    let time = lines.time.time_since_startup().as_secs_f32();
    for (mesh_handle, retain_mod) in debug_line_meshes.iter() {
        let mesh = meshes.get_mut(mesh_handle).unwrap();
        match *retain_mod {
            Immediate(i) => lines.immediate.fill_attributes(mesh, i),
            Retained(i) => lines.retained.fill_attributes(time, mesh, i),
        }
    }
    // This needs to be done after `fill_attributes` because of the immediate buffer clears
    lines.mark_expired();
}

/// Initialize [`DebugLinesMesh`]'s [`Mesh`].
fn debug_lines_mesh(is_immediate: bool) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::LineList);
    mesh.set_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::Float32x3(Vec::with_capacity(256)),
    );
    mesh.set_attribute(
        Mesh::ATTRIBUTE_COLOR,
        VertexAttributeValues::Float32x4(Vec::with_capacity(256)),
    );
    if !is_immediate {
        mesh.set_indices(Some(Indices::U16(Vec::with_capacity(256))));
    }
    mesh
}

/// Move the DebugLinesMesh marker Component to the render context.
fn extract_debug_lines(mut commands: Commands, query: Query<Entity, With<DebugLinesMesh>>) {
    for entity in query.iter() {
        commands.get_or_spawn(entity).insert(RenderDebugLinesMesh);
    }
}

/// Marker Component for the [`Entity`] associated with the meshes rendered with the
/// debuglines.wgsl shader.
///
/// Stores the index of the mesh for the logic of `ImmLinesStorage` and `RetLinesStorage`
#[derive(Component)]
enum DebugLinesMesh {
    /// Meshes for duration=0.0 lines
    Immediate(usize),
    /// Meshes for duration≠0.0 lines
    Retained(usize),
}
impl Default for DebugLinesMesh {
    fn default() -> Self {
        DebugLinesMesh::Immediate(0)
    }
}

#[derive(Component)]
struct RenderDebugLinesMesh;

// NOTE: consider this: we could just hold a Handle<Mesh> to the DebugLinesMesh
// and modify it in-place, so that there is no need to update the mesh every
// frame on top of keeping track of all those buffers in `LinesStorage`.
// However, I implemented that, and found out it was about 3 times slower in
// the `bench.rs` example. Probably the 2 levels of indirection and the error
// checking is what kills it.
//
// TODO: Use a u32 for colors, this may improve performance if decoding and
// encoding the color is not more expensive than moving 4 values in memory 3
// times
/// The [`DebugLines`] storage for immediate mod lines.
///
/// This is `pub` because of the `SystemParam` macro on [`DebugLines`]. The end
/// user **should absolutely not interact with this**.
#[derive(Debug, Default)]
#[doc(hidden)]
pub struct ImmLinesStorage {
    positions: Vec<[f32; 3]>,
    colors: Vec<[f32; 4]>,
}
impl ImmLinesStorage {
    fn add_at(&mut self, i: usize, start: Vec3, end: Vec3, start_color: Color, end_color: Color) {
        self.colors[i] = start_color.into();
        self.colors[i + 1] = end_color.into();
        self.positions[i] = start.into();
        self.positions[i + 1] = end.into();
    }
    fn push(&mut self, start: Vec3, end: Vec3, start_color: Color, end_color: Color) {
        self.colors.push(start_color.into());
        self.colors.push(end_color.into());
        self.positions.push(start.into());
        self.positions.push(end.into());
    }
    fn add_line(&mut self, start: Vec3, end: Vec3, start_color: Color, end_color: Color) {
        if self.positions.len() >= MAX_POINTS {
            self.add_at(MAX_POINTS - 2, start, end, start_color, end_color);
        } else {
            self.push(start, end, start_color, end_color);
        }
    }

    fn mark_expired(&mut self) {
        self.positions.clear();
        self.colors.clear();
    }

    fn fill_colors(&self, buffer: &mut Vec<[f32; 4]>, mesh: usize) {
        buffer.clear();
        if let Some(new_content) = self.colors.chunks(MAX_POINTS_PER_MESH).nth(mesh) {
            buffer.extend(new_content);
        }
    }

    fn fill_vertexes(&self, buffer: &mut Vec<[f32; 3]>, mesh: usize) {
        buffer.clear();
        if let Some(new_content) = self.positions.chunks(MAX_POINTS_PER_MESH).nth(mesh) {
            buffer.extend(new_content);
        }
    }

    fn fill_attributes(&self, mesh: &mut Mesh, mesh_index: usize) {
        use VertexAttributeValues::{Float32x3, Float32x4};
        if let Some(Float32x3(vbuffer)) = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
            self.fill_vertexes(vbuffer, mesh_index);
        }
        if let Some(Float32x4(cbuffer)) = mesh.attribute_mut(Mesh::ATTRIBUTE_COLOR) {
            self.fill_colors(cbuffer, mesh_index);
        }
    }
}
/// The [`DebugLines`] storage for retained mod lines.
///
/// This is `pub` because of the `SystemParam` macro on [`DebugLines`]. The end
/// user **should absolutely not interact with this**.
///
/// This holds the buffers for the mesh assigned to render the debug lines. It
/// dynamically generates the indexes to disable/enable expired lines without
/// changing the layout of the buffers.
/// The [`DebugLines`] storage.
#[derive(Debug, Default)]
#[doc(hidden)]
pub struct RetLinesStorage {
    inner: ImmLinesStorage,
    expiry: Vec<f32>,
    expired: Vec<u32>,
}
impl RetLinesStorage {
    fn add_line(
        &mut self,
        start: Vec3,
        end: Vec3,
        start_color: Color,
        end_color: Color,
        expiry: f32,
    ) {
        if let Some(replaceable) = self.expired.pop() {
            let i = replaceable as usize;
            self.inner.add_at(i, start, end, start_color, end_color);
            self.expiry[i] = expiry;
            self.expiry[i + 1] = expiry;
        } else if self.expiry.len() >= MAX_POINTS {
            self.inner
                .add_at(MAX_POINTS - 2, start, end, start_color, end_color);
            self.expiry[MAX_POINTS - 2] = expiry;
            self.expiry[MAX_POINTS - 1] = expiry;
        } else {
            self.inner.push(start, end, start_color, end_color);
            self.expiry.push(expiry);
            self.expiry.push(expiry);
        }
    }

    fn fill_indices(&self, time: f32, buffer: &mut Vec<u16>, mesh: usize) {
        buffer.clear();
        if let Some(new_content) = self.expiry.chunks(MAX_POINTS_PER_MESH).nth(mesh) {
            buffer.extend(
                new_content
                    .iter()
                    .enumerate()
                    .filter(|(_, e)| **e >= time)
                    .map(|(i, _)| i as u16),
            );
        }
    }

    fn mark_expired(&mut self, time: f32) {
        self.expired.extend(
            self.expiry
                .iter()
                .enumerate()
                .filter(|(i, e)| **e < time && i % 2 == 0)
                .map(|(i, _)| i as u32),
        );
    }

    fn fill_attributes(&self, time: f32, mesh: &mut Mesh, mesh_index: usize) {
        use VertexAttributeValues::{Float32x3, Float32x4};
        if let Some(Indices::U16(indices)) = mesh.indices_mut() {
            self.fill_indices(time, indices, mesh_index);
        }
        if let Some(Float32x3(vbuffer)) = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
            self.inner.fill_vertexes(vbuffer, mesh_index);
        }
        if let Some(Float32x4(cbuffer)) = mesh.attribute_mut(Mesh::ATTRIBUTE_COLOR) {
            self.inner.fill_colors(cbuffer, mesh_index);
        }
    }
}

/// Bevy resource providing facilities to draw lines.
///
/// # Usage
/// ```
/// use bevy::prelude::*;
/// use bevy_prototype_debug_lines::*;
///
/// // Draws 3 horizontal lines, which disappear after 1 frame.
/// fn some_system(mut lines: DebugLines) {
///     lines.line(Vec3::new(-1.0, 1.0, 0.0), Vec3::new(1.0, 1.0, 0.0), 0.0);
///     lines.line_colored(
///         Vec3::new(-1.0, 0.0, 0.0),
///         Vec3::new(1.0, 0.0, 0.0),
///         0.0,
///         Color::WHITE
///     );
///     lines.line_gradient(
///         Vec3::new(-1.0, -1.0, 0.0),
///         Vec3::new(1.0, -1.0, 0.0),
///         0.0,
///         Color::WHITE, Color::PINK
///     );
/// }
/// ```
#[derive(SystemParam)]
pub struct DebugLines<'w, 's> {
    immediate: ResMut<'w, ImmLinesStorage>,
    retained: ResMut<'w, RetLinesStorage>,
    time: Res<'w, Time>,
    #[system_param(ignore)]
    _lifetimes: PhantomData<&'s ()>,
}

impl<'w, 's> DebugLines<'w, 's> {
    /// Draw a line in world space, or update an existing line
    ///
    /// # Arguments
    ///
    /// * `start` - The start of the line in world space
    /// * `end` - The end of the line in world space
    /// * `duration` - Duration (in seconds) that the line should show for -- a value of
    ///   zero will show the line for 1 frame.
    pub fn line(&mut self, start: Vec3, end: Vec3, duration: f32) {
        self.line_colored(start, end, duration, Color::WHITE);
    }

    /// Draw a line in world space with a specified color, or update an existing line
    ///
    /// # Arguments
    ///
    /// * `start` - The start of the line in world space
    /// * `end` - The end of the line in world space
    /// * `duration` - Duration (in seconds) that the line should show for -- a value of
    ///   zero will show the line for 1 frame.
    /// * `color` - Line color
    pub fn line_colored(&mut self, start: Vec3, end: Vec3, duration: f32, color: Color) {
        self.line_gradient(start, end, duration, color, color);
    }

    /// Draw a line in world space with a specified gradient color, or update an existing line
    ///
    /// # Arguments
    ///
    /// * `start` - The start of the line in world space
    /// * `end` - The end of the line in world space
    /// * `duration` - Duration (in seconds) that the line should show for -- a value of
    ///   zero will show the line for 1 frame.
    /// * `start_color` - Line color
    /// * `end_color` - Line color
    pub fn line_gradient(
        &mut self,
        start: Vec3,
        end: Vec3,
        duration: f32,
        start_color: Color,
        end_color: Color,
    ) {
        if duration == 0.0 {
            self.immediate.add_line(start, end, start_color, end_color);
        } else {
            let expiry = self.time.time_since_startup().as_secs_f32() + duration;
            self.retained
                .add_line(start, end, start_color, end_color, expiry);
        }
    }

    fn mark_expired(&mut self) {
        let time = self.time.time_since_startup().as_secs_f32();
        self.immediate.mark_expired();
        self.retained.mark_expired(time);
    }
}

struct DebugLinePipeline {
    mesh_pipeline: MeshPipeline,
    shader: Handle<Shader>,
    always_on_top: bool,
}
impl FromWorld for DebugLinePipeline {
    fn from_world(render_world: &mut World) -> Self {
        let dbl_plugin = render_world.get_resource::<DebugLinesPlugin>().unwrap();
        DebugLinePipeline {
            mesh_pipeline: render_world.get_resource::<MeshPipeline>().unwrap().clone(),
            shader: DEBUG_LINES_SHADER_HANDLE.typed(),
            always_on_top: dbl_plugin.always_on_top,
        }
    }
}

impl SpecializedPipeline for DebugLinePipeline {
    type Key = MeshPipelineKey;

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        use VertexFormat::{Float32x3, Float32x4};
        let mut descriptor = self.mesh_pipeline.specialize(key);
        descriptor.vertex.shader = self.shader.clone_weak();
        descriptor.vertex.buffers[0] = VertexBufferLayout {
            // NOTE: I've no idea why, but `color` is at offset zero and
            // `position` at 4*4. Swapping breaks everything
            array_stride: 4 * 4 + 4 * 3, // sizeof(Float32x4) + sizeof(Float32x3)
            step_mode: VertexStepMode::Vertex,
            attributes: vec![
                VertexAttribute {
                    // Vertex.color
                    format: Float32x4,
                    offset: 0,
                    shader_location: 0,
                },
                VertexAttribute {
                    // Vertex.place (position)
                    format: Float32x3,
                    offset: 4 * 4, // sizeof(Float32x4)
                    shader_location: 1,
                },
            ],
        };
        descriptor.fragment.as_mut().unwrap().shader = self.shader.clone_weak();
        descriptor.primitive.topology = PrimitiveTopology::LineList;
        descriptor.primitive.cull_mode = None;
        let depth_rate = if self.always_on_top { 10000.0 } else { 1.0 };
        descriptor.depth_stencil.as_mut().unwrap().bias.slope_scale = depth_rate;
        descriptor
    }
}

fn queue_debug_lines(
    opaque_3d_draw_functions: Res<DrawFunctions<Opaque3d>>,
    debug_line_pipeline: Res<DebugLinePipeline>,
    mut pipeline_cache: ResMut<RenderPipelineCache>,
    mut specialized_pipelines: ResMut<SpecializedPipelines<DebugLinePipeline>>,
    msaa: Res<Msaa>,
    material_meshes: Query<(Entity, &MeshUniform), With<RenderDebugLinesMesh>>,
    mut views: Query<(&ExtractedView, &mut RenderPhase<Opaque3d>)>,
) {
    let draw_custom = opaque_3d_draw_functions
        .read()
        .get_id::<DrawDebugLines>()
        .unwrap();
    let key = MeshPipelineKey::from_msaa_samples(msaa.samples);
    for (view, mut transparent_phase) in views.iter_mut() {
        let view_matrix = view.transform.compute_matrix();
        let view_row_2 = view_matrix.row(2);

        let add_render_phase = |(entity, mesh_uniform): (Entity, &MeshUniform)| {
            transparent_phase.add(Opaque3d {
                entity,
                pipeline: specialized_pipelines.specialize(
                    &mut pipeline_cache,
                    &debug_line_pipeline,
                    key,
                ),
                draw_function: draw_custom,
                distance: view_row_2.dot(mesh_uniform.transform.col(3)),
            });
        };
        material_meshes.iter().for_each(add_render_phase);
    }
}

type DrawDebugLines = (
    SetItemPipeline,
    SetMeshViewBindGroup<0>,
    SetMeshBindGroup<1>,
    DrawMesh,
);
