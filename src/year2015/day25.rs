use regex::Regex;

pub fn parse(input: &str) -> (usize, usize) {
    let re = Regex::new(r" (\d+), .* (\d+)").unwrap();
    let cap = re.captures(input).unwrap();
    (cap[1].parse().unwrap(), cap[2].parse().unwrap())
}

pub fn part1((y, x): &(usize, usize)) -> usize {
    let diagonal = y + x - 2;
    let first = diagonal * (diagonal + 1) / 2 + 1;
    let i = first + x - 1;
    let mut code = 20_151_125;
    for _ in 1..i {
        code = (code * 252_533) % 33_554_393;
    }
    code
}

pub fn part2(_: &(usize, usize)) -> usize {
    0
}
