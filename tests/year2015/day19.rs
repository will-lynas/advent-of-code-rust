use advent_of_code::year2015::day19 as solution;

const EXAMPLE: &str = "\
H => HO
H => OH
O => HH

HOH";

#[test]
fn part1() {
    let input = solution::parse(EXAMPLE);
    assert_eq!(solution::part1(&input), 4);
}

#[test]
fn part2() {}
