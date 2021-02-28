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
        points: vec![Vec4::zero(); MAX_POINTS],
        colors: vec![Color::WHITE; MAX_POINTS],
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
    // I don't love having 2 buffers here.  It would be cleaner if we can do a custom line structure.
    // We should also consider the memory imprint here.  We should maybe instead allow a predefined
    // set of colors which would dramatically reduce that.
    #[render_resources(buffer)]
    pub points: Vec<Vec4>,
    #[render_resources(buffer)]
    pub colors: Vec<Color>,
}

pub struct Line {
    start: Vec3,
    end: Vec3,
    color: Color,
    thickness: f32,
}

impl Line {
    pub fn new(start: Vec3, end: Vec3, thickness: f32, color: Color) -> Self {
        Self { start, end, color, thickness }
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
    /// Draw a line in world space, or update an existing line
    ///
    /// # Arguments
    ///
    /// * `id` - A unique identifier for the line
    /// * `start` - The start of the line in world space
    /// * `end` - The end of the line in world space
    /// * `thickness` - Line thickness
    pub fn line(&mut self, id: u8, start: Vec3, end: Vec3, thickness: f32) {
        self.line_colored(id, start, end, thickness, Color::WHITE);
    }

    /// Draw a line in world space with a specified color, or update an existing line
    ///
    /// # Arguments
    ///
    /// * `id` - A unique identifier for the line
    /// * `start` - The start of the line in world space
    /// * `end` - The end of the line in world space
    /// * `thickness` - Line thickness
    /// * `color` - Line color
    pub fn line_colored(&mut self, id: u8, start: Vec3, end: Vec3, thickness: f32, color: Color) {
        let line = self.lines.get_mut(&id);
        if let Some(line) = line {
            line.start = start;
            line.end = end;
            line.thickness = thickness;
            line.color = color;
            self.dirty = true;
        } else {
            self.lines.insert(id, Line::new(start, end, thickness, color));
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
                // point.w property is used for thickness.
                shader.points[i] = line.start.extend(line.thickness);
                shader.points[i+1] = line.end.extend(line.thickness);
                shader.colors[i] = line.color;
                shader.colors[i+1] = line.color;

                i += 2;
            }
        }
    }

    lines.dirty = false;
}
