use crate::equation::Symbol;
use bevy::math::prelude::*;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinates {
    pub x: i32,
    pub y: i32,
}

impl Coordinates {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CellSize {
    pub size: u32,
    pub padding: u32,
}

#[derive(Debug, Resource)]
pub struct Board {
    pub grid: Vec<Vec<Symbol>>,
    pub cell_size: CellSize,
    pub covered_tiles: HashMap<Coordinates, Entity>,
}

#[derive(Component)]
pub struct Cover;

pub fn setup_board(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let grid_size = 10;
    let grid = vec![vec![Symbol::Empty; grid_size]; grid_size];
    let cell_size = CellSize {
        size: 60,
        padding: 14,
    };

    commands.insert_resource(Board {
        grid,
        cell_size,
        covered_tiles: HashMap::new(),
    });

    for x in 0..grid_size {
        for y in 0..grid_size {
            let x_pos = (cell_size.size + cell_size.padding) as f32 * x as f32;
            let y_pos = (cell_size.size + cell_size.padding) as f32 * y as f32;

            commands
                .spawn((
                    Name::new(format!("Symbol ({}, {})", x, y)),
                    Coordinates::new(x as i32, y as i32),
                    Text2d::new("foo".to_string()),
                    TextShadow::default(),
                    Transform::from_xyz(x_pos, y_pos, 0.0),
                ))
                .with_children(|child_builder| {
                    child_builder.spawn((
                        Name::new(format!("Cell ({}, {})", x, y)),
                        Mesh2d(meshes.add(Rectangle {
                            half_size: Vec2::new(30.0, 30.0),
                        })),
                        MeshMaterial2d(materials.add(Color::srgb(0.9, 0.8, 0.6))),
                        Transform::from_xyz(0.0, 0.0, 1.0),
                        Cover,
                    ));
                });
        }
    }
}
