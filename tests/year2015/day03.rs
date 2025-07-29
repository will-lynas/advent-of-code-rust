use advent_of_code::year2015::day03 as solution;

const EXAMPLE: &str = "^v^v^v^v^v";

#[test]
fn part1() {
    let input = solution::parse(EXAMPLE);
    assert_eq!(solution::part1(&input), 2);
}

#[test]
fn part2() {
    let input = solution::parse(EXAMPLE);
    assert_eq!(solution::part2(&input), 11);
}
