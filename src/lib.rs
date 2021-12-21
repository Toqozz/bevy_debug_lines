use std::marker::PhantomData;

use bevy::{
    asset::{Assets, Handle, HandleUntyped},
    core_pipeline::Opaque3d,
    ecs::{reflect::ReflectComponent, system::SystemParam},
    pbr::{
        DrawMesh, MeshPipeline, MeshPipelineKey, MeshUniform, SetMeshBindGroup,
        SetMeshViewBindGroup,
    },
    prelude::*,
    reflect::{Reflect, TypeUuid},
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
#[derive(Debug, Default)]
pub struct DebugLinesPlugin;
impl Plugin for DebugLinesPlugin {
    fn build(&self, app: &mut App) {
        let mut shaders = app.world.get_resource_mut::<Assets<Shader>>().unwrap();
        shaders.set_untracked(
            DEBUG_LINES_SHADER_HANDLE,
            Shader::from_wgsl(include_str!("debuglines.wgsl")),
        );
        app.init_resource::<LinesStorage>();
        app.add_startup_system(setup_system)
            .add_system_to_stage(CoreStage::Last, update_debug_lines_mesh.label("draw_lines"));
        app.sub_app(RenderApp)
            .add_render_command::<Opaque3d, DrawDebugLines>()
            .init_resource::<DebugLinePipeline>()
            .init_resource::<SpecializedPipelines<DebugLinePipeline>>()
            .add_system_to_stage(RenderStage::Extract, extract_debug_lines)
            .add_system_to_stage(RenderStage::Queue, queue_debug_lines);
        info!("Loaded debug lines plugin.");
    }
}

/// Maximum number of unique lines to draw at once.
pub const MAX_LINES: usize = MAX_POINTS / 2;
/// Maximum number of points.
pub const MAX_POINTS: usize = 2_usize.pow(18);

const MAX_POINTS_PER_MESH: usize = 2_usize.pow(16);

fn setup_system(mut cmds: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    for _ in 0..4 {
        cmds.spawn_bundle((
            meshes.add(debug_lines_mesh()),
            Transform::default(),
            GlobalTransform::default(),
            Visibility::default(),
            ComputedVisibility::default(),
            DebugLinesMesh,
        ));
    }
}

fn update_debug_lines_mesh(
    debug_line_mesh: Query<&Handle<Mesh>, With<DebugLinesMesh>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut debug_lines: DebugLines,
) {
    debug_lines.mark_expired();
    for (i, mesh_handle) in debug_line_mesh.iter().enumerate() {
        let mesh = meshes.get_mut(mesh_handle).unwrap();
        debug_lines.fill_mesh_attributes(mesh, i);
    }
}

/// Initialize [`DebugLinesMesh`]'s [`Mesh`].
fn debug_lines_mesh() -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::LineList);
    mesh.set_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::Float32x3(Vec::with_capacity(256)),
    );
    mesh.set_attribute(
        Mesh::ATTRIBUTE_COLOR,
        VertexAttributeValues::Float32x4(Vec::with_capacity(256)),
    );
    mesh.set_indices(Some(Indices::U16(Vec::with_capacity(256))));
    mesh
}

/// Move the DebugLinesMesh marker Compnent to the render context.
fn extract_debug_lines(mut commands: Commands, query: Query<Entity, With<DebugLinesMesh>>) {
    for entity in query.iter() {
        commands.get_or_spawn(entity).insert(DebugLinesMesh);
    }
}

/// Marker Component to signle out the [`Entity`] associated with the mesh
/// rendered with the debuglines.wgsl shader.
#[derive(Component, Debug, Clone, Default, Reflect)]
#[reflect(Component)]
struct DebugLinesMesh;

