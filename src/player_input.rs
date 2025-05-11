use crate::board;
use crate::board::Cover;
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct PlayerKeyboardInput {
    pub enabled: bool,
    pub entity: Option<Entity>,
    pub curr_input: String,
    pub target_digit: String,
}

pub fn handle_mouse_click(
    mut commands: Commands,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    children_with_covers: Query<(Entity, &ChildOf), With<Cover>>,
    coordinates: Query<&board::Coordinates>,
    unknowns: Query<(Entity, &board::Unknown), Without<Cover>>,
    mut taking_keyboard_input: ResMut<PlayerKeyboardInput>,
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

        // unknowns
        for (unknown_entity, unknown) in unknowns.iter() {
            if let Ok(coordinate) = coordinates.get(unknown_entity) {
                if coordinate.x == x && coordinate.y == y {
                    taking_keyboard_input.entity = Some(unknown_entity);
                    taking_keyboard_input.target_digit = unknown.number.to_string();
                    taking_keyboard_input.enabled = true;
                    println!(
                        "Clicked on unknown: {:?}, result is {}",
                        unknown_entity, unknown.number
                    );
                    break;
                }
            }
        }

        // untile
        for (child, child_of) in children_with_covers.iter() {
            if let Ok(coordinate) = coordinates.get(child_of.parent()) {
                if coordinate.x == x && coordinate.y == y {
                    commands.entity(child).despawn();
                    break;
                }
            }
        }
    }
}

pub fn handle_keyboard_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut taking_keyboard_input: ResMut<PlayerKeyboardInput>,
    mut event_writer: EventWriter<UpdateSymbolEvent>,
) {
    for key in keyboard_input.get_just_pressed() {
        if *key == KeyCode::Enter {
            if taking_keyboard_input
                .curr_input
                .eq(&taking_keyboard_input.target_digit)
            {
                println!("Correct input: {}", taking_keyboard_input.curr_input);

                // Change Unknown to Number
                event_writer.write(UpdateSymbolEvent {
                    entity: taking_keyboard_input.entity.unwrap(),
                });
            } else {
                println!(
                    "Incorrect input, got {}, expected {}",
                    taking_keyboard_input.curr_input, taking_keyboard_input.target_digit
                );
            }

            taking_keyboard_input.curr_input.clear();
            taking_keyboard_input.target_digit.clear();
            taking_keyboard_input.entity = None;
            taking_keyboard_input.enabled = false;
            return;
        }

        if let Some(digit) = key_to_digit(*key) {
            taking_keyboard_input.curr_input.push(digit);
            println!("Got digit: {}", digit);
        }
    }
}

fn key_to_digit(key: KeyCode) -> Option<char> {
    match key {
        KeyCode::Minus => Some('-'),
        KeyCode::Digit0 => Some('0'),
        KeyCode::Digit1 => Some('1'),
        KeyCode::Digit2 => Some('2'),
        KeyCode::Digit3 => Some('3'),
        KeyCode::Digit4 => Some('4'),
        KeyCode::Digit5 => Some('5'),
        KeyCode::Digit6 => Some('6'),
        KeyCode::Digit7 => Some('7'),
        KeyCode::Digit8 => Some('8'),
        KeyCode::Digit9 => Some('9'),
        _ => None,
    }
}

#[derive(Event)]
pub struct UpdateSymbolEvent {
    pub entity: Entity,
}

pub fn update_symbol_system(
    mut commands: Commands,
    mut event_reader: EventReader<UpdateSymbolEvent>,
    mut unknowns: Query<(Entity, &board::Unknown)>,
) {
    for event in event_reader.read() {
        if let Ok((entity, unknown)) = unknowns.get_mut(event.entity) {
            // Update the entity's symbol to Symbol::Number
            commands.entity(entity).remove::<board::Unknown>();
            commands
                .entity(entity)
                .insert(Text2d::new(unknown.number.to_string()));

            println!(
                "Updated entity {:?} from Unknown to Symbol::Number({})",
                entity, unknown.number
            );
        }
    }
}

#[derive(Component)]
pub struct PlayerText;

pub fn spawn_player_input(mut commands: Commands) {
    commands.spawn((
        PlayerText,
        Text::new("Input: "),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(15.0),
            left: Val::Px(15.0),
            ..default()
        },
    ));
}

pub fn update_player_text(
    mut query: Query<&mut Text, With<PlayerText>>,
    taking_keyboard_input: Res<PlayerKeyboardInput>,
) {
    for mut span in &mut query {
        **span = format!("Input: {}", taking_keyboard_input.curr_input);
    }
}
