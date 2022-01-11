use bevy::{
    asset::Handle,
    core::FloatOrd,
    core_pipeline::Transparent2d,
    prelude::*,
    render::{
        render_phase::{DrawFunctions, RenderPhase, SetItemPipeline},
        render_resource::{
            PrimitiveTopology, RenderPipelineCache, RenderPipelineDescriptor, Shader,
            SpecializedPipeline, SpecializedPipelines, VertexAttribute, VertexBufferLayout,
            VertexFormat, VertexStepMode,
        },
        view::{Msaa, VisibleEntities},
    },
    sprite::{
        DrawMesh2d, Mesh2dPipeline, Mesh2dPipelineKey, Mesh2dUniform, SetMesh2dBindGroup,
        SetMesh2dViewBindGroup,
    },
};

use crate::{RenderDebugLinesMesh, DEBUG_LINES_SHADER_HANDLE};

pub(crate) struct DebugLinePipeline {
    mesh_pipeline: Mesh2dPipeline,
    shader: Handle<Shader>,
}
impl FromWorld for DebugLinePipeline {
    fn from_world(render_world: &mut World) -> Self {
        DebugLinePipeline {
            mesh_pipeline: render_world
                .get_resource::<Mesh2dPipeline>()
                .unwrap()
                .clone(),
            shader: DEBUG_LINES_SHADER_HANDLE.typed(),
        }
    }
}

impl SpecializedPipeline for DebugLinePipeline {
    type Key = Mesh2dPipelineKey;

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
        descriptor
    }
}

pub(crate) fn queue_debug_lines(
    draw2d_functions: Res<DrawFunctions<Transparent2d>>,
    debug_line_pipeline: Res<DebugLinePipeline>,
    mut pipeline_cache: ResMut<RenderPipelineCache>,
    mut specialized_pipelines: ResMut<SpecializedPipelines<DebugLinePipeline>>,
    msaa: Res<Msaa>,
    material_meshes: Query<&Mesh2dUniform, With<RenderDebugLinesMesh>>,
    mut views: Query<(&VisibleEntities, &mut RenderPhase<Transparent2d>)>,
) {
    for (view, mut phase) in views.iter_mut() {
        let draw_mesh2d = draw2d_functions.read().get_id::<DrawDebugLines>().unwrap();
        let mesh_key = Mesh2dPipelineKey::from_msaa_samples(msaa.samples);

        for visible_entity in &view.entities {
            if let Ok(uniform) = material_meshes.get(*visible_entity) {
                let mesh2d_key = mesh_key
                    | Mesh2dPipelineKey::from_primitive_topology(PrimitiveTopology::LineList);
                let mesh_z = uniform.transform.w_axis.z;
                phase.add(Transparent2d {
                    entity: *visible_entity,
                    draw_function: draw_mesh2d,
                    pipeline: specialized_pipelines.specialize(
                        &mut pipeline_cache,
                        &debug_line_pipeline,
                        mesh2d_key,
                    ),
                    sort_key: FloatOrd(mesh_z),
                    batch_range: None,
                });
            }
        }
    }
}

pub(crate) type DrawDebugLines = (
    SetItemPipeline,
    SetMesh2dViewBindGroup<0>,
    SetMesh2dBindGroup<1>,
    DrawMesh2d,
);