// TODO: consider this: we could just hold a Handle<Mesh> to the DebugLinesMesh
// and modify it in-place, so that there is no need to update the mesh every
// frame on top of keeping track of all those buffers in `LinesStorage`.
// However, I implemented that, and found out it was about 3 times slower in
// the `bench.rs` example. Probably the 2 levels of indirection and the error
// checking is what kills it.
//
// TODO: Use a u32 for colors, this may improve performance if decoding and
// encoding the color is not more expensive than moving 4 values in memory 3
// times
//
// TODO: separate "retained mod" and "immediate mod" lines. Since the retained
// mod case is comparatively rare to the more normal duration=0.0, we should
// treat it as special and make the happy path as fast as possible (notably
// removing the duration check, and just clearing the buffers)
//
// TODO: add a `segment` method that let you define a line with several
// points, this would enable sparing a lot of space using indexing, we could
// store those as a `segments` field maybe?
/// The [`DebugLines`] storage.
///
/// This is `pub` because of the `SystemParam` macro on [`DebugLines`]. The end
/// user **should absolutely not interact with this**.
///
/// This holds the buffers for the mesh assigned to render the debug lines. It
/// dynamically generates the indexes to disable/enable expired lines without
/// changing the layout of the buffers.
/// The [`DebugLines`] storage.
///
/// This is `pub` because of the `SystemParam` macro on [`DebugLines`]. The end
/// user **should absolutely not interact with this**.
#[derive(Debug, Default)]
#[doc(hidden)]
pub struct LinesStorage {
    positions: Vec<[f32; 3]>,
    colors: Vec<[f32; 4]>,
    expiry: Vec<f32>,
    expired: Vec<u32>,
}
impl LinesStorage {
    fn add_line(
        &mut self,
        start: Vec3,
        end: Vec3,
        start_color: Color,
        end_color: Color,
        expiry: f32,
    ) {
        // There is no risk of index missalginment, because we only
        // pop_front/push_back if there are no replaceable left to replace
        if let Some(replaceable) = self.expired.pop() {
            let i = replaceable as usize;
            self.colors[i] = start_color.into();
            self.colors[i + 1] = end_color.into();
            self.positions[i] = start.into();
            self.positions[i + 1] = end.into();
            self.expiry[i] = expiry;
            self.expiry[i + 1] = expiry;
        } else if self.expiry.len() >= MAX_POINTS {
            self.colors[MAX_POINTS - 2] = start_color.into();
            self.colors[MAX_POINTS - 1] = end_color.into();
            self.positions[MAX_POINTS - 2] = start.into();
            self.positions[MAX_POINTS - 1] = end.into();
            self.expiry[MAX_POINTS - 2] = expiry;
            self.expiry[MAX_POINTS - 1] = expiry;
        } else {
            self.colors.push(start_color.into());
            self.colors.push(end_color.into());
            self.positions.push(start.into());
            self.positions.push(end.into());
            self.expiry.push(expiry);
            self.expiry.push(expiry);
        }
    }

    fn fill_indices(&mut self, time: f32, buffer: &mut Vec<u16>, mesh: usize) {
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
    lines: ResMut<'w, LinesStorage>,
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
        let expiry = self.time.time_since_startup().as_secs_f32() + duration;
        self.lines
            .add_line(start, end, start_color, end_color, expiry);
    }

    fn mark_expired(&mut self) {
        let time = self.time.time_since_startup().as_secs_f32();
        self.lines.mark_expired(time);
    }

    fn fill_mesh_attributes(&mut self, mesh: &mut Mesh, mesh_index: usize) {
        use VertexAttributeValues::{Float32x3, Float32x4};
        let time = self.time.time_since_startup().as_secs_f32();
        if let Some(Indices::U16(indices)) = mesh.indices_mut() {
            self.lines.fill_indices(time, indices, mesh_index);
        }
        if let Some(Float32x3(vbuffer)) = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
            self.lines.fill_vertexes(vbuffer, mesh_index);
        }
        if let Some(Float32x4(cbuffer)) = mesh.attribute_mut(Mesh::ATTRIBUTE_COLOR) {
            self.lines.fill_colors(cbuffer, mesh_index);
        }
    }
}

struct DebugLinePipeline {
    mesh_pipeline: MeshPipeline,
    shader: Handle<Shader>,
}
impl FromWorld for DebugLinePipeline {
    fn from_world(render_world: &mut World) -> Self {
        DebugLinePipeline {
            mesh_pipeline: render_world.get_resource::<MeshPipeline>().unwrap().clone(),
            shader: DEBUG_LINES_SHADER_HANDLE.typed(),
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
        // TODO: set this to None to remove depth check
        descriptor.depth_stencil.as_mut().unwrap().bias.slope_scale = 1.0;
        descriptor
    }
}

fn queue_debug_lines(
    opaque_3d_draw_functions: Res<DrawFunctions<Opaque3d>>,
    debug_line_pipeline: Res<DebugLinePipeline>,
    mut pipeline_cache: ResMut<RenderPipelineCache>,
    mut specialized_pipelines: ResMut<SpecializedPipelines<DebugLinePipeline>>,
    msaa: Res<Msaa>,
    material_meshes: Query<(Entity, &MeshUniform), With<DebugLinesMesh>>,
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
