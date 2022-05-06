use bevy::{
    ecs::system::{lifetimeless::SRes, SystemParamItem},
    pbr::MaterialPipeline,
    prelude::*,
    reflect::TypeUuid,
    render::{
        render_asset::{PrepareAssetError, RenderAsset},
        render_resource::{
            std140::{AsStd140, Std140},
            BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
            BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, Buffer,
            BufferBindingType, BufferInitDescriptor, BufferSize, BufferUsages, ShaderStages,
        },
        renderer::RenderDevice,
    },
};
use bevy_egui::egui::{self, CollapsingHeader, Ui};

use crate::{impl_shader_material, impl_shader_render_asset, inspector::inspect_transform, Label};

pub fn inspector(
    ui: &mut Ui,
    label: &Label,
    material: &mut GradientMaterial,
    transform: &mut Transform,
) {
    CollapsingHeader::new(label.0.as_str())
        .default_open(true)
        .show(ui, |ui| {
            // inspect_transform(ui, transform.local_x());
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

impl_shader_render_asset!(GradientMaterial);
impl_shader_material!(GradientMaterial, "shaders/gradient.wgsl");
