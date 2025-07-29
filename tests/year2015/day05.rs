use advent_of_code::year2015::day05 as solution;

#[test]
fn part1() {
    let input = solution::parse("ugknbfddgicrmopn");
    assert_eq!(solution::part1(&input), 1);

    let input = solution::parse("aaa");
    assert_eq!(solution::part1(&input), 1);

    let input = solution::parse("jchzalrnumimnmhp");
    assert_eq!(solution::part1(&input), 0);

    let input = solution::parse("haegwjzuvuyypxyu");
    assert_eq!(solution::part1(&input), 0);

    let input = solution::parse("dvszwmarrgswjxmb");
    assert_eq!(solution::part1(&input), 0);
}

#[test]
fn part2() {
    let input = solution::parse("qjhvhtzxzqqjkmpb");
    assert_eq!(solution::part2(&input), 1);

    let input = solution::parse("xxyxx");
    assert_eq!(solution::part2(&input), 1);

    let input = solution::parse("uurcxstgmygtbstg");
    assert_eq!(solution::part2(&input), 0);

    let input = solution::parse("ieodomkazucvgmuy");
    assert_eq!(solution::part2(&input), 0);
}
