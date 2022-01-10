use bevy::prelude::*;
use bevy_egui::{
    egui::{self},
    EguiContext,
};

use crate::{
    custom_material::{self, CustomMaterial},
    gradient::{self, GradientMaterial},
    Label,
};

pub fn inspector_panel(
    egui_context: Res<EguiContext>,
    mut color_materials_query: Query<(&Label, &Handle<CustomMaterial>)>,
    mut gradient_materials_query: Query<(&Label, &Handle<GradientMaterial>)>,
    mut custom_materials: ResMut<Assets<CustomMaterial>>,
    mut gradient_materials: ResMut<Assets<GradientMaterial>>,
) {
    egui::panel::SidePanel::new(egui::panel::Side::Left, "side_panel").show(
        egui_context.ctx(),
        |ui| {
            ui.heading("Inspector");
            ui.separator();
            ui.label("Custom Materials");
            for (label, mat) in color_materials_query.iter_mut() {
                if let Some(mat) = custom_materials.get_mut(mat) {
                    custom_material::inspector(ui, label, mat);
                }
            }
            ui.separator();
            ui.label("Gradient Materials");
            for (label, mat) in gradient_materials_query.iter_mut() {
                if let Some(mat) = gradient_materials.get_mut(mat) {
                    gradient::inspector(ui, label, mat);
                }
            }
        },
    );
}
