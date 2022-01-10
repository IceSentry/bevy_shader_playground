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
    mut custom_materials: ResMut<Assets<CustomMaterial>>,
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
        },
    );
}
