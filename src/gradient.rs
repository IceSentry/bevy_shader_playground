use bevy::{
    ecs::system::{lifetimeless::SRes, SystemParamItem},
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

pub fn inspector(ui: &mut Ui, label: &Label, material: &mut GradientMaterial) {
    CollapsingHeader::new(label.0.as_str())
        .default_open(true)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Color A: ");
                let mut color = material.color_a.to_array();
                ui.color_edit_button_rgba_unmultiplied(&mut color);
                material.color_a = Vec4::from_slice(&color);
            });
            ui.horizontal(|ui| {
                ui.label("Color B: ");
                let mut color = material.color_b.to_array();
                ui.color_edit_button_rgba_unmultiplied(&mut color);
                material.color_b = Vec4::from_slice(&color);
            });
            ui.horizontal(|ui| {
                ui.label("Color start: ");
                ui.add(egui::Slider::new(&mut material.color_start, 0.0..=1.0));
            });
            ui.horizontal(|ui| {
                ui.label("Color end: ");
                ui.add(egui::Slider::new(&mut material.color_end, 0.0..=1.0));
            });
        });
}

#[derive(Debug, Clone, TypeUuid, AsStd140)]
#[uuid = "9ad452f9-54e9-4977-a41a-9b674b61ee94"]
pub struct GradientMaterial {
    pub color_a: Vec4,
    pub color_b: Vec4,
    pub color_start: f32,
    pub color_end: f32,
}

impl GradientMaterial {
    pub fn new(color_a: Color, color_b: Color) -> Self {
        Self {
            color_a: Vec4::from_slice(&color_a.as_linear_rgba_f32()),
            color_b: Vec4::from_slice(&color_b.as_linear_rgba_f32()),
            color_start: 0.0,
            color_end: 1.0,
        }
    }
}

#[derive(Clone)]
pub struct GpuGradientMaterial {
    _buffer: Buffer,
    bind_group: BindGroup,
}

impl RenderAsset for GradientMaterial {
    type ExtractedAsset = GradientMaterial;
    type PreparedAsset = GpuGradientMaterial;
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
            label: Some("GradientMaterial"),
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

        Ok(GpuGradientMaterial {
            _buffer: buffer,
            bind_group,
        })
    }
}

impl Material for GradientMaterial {
    fn vertex_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(asset_server.load("shaders/gradient.wgsl"))
    }

    fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(asset_server.load("shaders/gradient.wgsl"))
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
                    min_binding_size: BufferSize::new(GradientMaterial::std140_size_static() as u64),
                },
                count: None,
            }],
            label: None,
        })
    }
}
