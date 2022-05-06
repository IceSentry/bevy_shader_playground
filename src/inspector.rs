use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Ui},
    EguiContext,
};

use crate::{
    custom_material::{self, CustomMaterial},
    gradient::{self, GradientMaterial},
    Label,
};

pub fn inspector_panel(
    mut egui_context: ResMut<EguiContext>,
    mut color_materials_query: Query<(&Label, &Handle<CustomMaterial>)>,
    mut gradient_materials_query: Query<(&Label, &Handle<GradientMaterial>, &mut Transform)>,
    mut custom_materials: ResMut<Assets<CustomMaterial>>,
    mut gradient_materials: ResMut<Assets<GradientMaterial>>,
) {
    egui::panel::SidePanel::new(egui::panel::Side::Left, "side_panel").show(
        egui_context.ctx_mut(),
        |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
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
                for (label, mat, mut transform) in gradient_materials_query.iter_mut() {
                    if let Some(mat) = gradient_materials.get_mut(mat) {
                        gradient::inspector(ui, label, mat, &mut transform);
                    }
                }
            });
        },
    );
}

pub fn inspect_transform(ui: &mut Ui, transform: &mut Transform) {
    ui.label("Translation");
    inspect_vec3(ui, &mut transform.translation, 0.1);

    ui.label("Rotation");
    let mut rot = Vec3::from(transform.rotation.to_euler(EulerRot::XYZ));
    rot.x = rot.x.to_degrees();
    rot.y = rot.y.to_degrees();
    rot.z = rot.z.to_degrees();
    inspect_vec3(ui, &mut rot, 1.0);
    transform.rotation = Quat::from_euler(
        EulerRot::XYZ,
        rot.x.to_radians(),
        rot.y.to_radians(),
        rot.z.to_radians(),
    );
}

pub fn inspect_vec3(ui: &mut Ui, vec: &mut Vec3, speed: f32) {
    ui.horizontal(|ui| {
        ui.label("X: ");
        ui.add(egui::DragValue::new(&mut vec.x).speed(speed));
        ui.label("Y: ");
        ui.add(egui::DragValue::new(&mut vec.y).speed(speed));
        ui.label("Z: ");
        ui.add(egui::DragValue::new(&mut vec.z).speed(speed));
    });
}
