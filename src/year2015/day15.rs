use regex::Regex;

pub struct Ingredient {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

pub fn parse(input: &str) -> (i64, i64) {
    let re = Regex::new(r" (-?\d+).* (-?\d+).* (-?\d+).* (-?\d+).* (-?\d+)").unwrap();
    let r: Vec<Ingredient> = input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            Ingredient {
                capacity: caps[1].parse().unwrap(),
                durability: caps[2].parse().unwrap(),
                flavor: caps[3].parse().unwrap(),
                texture: caps[4].parse().unwrap(),
                calories: caps[5].parse().unwrap(),
            }
        })
        .collect();

    let mut max_score = 0;
    let mut max_score_500_calories = 0;

    for i in 0..=100 {
        for j in 0..=100 - i {
            for k in 0..=100 - i - j {
                let l = 100 - i - j - k;
                let score = (i * r[0].capacity
                    + j * r[1].capacity
                    + k * r[2].capacity
                    + l * r[3].capacity)
                    .max(0)
                    * (i * r[0].durability
                        + j * r[1].durability
                        + k * r[2].durability
                        + l * r[3].durability)
                        .max(0)
                    * (i * r[0].flavor + j * r[1].flavor + k * r[2].flavor + l * r[3].flavor)
                        .max(0)
                    * (i * r[0].texture + j * r[1].texture + k * r[2].texture + l * r[3].texture)
                        .max(0);

                max_score = max_score.max(score);
                if i * r[0].calories + j * r[1].calories + k * r[2].calories + l * r[3].calories
                    == 500
                {
                    max_score_500_calories = max_score_500_calories.max(score);
                }
            }
        }
    }
    (max_score, max_score_500_calories)
}

pub fn part1(&(ans, _): &(i64, i64)) -> i64 {
    ans
}

pub fn part2(&(_, ans): &(i64, i64)) -> i64 {
    ans
}
