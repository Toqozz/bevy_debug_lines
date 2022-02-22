pub mod r3d {
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

    use crate::{RenderDebugLinesMesh, DebugLinesConfig, DEBUG_LINES_SHADER_HANDLE};

    pub(crate) struct DebugLinePipeline {
        mesh_pipeline: MeshPipeline,
        shader: Handle<Shader>,
        //always_in_front: bool,
    }
    impl FromWorld for DebugLinePipeline {
        fn from_world(render_world: &mut World) -> Self {
            //let config = render_world.get_resource::<DebugLinesConfig>().unwrap();
            DebugLinePipeline {
                mesh_pipeline: render_world.get_resource::<MeshPipeline>().unwrap().clone(),
                shader: DEBUG_LINES_SHADER_HANDLE.typed(),
                //always_in_front: config.always_in_front,
            }
        }
    }

    impl SpecializedPipeline for DebugLinePipeline {
        type Key = (bool, MeshPipelineKey);

        fn specialize(&self, (depth_test, key): Self::Key) -> RenderPipelineDescriptor {
            use VertexFormat::{Float32x3, Float32x4};

            let mut shader_defs = Vec::new();
            shader_defs.push("LINES_3D".to_string());
            if depth_test {
                shader_defs.push("DEPTH_TEST_ENABLED".to_string());
            }

            let mut descriptor = self.mesh_pipeline.specialize(key);
            descriptor.vertex.shader = self.shader.clone_weak();
            descriptor.vertex.shader_defs = shader_defs.clone();
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
            let fragment = descriptor.fragment.as_mut().unwrap();
            fragment.shader = self.shader.clone_weak();
            fragment.shader_defs = shader_defs.clone();
            descriptor.primitive.topology = PrimitiveTopology::LineList;
            descriptor.primitive.cull_mode = None;
            //if self.always_in_front {
                //descriptor.depth_stencil.as_mut().unwrap().bias.constant = i32::MAX;
            //}
            descriptor
        }
    }

    pub(crate) fn queue(
        opaque_3d_draw_functions: Res<DrawFunctions<Opaque3d>>,
        debug_line_pipeline: Res<DebugLinePipeline>,
        mut pipelines: ResMut<SpecializedPipelines<DebugLinePipeline>>,
        mut pipeline_cache: ResMut<RenderPipelineCache>,
        msaa: Res<Msaa>,
        material_meshes: Query<(Entity, &MeshUniform), With<RenderDebugLinesMesh>>,
        config: Res<DebugLinesConfig>,
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
            for (entity, mesh_uniform) in material_meshes.iter() {
                let pipeline = pipelines.specialize(&mut pipeline_cache, &debug_line_pipeline, (config.depth_test, key));
                transparent_phase.add(Opaque3d {
                    entity,
                    pipeline,
                    draw_function: draw_custom,
                    distance: view_row_2.dot(mesh_uniform.transform.col(3)),
                });
            }
        }
    }

    pub(crate) type DrawDebugLines = (
        SetItemPipeline,
        SetMeshViewBindGroup<0>,
        SetMeshBindGroup<1>,
        DrawMesh,
    );
}

pub mod r2d {
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

            //let mut shader_defs = Vec::new();
            //shader_defs.push("2D".to_string());

            let mut descriptor = self.mesh_pipeline.specialize(key);
            descriptor.vertex.shader = self.shader.clone_weak();
            //descriptor.vertex.shader_defs = shader_defs.clone();
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
            let fragment = descriptor.fragment.as_mut().unwrap();
            fragment.shader = self.shader.clone_weak();
            //fragment.shader_defs = shader_defs.clone();
            descriptor.primitive.topology = PrimitiveTopology::LineList;
            descriptor.primitive.cull_mode = None;
            descriptor
        }
    }

    pub(crate) fn queue(
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
}

