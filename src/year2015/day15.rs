use regex::Regex;

type Input = Vec<(i64, i64, i64, i64, i64)>;

pub fn parse(input: &str) -> Input {
    let re = Regex::new(r" (-?\d+).* (-?\d+).* (-?\d+).* (-?\d+).* (-?\d+)").unwrap();
    input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            (
                caps[1].parse().unwrap(),
                caps[2].parse().unwrap(),
                caps[3].parse().unwrap(),
                caps[4].parse().unwrap(),
                caps[5].parse().unwrap(),
            )
        })
        .collect()
}

pub fn part1(ingredients: &Input) -> i64 {
    let mut max_score = 0;
    for i in 0..100 {
        let j = 100 - i;
        let mut score = (i * ingredients[0].0 + j * ingredients[1].0).max(0);
        score *= (i * ingredients[0].1 + j * ingredients[1].1).max(0);
        score *= (i * ingredients[0].2 + j * ingredients[1].2).max(0);
        score *= (i * ingredients[0].3 + j * ingredients[1].3).max(0);
        max_score = max_score.max(score);
    }
    max_score
}

pub fn part2(input: &Input) -> usize {
    input.len()
}
