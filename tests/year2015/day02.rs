use advent_of_code::year2015::day02 as solution;

const EXAMPLE: &str = "2x3x4";

#[test]
fn part1() {
    let input = solution::parse(EXAMPLE);
    assert_eq!(solution::part1(&input), 58);
}

#[test]
fn part2() {
    let input = solution::parse(EXAMPLE);
    assert_eq!(solution::part2(&input), 34);
}
