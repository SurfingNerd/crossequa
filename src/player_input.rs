use crate::board;
use crate::board::Cover;
use bevy::prelude::*;

pub fn handle_mouse_click(
    mut commands: Commands,
    children: Query<(Entity, &ChildOf), With<Cover>>,
    coordinates: Query<&board::Coordinates>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
) {
    let half_win_size = window.size() / 2.0;

    if mouse_button_input.just_pressed(MouseButton::Left) {
        let mut cursor_position_centered = window.cursor_position().unwrap() - half_win_size;
        cursor_position_centered.y *= -1.0; // Invert y-axis for the Bevy coordinate system

        // TODO: Put this in a a constant/resource
        let grid_size = 60;
        let grid_padding = 14;

        let cell_span = (grid_size + grid_padding) as f32;
        let x = ((cursor_position_centered.x + cell_span / 2.0) / cell_span).floor() as i32;
        let y = ((cursor_position_centered.y + cell_span / 2.0) / cell_span).floor() as i32;

        for (child, child_of) in children.iter() {
            if let Ok(coordinate) = coordinates.get(child_of.parent()) {
                if coordinate.x == x && coordinate.y == y {
                    commands.entity(child).despawn();
                    break;
                }
            }
        }
    }
}
