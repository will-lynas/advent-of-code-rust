use gxhash::{
    HashMap,
    HashMapExt,
};
use itertools::Itertools;

type Input = Vec<String>;

const VOWELS: &[char] = &['a', 'e', 'i', 'o', 'u'];
const FORBIDDEN: &[&str] = &["ab", "cd", "pq", "xy"];

pub fn parse(input: &str) -> Input {
    input.lines().map(Into::into).collect()
}

pub fn part1(input: &Input) -> usize {
    input
        .iter()
        .filter(|s| s.chars().filter(|c| VOWELS.contains(c)).count() >= 3)
        .filter(|s| {
            ('a'..='z')
                .map(|c| c.to_string().repeat(2))
                .any(|f| s.contains(&f.to_string()))
        })
        .filter(|s| !FORBIDDEN.iter().any(|f| s.contains(f)))
        .count()
}

pub fn part2(input: &Input) -> usize {
    input
        .iter()
        .filter(|s| {
            let mut pairs: HashMap<(char, char), Vec<usize>> = HashMap::new();
            for (i, (a, b)) in s.chars().tuple_windows().enumerate() {
                let entry = pairs.entry((a, b)).or_default();
                if entry.iter().any(|j| i - j > 1) {
                    return true;
                }
                entry.push(i);
            }
            false
        })
        .filter(|s| {
            for (a, _, c) in s.chars().tuple_windows() {
                if a == c {
                    return true;
                }
            }
            false
        })
        .count()
}
