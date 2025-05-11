use bevy::prelude::*;
use rand::Rng;
use rand::prelude::IndexedRandom;

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

pub struct BinaryOperation {
    pub num1: i32,
    pub operator: Operator,
    pub num2: i32,
    pub result: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Symbol {
    Number(i32),
    Operator(Operator),
    Equals,
    Unknown(i32),
    Empty,
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Symbol::Number(n) => format!("{n}"),
            Symbol::Operator(Operator::Add) => "+".to_string(),
            Symbol::Operator(Operator::Subtract) => "-".to_string(),
            Symbol::Operator(Operator::Multiply) => "*".to_string(),
            Symbol::Operator(Operator::Divide) => "/".to_string(),
            Symbol::Equals => "=".to_string(),
            Symbol::Unknown(_) => "x".to_string(),
            Symbol::Empty => " ".to_string(),
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Equation {
    lhs: Vec<Symbol>,
    rhs: Vec<Symbol>,
}

impl Equation {
    pub fn new(lhs: Vec<Symbol>, rhs: Vec<Symbol>) -> Self {
        Self { lhs, rhs }
    }

    pub fn nth(&self, n: usize) -> Option<&Symbol> {
        if n < self.lhs.len() {
            self.lhs.get(n)
        } else {
            self.rhs.get(n - self.lhs.len())
        }
    }

    pub fn len(&self) -> usize {
        self.lhs.len() + self.rhs.len() + 1 // +1 for the equals sign
    }

    pub fn symbols(&self) -> Vec<Symbol> {
        let mut full_equation = self.lhs.clone();
        full_equation.push(Symbol::Equals);
        full_equation.extend(self.rhs.clone());
        full_equation
    }
}

pub struct EquationGenerator {}

impl EquationGenerator {
    pub fn generate_equation(running_result: Option<i32>, num_operations: u32) -> Equation {
        let mut curr_equation: Vec<Symbol> = Vec::new();
        let mut running_result = running_result;

        for i in 0..num_operations {
            let op = Self::generate_operation(i, running_result);
            if i == 0 || running_result.is_none() {
                curr_equation.push(Symbol::Number(op.num1));
            }
            curr_equation.push(Symbol::Operator(op.operator));
            curr_equation.push(Symbol::Number(op.num2));

            running_result = Some(op.result);
        }

        Equation::new(curr_equation, vec![Symbol::Number(running_result.unwrap())])
    }

    fn generate_operation(i: u32, prev_a: Option<i32>) -> BinaryOperation {
        let mut rng = rand::rng();

        let a = prev_a.unwrap_or_else(|| rng.random_range(1..10));
        let b = rng.random_range(1..10);
        let mut operators = vec![Operator::Add, Operator::Subtract];
        if i == 0 {
            operators.push(Operator::Multiply);
            if b != 0 {
                operators.push(Operator::Divide);
            }
        }
        let op = operators.choose(&mut rng).unwrap();

        let (num1, num2, result) = match op {
            Operator::Add => (a, b, a + b),
            Operator::Subtract => (a, b, a - b),
            Operator::Multiply => (a, b, a * b),
            Operator::Divide => (a, b, a / b),
        };

        return BinaryOperation {
            num1,
            operator: op.clone(),
            num2,
            result,
        };
    }
}

// #[derive(Reflect, Resource, Default, InspectorOptions, Deref)]
// #[reflect(Resource, InspectorOptions)]
#[derive(Resource, Deref)]
pub struct GridEquations(pub Vec<GridEquation>);

#[derive(Debug, Clone, PartialEq)]
pub struct GridEquation {
    pub eq: Equation,
    pub start_pos: (usize, usize),
    pub direction: Direction,
}

impl GridEquation {
    pub fn new(eq: Equation, start_pos: (usize, usize), direction: Direction) -> Self {
        Self {
            eq,
            start_pos,
            direction,
        }
    }

    pub fn end_pos(&self) -> (usize, usize) {
        let (start_x, start_y) = self.start_pos;
        let len = self.len();
        match self.direction {
            Direction::Horizontal => (start_x + len - 1, start_y),
            Direction::Vertical => (start_x, start_y + len - 1),
        }
    }

    pub fn symbols(&self) -> Vec<Symbol> {
        self.eq.symbols()
    }

    pub fn len(&self) -> usize {
        self.eq.len()
    }

    pub fn get_symbol_idx(&self, idx: usize) -> &Symbol {
        self.eq.lhs.get(idx).unwrap()
    }

    pub fn get_symbol(&self, position: (usize, usize)) -> Option<&Symbol> {
        let (x, y) = position;
        let (start_x, start_y) = self.start_pos;

        if x < start_x || y < start_y {
            return None;
        }

        let offset_x = x - start_x;
        let offset_y = y - start_y;

        let offset = match self.direction {
            Direction::Horizontal => {
                if offset_y == 0 && offset_x < self.len() {
                    Some(offset_x)
                } else {
                    None
                }
            }
            Direction::Vertical => {
                if offset_x == 0 && offset_y < self.len() {
                    Some(offset_y)
                } else {
                    None
                }
            }
        };

        if let Some(offset) = offset {
            return self.eq.nth(offset);
        }

        None
    }

    pub fn set_symbol(&mut self, position: (usize, usize), symbol: Symbol) {
        let (x, y) = position;
        let (start_x, start_y) = self.start_pos;

        if x < start_x || y < start_y {
            return;
        }

        let offset_x = x - start_x;
        let offset_y = y - start_y;

        let offset = match self.direction {
            Direction::Horizontal => {
                if offset_y == 0 && offset_x < self.len() {
                    Some(offset_x)
                } else {
                    None
                }
            }
            Direction::Vertical => {
                if offset_x == 0 && offset_y < self.len() {
                    Some(offset_y)
                } else {
                    None
                }
            }
        };

        if let Some(offset) = offset {
            self.eq.lhs[offset] = symbol;
        }
    }

    pub fn set_symbol_idx(&mut self, idx: usize, symbol: Symbol) {
        self.eq.lhs[idx] = symbol;
    }

    pub fn pos_of_rand_number(&self) -> (usize, usize) {
        let max_n = (self.eq.lhs.len() / 2 + 1) as u32;
        let n = (rand::random::<u32>() % max_n) + 1;

        self.pos_of_nth_number(n as usize).unwrap()
    }

    pub fn pos_of_rand_number_with_n(&self) -> (usize, usize, usize) {
        let max_n = (self.eq.lhs.len() / 2 + 1) as u32;
        let n = (rand::random::<u32>() % max_n) + 1;

        self.pos_of_nth_number_with_idx(n as usize).unwrap()
    }

    pub fn pos_of_nth_number(&self, n: usize) -> Option<(usize, usize)> {
        let (start_x, start_y) = self.start_pos;

        let mut counter_idx = 0;
        let mut counter = 0;
        for sym in self.eq.lhs.clone() {
            counter_idx += 1;
            if let Symbol::Number(_) = sym {
                counter += 1;
                if counter == n {
                    break;
                }
            }
        }

        counter_idx -= 1;

        if counter_idx >= self.eq.lhs.len() {
            return None; // Out of bounds
        }

        let (x, y) = match self.direction {
            Direction::Horizontal => (start_x + counter_idx, start_y),
            Direction::Vertical => (start_x, start_y + counter_idx),
        };

        Some((x, y))
    }

    pub fn pos_of_nth_number_with_idx(&self, n: usize) -> Option<(usize, usize, usize)> {
        let (start_x, start_y) = self.start_pos;

        let mut counter_idx = 0;
        let mut counter = 0;
        for sym in self.eq.lhs.clone() {
            counter_idx += 1;
            if let Symbol::Number(_) = sym {
                counter += 1;
                if counter == n {
                    break;
                }
            }
        }

        counter_idx -= 1;

        if counter_idx >= self.eq.lhs.len() {
            return None; // Out of bounds
        }

        let (x, y) = match self.direction {
            Direction::Horizontal => (start_x + counter_idx, start_y),
            Direction::Vertical => (start_x, start_y + counter_idx),
        };

        Some((x, y, counter_idx))
    }

    pub fn contains_point(&self, point: (usize, usize)) -> bool {
        let (start_x, start_y) = self.start_pos;
        let (x, y) = point;

        match self.direction {
            Direction::Horizontal => y == start_y && x >= start_x && x <= start_x + self.len(),
            Direction::Vertical => x == start_x && y >= start_y && y <= start_y + self.len(),
        }
    }
}

impl std::fmt::Display for GridEquation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbols = self.symbols();
        let s = symbols
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        write!(f, "({:?}) - ({:?}): {}", self.start_pos, self.end_pos(), s)
    }
}

