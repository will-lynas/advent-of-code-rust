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
    input.len()
}
