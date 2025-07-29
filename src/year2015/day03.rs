use gxhash::{
    HashSet,
    HashSetExt,
};

type Input = String;

pub fn parse(input: &str) -> Input {
    input.into()
}

pub fn part1(input: &Input) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut visited = HashSet::new();
    visited.insert((x, y));
    for c in input.chars() {
        match c {
            '^' => y += 1,
            'v' => y -= 1,
            '>' => x += 1,
            '<' => x -= 1,
            _ => unreachable!(),
        }
        visited.insert((x, y));
    }
    visited.len()
}

pub fn part2(input: &Input) -> usize {
    let mut x1 = 0;
    let mut y1 = 0;
    let mut x2 = 0;
    let mut y2 = 0;
    let mut visited = HashSet::new();
    visited.insert((x1, y1));
    for (i, c) in input.chars().enumerate() {
        if i % 2 == 0 {
            match c {
                '^' => y1 += 1,
                'v' => y1 -= 1,
                '>' => x1 += 1,
                '<' => x1 -= 1,
                _ => unreachable!(),
            }
            visited.insert((x1, y1));
        } else {
            match c {
                '^' => y2 += 1,
                'v' => y2 -= 1,
                '>' => x2 += 1,
                '<' => x2 -= 1,
                _ => unreachable!(),
            }
            visited.insert((x2, y2));
        }
    }
    visited.len()
}