pub fn generate_equations(mut commands: Commands) {
    let mut rng = rand::rng();
    let mut position = (0, 0);
    let mut dir = Direction::Horizontal;
    let mut running_result = None;
    let mut grid_equations = Vec::new();

    for _ in 0..20 {
        let equation = EquationGenerator::generate_equation(running_result, 2);
        let grid_equation = GridEquation::new(equation.clone(), position, dir.clone());

        let nth_num = rng.random_range(2..=3);
        position = grid_equation.pos_of_nth_number(nth_num).unwrap();
        dir = match &dir {
            Direction::Horizontal => Direction::Vertical,
            Direction::Vertical => Direction::Horizontal,
        };

        let next_symbol = grid_equation.get_symbol(position).unwrap();
        if let Symbol::Number(n) = next_symbol {
            running_result = Some(*n);
        } else {
            running_result = None;
        }

        println!("equation: {}", grid_equation);
        grid_equations.push(grid_equation);
    }

    set_unknowns_in_equations(&mut grid_equations);

    commands.insert_resource(GridEquations(grid_equations));
}

// // Returns true if the new equation overlaps with 2 or more tiles with one of the existing equations
// pub fn is_grid_equation_valid(grid_equations: &Vec<GridEquation>, new_eq: &GridEquation) -> bool {
//     let mut overlap_count = 0;

