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

pub const MAX_LINES: usize = 128000;
const MAX_POINTS: usize = MAX_LINES * 2;
const VERTICES_PER_LINE: usize = 4;
const TRIANGLES_PER_LINE: usize = 2;

fn create_mesh() -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let positions = vec![[0.0, 0.0, 0.0]; MAX_LINES * VERTICES_PER_LINE];
    let mut indices = vec![0; MAX_LINES * TRIANGLES_PER_LINE * 3];
    // For each line, we set up indices to make a rectangle out of 2 triangles, connecting the points.
    for i in 0..MAX_LINES {
        // Index of first triangle for this line.
        let idx: usize = i * TRIANGLES_PER_LINE * 3;
        // Index of first vertex of this line.
        let v_idx: u32 = (i * VERTICES_PER_LINE) as u32;

        // Bevy uses a COUNTER-CLOCKWISE WINDING ORDER -- counter-clickwise tri indices are facing the camera.
        indices[idx + 0] = v_idx;           // v1 top
        indices[idx + 1] = v_idx + 1;       // v1 bottom
        indices[idx + 2] = v_idx + 3;       // v2 bottom
        indices[idx + 3] = v_idx;           // v1 top
        indices[idx + 4] = v_idx + 3;       // v2 bottom
        indices[idx + 5] = v_idx + 2;       // v2 top
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
        num_lines: 0,
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
    pub num_lines: u32, // max number of lines: 65535
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
    color: [Color; 2],
    thickness: f32,
}

impl Line {
    pub fn new(start: Vec3, end: Vec3, thickness: f32, start_color: Color, end_color: Color) -> Self {
        Self { start, end, color: [start_color, end_color], thickness }
    }
}

pub struct DebugLines {
    pub lines: HashMap<u32, Line>,
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
    pub fn line(&mut self, id: u32, start: Vec3, end: Vec3, thickness: f32) {
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
    pub fn line_colored(&mut self, id: u32, start: Vec3, end: Vec3, thickness: f32, color: Color) {
        let line = self.lines.get_mut(&id);
        if let Some(line) = line {
            line.start = start;
            line.end = end;
            line.thickness = thickness;
            line.color = [color, color];
            self.dirty = true;
        } else {
            self.lines.insert(id, Line::new(start, end, thickness, color, color));
        }
    }

    pub fn line_gradient(&mut self, id: u32, start: Vec3, end: Vec3, thickness: f32, start_color: Color, end_color: Color) {
        let line = self.lines.get_mut(&id);
        if let Some(line) = line {
            line.start = start;
            line.end = end;
            line.thickness = thickness;
            line.color = [start_color, end_color];
            self.dirty = true;
        } else {
            self.lines.insert(id, Line::new(start, end, thickness, start_color, end_color));
        }
    }

    pub fn set_dirty(&mut self) {
        self.dirty = true;
    }

}

use std::convert::TryFrom;

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
        // This could probably be faster if we can simplify to a memcpy instead.
        if let Some(shader) = assets.get_mut(line_handle) {
            let mut i = 0;
            for (_id, line) in &lines.lines {
                // First point is start of line, second is end.
                // point.w property is used for thickness.
                shader.points[i] = line.start.extend(line.thickness);
                shader.points[i+1] = line.end.extend(line.thickness);
                shader.colors[i] = line.color[0];
                shader.colors[i+1] = line.color[1];

                i += 2;
            }

            let size = u32::try_from(lines.lines.len()).unwrap_or_else(|_| {
                warn!("DebugLines: Maximum number of lines exceeded: line count: {}, max lines: {}", lines.lines.len(), u16::MAX);
                u16::MAX as u32
            });
            shader.num_lines = size;
        }
    }

    lines.dirty = false;
}
