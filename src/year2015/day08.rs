pub fn parse(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|line| line[1..line.len() - 1].to_string())
        .collect()
}

fn unescaped(input: &str) -> usize {
    let mut result = 0;
    let mut chars = input.chars();
    while let Some(c) = chars.next() {
        if c == '\\' && chars.next().unwrap() == 'x' {
            chars.next();
            chars.next();
        }
        result += 1;
    }
    result
}

fn escaped(input: &str) -> usize {
    let mut result = 0;
    for c in input.chars() {
        if c == '\\' || c == '"' {
            result += 1;
        }
    }
    result
}

pub fn part1(input: &[String]) -> usize {
    input
        .iter()
        .map(|line| 2 + line.len() - unescaped(line))
        .sum()
}

pub fn part2(input: &[String]) -> usize {
    input.iter().map(|line| 4 + escaped(line)).sum()
}