//     for eq in grid_equations.iter() {
//         let curr_dir = match eq.direction {
//             Direction::Horizontal => (1, 0),
//             Direction::Vertical => (0, 1),
//         };
//         let mut curr_pos = eq.start_pos;

//         for _ in 0..eq.len() {
//             overlap_count += new_eq.contains_point(curr_pos) as usize;
//             curr_pos.0 += curr_dir.0;
//             curr_pos.1 += curr_dir.1;
//         }
//     }

//     overlap_count <= 1
// }

// fn are_two_equations_non_valid_overlapping(eq1: &GridEquation, eq2: &GridEquation) -> bool {
//     let mut overlap_count = 0;
//     let curr_dir = match eq1.direction {
//         Direction::Horizontal => (1, 0),
//         Direction::Vertical => (0, 1),
//     };
//     let mut curr_pos = eq1.start_pos;

//     for _ in 0..eq1.len() {
//         overlap_count += eq2.contains_point(curr_pos) as usize;
//         curr_pos.0 += curr_dir.0;
//         curr_pos.1 += curr_dir.1;
//     }

//     return overlap_count >= 2;
// }

pub fn set_unknowns_in_equations(grid_equations: &mut Vec<GridEquation>) {
    let mut updated_positions = Vec::new();

    for grid_eq in grid_equations.iter_mut() {
        // Find a position of a number to convert to `Unknown`
        let (x, y, idx) = grid_eq.pos_of_rand_number_with_n();
        // Check if this position is already updated in another equation
        if updated_positions.contains(&(x, y)) {
            continue;
        }

        let curr_symbol = grid_eq.get_symbol_idx(idx as usize);
        if let Symbol::Number(n) = curr_symbol {
            grid_eq.set_symbol_idx(idx as usize, Symbol::Unknown(*n));
            println!("Set unknown to: ({}, {}), {}", x, y, grid_eq);
            updated_positions.push((x, y));
        } else {
            println!("Did not find number: {}", curr_symbol);
        }
    }
}
