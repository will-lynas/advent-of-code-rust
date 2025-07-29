use std::ops::Not;

use regex::Regex;

use crate::utils::{
    grid::Grid,
    point::Point,
};

pub struct Instruction {
    action: Action,
    p1: Point,
    p2: Point,
}

enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Default, Clone, Copy, PartialEq)]
enum State {
    On,
    #[default]
    Off,
}

impl Not for State {
    type Output = Self;
    fn not(self) -> Self {
        match self {
            State::On => State::Off,
            State::Off => State::On,
        }
    }
}

pub fn parse(input: &str) -> Vec<Instruction> {
    let re =
        Regex::new(r"\b(turn on|turn off|toggle)\s*(\d+),(\d+)\s*through\s*(\d+),(\d+)\b").unwrap();
    input
        .lines()
        .map(|line| {
            let captures = re.captures(line).unwrap();
            let instruction = &captures[1];
            let x1 = captures[2].parse().unwrap();
            let y1 = captures[3].parse().unwrap();
            let x2 = captures[4].parse().unwrap();
            let y2 = captures[5].parse().unwrap();
            let p1 = Point::new(x1, y1);
            let p2 = Point::new(x2, y2);
            let action = match instruction {
                "turn on" => Action::TurnOn,
                "turn off" => Action::TurnOff,
                "toggle" => Action::Toggle,
                _ => unreachable!(),
            };
            Instruction { action, p1, p2 }
        })
        .collect()
}

pub fn part1(input: &[Instruction]) -> usize {
    let mut grid: Grid<State> = Grid::new(1000, 1000);
    for instruction in input {
        for point in instruction.p1.between(instruction.p2) {
            match instruction.action {
                Action::TurnOn => grid[point] = State::On,
                Action::TurnOff => grid[point] = State::Off,
                Action::Toggle => grid[point] = !grid[point],
            }
        }
    }
    grid.iter()
        .filter(|&(_, &state)| state == State::On)
        .count()
}

pub fn part2(input: &[Instruction]) -> i32 {
    let mut grid: Grid<i32> = Grid::new(1000, 1000);
    for instruction in input {
        for point in instruction.p1.between(instruction.p2) {
            match instruction.action {
                Action::TurnOn => grid[point] += 1,
                Action::Toggle => grid[point] += 2,
                Action::TurnOff => {
                    if grid[point] > 0 {
                        grid[point] -= 1;
                    }
                }
            }
        }
    }
    grid.iter().map(|(_, &brightness)| brightness).sum()
}
