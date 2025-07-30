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

#[derive(Debug, Clone)]
pub enum Operator {
    And(Operand, Operand),
    Or(Operand, Operand),
    LShift(Operand, Operand),
    RShift(Operand, Operand),
    Not(Operand),
    Identity(Operand),
}

impl Operand {
    fn parse(operand: &str) -> Self {
        match operand.parse::<u16>() {
            Ok(literal) => Operand::Literal(literal),
            Err(_) => Operand::Wire(operand.into()),
        }
    }
}

pub fn parse(input: &str) -> Connections {
    let mut connections = HashMap::new();
    for line in input.lines() {
        let (expression, wire) = line.split_once(" -> ").unwrap();
        let expression: Vec<_> = expression.split_whitespace().collect();
        let operator = match expression.len() {
            1 => Operator::Identity(Operand::parse(expression[0])),
            2 => Operator::Not(Operand::parse(expression[1])),
            3 => {
                let lhs = Operand::parse(expression[0]);
                let rhs = Operand::parse(expression[2]);
                #[allow(clippy::match_on_vec_items)]
                match expression[1] {
                    "AND" => Operator::And(lhs, rhs),
                    "OR" => Operator::Or(lhs, rhs),
                    "LSHIFT" => Operator::LShift(lhs, rhs),
                    "RSHIFT" => Operator::RShift(lhs, rhs),
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        };
        connections.insert(wire.into(), operator);
    }
    connections
}

fn evaluate_operand(
    cache: &mut HashMap<String, u16>,
    connections: &Connections,
    operand: &Operand,
) -> u16 {
    match operand {
        Operand::Literal(value) => *value,
        Operand::Wire(wire) => {
            if let Some(value) = cache.get(wire) {
                *value
            } else {
                let value = evaluate_operator(cache, connections, connections.get(wire).unwrap());
                cache.insert(wire.clone(), value);
                value
            }
        }
    }
}

fn evaluate_operator(
    cache: &mut HashMap<String, u16>,
    connections: &Connections,
    operator: &Operator,
) -> u16 {
    match operator {
        Operator::Identity(operand) => evaluate_operand(cache, connections, operand),
        Operator::Not(operand) => !evaluate_operand(cache, connections, operand),
        Operator::And(lhs, rhs) => {
            evaluate_operand(cache, connections, lhs) & evaluate_operand(cache, connections, rhs)
        }
        Operator::Or(lhs, rhs) => {
            evaluate_operand(cache, connections, lhs) | evaluate_operand(cache, connections, rhs)
        }
        Operator::LShift(lhs, rhs) => {
            evaluate_operand(cache, connections, lhs) << evaluate_operand(cache, connections, rhs)
        }
        Operator::RShift(lhs, rhs) => {
            evaluate_operand(cache, connections, lhs) >> evaluate_operand(cache, connections, rhs)
        }
    }
}

pub fn part1(connections: &Connections) -> u16 {
    let mut cache = HashMap::new();
    evaluate_operand(&mut cache, connections, &Operand::Wire("a".into()))
}

pub fn part2(connections: &Connections) -> u16 {
    let signal = part1(connections);
    let mut connections = connections.clone();
    connections.insert("b".into(), Operator::Identity(Operand::Literal(signal)));
    let mut cache = HashMap::new();
    evaluate_operand(&mut cache, &connections, &Operand::Wire("a".into()))
}
