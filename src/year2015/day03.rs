use gxhash::{
    HashSet,
    HashSetExt,
};

use crate::utils::point::Point;

pub fn parse(input: &str) -> Vec<Point> {
    input
        .chars()
        .map(|c| match c {
            '^' => Point::UP,
            'v' => Point::DOWN,
            '>' => Point::RIGHT,
            '<' => Point::LEFT,
            _ => unreachable!(),
        })
        .collect()
}

pub fn part1(input: &[Point]) -> usize {
    let mut position = Point::ORIGIN;
    let mut visited = HashSet::new();
    visited.insert(position);

    for &delta in input {
        position += delta;
        visited.insert(position);
    }
    visited.len()
}

pub fn part2(input: &[Point]) -> usize {
    let mut positions = [Point::ORIGIN, Point::ORIGIN];
    let mut visited = HashSet::new();
    visited.insert(positions[0]);

    for (i, &delta) in input.iter().enumerate() {
        let idx = i % 2;
        positions[idx] += delta;
        visited.insert(positions[idx]);
    }
    visited.len()
}
