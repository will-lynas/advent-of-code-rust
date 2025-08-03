use gxhash::{
    HashMap,
    HashMapExt,
};
use itertools::Itertools;
use regex::Regex;

type Pairs = HashMap<String, HashMap<String, i32>>;

pub fn parse(input: &str) -> Pairs {
    let re = Regex::new(r"(\w+) would (gain|lose) (\d+) happiness units by sitting next to (\w+).")
        .unwrap();
    let mut map: Pairs = HashMap::new();
    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let name1 = caps[1].to_string();
        let mult = if &caps[2] == "gain" { 1 } else { -1 };
        let num: i32 = caps[3].parse().unwrap();
        let name2 = caps[4].to_string();
        map.entry(name1).or_default().insert(name2, mult * num);
    }
    map
}

pub fn part1(input: &Pairs) -> i32 {
    let names: Vec<_> = input.keys().cloned().collect();
    names
        .iter()
        .skip(1)
        .permutations(names.len() - 1)
        .map(|perm| {
            let mut perm = perm;
            perm.push(&names[0]);
            perm.iter()
                .circular_tuple_windows()
                .map(|(&a, &b)| input[a][b] + input[b][a])
                .sum()
        })
        .max()
        .unwrap()
}

pub fn part2(input: &Pairs) -> i32 {
    let mut input = input.clone();
    let previous_names: Vec<_> = input.keys().cloned().collect();
    for name in previous_names {
        input
            .entry(name.clone())
            .or_default()
            .insert("me".to_string(), 0);
        input.entry("me".to_string()).or_default().insert(name, 0);
    }

    part1(&input)
}
