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

// #[derive(Reflect, Resource, Default, InspectorOptions, Deref)]
// #[reflect(Resource, InspectorOptions)]
#[derive(Resource, Deref)]
pub struct GridEquations(pub Vec<GridEquation>);

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

    pub fn pos_of_rand_number(&self) -> (usize, usize) {
        let max_n = (self.eq.lhs.len() / 2 + 1) as u32;
        let n = (rand::random::<u32>() % max_n) + 1;

        self.pos_of_nth_number(n as usize).unwrap()
    }

    pub fn pos_of_nth_number(&self, n: usize) -> Option<(usize, usize)> {
        let (start_x, start_y) = self.start_pos;

        let mut counter_n = 0;
        let mut counter = 0;
        for sym in self.eq.lhs.clone() {
            counter_n += 1;
            if let Symbol::Number(_) = sym {
                counter += 1;
                if counter == n {
                    break;
                }
            }
        }

        if counter_n > self.eq.lhs.len() {
            return None; // Out of bounds
        }

        let (x, y) = match self.direction {
            Direction::Horizontal => (start_x + counter_n - 1, start_y),
            Direction::Vertical => (start_x, start_y + counter_n - 1),
        };

        Some((x, y))
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

pub fn generate_grid(mut commands: Commands) {
    let mut position = (0, 0);
    let mut dir = Direction::Horizontal;
    let mut running_result = None;
    let mut grid_equations = Vec::new();

    for _ in 0..10 {
        let equation = EquationGenerator::generate_equation(running_result, 1);
        let grid_equation = GridEquation::new(equation, position, dir.clone());

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
        grid_equations.push(grid_equation);
    }

    commands.insert_resource(GridEquations(grid_equations));
}
