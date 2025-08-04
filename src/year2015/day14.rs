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
    let t = 2503;
    let mut points = vec![0; input.len()];
    let mut distances = vec![0; input.len()];
    for i in 0..t {
        for (j, (speed, fly, cycle)) in input.iter().enumerate() {
            if i % cycle < *fly {
                distances[j] += *speed;
            }
        }
        let max_distance = distances.iter().max().unwrap();
        for j in 0..input.len() {
            if distances[j] == *max_distance {
                points[j] += 1;
            }
        }
    }
    *points.iter().max().unwrap()
}
