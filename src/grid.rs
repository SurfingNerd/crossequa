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
    Unknown, // TODO: Store the solution here
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
            Symbol::Unknown => "x".to_string(),
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
            let op = Self::generate_operation(running_result);
            if i == 0 || running_result.is_none() {
                curr_equation.push(Symbol::Number(op.num1));
            }
            curr_equation.push(Symbol::Operator(op.operator));
            curr_equation.push(Symbol::Number(op.num2));

            running_result = Some(op.result);
        }

        Equation::new(curr_equation, vec![Symbol::Number(running_result.unwrap())])
    }

    fn generate_operation(prev_a: Option<i32>) -> BinaryOperation {
        let mut rng = rand::rng();

        let a = prev_a.unwrap_or_else(|| rng.random_range(1..10));
        let b = rng.random_range(1..10);
        let mut operators = vec![Operator::Add, Operator::Subtract, Operator::Multiply];
        if b != 0 {
            operators.push(Operator::Divide);
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

pub struct GridEquation {
    eq: Equation,
    start_pos: (usize, usize),
    direction: Direction,
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
        let (start_row, start_col) = self.start_pos;
        let len = self.len();
        match self.direction {
            Direction::Horizontal => (start_row, start_col + len - 1),
            Direction::Vertical => (start_row + len - 1, start_col),
        }
    }

    pub fn symbols(&self) -> Vec<Symbol> {
        self.eq.symbols()
    }

    pub fn len(&self) -> usize {
        self.eq.len()
    }

    pub fn get_symbol(&self, position: (usize, usize)) -> Option<&Symbol> {
        let (row, col) = position;
        let (start_row, start_col) = self.start_pos;

        if row < start_row || col < start_col {
            return None;
        }

        let offset_row = row - start_row;
        let offset_col = col - start_col;

        let offset = match self.direction {
            Direction::Horizontal => {
                if offset_row == 0 && offset_col < self.len() {
                    Some(offset_col)
                } else {
                    None
                }
            }
            Direction::Vertical => {
                if offset_col == 0 && offset_row < self.len() {
                    Some(offset_row)
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

    pub fn pos_of_rand_number(&self) -> (usize, usize) {
        let max_n = (self.eq.lhs.len() / 2 + 1) as u32;
        let n = (rand::random::<u32>() % max_n) + 1;

        self.pos_of_nth_number(n as usize).unwrap()
    }

    pub fn pos_of_nth_number(&self, n: usize) -> Option<(usize, usize)> {
        let (start_row, start_col) = self.start_pos;

        let x = self.eq.lhs.clone();
        let mut counter_n = 0;
        let mut counter = 0;
        for e in x {
            counter_n += 1;
            if let Symbol::Number(_) = e {
                counter += 1;
                if counter == n {
                    break;
                }
            }
        }

        if counter_n > self.eq.lhs.len() {
            return None; // Out of bounds
        }

        let (row, col) = match self.direction {
            Direction::Horizontal => (start_row, start_col + counter_n - 1),
            Direction::Vertical => (start_row + counter_n - 1, start_col),
        };

        Some((row, col))
    }

    pub fn contains_point(&self, point: (usize, usize)) -> bool {
        let (start_row, start_col) = self.start_pos;
        let (row, col) = point;

        match self.direction {
            Direction::Horizontal => {
                row == start_row && col >= start_col && col <= start_col + self.len()
            }
            Direction::Vertical => {
                col == start_col && row >= start_row && row <= start_row + self.len()
            }
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

pub fn generate_grid() {
    let mut position = (0, 0);
    let mut dir = Direction::Horizontal;
    let mut running_result = None;
    for _ in 0..10 {
        let g = EquationGenerator::generate_equation(running_result, 1);
        let grid_equation = GridEquation::new(g, position, dir.clone());

        position = grid_equation.pos_of_rand_number();
        dir = match &dir {
            Direction::Horizontal => Direction::Vertical,
            Direction::Vertical => Direction::Horizontal,
        };

        let next_symbol = grid_equation.get_symbol(position).unwrap();
        match next_symbol {
            &Symbol::Number(n) => {
                running_result = Some(n);
            }
            _ => {}
        }

        println!("equation: {}", grid_equation);
    }
}