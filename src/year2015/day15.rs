use regex::Regex;

// Solve part1 and part2 at the same time
pub fn parse(input: &str) -> (i32, i32) {
    let re = Regex::new(r" (-?\d+).* (-?\d+).* (-?\d+).* (-?\d+).* (-?\d+)").unwrap();
    let r: [[i32; 5]; 4] = input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            [
                caps[1].parse().unwrap(),
                caps[2].parse().unwrap(),
                caps[3].parse().unwrap(),
                caps[4].parse().unwrap(),
                caps[5].parse().unwrap(),
            ]
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let mut max_score = 0;
    let mut max_score_500_calories = 0;

    for i0 in 0..=100 {
        for i1 in 0..=100 - i0 {
            for i2 in 0..=100 - i0 - i1 {
                let i3 = 100 - i0 - i1 - i2;
                let score = (i0 * r[0][0] + i1 * r[1][0] + i2 * r[2][0] + i3 * r[3][0]).max(0)
                    * (i0 * r[0][1] + i1 * r[1][1] + i2 * r[2][1] + i3 * r[3][1]).max(0)
                    * (i0 * r[0][2] + i1 * r[1][2] + i2 * r[2][2] + i3 * r[3][2]).max(0)
                    * (i0 * r[0][3] + i1 * r[1][3] + i2 * r[2][3] + i3 * r[3][3]).max(0);

                max_score = max_score.max(score);
                if i0 * r[0][4] + i1 * r[1][4] + i2 * r[2][4] + i3 * r[3][4] == 500 {
                    max_score_500_calories = max_score_500_calories.max(score);
                }
            }
        }
    }

    (max_score, max_score_500_calories)
}

pub fn part1(&(ans, _): &(i32, i32)) -> i32 {
    ans
}

pub fn part2(&(_, ans): &(i32, i32)) -> i32 {
    ans
}
