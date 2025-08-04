use regex::Regex;

type Input = Vec<(u32, u32, u32)>;

pub fn parse(input: &str) -> Input {
    let re = Regex::new(r"(\d+) .* (\d+) .* (\d+)").unwrap();
    input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            let speed = caps[1].parse().unwrap();
            let fly = caps[2].parse().unwrap();
            let rest: u32 = caps[3].parse().unwrap();
            let cycle = fly + rest;
            (speed, fly, cycle)
        })
        .collect()
}

pub fn part1(input: &Input) -> u32 {
    let t = 2503;
    input
        .iter()
        .map(|(speed, fly, cycle)| {
            let full_cycles = t / cycle;
            let remaining = t % cycle;
            full_cycles * speed * fly + speed * remaining.min(*fly)
        })
        .max()
        .unwrap()
}

pub fn part2(input: &Input) -> usize {
    input.len()
}
