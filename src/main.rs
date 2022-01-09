mod camera;

use bevy::{
    ecs::system::{lifetimeless::SRes, SystemParamItem},
    pbr::{MaterialPipeline, NotShadowCaster, NotShadowReceiver},
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
use bevy_inspector_egui::{widgets::InspectorQuery, Inspectable, InspectorPlugin};

use camera::{pan_orbit_camera, PanOrbitCamera};

#[derive(Inspectable, Default)]
struct MaterialsInspector {
    materials: InspectorQuery<&'static mut Handle<CustomMaterial>>,
}

// #[derive(Component, Inspectable, Default)]
// struct Label {
//     value: String,
// }

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(MaterialPlugin::<CustomMaterial>::default())
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_scene)
        .add_system(pan_orbit_camera)
        .add_plugin(InspectorPlugin::<MaterialsInspector>::new())
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

#[derive(Debug, Clone, TypeUuid, AsStd140, Inspectable)]
#[uuid = "18600cbe-b8b5-41e8-bbf6-1cad0005b309"]
struct CustomMaterial {
    color: Vec4,
    #[inspectable(min = 0.0, max = 10.0)]
    scale: f32,
}

impl CustomMaterial {
    fn new(color: Color, scale: f32) -> Self {
        Self {
            color: Vec4::from_slice(&color.as_linear_rgba_f32()),
            scale,
        }
    }
}

impl Default for CustomMaterial {
    fn default() -> Self {
        Self::new(Color::CYAN, 1.0)
    }
}

#[derive(Clone)]
struct GpuCustomMaterial {
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
