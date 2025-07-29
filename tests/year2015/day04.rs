use advent_of_code::year2015::day04 as solution;

const EXAMPLE: &str = "abcdef";

#[test]
fn part1() {
    let input = solution::parse(EXAMPLE);
    assert_eq!(solution::part1(&input), 609043);
}

#[test]
fn part2() {
    // No part 2 example
}
