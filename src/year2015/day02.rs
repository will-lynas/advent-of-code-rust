type Input = Vec<Vec<usize>>;

pub fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let mut dimensions = line
                .split('x')
                .map(str::parse)
                .collect::<Result<Vec<_>, _>>()
                .unwrap();
            dimensions.sort_unstable();
            dimensions
        })
        .collect()
}

pub fn part1(input: &Input) -> usize {
    input
        .iter()
        .map(|dimensions| {
            (1 + 2) * dimensions[0] * dimensions[1]
                + 2 * dimensions[1] * dimensions[2]
                + 2 * dimensions[2] * dimensions[0]
        })
        .sum()
}

pub fn part2(_input: &Input) -> usize {
    0
}
