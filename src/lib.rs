#![allow(dead_code)]

use bevy::{prelude::*, render::pipeline::{CullMode, RasterizationStateDescriptor}};
use bevy::render::mesh::{VertexAttributeValues, Indices};
use bevy::render::pipeline::{PrimitiveTopology, PipelineDescriptor, RenderPipeline };
use bevy::render::shader::{ShaderStages, ShaderStage};
use bevy::render::render_graph::{AssetRenderResourcesNode, RenderGraph};
use bevy::render::render_graph::base;
use bevy::render::renderer::RenderResources;
use bevy::reflect::TypeUuid;

use std::collections::HashMap;

pub struct DebugLinesPlugin;
impl Plugin for DebugLinesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_asset::<LineShader>()
            .init_resource::<DebugLines>()
            .add_startup_system(setup.system())
            .add_system(draw_lines.system());
    }
}

const MAX_LINES: usize = 128;
const MAX_POINTS: usize = MAX_LINES * 2;
const VERTICES_PER_NODE: usize = 4;
const TRIANGLES_PER_NODE: usize = 4;

fn create_mesh() -> Mesh {

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let positions = [[0.0, 0.0, 0.0]; MAX_POINTS * VERTICES_PER_NODE];
    let mut indices = [0; MAX_POINTS * TRIANGLES_PER_NODE * 3];
    for i in 0..MAX_POINTS {
        let idx: usize = i * TRIANGLES_PER_NODE * 3;
        let v_idx: u32 = (i * VERTICES_PER_NODE) as u32;

        // Bevy uses a COUNTER-CLOCKWISE WINDING ORDER -- counter-clickwise tri indices are facing the camera.
        indices[idx + 0] = v_idx;           // v1 top
        indices[idx + 1] = v_idx + 1;       // v1 bottom
        indices[idx + 2] = v_idx + 3;       // v2 bottom
        indices[idx + 3] = v_idx;           // v1 top
        indices[idx + 4] = v_idx + 3;       // v2 bottom
        indices[idx + 5] = v_idx + 2;       // v2 top

        // If we want circle caps, fix this.
        //indices[idx + 6] = v_idx + 4;       // tl
        //indices[idx + 7] = v_idx + 7;       // br
        //indices[idx + 8] = v_idx + 6;       // bl
        //indices[idx + 9] = v_idx + 4;       // tl
        //indices[idx +10] = v_idx + 5;       // tr
        //indices[idx +11] = v_idx + 7;       // br
    }

    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION,VertexAttributeValues::Float3(positions.into()));
    mesh.set_indices(Some(Indices::U32(indices.into())));

    mesh
}

pub fn setup(
    commands: &mut Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<LineShader>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    let mut p = 
        PipelineDescriptor::default_config(
            ShaderStages {
                vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, include_str!("line.vert"))),
                fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, include_str!("line.frag")))),
            }
        );

    // Disable backface culling (enable two sided rendering).
    p.rasterization_state = Some(
        RasterizationStateDescriptor {
            cull_mode: CullMode::None,
            ..Default::default()
        }
    );

    // Create new shader pipeline.
    let pipeline_handle = pipelines.add(p);

    render_graph.add_system_node(
        "line_shader",
        AssetRenderResourcesNode::<LineShader>::new(false),
    );

    render_graph
        .add_node_edge("line_shader", base::node::MAIN_PASS)
        .unwrap();


    let pipes = RenderPipelines::from_pipelines(vec![RenderPipeline::new(pipeline_handle)]);

    let mesh = create_mesh();
    let shader = materials.add(LineShader {
        // TODO: need to use MAX_POINTS for this.
        points: vec![Vec4::zero(); MAX_POINTS],
    });

    commands
        .spawn(MeshBundle {
            mesh: meshes.add(mesh),
            render_pipelines: pipes,
            transform: Transform::from_translation(Vec3::zero()),
            ..Default::default()
        })
        .with(shader);

    info!("Loaded debug lines plugin.");
}

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "f093e7c5-634c-45f8-a2af-7fcd0245f259"]
pub struct LineShader {
    #[render_resources(buffer)]
    pub points: Vec<Vec4>,
}

pub struct Line {
    start: Vec3,
    end: Vec3,
}

impl Line {
    pub fn new(start: Vec3, end: Vec3) -> Self {
        Self { start, end }
    }
}

pub struct DebugLines {
    pub lines: HashMap<u8, Line>,
    pub dirty: bool,
}

impl Default for DebugLines {
    fn default() -> Self {
        Self {
            lines: HashMap::new(),
            dirty: false,
        }
    }
}

impl DebugLines {
    pub fn add_line(&mut self, id: u8, line: Line) {
        let result = self.lines.insert(id, line);
        if result.is_none() {
            self.dirty = true;
        }
    }

    pub fn add_or_update_line(&mut self, id: u8, start: Vec3, end: Vec3) {
        let line = self.lines.get_mut(&id);
        if let Some(line) = line {
            line.start = start;
            line.end = end;
            self.dirty = true;
        } else {
            self.lines.insert(id, Line::new(start, end));
        }
    }

    pub fn set_dirty(&mut self) {
        self.dirty = true;
    }
}

pub fn draw_lines(
    mut assets: ResMut<Assets<LineShader>>,
    mut lines: ResMut<DebugLines>,
    query: Query<&Handle<LineShader>>,
) {
    // One line changing makes us update all lines.
    // We can probably resolve this is it becomes a problem -- consider creating a number of "Line" entities to
    // split up the processing.
    if !lines.dirty {
        return;
    }

    for line_handle in query.iter() {
        if let Some(shader) = assets.get_mut(line_handle) {
            let mut i = 0;
            for (_id, line) in &lines.lines {
                // First point is start of line, second is end.
                shader.points[i] = line.start.extend(0.0);
                shader.points[i+1] = line.end.extend(0.0);

                i += 2;
            }
        }
    }

    lines.dirty = false;
}

fn line_demo(
    mut lines: ResMut<DebugLines>,
    time: Res<Time>,
) {
    let x = 1.0 + (time.seconds_since_startup() as f32).sin();
    lines.add_or_update_line(0, Vec3::new(-x, -0.5, 0.0), Vec3::new(x, 0.5, 0.0));
}


// Screen to world point code.
/*
pub fn draw_lines(
    mut state: ResMut<State>,
    ev_cursor: Res<Events<CursorMoved>>,
    mut assets: ResMut<Assets<LineShader>>,
    windows: Res<Windows>,
    mut line_query: Query<(&mut DebugLines, &Handle<LineShader>)>,
    camera_query: Query<(&Transform, &PerspectiveProjection, &Camera)>,
) {
    // Find a camera and transform.
    let mut camera = None;
    let mut perspective = None;
    let mut transform = None;
    for (t, p, c) in camera_query.iter() {
        camera = Some(c);
        transform = Some(t);
        perspective = Some(p);
        break;
    }

    let window = windows.get(WindowId::primary()).unwrap();
    let screen_size = Vec2::from([window.width() as f32, window.height() as f32]);

    for (mut _line, handle) in line_query.iter_mut() {
        for ev in state.cursor.iter(&ev_cursor) {
            let pos = ev.position;
            let shader = assets.get_mut(handle).unwrap();
            let real_pos = utility::screen_to_world_coord(&pos, 0.0, &screen_size, perspective.unwrap(), camera.unwrap(), transform.unwrap());
            shader.points[1] = Vec4::new(real_pos.x(), real_pos.y(), real_pos.z(), 1.0);
        }
    }
}
*/
