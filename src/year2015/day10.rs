pub fn parse(input: &str) -> String {
    input.to_string()
}

fn solve(input: &str, iterations: usize) -> usize {
    if iterations == 0 {
        return input.len();
    }

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

    solve(&result, iterations - 1)
}

pub fn part1(input: &str) -> usize {
    solve(input, 40)
}

pub fn part2(input: &str) -> usize {
    solve(input, 50)
}
