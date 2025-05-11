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
            .init_resource::<player_input::PlayerKeyboardInput>()
            .add_event::<player_input::UpdateSymbolEvent>()
            .add_systems(
                Startup,
                (
                    startup,
                    equation::generate_equations,
                    board::setup_board,
                    player_input::spawn_player_input,
                )
                    .chain(),
            )
            .add_systems(
                Update,
                (
                    player_input::handle_mouse_click,
                    player_input::update_symbol_system,
                    player_input::update_player_text,
                ),
            )
            .add_systems(
                Update,
                player_input::handle_keyboard_input.run_if(keyboard_condition),
            );
    }
}

fn keyboard_condition(taking_keyboard_input: Res<player_input::PlayerKeyboardInput>) -> bool {
    taking_keyboard_input.enabled
}
