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

pub fn part1(ingredients: &[Ingredient]) -> i64 {
    let mut max_score = 0;
    for i in 0..100 {
        let j = 100 - i;
        let mut score = (i * ingredients[0].capacity + j * ingredients[1].capacity).max(0);
        score *= (i * ingredients[0].durability + j * ingredients[1].durability).max(0);
        score *= (i * ingredients[0].flavor + j * ingredients[1].flavor).max(0);
        score *= (i * ingredients[0].texture + j * ingredients[1].texture).max(0);
        max_score = max_score.max(score);
    }
    max_score
}

pub fn part2(ingredients: &[Ingredient]) -> i64 {
    todo!()
}
