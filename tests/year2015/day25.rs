use advent_of_code::year2015::day25 as solution;

#[test]
fn part1() {
    let example = "To continue, please consult the code grid in the manual.  \
Enter the code at row 1, column 1.";
    let input = solution::parse(example);
    assert_eq!(solution::part1(&input), 20151125);

    let example = "To continue, please consult the code grid in the manual.  \
Enter the code at row 4, column 3.";
    let input = solution::parse(example);
    assert_eq!(solution::part1(&input), 21345942);
}

#[test]
fn part2() {}
