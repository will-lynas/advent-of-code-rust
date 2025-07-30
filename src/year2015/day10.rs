pub fn parse(input: &str) -> String {
    input.to_string()
}

fn transform(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars();
    let mut count = 1;
    let mut current = chars.next().unwrap();
    for c in chars {
        if c == current {
            count += 1;
        } else {
            result.push_str(&count.to_string());
            result.push(current);
            count = 1;
            current = c;
        }
    }
    result.push_str(&count.to_string());
    result.push(current);
    result
}

pub fn part1(input: &str) -> usize {
    (0..40)
        .fold(input.to_string(), |acc, _| transform(&acc))
        .len()
}

pub fn part2(input: &str) -> usize {
    input.len()
}
