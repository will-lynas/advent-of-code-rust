use gxhash::{
    HashSet,
    HashSetExt,
};

use crate::utils::point::{
    DOWN,
    LEFT,
    ORIGIN,
    RIGHT,
    UP,
};

type Input = String;

pub fn parse(input: &str) -> Input {
    input.into()
}

pub fn part1(input: &Input) -> usize {
    let mut position = ORIGIN;
    let mut visited = HashSet::new();
    visited.insert(position);
    for c in input.chars() {
        match c {
            '^' => position += UP,
            'v' => position += DOWN,
            '>' => position += RIGHT,
            '<' => position += LEFT,
            _ => unreachable!(),
        }
        visited.insert(position);
    }
    visited.len()
}

pub fn part2(input: &Input) -> usize {
    let mut positions = [ORIGIN, ORIGIN];
    let mut visited = HashSet::new();
    visited.insert(positions[0]);

    for (i, c) in input.chars().enumerate() {
        let idx = i % 2;
        match c {
            '^' => positions[idx] += UP,
            'v' => positions[idx] += DOWN,
            '>' => positions[idx] += RIGHT,
            '<' => positions[idx] += LEFT,
            _ => unreachable!(),
        }
        visited.insert(positions[idx]);
    }
    visited.len()
}
