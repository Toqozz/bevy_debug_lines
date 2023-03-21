use bevy::{
    asset::{Assets, HandleUntyped},
    pbr::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::{/*Indices,*/ Mesh, VertexAttributeValues},
        render_phase::AddRenderCommand,
        render_resource::PrimitiveTopology,
        render_resource::Shader,
        view::{NoFrustumCulling, RenderLayers},
        Extract,
    },
};
use shapes::AddLines;

#[cfg(feature = "shapes")]
pub use crate::shapes::DebugShapes;

#[cfg(feature = "shapes")]
pub mod shapes;

mod render_dim;

// This module exists to "isolate" the `#[cfg]` attributes to this part of the
// code. Otherwise, we would pollute the code with a lot of feature
// gates-specific code.
#[cfg(feature = "3d")]
mod dim {
    pub(crate) use crate::render_dim::r3d::{queue, DebugLinePipeline, DrawDebugLines};
    pub(crate) use bevy::core_pipeline::core_3d::Opaque3d as Phase;
    use bevy::{asset::Handle, render::mesh::Mesh};

    pub(crate) type MeshHandle = Handle<Mesh>;
    pub(crate) fn from_handle(from: &MeshHandle) -> &Handle<Mesh> {
        from
    }
    pub(crate) fn into_handle(from: Handle<Mesh>) -> MeshHandle {
        from
    }
    pub(crate) const SHADER_FILE: &str = include_str!("debuglines.wgsl");
    pub(crate) const DIMMENSION: &str = "3d";
}
#[cfg(not(feature = "3d"))]
mod dim {
    pub(crate) use crate::render_dim::r2d::{queue, DebugLinePipeline, DrawDebugLines};
    pub(crate) use bevy::core_pipeline::core_2d::Transparent2d as Phase;
    use bevy::{asset::Handle, render::mesh::Mesh, sprite::Mesh2dHandle};

    pub(crate) type MeshHandle = Mesh2dHandle;
    pub(crate) fn from_handle(from: &MeshHandle) -> &Handle<Mesh> {
        &from.0
    }
    pub(crate) fn into_handle(from: Handle<Mesh>) -> MeshHandle {
        Mesh2dHandle(from)
    }
    pub(crate) const SHADER_FILE: &str = include_str!("debuglines2d.wgsl");
    pub(crate) const DIMMENSION: &str = "2d";
}

// See debuglines.wgsl for explanation on 2 shaders.
//pub(crate) const SHADER_FILE: &str = include_str!("debuglines.wgsl");
pub(crate) const DEBUG_LINES_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 17477439189930443325);

#[derive(Resource)]
pub(crate) struct DebugLinesConfig {
    depth_test: bool,
}

#[derive(Resource)]
pub(crate) struct DebugLinesRenderLayer {
    render_layers: Vec<u8>,
}

/// The `SystemSet` in which the debug lines update system runs.
///
/// This set is nested in `CoreSet::PostUpdate`, so it runs after all update systems.
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum DebugLinesSet {
    DrawLines,
}

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
///     .add_plugin(DebugLinesPlugin::default())
///     .run();
/// ```
///
/// Alternatively, you can initialize the plugin with depth testing, so that
/// debug lines cut through geometry. To do this, use [`DebugLinesPlugin::with_depth_test(true)`].
/// ```
/// use bevy::prelude::*;
/// use bevy_prototype_debug_lines::*;
///
/// App::new()
///     .add_plugins(DefaultPlugins)
///     .add_plugin(DebugLinesPlugin::with_depth_test(true))
///     .run();
/// ```
#[derive(Debug, Clone)]
pub struct DebugLinesPlugin {
    depth_test: bool,
    render_layers: Vec<u8>,
}
impl Default for DebugLinesPlugin {
    fn default() -> Self {
        Self {
            depth_test: false,
            render_layers: vec![0], // All entitities are renderered in layer 0 if not otherwise specified.
        }
    }
}

impl DebugLinesPlugin {
    /// Controls whether debug lines should be drawn with depth testing enabled
    /// or disabled.
    ///
    /// # Arguments
    ///
    /// * `val` - True if lines should intersect with other geometry, or false
    ///   if lines should always draw on top be drawn on top (the default).
    pub fn with_depth_test(val: bool) -> Self {
        Self {
            depth_test: val,
            ..default()
        }
    }
}

