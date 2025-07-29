use gxhash::{
    HashSet,
    HashSetExt,
};

use crate::utils::point::Point;

type Input = String;

pub fn parse(input: &str) -> Input {
    input.into()
}

pub fn part1(input: &Input) -> usize {
    let mut position = Point::ORIGIN;
    let mut visited = HashSet::new();
    visited.insert(position);
    for c in input.chars() {
        match c {
            '^' => position += Point::UP,
            'v' => position += Point::DOWN,
            '>' => position += Point::RIGHT,
            '<' => position += Point::LEFT,
            _ => unreachable!(),
        }
        visited.insert(position);
    }
    visited.len()
}

pub fn part2(input: &Input) -> usize {
    let mut positions = [Point::ORIGIN, Point::ORIGIN];
    let mut visited = HashSet::new();
    visited.insert(positions[0]);

    for (i, c) in input.chars().enumerate() {
        let idx = i % 2;
        match c {
            '^' => positions[idx] += Point::UP,
            'v' => positions[idx] += Point::DOWN,
            '>' => positions[idx] += Point::RIGHT,
            '<' => positions[idx] += Point::LEFT,
            _ => unreachable!(),
        }
        visited.insert(positions[idx]);
    }
    visited.len()
}
