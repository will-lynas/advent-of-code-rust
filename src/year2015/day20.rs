use divisors::get_divisors;

pub fn parse(n: &str) -> u32 {
    let n: u32 = n.parse().unwrap();
    // -1 because get_divisors excludes 1
    // and every house has 1 as a divisor
    n / 10 - 1
}

pub fn part1(&n: &u32) -> u32 {
    #[allow(clippy::maybe_infinite_iter)]
    (1..)
        .find(|&i| get_divisors(i).iter().sum::<u32>() + i >= n)
        .unwrap()
}

pub fn part2(_n: &u32) -> u32 {
    0
}
