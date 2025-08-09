use std::iter;

use divisors::get_divisors;

pub fn parse(n: &str) -> u32 {
    n.parse().unwrap()
}

pub fn part1(&n: &u32) -> u32 {
    // -1 because get_divisors excludes 1
    // and every house has 1 as a divisor
    let n = n / 10 - 1;
    #[allow(clippy::maybe_infinite_iter)]
    (1..)
        .find(|&i| get_divisors(i).iter().sum::<u32>() + i >= n)
        .unwrap()
}

pub fn part2(&n: &u32) -> u32 {
    #[allow(clippy::maybe_infinite_iter)]
    (1..)
        .find(|&i| {
            get_divisors(i)
                .iter()
                .chain(iter::once(&1))
                .chain(iter::once(&i))
                .filter(|&d| d * 50 >= i)
                .sum::<u32>()
                * 11
                >= n
        })
        .unwrap()
}
