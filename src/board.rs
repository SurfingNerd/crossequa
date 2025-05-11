use crate::equation;
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
    equations: Res<equation::GridEquations>,
) {
    let grid_size = 10;
    let cell_size = CellSize {
        size: 60,
        padding: 14,
    };

    let mut grid = vec![vec![Symbol::Empty; grid_size]; grid_size];
    for eq in equations.iter() {
        let symbols = eq.symbols();
        let pos = eq.start_pos;
        let dir = eq.direction.clone();
        for (i, symbol) in symbols.iter().enumerate() {
            let (x, y) = match dir {
                equation::Direction::Horizontal => (pos.0 + i, pos.1),
                equation::Direction::Vertical => (pos.0, pos.1 + i),
            };
            grid[x as usize][y as usize] = symbol.clone();
        }
    }

    for x in 0..grid_size {
        for y in 0..grid_size {
            let symbol = &grid[x as usize][y as usize];
            let x_pos = (cell_size.size + cell_size.padding) as f32 * x as f32;
            let y_pos = (cell_size.size + cell_size.padding) as f32 * y as f32;

            commands
                .spawn((
                    Name::new(format!("({}, {}) symbol '{}'", x, y, symbol.to_string())),
                    Coordinates::new(x as i32, y as i32),
                    Text2d::new(symbol.to_string()),
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

    commands.insert_resource(Board {
        grid,
        cell_size,
        covered_tiles: HashMap::new(),
    });
}
