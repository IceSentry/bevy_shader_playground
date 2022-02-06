/// This macro will generate a default impl `RenderAsset` for a Maeterial
/// It assumes that the struct derives `AsStd140`
#[macro_export]
macro_rules! impl_shader_render_asset {
    ($material:ident) => {
        #[derive(Clone)]
        pub struct GpuMaterial {
            _buffer: Buffer,
            bind_group: BindGroup,
        }

        impl RenderAsset for $material {
            type ExtractedAsset = $material;
            type PreparedAsset = GpuMaterial;
            type Param = (SRes<RenderDevice>, SRes<MaterialPipeline<Self>>);

            fn extract_asset(&self) -> Self::ExtractedAsset {
                self.clone()
            }

            fn prepare_asset(
                extracted_asset: Self::ExtractedAsset,
                (render_device, material_pipeline): &mut SystemParamItem<Self::Param>,
            ) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
                let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                    contents: extracted_asset.as_std140().as_bytes(),
                    label: Some(stringify!($material)),
                    usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
                });
                let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
                    entries: &[BindGroupEntry {
                        binding: 0,
                        resource: buffer.as_entire_binding(),
                    }],
                    label: None,
                    layout: &material_pipeline.material_layout,
                });
                Ok(GpuMaterial {
                    _buffer: buffer,
                    bind_group,
                })
            }
        }
    };
}

/// This macro will generate a default impl Material.
/// It assumes that:
/// - The struct derives `AsStd140`
/// - The shader file contains a vertex and fragment shader
#[macro_export]
macro_rules! impl_shader_material {
    ($material:ident, $shader_file:expr) => {
        impl_shader_material!($material, $shader_file, ShaderStages::VERTEX_FRAGMENT);
    };
    ($material:ident, $shader_file:expr, $visibility:expr) => {
        impl Material for $material {
            fn vertex_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
                Some(asset_server.load($shader_file))
            }

            fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
                Some(asset_server.load($shader_file))
            }

            fn bind_group(render_asset: &<Self as RenderAsset>::PreparedAsset) -> &BindGroup {
                &render_asset.bind_group
            }

            fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
                render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                    entries: &[BindGroupLayoutEntry {
                        binding: 0,
                        visibility: $visibility,
                        ty: BindingType::Buffer {
                            ty: BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: BufferSize::new(
                                $material::std140_size_static() as u64
                            ),
                        },
                        count: None,
                    }],
                    label: None,
                })
            }
        }
    };
}
