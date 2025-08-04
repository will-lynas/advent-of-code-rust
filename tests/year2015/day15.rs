use advent_of_code::year2015::day15 as solution;

// Dummy ingredients are used so that the length of the input can be
// hardcoded as 4
// These ingredients are all negative, so they will never be used
const EXAMPLE: &str = "\
Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
Dummy1: capacity -100, durability -100, flavor -100, texture -100, calories 0
Dummy2: capacity -100, durability -100, flavor -100, texture -100, calories 0";

#[test]
fn part1() {
    let input = solution::parse(EXAMPLE);
    assert_eq!(solution::part1(&input), 62842880);
}

#[test]
fn part2() {
    let input = solution::parse(EXAMPLE);
    assert_eq!(solution::part2(&input), 57600000);
}
