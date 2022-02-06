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
            BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
            BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, Buffer,
            BufferBindingType, BufferInitDescriptor, BufferSize, BufferUsages, ShaderStages,
        },
        renderer::RenderDevice,
    },
};
use bevy_egui::egui::{self, CollapsingHeader, Ui};

use crate::{impl_shader_material, impl_shader_render_asset, Label};

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

impl_shader_render_asset!(CustomMaterial);
impl_shader_material!(
    CustomMaterial,
    "shaders/custom_material.wgsl",
    ShaderStages::VERTEX
);
