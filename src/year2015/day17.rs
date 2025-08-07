pub fn parse(input: &str) -> Vec<usize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn part1(containers: &[usize]) -> usize {
    let mut count = 0;
    for mask in 0..(1 << containers.len()) {
        let mut sum = 0;
        for (i, container) in containers.iter().enumerate() {
            if mask & (1 << i) != 0 {
                sum += container;
            }
        }
        if sum == 150 {
            count += 1;
        }
    }
    count
}

pub fn part2(containers: &[usize]) -> usize {
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
    let min = counts.iter().position(|&c| c > 0).unwrap();
    counts[min]
}
