use gxhash::{
    HashSet,
    HashSetExt,
};

type Input = String;

pub fn parse(input: &str) -> Input {
    input.into()
}

pub fn part1(input: &Input) -> usize {
    let mut position = (0, 0);
    let mut visited = HashSet::new();
    visited.insert(position);
    for c in input.chars() {
        match c {
            '^' => position.1 += 1,
            'v' => position.1 -= 1,
            '>' => position.0 += 1,
            '<' => position.0 -= 1,
            _ => unreachable!(),
        }
        visited.insert(position);
    }
    visited.len()
}

pub fn part2(input: &Input) -> usize {
    let mut positions = [(0, 0), (0, 0)];
    let mut visited = HashSet::new();
    visited.insert(positions[0]);

    for (i, c) in input.chars().enumerate() {
        let idx = i % 2;
        match c {
            '^' => positions[idx].1 += 1,
            'v' => positions[idx].1 -= 1,
            '>' => positions[idx].0 += 1,
            '<' => positions[idx].0 -= 1,
            _ => unreachable!(),
        }
        visited.insert(positions[idx]);
    }
    visited.len()
}
