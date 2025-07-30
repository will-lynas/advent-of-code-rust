use advent_of_code::year2015::day09 as solution;

const EXAMPLE: &str = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";

#[test]
fn part1() {
    let input = solution::parse(EXAMPLE);
    assert_eq!(solution::part1(&input), 605);
}

#[test]
fn part2() {
    let input = solution::parse(EXAMPLE);
    assert_eq!(solution::part2(&input), 982);
}
