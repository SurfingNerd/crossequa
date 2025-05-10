

use bevy::{app::{App, Plugin, Startup}, asset::{AssetServer, Assets}, color::Srgba, core_pipeline::core_3d::Camera3d, ecs::system::{Commands, Res, ResMut}, math::Vec3, pbr::{environment_map::EnvironmentMapLight, DirectionalLight, MeshMaterial3d, StandardMaterial}, render::{camera::{OrthographicProjection, Projection, ScalingMode}, mesh::{Mesh, Mesh3d}}, transform::components::Transform, utils::default, DefaultPlugins};


pub struct CrossequaPlugin;



fn startup(mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,) {

        

        let cube = bevy::math::primitives::Cuboid::new(1.0, 1.0, 1.0);
        let cube_mesh = meshes.add(cube);
        let base_color_texture = asset_server.load("textures/Scifi_Panels_01_basecolor.png");
        let metallic_roughtness_texture = asset_server.load("textures/Scifi_Panels_01_roughness.png");
        let emissive_texture = asset_server.load("textures/Scifi_Panels_01_emissive.png");
        let occlusion_texture = asset_server.load("textures/Scifi_Panels_01_ambientocclusion.png");
        let normal_map_texture = asset_server.load("textures/Scifi_Panels_01_normal.png");
        
        // add entities to the world
        for y in -2..=2 {
            for x in -5..=5 {
                let x01 = (x + 5) as f32 / 10.0;
                let y01 = (y + 2) as f32 / 4.0;
                // sphere
                commands.spawn((
                    Mesh3d(cube_mesh.clone()),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        // vary key PBR parameters on a grid of spheres to show the effect
                        base_color_texture: Some(base_color_texture.clone()),
                        metallic_roughness_texture: Some(metallic_roughtness_texture.clone()),
                        emissive_texture: Some(emissive_texture.clone()),
                        occlusion_texture: Some(occlusion_texture.clone()),
                        normal_map_texture: Some(normal_map_texture.clone()),
                        metallic: y01,
                        perceptual_roughness: x01,
                        ..default()
                    })),
                    Transform::from_xyz(x as f32, y as f32 + 0.5, 0.0),
                ));
            }
        }


    commands.spawn((
        DirectionalLight {
            illuminance: 1_500.,
            ..default()
        },
        Transform::from_xyz(50.0, 50.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));


    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 8.0).looking_at(Vec3::default(), Vec3::Y),
        Projection::from(OrthographicProjection {
            scale: 0.01,
            scaling_mode: ScalingMode::WindowSize,
            ..OrthographicProjection::default_3d()
        }),
        EnvironmentMapLight {
            diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
            specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
            intensity: 900.0,
            ..default()
        },
    ));
    
}

impl Plugin for CrossequaPlugin {
    
    fn build(&self, app: &mut App) {
        // add things to your app here
        app
            .add_plugins(DefaultPlugins)
            .add_systems(Startup, startup);

    }
}
