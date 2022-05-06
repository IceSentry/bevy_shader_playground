#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::default_trait_access)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_possible_truncation)]

mod camera;
mod custom_material;
mod gradient;
mod inspector;
mod macros;
mod shapes;

use bevy::{
    input::system::exit_on_esc_system,
    pbr::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
};
use bevy_egui::EguiPlugin;

use camera::{pan_orbit_camera, PanOrbitCamera};
use custom_material::CustomMaterial;
use gradient::GradientMaterial;
use inspector::inspector_panel;

#[derive(Component)]
pub struct Label(String);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_startup_system(hot_reload)
        .add_startup_system(spawn_camera)
        .add_system(pan_orbit_camera)
        .add_plugin(MaterialPlugin::<CustomMaterial>::default())
        .add_startup_system(spawn_scene)
        .add_startup_system(spawn_colored_spheres)
        .add_plugin(MaterialPlugin::<GradientMaterial>::default())
        .add_startup_system(spawn_scene_gradient)
        .add_system(inspector_panel)
        .add_system(exit_on_esc_system)
        .run();
}

#[allow(clippy::needless_pass_by_value)]
fn hot_reload(asset_server: Res<AssetServer>) {
    asset_server
        .watch_for_changes()
        .expect("Failed to start hot reload");
}

fn spawn_camera(mut commands: Commands) {
    let camera_translation = Vec3::new(3.0, 3.5, 10.0);
    let radius = camera_translation.length();
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_translation(camera_translation)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(PanOrbitCamera {
            radius,
            ..Default::default()
        });
}

fn spawn_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
    mut custom_materials: ResMut<Assets<CustomMaterial>>,
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 20.0 })),
        material: standard_materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });

    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    // white cube
    commands
        .spawn()
        .insert(Label("WHITE cube".into()))
        .insert_bundle(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Cube {
                ..Default::default()
            })),
            transform: Transform::from_xyz(0.0, 1.0, 3.0),
            material: custom_materials.add(CustomMaterial::new(Color::WHITE)),
            ..Default::default()
        })
        .insert_bundle((NotShadowCaster, NotShadowReceiver));

    // plane
    commands
        .spawn()
        .insert(Label("WHITE plane".into()))
        .insert_bundle(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 2.5 })),
            transform: Transform::from_xyz(0.0, 2.0, -5.0).with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                std::f32::consts::FRAC_PI_2,
                0.0,
                0.0,
            )),
            material: custom_materials.add(CustomMaterial::new(Color::WHITE)),
            ..Default::default()
        })
        .insert_bundle((NotShadowCaster, NotShadowReceiver));
}

fn spawn_colored_spheres(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut custom_materials: ResMut<Assets<CustomMaterial>>,
) {
    let sphere_mesh = meshes.add(Mesh::from(shape::UVSphere {
        ..Default::default()
    }));

    // red sphere
    commands
        .spawn()
        .insert(Label("RED sphere".into()))
        .insert_bundle(MaterialMeshBundle {
            mesh: sphere_mesh.clone(),
            transform: Transform::from_xyz(-2.25, 1.0, 0.0),
            material: custom_materials.add(CustomMaterial::new(Color::RED)),
            ..Default::default()
        })
        .insert_bundle((NotShadowCaster, NotShadowReceiver));

    // green sphere
    commands
        .spawn()
        .insert(Label("GREEN sphere".into()))
        .insert_bundle(MaterialMeshBundle {
            mesh: sphere_mesh.clone(),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            material: custom_materials.add(CustomMaterial::new(Color::GREEN)),
            ..Default::default()
        })
        .insert_bundle((NotShadowCaster, NotShadowReceiver));

    // blue sphere
    commands
        .spawn()
        .insert(Label("BLUE sphere".into()))
        .insert_bundle(MaterialMeshBundle {
            mesh: sphere_mesh,
            transform: Transform::from_xyz(2.25, 1.0, 0.0),
            material: custom_materials.add(CustomMaterial::new(Color::BLUE)),
            ..Default::default()
        })
        .insert_bundle((NotShadowCaster, NotShadowReceiver));
}

fn spawn_scene_gradient(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut gradient_materials: ResMut<Assets<GradientMaterial>>,
) {
    // gradient plane
    commands
        .spawn()
        .insert(Label("Gradient plane".into()))
        .insert_bundle(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 2.5 })),
            transform: Transform::from_xyz(3.0, 2.0, -5.0).with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                std::f32::consts::FRAC_PI_2,
                0.0,
                0.0,
            )),
            material: gradient_materials.add(GradientMaterial::new(Color::RED, Color::BLUE)),
            ..Default::default()
        })
        .insert_bundle((NotShadowCaster, NotShadowReceiver));

    // Cylinder
    commands
        .spawn()
        .insert(Label("Gradient cylinder".into()))
        .insert_bundle(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shapes::Cylinder {
                height: 2.5,
                radius: 1.0,
                ..Default::default()
            })),
            transform: Transform {
                translation: Vec3::new(6.0, 2.0, -5.0),
                rotation: Quat::from_euler(
                    EulerRot::XYZ,
                    0.0,
                    std::f32::consts::FRAC_PI_2,
                    std::f32::consts::FRAC_PI_2,
                ),
                ..Default::default()
            },
            material: gradient_materials.add(GradientMaterial::new(Color::RED, Color::BLUE)),
            ..Default::default()
        })
        .insert_bundle((NotShadowCaster, NotShadowReceiver));
}
