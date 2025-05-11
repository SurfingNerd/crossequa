use bevy::{
    DefaultPlugins,
    app::{App, Plugin, Startup},
    ecs::system::{Commands, Res},
    math::Vec3,
    prelude::*,
    transform::components::Transform,
    utils::default,
};

use crate::texture_manager::TextureManager;

use crate::board;
use crate::equation;
use crate::player_input;

pub struct CrossequaPlugin;


use std::f32::consts::TAU;

// Define a component to designate a rotation speed to an entity.
#[derive(Component)]
struct Rotatable {
    speed: f32,
}

// fn startup(mut commands: Commands) {
//     commands.spawn((
//         Camera2d::default(),
//         Camera {
//             hdr: true,
//             clear_color: ClearColorConfig::Custom(Color::LinearRgba(LinearRgba {
//                 red: 0.0,
//                 green: 0.21,
//                 blue: 0.56,
//                 alpha: 1.0,
//             })),
//             ..default()
//         },
//         Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
//     ));
// }

// This system will rotate any entity in the scene with a Rotatable component around its y-axis.
fn rotate_cube(mut cubes: Query<(&mut Transform, &Rotatable)>, timer: Res<Time>) {
    for (mut transform, cube) in &mut cubes {
        // The speed is first multiplied by TAU which is a full rotation (360deg) in radians,
        // and then multiplied by delta_secs which is the time that passed last frame.
        // In other words. Speed is equal to the amount of rotations per second.
        transform.rotate_y(cube.speed * TAU * timer.delta_secs());
        transform.rotate_z(cube.speed * TAU * timer.delta_secs());
    }
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
     images: Res<Assets<Image>>,
    asset_server: Res<AssetServer>,
) {
    let cube = bevy::math::primitives::Cuboid::new(0.5, 0.5, 0.5);
    let cube_mesh = meshes.add(cube);

    // TODO: ...
    // let a = asset_server.as_ref();

    let texture_manager = TextureManager::new(asset_server.as_ref(), &images);

    let material = texture_manager.get_tile_material();

    // add entities to the world
    for y in 0..=1 {
        for x in 0..=1 {
            bevy::log::info!("cube {} {}", x, y);
            // sphere
            commands.spawn((
                Mesh3d(cube_mesh.clone()),
                MeshMaterial3d(materials.add(material.clone())),
                Transform::from_xyz((x + 1) as f32, (y + 1)as f32, 0.0),
                Rotatable { speed: 0.3 },
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
        Transform::from_xyz(0.0, 0.0, 800.0).looking_at(Vec3::default(), Vec3::new(0.1,0.8, 0.1)),
        Projection::from(OrthographicProjection {
            scale: 0.01,

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
        app.add_plugins(DefaultPlugins)
            .add_systems(
                Startup,
                (equation::generate_equations, startup, board::setup_board).chain(),
            )
            .add_systems(Update, (rotate_cube, player_input::handle_mouse_click));
    }
}
