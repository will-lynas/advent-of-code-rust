use gxhash::{
    HashMap,
    HashMapExt,
    HashSet,
    HashSetExt,
};
use regex::Regex;

type Input = (Vec<(String, Vec<String>)>, Vec<String>);

pub fn parse(input: &str) -> Input {
    let (rules, molecule) = input.split_once("\n\n").unwrap();

    let re = Regex::new(r"([A-Z][a-z]?)").unwrap();

    let rules: Vec<_> = rules
        .lines()
        .map(|line| {
            let (from, to) = line.split_once(" => ").unwrap();
            let to = re.captures_iter(to).map(|cap| cap[0].to_string()).collect();
            (from.to_string(), to)
        })
        .collect();

    let molecule = re
        .captures_iter(molecule)
        .map(|cap| cap[0].to_string())
        .collect();

    (rules, molecule)
}

pub fn part1((rules, molecule): &Input) -> usize {
    let mut possible: HashSet<Vec<String>> = HashSet::new();
    for (from, to) in rules {
        for pos in molecule
            .iter()
            .enumerate()
            .filter_map(|(i, s)| if s == from { Some(i) } else { None })
        {
            let new = molecule
                .iter()
                .take(pos)
                .chain(to.iter())
                .chain(molecule.iter().skip(pos + 1))
                .cloned()
                .collect();
            possible.insert(new);
        }
    }
    possible.len()
}

pub fn part2((_rules, molecule): &Input) -> usize {
    // needs explanation
    let mut counts: HashMap<&str, usize> = HashMap::new();
    for s in molecule {
        *counts.entry(s).or_insert(0) += 1;
    }
    let total: usize = counts.values().sum();
    total - counts["Rn"] - counts["Ar"] - 2 * counts["Y"] - 1
}
