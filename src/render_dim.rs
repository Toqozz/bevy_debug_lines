pub mod r3d {
    use bevy::{
        core_pipeline::core_3d::Transparent3d,
        pbr::{
            DrawMesh, MeshPipeline, MeshPipelineKey, MeshPipelineViewLayoutKey, RenderMeshInstances, SetMeshBindGroup,
            SetMeshViewBindGroup, MAX_CASCADES_PER_LIGHT, MAX_DIRECTIONAL_LIGHTS,
        },
        prelude::*,
        render::{
            mesh::MeshVertexBufferLayout,
            render_asset::RenderAssets,
            render_phase::{DrawFunctions, RenderPhase, SetItemPipeline},
            render_resource::{
                BlendState, ColorTargetState, ColorWrites, CompareFunction, DepthBiasState, DepthStencilState,
                FragmentState, FrontFace, MultisampleState, PipelineCache, PolygonMode, PrimitiveState,
                PrimitiveTopology, RenderPipelineDescriptor, ShaderDefVal, SpecializedMeshPipeline,
                SpecializedMeshPipelineError, SpecializedMeshPipelines, StencilFaceState, StencilState, TextureFormat,
                VertexState,
            },
            texture::BevyDefault,
            view::{ExtractedView, Msaa, ViewTarget},
        },
    };

    use crate::{DebugLinesConfig, RenderDebugLinesMesh, DEBUG_LINES_SHADER_HANDLE};

    #[derive(Resource)]
    pub(crate) struct DebugLinePipeline {
        mesh_pipeline: MeshPipeline,
        shader: Handle<Shader>,
    }
    impl FromWorld for DebugLinePipeline {
        fn from_world(render_world: &mut World) -> Self {
            DebugLinePipeline {
                mesh_pipeline: render_world.get_resource::<MeshPipeline>().unwrap().clone(),
                shader: DEBUG_LINES_SHADER_HANDLE,
            }
        }
    }

    impl SpecializedMeshPipeline for DebugLinePipeline {
        type Key = (bool, MeshPipelineKey);

        fn specialize(
            &self,
            (depth_test, key): Self::Key,
            layout: &MeshVertexBufferLayout,
        ) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError> {
            let mut shader_defs = Vec::new();
            shader_defs.push("LINES_3D".into());
            shader_defs.push(ShaderDefVal::UInt(
                "MAX_CASCADES_PER_LIGHT".to_string(),
                MAX_CASCADES_PER_LIGHT as u32,
            ));
            shader_defs.push(ShaderDefVal::UInt(
                "MAX_DIRECTIONAL_LIGHTS".to_string(),
                MAX_DIRECTIONAL_LIGHTS as u32,
            ));
            if depth_test {
                shader_defs.push("DEPTH_TEST_ENABLED".into());
            }

            let (label, blend, depth_write_enabled);
            if key.contains(MeshPipelineKey::BLEND_ALPHA) {
                label = "transparent_mesh_pipeline".into();
                blend = Some(BlendState::ALPHA_BLENDING);
                // For the transparent pass, fragments that are closer will be alpha
                // blended but their depth is not written to the depth buffer.
                depth_write_enabled = false;
            } else {
                label = "opaque_mesh_pipeline".into();
                blend = Some(BlendState::REPLACE);
                // For the opaque and alpha mask passes, fragments that are closer
                // will replace the current fragment value in the output and the depth is
                // written to the depth buffer.
                depth_write_enabled = true;
            }

            let vertex_buffer_layout = layout.get_layout(&[
                Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
                Mesh::ATTRIBUTE_COLOR.at_shader_location(1),
            ])?;

            let bind_group_layout = match key.msaa_samples() {
                1 => vec![self
                    .mesh_pipeline
                    .get_view_layout(MeshPipelineViewLayoutKey::NORMAL_PREPASS)
                    .clone()],
                _ => {
                    shader_defs.push("MULTISAMPLED".into());
                    vec![self
                        .mesh_pipeline
                        .get_view_layout(MeshPipelineViewLayoutKey::MULTISAMPLED)
                        .clone()]
                },
            };

            let format = if key.contains(MeshPipelineKey::HDR) {
                ViewTarget::TEXTURE_FORMAT_HDR
            } else {
                TextureFormat::bevy_default()
            };

            Ok(RenderPipelineDescriptor {
                vertex: VertexState {
                    shader: self.shader.clone_weak(),
                    entry_point: "vertex".into(),
                    shader_defs: shader_defs.clone(),
                    buffers: vec![vertex_buffer_layout],
                },
                fragment: Some(FragmentState {
                    shader: self.shader.clone_weak(),
                    shader_defs,
                    entry_point: "fragment".into(),
                    targets: vec![Some(ColorTargetState {
                        format,
                        blend,
                        write_mask: ColorWrites::ALL,
                    })],
                }),
                layout: bind_group_layout,
                primitive: PrimitiveState {
                    front_face: FrontFace::Ccw,
                    cull_mode: None,
                    unclipped_depth: false,
                    polygon_mode: PolygonMode::Fill,
                    conservative: false,
                    topology: PrimitiveTopology::LineList,
                    strip_index_format: None,
                },
                depth_stencil: Some(DepthStencilState {
                    format: TextureFormat::Depth32Float,
                    depth_write_enabled,
                    depth_compare: CompareFunction::Greater,
                    stencil: StencilState {
                        front: StencilFaceState::IGNORE,
                        back: StencilFaceState::IGNORE,
                        read_mask: 0,
                        write_mask: 0,
                    },
                    bias: DepthBiasState {
                        constant: 0,
                        slope_scale: 0.0,
                        clamp: 0.0,
                    },
                }),
                multisample: MultisampleState {
                    count: key.msaa_samples(),
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                label: Some(label),
                push_constant_ranges: vec![],
            })
        }
    }

    #[allow(unused, clippy::complexity)]
    pub(crate) fn queue(
        opaque_3d_draw_functions: Res<DrawFunctions<Transparent3d>>,
        debug_line_pipeline: Res<DebugLinePipeline>,
        mut pipelines: ResMut<SpecializedMeshPipelines<DebugLinePipeline>>,
        pipeline_cache: Res<PipelineCache>,
        render_meshes: Res<RenderAssets<Mesh>>,
        msaa: Res<Msaa>,
        render_mesh_instances: Res<RenderMeshInstances>,
        instance_entities: Query<Entity, With<RenderDebugLinesMesh>>,
        config: Res<DebugLinesConfig>,
        mut views: Query<(&ExtractedView, &mut RenderPhase<Transparent3d>)>,
    ) {
        let draw_custom = opaque_3d_draw_functions.read().get_id::<DrawDebugLines>().unwrap();
        let msaa_key = MeshPipelineKey::from_msaa_samples(msaa.samples());
        for (view, mut transparent_phase) in views.iter_mut() {
            let view_matrix = view.transform.compute_matrix();
            let view_row_2 = view_matrix.row(2);
            for (entity) in instance_entities.iter() {
                if let Some(render_mesh_instance) = render_mesh_instances.get(&entity) {
                    if let Some(mesh) = render_meshes.get(render_mesh_instance.mesh_asset_id) {
                        let mesh_key = msaa_key
                            | MeshPipelineKey::from_primitive_topology(PrimitiveTopology::LineList)
                            | MeshPipelineKey::from_hdr(view.hdr);
                        let pipeline = pipelines
                            .specialize(
                                &pipeline_cache,
                                &debug_line_pipeline,
                                (config.depth_test, mesh_key),
                                &mesh.layout,
                            )
                            .unwrap();

                        let transform = render_mesh_instance.transforms.transform.translation.extend(1.0);

                        transparent_phase.add(Transparent3d {
                            entity,
                            pipeline,
                            draw_function: draw_custom,
                            distance: view_row_2.dot(transform),
                            batch_range: 0..0,
                            dynamic_offset: None,
                        });
                    }
                }
            }
        }
    }

    pub(crate) type DrawDebugLines = (SetItemPipeline, SetMeshViewBindGroup<0>, SetMeshBindGroup<1>, DrawMesh);
}

pub mod r2d {
    use bevy::{
        asset::Handle,
        core_pipeline::core_2d::Transparent2d,
        prelude::*,
        render::{
            mesh::MeshVertexBufferLayout,
            render_asset::RenderAssets,
            render_phase::{DrawFunctions, RenderPhase, SetItemPipeline},
            render_resource::{
                BlendState, ColorTargetState, ColorWrites, FragmentState, FrontFace, MultisampleState, PipelineCache,
                PolygonMode, PrimitiveState, PrimitiveTopology, RenderPipelineDescriptor, Shader,
                SpecializedMeshPipeline, SpecializedMeshPipelineError, SpecializedMeshPipelines, TextureFormat,
                VertexState,
            },
            texture::BevyDefault,
            view::{ExtractedView, Msaa, ViewTarget, VisibleEntities},
        },
        sprite::{
            DrawMesh2d, Mesh2dPipeline, Mesh2dPipelineKey, RenderMesh2dInstances, SetMesh2dBindGroup,
            SetMesh2dViewBindGroup,
        },
        utils::FloatOrd,
    };

    use crate::{RenderDebugLinesMesh, DEBUG_LINES_SHADER_HANDLE};

    #[derive(Resource)]
    pub(crate) struct DebugLinePipeline {
        mesh_pipeline: Mesh2dPipeline,
        shader: Handle<Shader>,
    }
    impl FromWorld for DebugLinePipeline {
        fn from_world(render_world: &mut World) -> Self {
            DebugLinePipeline {
                mesh_pipeline: Mesh2dPipeline::from_world(render_world),
                shader: DEBUG_LINES_SHADER_HANDLE,
            }
        }
    }

    impl SpecializedMeshPipeline for DebugLinePipeline {
        type Key = Mesh2dPipelineKey;

        fn specialize(
            &self,
            key: Self::Key,
            layout: &MeshVertexBufferLayout,
        ) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError> {
            let vertex_buffer_layout = layout.get_layout(&[
                Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
                Mesh::ATTRIBUTE_COLOR.at_shader_location(1),
            ])?;

            Ok(RenderPipelineDescriptor {
                vertex: VertexState {
                    shader: self.shader.clone_weak(),
                    entry_point: "vertex".into(),
                    shader_defs: vec![],
                    buffers: vec![vertex_buffer_layout],
                },
                fragment: Some(FragmentState {
                    shader: self.shader.clone_weak(),
                    shader_defs: vec![],
                    entry_point: "fragment".into(),
                    targets: vec![Some(ColorTargetState {
                        format: if key.contains(Mesh2dPipelineKey::HDR) {
                            ViewTarget::TEXTURE_FORMAT_HDR
                        } else {
                            TextureFormat::bevy_default()
                        },
                        blend: Some(BlendState::ALPHA_BLENDING),
                        write_mask: ColorWrites::ALL,
                    })],
                }),
                layout: vec![self.mesh_pipeline.view_layout.clone()],
                primitive: PrimitiveState {
                    front_face: FrontFace::Ccw,
                    cull_mode: None,
                    unclipped_depth: false,
                    polygon_mode: PolygonMode::Fill,
                    conservative: false,
                    topology: PrimitiveTopology::LineList,
                    strip_index_format: None,
                },
                depth_stencil: None,
                multisample: MultisampleState {
                    count: key.msaa_samples(),
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                label: None,
                push_constant_ranges: vec![],
            })
        }
    }

    #[allow(unused, clippy::complexity)]
    pub(crate) fn queue(
        draw2d_functions: Res<DrawFunctions<Transparent2d>>,
        debug_line_pipeline: Res<DebugLinePipeline>,
        pipeline_cache: Res<PipelineCache>,
        mut specialized_pipelines: ResMut<SpecializedMeshPipelines<DebugLinePipeline>>,
        render_meshes: Res<RenderAssets<Mesh>>,
        msaa: Res<Msaa>,
        render_mesh_instances: Res<RenderMesh2dInstances>,
        instance_entities: Query<Entity, With<RenderDebugLinesMesh>>,
        mut views: Query<(&ExtractedView, &VisibleEntities, &mut RenderPhase<Transparent2d>)>,
    ) {
        for (view, visible_entities, mut phase) in views.iter_mut() {
            let draw_mesh2d = draw2d_functions.read().get_id::<DrawDebugLines>().unwrap();
            let msaa_key = Mesh2dPipelineKey::from_msaa_samples(msaa.samples());

            for visible_entity in &visible_entities.entities {
                if let Some(render_mesh_instance) = render_mesh_instances.get(visible_entity) {
                    if let Some(mesh) = render_meshes.get(render_mesh_instance.mesh_asset_id) {
                        let mesh_key = msaa_key
                            | Mesh2dPipelineKey::from_primitive_topology(PrimitiveTopology::LineList)
                            | Mesh2dPipelineKey::from_hdr(view.hdr);
                        let pipeline = specialized_pipelines
                            .specialize(&pipeline_cache, &debug_line_pipeline, mesh_key, &mesh.layout)
                            .unwrap();
                        phase.add(Transparent2d {
                            entity: *visible_entity,
                            draw_function: draw_mesh2d,
                            pipeline,
                            sort_key: FloatOrd(f32::INFINITY),
                            batch_range: 0..0,
                            dynamic_offset: None,
                        });
                    }
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
