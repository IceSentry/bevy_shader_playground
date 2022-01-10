use bevy::prelude::*;
use bevy_egui::{
    egui::{self, CollapsingHeader, Ui},
    EguiContext,
};

use crate::{custom_material::CustomMaterial, Label};

pub fn inspector_panel(
    egui_context: Res<EguiContext>,
    mut query: Query<(&Label, &Handle<CustomMaterial>)>,
    mut custom_materials: ResMut<Assets<CustomMaterial>>,
) {
    egui::panel::SidePanel::new(egui::panel::Side::Left, "side_panel").show(
        egui_context.ctx(),
        |ui| {
            ui.heading("Inspector");
            ui.separator();
            ui.label("Custom Materials");
            for (label, mat) in query.iter_mut() {
                if let Some(mat) = custom_materials.get_mut(mat) {
                    material_inspector(ui, label, mat);
                }
            }
        },
    );
}

fn material_inspector(ui: &mut Ui, label: &Label, material: &mut CustomMaterial) {
    CollapsingHeader::new(label.0.as_str())
        .default_open(true)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Color: ");
                let mut color = material.color.to_array();
                ui.color_edit_button_rgba_unmultiplied(&mut color);
                material.color = Vec4::from_slice(&color);
            });
            ui.add(egui::Slider::new(&mut material.scale, 0.0..=5.0).text("Scale: "));
        });
}
