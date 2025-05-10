use bevy::prelude::*;
use rand::Rng;
use rand::prelude::IndexedRandom;
use symbolica::*;
use symbolica::{atom::AtomCore, parse};

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            Operator::Add => "+",
            Operator::Subtract => "-",
            Operator::Multiply => "*",
            Operator::Divide => "/",
        };
        write!(f, "{}", symbol)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Cell {
    Number(i32),
    Operator(Operator),
    Equals,
    Unknown, // TODO: Store the solution here
    Empty,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Cell::Number(n) => format!("{n}"),
            Cell::Operator(Operator::Add) => "+".to_string(),
            Cell::Operator(Operator::Subtract) => "-".to_string(),
            Cell::Operator(Operator::Multiply) => "*".to_string(),
            Cell::Operator(Operator::Divide) => "/".to_string(),
            Cell::Equals => "=".to_string(),
            Cell::Unknown => "x".to_string(),
            Cell::Empty => " ".to_string(),
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    Horizontal,
    Vertical,
}

pub fn generate_grid() {
    let rows = 10;
    let cols = 10;
    let num_equations = 6;

    let grid = generate_crossword_grid(rows, cols, num_equations);
    print_grid(&grid);
}

pub fn print_grid(grid: &[Vec<Cell>]) {
    for row in grid {
        for cell in row {
            print!("{:>2} ", cell); // Adjust spacing for alignment
        }
        println!();
    }
}

fn generate_crossword_grid(rows: usize, cols: usize, num_equations: usize) -> Vec<Vec<Cell>> {
    let mut grid = vec![vec![Cell::Empty; cols]; rows];
    let mut rng = rand::rng();

    let directions = [Direction::Horizontal, Direction::Vertical];

    let mut attempts = 0;
    let mut placed = 0;

    while placed < num_equations && attempts < num_equations * 100 {
        let dir = directions.choose(&mut rng).unwrap().clone();
        let row = rng.random_range(0..rows);
        let col = rng.random_range(0..cols);

        if !can_place_equation(&grid, row, col, &dir) {
            attempts += 1;
            continue;
        }

        let equation = generate_equation_with_unknown();

        if place_equation(&mut grid, row, col, &dir, &equation) {
            placed += 1;
        } else {
            attempts += 1;
        }
    }

    grid
}

pub fn generate_equation_with_unknown() -> Vec<Cell> {
    let mut rng = rand::rng();
    let operators = [
        Operator::Add,
        Operator::Subtract,
        Operator::Multiply,
        Operator::Divide,
    ];
    let op = operators.choose(&mut rng).unwrap().clone();

    let (a, b, c) = match op {
        Operator::Add => {
            let a = rng.random_range(1..10);
            let b = rng.random_range(1..10);
            (a, b, a + b)
        }
        Operator::Subtract => {
            let a = rng.random_range(1..10);
            let b = rng.random_range(1..10);
            (a, b, a - b)
        }
        Operator::Multiply => {
            let a = rng.random_range(1..10);
            let b = rng.random_range(1..10);
            (a, b, a * b)
        }
        Operator::Divide => {
            let b = rng.random_range(1..10);
            let c = rng.random_range(1..10);
            let a = b * c;
            (a, b, c)
        }
    };

    let mut cells = vec![
        Cell::Number(a),
        Cell::Operator(op),
        Cell::Number(b),
        Cell::Equals,
        Cell::Number(c),
    ];

    // Replace one random non-operator, non-equals Cell with Unknown
    let possible_indices = [0, 2, 4];
    let &replace_idx = possible_indices.choose(&mut rng).unwrap();
    cells[replace_idx] = Cell::Unknown;

    cells
}

fn can_place_equation(
    grid: &Vec<Vec<Cell>>,
    row: usize,
    col: usize,
    direction: &Direction,
) -> bool {
    let (dr, dc) = match direction {
        Direction::Horizontal => (0, 1),
        Direction::Vertical => (1, 0),
    };

    // Ensure 5 cells fit in the grid
    for i in 0..5 {
        let r = row + i * dr;
        let c = col + i * dc;
        if r >= grid.len() || c >= grid[0].len() {
            return false;
        }
        if grid[r][c] != Cell::Empty {
            return false; // Or relax this check to allow matching overlapping content
        }
    }
    true
}

fn place_equation(
    grid: &mut Vec<Vec<Cell>>,
    row: usize,
    col: usize,
    direction: &Direction,
    equation: &[Cell],
) -> bool {
    let (dr, dc) = match direction {
        Direction::Horizontal => (0, 1),
        Direction::Vertical => (1, 0),
    };

    for i in 0..5 {
        let r = row + i * dr;
        let c = col + i * dc;
        match grid[r][c] {
            Cell::Empty => grid[r][c] = equation[i].clone(),
            ref existing => {
                if *existing != equation[i] {
                    return false;
                }
            }
        }
    }

    true
}

fn equation_solver(lhs: &str, rhs: &str) {
    let expr = parse!(lhs).unwrap() - parse!(rhs).unwrap();
    let system = &[expr];
    let vars = &[parse!("x").unwrap()];
    let solutions = atom::Atom::solve_linear_system::<u8, _, _>(system, vars).unwrap();

    solutions.iter().for_each(|atom| {
        println!("{}", atom.as_view().to_string());
    });
}
