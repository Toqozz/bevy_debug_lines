use bevy::{
    core_pipeline::Opaque3d,
    pbr::{
        DrawMesh, MeshPipeline, MeshPipelineKey, MeshUniform, SetMeshBindGroup,
        SetMeshViewBindGroup,
    },
    prelude::*,
    render::{
        render_phase::{DrawFunctions, RenderPhase, SetItemPipeline},
        render_resource::{
            PrimitiveTopology, RenderPipelineCache, RenderPipelineDescriptor, SpecializedPipeline,
            SpecializedPipelines, VertexAttribute, VertexBufferLayout, VertexFormat,
            VertexStepMode,
        },
        view::{ExtractedView, Msaa},
    },
};

use crate::{DebugLinesConfig, RenderDebugLinesMesh, DEBUG_LINES_SHADER_HANDLE};

pub(crate) struct DebugLinePipeline {
    mesh_pipeline: MeshPipeline,
    shader: Handle<Shader>,
    always_in_front: bool,
}
impl FromWorld for DebugLinePipeline {
    fn from_world(render_world: &mut World) -> Self {
        let config = render_world.get_resource::<DebugLinesConfig>().unwrap();
        DebugLinePipeline {
            mesh_pipeline: render_world.get_resource::<MeshPipeline>().unwrap().clone(),
            shader: DEBUG_LINES_SHADER_HANDLE.typed(),
            always_in_front: config.always_in_front,
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
        if self.always_in_front {
            descriptor.depth_stencil.as_mut().unwrap().bias.constant = i32::MAX;
        }
        descriptor
    }
}

pub(crate) fn queue_debug_lines(
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

pub(crate) type DrawDebugLines = (
    SetItemPipeline,
    SetMeshViewBindGroup<0>,
    SetMeshBindGroup<1>,
    DrawMesh,
);
