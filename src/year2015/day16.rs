use gxhash::HashMap;
use regex::Regex;

const TAPE: &str = "\
children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1";

type Stat = HashMap<String, i32>;

pub fn parse(input: &str) -> (Stat, Vec<Stat>) {
    // same re can be used for tape and input
    let re = Regex::new(r"(\w+): (\d+)").unwrap();

    let tape: Stat = re
        .captures_iter(TAPE)
        .map(|c| (c[1].to_string(), c[2].parse().unwrap()))
        .collect();

    let sues: Vec<Stat> = input
        .lines()
        .map(|line| {
            re.captures_iter(line)
                .map(|c| (c[1].to_string(), c[2].parse().unwrap()))
                .collect()
        })
        .collect();

    (tape, sues)
}

pub fn part1((tape, sues): &(Stat, Vec<Stat>)) -> usize {
    sues.iter()
        .enumerate()
        .find_map(|(i, s)| {
            tape.iter()
                .all(|(k, v)| s.get(k).is_none_or(|s| s == v))
                .then_some(i)
        })
        .unwrap()
        + 1
}

pub fn part2(input: &(Stat, Vec<Stat>)) -> usize {
    input.1.len()
}
