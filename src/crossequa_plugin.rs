use bevy::{
    DefaultPlugins,
    app::{App, Plugin, Startup},
    ecs::system::{Commands, Res},
    math::Vec3,
    prelude::*,
    transform::components::Transform,
    utils::default,
};

pub struct CrossequaPlugin;
use crate::board;
use crate::equation;
use crate::player_input;

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
        app.add_plugins(DefaultPlugins)
            .add_systems(
                Startup,
                (startup, equation::generate_equations, board::setup_board).chain(),
            )
            .add_systems(Update, player_input::handle_mouse_click);
    }
}

fn draw_equations(mut commands: Commands, equations: Res<equation::GridEquations>) {
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
                equation::Direction::Horizontal => (pos.0 + i * grid_padding, pos.1),
                equation::Direction::Vertical => (pos.0, pos.1 + i * grid_padding),
            };
            commands.spawn((
                Text2d::new(symbol.to_string()),
                TextShadow::default(),
                Transform::from_translation(Vec3::new(x as f32, y as f32, 0.0)),
            ));
        }
    }
}