impl Plugin for DebugLinesPlugin {
    fn build(&self, app: &mut App) {
        use bevy::render::{render_resource::SpecializedMeshPipelines, RenderApp, RenderSet};
        let mut shaders = app.world.get_resource_mut::<Assets<Shader>>().unwrap();
        shaders.set_untracked(
            DEBUG_LINES_SHADER_HANDLE,
            Shader::from_wgsl(dim::SHADER_FILE),
        );

        app.init_resource::<DebugLines>();

        #[cfg(feature = "shapes")]
        app.init_resource::<DebugShapes>();

        app.insert_resource(DebugLinesRenderLayer {
            render_layers: self.render_layers.to_owned(),
        });

        app.add_startup_system(setup).add_system(
            update
                .in_base_set(CoreSet::PostUpdate)
                .in_set(DebugLinesSet::DrawLines),
        );

        app.sub_app_mut(RenderApp)
            .add_render_command::<dim::Phase, dim::DrawDebugLines>()
            .insert_resource(DebugLinesConfig {
                depth_test: self.depth_test,
            })
            .init_resource::<dim::DebugLinePipeline>()
            .init_resource::<SpecializedMeshPipelines<dim::DebugLinePipeline>>()
            .add_system(extract.in_schedule(ExtractSchedule))
            .add_system(dim::queue.in_set(RenderSet::Queue));

        info!("Loaded {} debug lines plugin.", dim::DIMMENSION);
    }
}

// Number of meshes to separate line buffers into.
// We don't really do culling currently but this is a gateway to that.
const MESH_COUNT: usize = 4;
// Maximum number of points for each individual mesh.
const MAX_POINTS_PER_MESH: usize = 2_usize.pow(16);
const _MAX_LINES_PER_MESH: usize = MAX_POINTS_PER_MESH / 2;
/// Maximum number of points.
pub const MAX_POINTS: usize = MAX_POINTS_PER_MESH * MESH_COUNT;
/// Maximum number of unique lines to draw at once.
pub const MAX_LINES: usize = MAX_POINTS / 2;

fn setup(mut cmds: Commands, mut meshes: ResMut<Assets<Mesh>>, config: Res<DebugLinesRenderLayer>) {
    // Spawn a bunch of meshes to use for lines.
    for i in 0..MESH_COUNT {
        // Create a new mesh with the number of vertices we need.
        let mut mesh = Mesh::new(PrimitiveTopology::LineList);
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            VertexAttributeValues::Float32x3(Vec::with_capacity(MAX_POINTS_PER_MESH)),
        );
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_COLOR,
            VertexAttributeValues::Float32x4(Vec::with_capacity(MAX_POINTS_PER_MESH)),
        );
        // https://github.com/Toqozz/bevy_debug_lines/issues/16
        //mesh.set_indices(Some(Indices::U16(Vec::with_capacity(MAX_POINTS_PER_MESH))));

        cmds.spawn((
            dim::into_handle(meshes.add(mesh)),
            NotShadowCaster,
            NotShadowReceiver,
            Transform::default(),
            GlobalTransform::default(),
            Visibility::default(),
            ComputedVisibility::default(),
            DebugLinesMesh(i),
            NoFrustumCulling, // disable frustum culling
            RenderLayers::from_layers(config.render_layers.as_slice()),
        ));
    }
}

