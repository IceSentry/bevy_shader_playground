use bevy::{
    ecs::system::{lifetimeless::SRes, SystemParamItem},
    math::Vec4,
    pbr::MaterialPipeline,
    prelude::*,
    reflect::TypeUuid,
    render::{
        render_asset::{PrepareAssetError, RenderAsset},
        render_resource::{
            std140::{AsStd140, Std140},
            *,
        },
        renderer::RenderDevice,
    },
};
use bevy_egui::egui::{self, CollapsingHeader, Ui};

use crate::Label;

pub fn inspector(ui: &mut Ui, label: &Label, material: &mut CustomMaterial) {
    CollapsingHeader::new(label.0.as_str())
        .default_open(true)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Color: ");
                let mut color = material.color.to_array();
                ui.color_edit_button_rgba_unmultiplied(&mut color);
                material.color = Vec4::from_slice(&color);
            });
            ui.horizontal(|ui| {
                ui.label("Scale: ");
                ui.add(egui::Slider::new(&mut material.scale, 0.0..=5.0));
            });
            ui.horizontal(|ui| {
                ui.label("Offset: ");
                ui.add(egui::Slider::new(&mut material.offset, -5.0..=5.0));
            });
        });
}

#[derive(Debug, Clone, TypeUuid, AsStd140)]
#[uuid = "18600cbe-b8b5-41e8-bbf6-1cad0005b309"]
pub struct CustomMaterial {
    pub color: Vec4,
    pub scale: f32,
    pub offset: f32,
}

impl CustomMaterial {
    pub fn new(color: Color) -> Self {
        Self {
            color: Vec4::from_slice(&color.as_linear_rgba_f32()),
            scale: 1.0,
            offset: 0.0,
        }
    }
}

#[derive(Clone)]
pub struct GpuCustomMaterial {
    _buffer: Buffer,
    bind_group: BindGroup,
}

impl RenderAsset for CustomMaterial {
    type ExtractedAsset = CustomMaterial;
    type PreparedAsset = GpuCustomMaterial;
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
            label: Some("CustomMaterial"),
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

        Ok(GpuCustomMaterial {
            _buffer: buffer,
            bind_group,
        })
    }
}

impl Material for CustomMaterial {
    fn vertex_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(asset_server.load("shaders/custom_material.wgsl"))
    }

    fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(asset_server.load("shaders/custom_material.wgsl"))
    }

    fn bind_group(render_asset: &<Self as RenderAsset>::PreparedAsset) -> &BindGroup {
        &render_asset.bind_group
    }

    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX_FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: BufferSize::new(CustomMaterial::std140_size_static() as u64),
                },
                count: None,
            }],
            label: None,
        })
    }
}
