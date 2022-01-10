mod camera;
mod custom_material;
mod inspector;

use bevy::{
    input::system::exit_on_esc_system,
    pbr::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
};
use bevy_egui::EguiPlugin;

use camera::{pan_orbit_camera, PanOrbitCamera};
use custom_material::CustomMaterial;
use inspector::inspector_panel;

#[derive(Component)]
pub struct Label(String);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(MaterialPlugin::<CustomMaterial>::default())
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_scene)
        .add_system(pan_orbit_camera)
        .add_system(inspector_panel)
        .add_system(exit_on_esc_system)
        .run();
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
        mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
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
            material: custom_materials.add(CustomMaterial::new(Color::RED, 1.0)),
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
            material: custom_materials.add(CustomMaterial::new(Color::GREEN, 1.0)),
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
            material: custom_materials.add(CustomMaterial::new(Color::BLUE, 1.0)),
            ..Default::default()
        })
        .insert_bundle((NotShadowCaster, NotShadowReceiver));

    // white cube
    commands
        .spawn()
        .insert(Label("WHITE cube".into()))
        .insert_bundle(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Cube {
                ..Default::default()
            })),
            transform: Transform::from_xyz(0.0, 1.0, 3.0),
            material: custom_materials.add(CustomMaterial::new(Color::WHITE, 2.0)),
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
            material: custom_materials.add(CustomMaterial::new(Color::WHITE, 2.0)),
            ..Default::default()
        })
        .insert_bundle((NotShadowCaster, NotShadowReceiver));
}
