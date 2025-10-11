use crate::utils::{
    direction::Direction,
    point::Point,
};

type Input = Vec<Instruction>;

enum Action {
    Left,
    Right,
}

impl From<&str> for Action {
    fn from(s: &str) -> Self {
        match s {
            "L" => Self::Left,
            "R" => Self::Right,
            _ => unreachable!(),
        }
    }
}

pub struct Instruction {
    action: Action,
    distance: i32,
}

impl From<&str> for Instruction {
    fn from(txt: &str) -> Self {
        Self {
            action: txt[0..1].into(),
            distance: txt[1..].parse().unwrap(),
        }
    }
}

pub fn parse(input: &str) -> Input {
    input.split(", ").map(Instruction::from).collect()
}

pub fn part1(input: &Input) -> i32 {
    input
        .iter()
        .fold((Point::ORIGIN, Point::UP), |(pos, dir), instruction| {
            let dir = match instruction.action {
                Action::Left => dir.rotated(Direction::Anticlockwise),
                Action::Right => dir.rotated(Direction::Clockwise),
            };
            (pos + dir * instruction.distance, dir)
        })
        .0
        .norm1()
}

pub fn part2(input: &Input) -> usize {
    input.len()
}
