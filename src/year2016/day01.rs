use crate::utils::point::Point;

type Input = Vec<Instruction>;

enum Action {
    Left,
    Right,
}

impl Action {
    fn parse(c: char) -> Self {
        match c {
            'R' => Self::Right,
            'L' => Self::Left,
            _ => unreachable!(),
        }
    }
}

pub struct Instruction {
    action: Action,
    distance: i32,
}

impl Instruction {
    fn parse(txt: &str) -> Self {
        let action = Action::parse(txt.chars().next().unwrap());
        let distance: i32 = txt[1..].parse().unwrap();
        Self { action, distance }
    }
}

pub fn parse(input: &str) -> Input {
    input.split(", ").map(Instruction::parse).collect()
}

pub fn part1(input: &Input) -> i32 {
    input
        .iter()
        .fold((Point::ORIGIN, Point::UP), |(pos, dir), instruction| {
            let dir = match instruction.action {
                Action::Left => dir.rotated_anticlockwise(),
                Action::Right => dir.rotated_clockwise(),
            };
            (pos + dir * instruction.distance, dir)
        })
        .0
        .norm1()
}

pub fn part2(input: &Input) -> usize {
    input.len()
}
