pub fn parse(input: &str) -> Vec<usize> {
    let containers: Vec<_> = input.lines().map(|line| line.parse().unwrap()).collect();
    solve(&containers)
}

pub fn solve(containers: &[usize]) -> Vec<usize> {
    let mut counts: Vec<usize> = vec![0; containers.len()];
    for mask in 0..(1 << containers.len()) {
        let mut sum = 0;
        let mut n = 0;
        for (i, container) in containers.iter().enumerate() {
            if mask & (1 << i) != 0 {
                sum += container;
                n += 1;
            }
        }
        if sum == 150 {
            counts[n] += 1;
        }
    }
    counts
}

pub fn part1(counts: &[usize]) -> usize {
    counts.iter().sum()
}

pub fn part2(counts: &[usize]) -> usize {
    let min = counts.iter().position(|&c| c > 0).unwrap();
    counts[min]
}
