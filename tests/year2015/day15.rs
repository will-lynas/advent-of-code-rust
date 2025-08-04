use advent_of_code::year2015::day15 as solution;

const EXAMPLE: &str = "
Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";

#[test]
fn part1() {
    let input = solution::parse(EXAMPLE.trim());
    assert_eq!(solution::part1(&input), 62842880);
}

#[test]
fn part2() {
    let input = solution::parse(EXAMPLE.trim());
    assert_eq!(solution::part2(&input), 0);
}
