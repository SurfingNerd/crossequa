use bevy::{
    DefaultPlugins,
    app::{App, Plugin, Startup},
    ecs::system::{Commands, Res},
    math::Vec3,
    prelude::*,
    transform::components::Transform,
    utils::default,
};

use crate::{grid, texture_manager::TextureManager};

pub struct CrossequaPlugin;

fn startup(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
        Camera {
            hdr: true,
            clear_color: ClearColorConfig::Custom(Color::LinearRgba(LinearRgba {
                red: 0.0,
                green: 0.21,
                blue: 0.56,
                alpha: 1.0,
            })),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}




impl Plugin for CrossequaPlugin {
    fn build(&self, app: &mut App) {
        // add things to your app here
        app.add_plugins(DefaultPlugins).add_systems(
            Startup,
            (startup, grid::generate_grid, print_equations).chain(),
        );
    }
}

fn print_equations(mut commands: Commands, equations: Res<grid::GridEquations>) {
    // TODO: Make an actual grid of symbols
    for eq in equations.iter() {
        let symbols = eq.symbols();
        let (grid_size, grid_padding) = (60, 24);
        let mut pos = eq.start_pos;
        pos.0 = pos.0 * grid_size;
        pos.1 = pos.1 * grid_size;

        let dir = eq.direction.clone();

        for (i, symbol) in symbols.iter().enumerate() {
            let (x, y) = match dir {
                grid::Direction::Horizontal => (pos.0 + i * grid_padding, pos.1),
                grid::Direction::Vertical => (pos.0, pos.1 + i * grid_padding),
            };
            commands.spawn((
                Text2d::new(symbol.to_string()),
                TextShadow::default(),
                Transform::from_translation(Vec3::new(x as f32, y as f32, 0.0)),
            ));
        }
    }
}
