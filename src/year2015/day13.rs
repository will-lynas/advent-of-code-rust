use gxhash::{
    HashMap,
    HashMapExt,
};
use itertools::Itertools;
use regex::Regex;

pub fn parse(input: &str) -> Vec<Vec<i32>> {
    let re = Regex::new(r"(\w+) would (gain|lose) (\d+) happiness units by sitting next to (\w+).")
        .unwrap();
    let mut map: HashMap<String, HashMap<String, i32>> = HashMap::new();
    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let name1 = caps[1].to_string();
        let mult = if &caps[2] == "gain" { 1 } else { -1 };
        let num: i32 = caps[3].parse().unwrap();
        let name2 = caps[4].to_string();
        map.entry(name1).or_default().insert(name2, mult * num);
    }
    let names_map: HashMap<String, usize> = map
        .keys()
        .enumerate()
        .map(|(i, k)| (k.clone(), i))
        .collect();
    let mut pairs: Vec<Vec<i32>> = vec![vec![0; names_map.len()]; names_map.len()];
    for (name1, map) in map {
        for (name2, value) in map {
            pairs[names_map[&name1]][names_map[&name2]] = value;
        }
    }
    pairs
}

pub fn part1(pairs: &[Vec<i32>]) -> i32 {
    let n = pairs.len();
    (1..n)
        .permutations(n - 1)
        .map(|perm| {
            perm.iter()
                .tuple_windows()
                .map(|(&a, &b)| pairs[a][b] + pairs[b][a])
                .sum::<i32>()
                + (pairs[0][perm[0]] + pairs[perm[0]][0])
                + (pairs[perm[n - 2]][0] + pairs[0][perm[n - 2]])
        })
        .max()
        .unwrap()
}

pub fn part2(pairs: &[Vec<i32>]) -> i32 {
    let mut pairs = pairs.to_vec();
    let n = pairs.len();
    for row in &mut pairs {
        row.push(0);
    }
    pairs.push(vec![0; n + 1]);
    part1(&pairs)
}
