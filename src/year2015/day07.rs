use gxhash::{
    HashMap,
    HashMapExt,
};

type Connections = HashMap<String, Operator>;

#[derive(Debug, Clone)]
pub enum Operand {
    Literal(u16),
    Wire(String),
}

impl Operand {
    fn parse(operand: &str) -> Self {
        match operand.parse::<u16>() {
            Ok(literal) => Operand::Literal(literal),
            Err(_) => Operand::Wire(operand.into()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Operator {
    And(Operand, Operand),
    Or(Operand, Operand),
    LShift(Operand, Operand),
    RShift(Operand, Operand),
    Not(Operand),
    Identity(Operand),
}

impl Operator {
    fn parse(expression: &str) -> Self {
        let expression: Vec<_> = expression.split_ascii_whitespace().collect();
        match expression.as_slice() {
            [a] => Operator::Identity(Operand::parse(a)),
            ["NOT", a] => Operator::Not(Operand::parse(a)),
            [a, "AND", b] => Operator::And(Operand::parse(a), Operand::parse(b)),
            [a, "OR", b] => Operator::Or(Operand::parse(a), Operand::parse(b)),
            [a, "LSHIFT", b] => Operator::LShift(Operand::parse(a), Operand::parse(b)),
            [a, "RSHIFT", b] => Operator::RShift(Operand::parse(a), Operand::parse(b)),
            _ => unreachable!(),
        }
    }
}

pub struct Circuit {
    pub connections: Connections,
    pub cache: HashMap<String, u16>,
}

impl Circuit {
    fn new(connections: Connections) -> Self {
        Self {
            connections,
            cache: HashMap::new(),
        }
    }

    fn eval_operand(&mut self, operand: &Operand) -> u16 {
        match operand {
            Operand::Literal(value) => *value,
            Operand::Wire(wire) => {
                if let Some(value) = self.cache.get(wire) {
                    *value
                } else {
                    let operator = self.connections[wire].clone();
                    let value = self.eval_operator(&operator);
                    self.cache.insert(wire.clone(), value);
                    value
                }
            }
        }
    }

    fn eval_operator(&mut self, operator: &Operator) -> u16 {
        match operator {
            Operator::Identity(operand) => self.eval_operand(operand),
            Operator::Not(operand) => !self.eval_operand(operand),
            Operator::And(lhs, rhs) => self.eval_operand(lhs) & self.eval_operand(rhs),
            Operator::Or(lhs, rhs) => self.eval_operand(lhs) | self.eval_operand(rhs),
            Operator::LShift(lhs, rhs) => self.eval_operand(lhs) << self.eval_operand(rhs),
            Operator::RShift(lhs, rhs) => self.eval_operand(lhs) >> self.eval_operand(rhs),
        }
    }

    pub fn signal(&mut self, wire: &str) -> u16 {
        self.eval_operand(&Operand::Wire(wire.into()))
    }

    pub fn override_wire(&mut self, wire: &str, value: u16) {
        self.connections
            .insert(wire.into(), Operator::Identity(Operand::Literal(value)));
    }
}

pub fn parse(input: &str) -> Connections {
    let mut connections = HashMap::new();
    for line in input.lines() {
        let (expression, wire) = line.split_once(" -> ").unwrap();
        connections.insert(wire.into(), Operator::parse(expression));
    }
    connections
}

pub fn part1(connections: &Connections) -> u16 {
    let mut circuit = Circuit::new(connections.clone());
    circuit.signal("a")
}

pub fn part2(connections: &Connections) -> u16 {
    let mut circuit = Circuit::new(connections.clone());
    let signal = circuit.signal("a");
    circuit.override_wire("b", signal);
    circuit.cache.clear();
    circuit.signal("a")
}