fn update(
    debug_line_meshes: Query<(&dim::MeshHandle, &DebugLinesMesh)>,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut lines: ResMut<DebugLines>,
    mut shapes: ResMut<DebugShapes>,
) {
    // Add lines from shapes
    #[cfg(feature = "shapes")]
    {
        for shape in &shapes.shapes {
            shape.add_lines(&mut lines);
        }
        shapes.shapes.clear();
    }

    // For each debug line mesh, fill its buffers with the relevant positions/colors chunks.
    for (mesh_handle, debug_lines_idx) in debug_line_meshes.iter() {
        let mesh = meshes.get_mut(dim::from_handle(mesh_handle)).unwrap();
        use VertexAttributeValues::{Float32x3, Float32x4};
        if let Some(Float32x3(vbuffer)) = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
            vbuffer.clear();
            if lines.enabled {
                if let Some(new_content) = lines
                    .positions
                    .chunks(MAX_POINTS_PER_MESH)
                    .nth(debug_lines_idx.0)
                {
                    vbuffer.extend(new_content);
                }
            }
        }

        if let Some(Float32x4(cbuffer)) = mesh.attribute_mut(Mesh::ATTRIBUTE_COLOR) {
            cbuffer.clear();
            if lines.enabled {
                if let Some(new_content) = lines
                    .colors
                    .chunks(MAX_POINTS_PER_MESH)
                    .nth(debug_lines_idx.0)
                {
                    cbuffer.extend(new_content);
                }
            }
        }

        /*
        // https://github.com/Toqozz/bevy_debug_lines/issues/16
        if let Some(Indices::U16(indices)) = mesh.indices_mut() {
            indices.clear();
            if let Some(new_content) = lines.durations.chunks(_MAX_LINES_PER_MESH).nth(debug_lines_idx.0) {
                indices.extend(
                    new_content.iter().enumerate().map(|(i, _)| i as u16).flat_map(|i| [i * 2, i*2 + 1])
                );
            }
        }
        */
    }

    // Processes stuff like getting rid of expired lines and stuff.
    lines.update(time.delta_seconds());
}

/// Move the DebugLinesMesh marker Component to the render context.
fn extract(mut commands: Commands, query: Extract<Query<Entity, With<DebugLinesMesh>>>) {
    for entity in query.iter() {
        commands.get_or_spawn(entity).insert(RenderDebugLinesMesh);
    }
}

/// Marker component for the debug lines mesh in the world.
#[derive(Component)]
pub struct DebugLinesMesh(usize);

#[derive(Component)]
struct RenderDebugLinesMesh;

/// Bevy resource providing facilities to draw lines.
///
/// # Usage
/// ```
/// use bevy::prelude::*;
/// use bevy_prototype_debug_lines::*;
///
/// // Draws 3 horizontal lines, which disappear after 1 frame.
/// fn some_system(mut lines: ResMut<DebugLines>) {
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
#[derive(Resource)]
pub struct DebugLines {
    pub enabled: bool,
    pub positions: Vec<[f32; 3]>,
    pub colors: Vec<[f32; 4]>,
    pub durations: Vec<f32>,
}

impl Default for DebugLines {
    fn default() -> Self {
        Self {
            enabled: true,
            positions: vec![],
            colors: vec![],
            durations: vec![],
        }
    }
}

impl DebugLines {
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
        if self.positions.len() >= MAX_POINTS {
            warn!("Tried to add a new line when existing number of lines was already at maximum, ignoring.");
            return;
        }

        self.positions.push(start.into());
        self.positions.push(end.into());
        self.colors.push(start_color.as_linear_rgba_f32());
        self.colors.push(end_color.as_linear_rgba_f32());
        self.durations.push(duration);
    }

    // Returns the indices of the start and end positions of the nth line.
    // The indices can also be used to access color data.
    fn nth(&self, idx: usize) -> (usize, usize) {
        let i = idx * 2;
        (i, i + 1)
    }

    // Prepare [`ImmediateLinesStorage`] and [`RetainedLinesStorage`] for next
    // frame.
    // This clears the immediate mod buffers and tells the retained mode
    // buffers to recompute expired lines list.
    fn update(&mut self, dt: f32) {
        // TODO: an actual line counter wouldn't hurt.
        let mut i = 0;
        let mut len = self.durations.len();
        while i != len {
            self.durations[i] -= dt;
            // <= instead of < is fine here because this is always called AFTER sending the
            // data to the mesh, so we're guaranteed at least a frame here.
            if self.durations[i] <= 0.0 {
                let (cur_s, cur_e) = self.nth(i);
                let (last_s, last_e) = self.nth(len - 1);
                self.positions.swap(cur_s, last_s);
                self.positions.swap(cur_e, last_e);
                self.colors.swap(cur_s, last_s);
                self.colors.swap(cur_e, last_e);
                self.durations.swap(i, len - 1);
                len -= 1;
            } else {
                i += 1;
            }
        }

        self.positions.truncate(len * 2);
        self.colors.truncate(len * 2);
        self.durations.truncate(len);
    }
}
