use gxhash::{
    HashMap,
    HashMapExt,
};
use regex::Regex;

pub fn parse(input: &str) -> Vec<Vec<i32>> {
    let re = Regex::new(r"(\w+) would (gain|lose) (\d+) happiness units by sitting next to (\w+).")
        .unwrap();
    let mut map: HashMap<String, HashMap<String, i32>> = HashMap::new();
    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let name1 = caps[1].to_string();
        let mult = if &caps[2] == "gain" { 1 } else { -1 };
        let num: i32 = caps[3].parse().unwrap();
        let name2 = caps[4].to_string();
        map.entry(name1).or_default().insert(name2, mult * num);
    }
    let names_map: HashMap<String, usize> = map
        .keys()
        .enumerate()
        .map(|(i, k)| (k.clone(), i))
        .collect();
    let mut pairs: Vec<Vec<i32>> = vec![vec![0; names_map.len()]; names_map.len()];
    for (name1, map) in map {
        for (name2, value) in map {
            // precompute both directions
            pairs[names_map[&name1]][names_map[&name2]] += value;
            pairs[names_map[&name2]][names_map[&name1]] += value;
        }
    }
    pairs
}

pub fn part1(pairs: &[Vec<i32>]) -> i32 {
    let n = pairs.len();
    let full = 1 << n;

    // dp[mask][i] is the maximum happiness of the position where:
    // - person 0 is seated at position 0 (WLOG, due to rotational symmetry)
    // - mask is the set of people already seated clockwise from person 0
    //     (arranged to maximise happiness)
    // - person i is seated at the last (most clockwise) position
    //
    // initialise to (effectively) -inf
    // divide by 2 to avoid overflow
    let mut dp = vec![vec![i32::MIN / 2; n]; full];

    // fix person 0 at position 0 in the table
    dp[1 << 0][0] = 0;

    for mask in 1..(1 << n) {
        #[allow(clippy::needless_range_loop)]
        for i in 0..n {
            if mask & (1 << i) == 0 {
                // if person i is not seated,
                //   then they certainly cannot be seated in the last position
                continue;
            }
            // we now have a valid seating arrangement
            for next in 0..n {
                if mask & (1 << next) != 0 {
                    // skip if the person is already seated
                    continue;
                }
                let new_mask = mask | (1 << next);
                let candidate = dp[mask][i] + pairs[i][next];
                // can we improve?
                dp[new_mask][next] = dp[new_mask][next].max(candidate);
            }
        }
    }

    // close the circle
    (1..n)
        .map(|last| dp[full - 1][last] + pairs[last][0])
        .max()
        .unwrap()
}

pub fn part2(pairs: &[Vec<i32>]) -> i32 {
    let mut pairs = pairs.to_vec();
    let n = pairs.len();
    for row in &mut pairs {
        row.push(0);
    }
    pairs.push(vec![0; n + 1]);
    part1(&pairs)
}
