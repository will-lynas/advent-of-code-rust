use regex::Regex;

pub struct Ingredient {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

pub fn parse(input: &str) -> Vec<Ingredient> {
    let re = Regex::new(r" (-?\d+).* (-?\d+).* (-?\d+).* (-?\d+).* (-?\d+)").unwrap();
    input
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
        .collect()
}

pub fn part1(r: &[Ingredient]) -> i64 {
    let mut max_score = 0;
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
            }
        }
    }
    max_score
}

pub fn part2(ingredients: &[Ingredient]) -> i64 {
    todo!()
}
