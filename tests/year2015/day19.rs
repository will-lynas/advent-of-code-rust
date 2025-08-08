use advent_of_code::year2015::day19 as solution;

#[test]
fn part1() {
    let example = "\
H => HO
H => OH
O => HH

HOH";
    let input = solution::parse(example);
    assert_eq!(solution::part1(&input), 4);
}

#[test]
fn part2() {}
