use gxhash::{
    HashSet,
    HashSetExt,
};

trait OverlapIndices {
    fn overlap_indices<'a>(&'a self, needle: &'a str) -> impl Iterator<Item = usize> + 'a;
}

impl OverlapIndices for str {
    fn overlap_indices<'a>(&'a self, needle: &'a str) -> impl Iterator<Item = usize> + 'a {
        (0..=self.len().saturating_sub(needle.len()))
            .filter(move |&i| self[i..].starts_with(needle))
    }
}

type Input = (Vec<(String, String)>, String);

pub fn parse(input: &str) -> Input {
    let (rules, molecule) = input.split_once("\n\n").unwrap();
    let rules: Vec<_> = rules
        .lines()
        .map(|line| {
            let (from, to) = line.split_once(" => ").unwrap();
            (from.to_string(), to.to_string())
        })
        .collect();
    (rules, molecule.to_string())
}

pub fn part1((rules, molecule): &Input) -> usize {
    let mut possible: HashSet<String> = HashSet::new();
    for (from, to) in rules {
        for pos in molecule.overlap_indices(from) {
            let mut new = molecule.to_string();
            new.replace_range(pos..pos + from.len(), to);
            possible.insert(new);
        }
    }
    possible.len()
}

fn solve(rules: &[(String, String)], molecule: &str) -> usize {
    if molecule == "e" {
        return 0;
    }

    rules
        .iter()
        .flat_map(|(from, to)| {
            molecule.overlap_indices(to).map(move |pos| {
                let mut previous = molecule.to_string();
                previous.replace_range(pos..pos + to.len(), from);
                previous
            })
        })
        .map(|s| solve(rules, &s) + 1)
        .min()
        .unwrap_or(usize::MAX / 2) // divide by 2 to avoid any overflows when adding 1
}

pub fn part2((rules, molecule): &Input) -> usize {
    solve(rules, molecule)
}
