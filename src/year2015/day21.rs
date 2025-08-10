use regex::Regex;

pub struct Player {
    hit_points: usize,
    damage: usize,
    armor: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Item {
    cost: usize,
    damage: usize,
    armor: usize,
}

impl Item {
    const fn new(cost: usize, damage: usize, armor: usize) -> Self {
        Self {
            cost,
            damage,
            armor,
        }
    }
}

pub struct Items {
    weapons: [Item; 5],
    armor: [Option<Item>; 6],
    rings: [Option<Item>; 7],
}

const ITEMS: Items = {
    let weapons = [
        Item::new(8, 4, 0),
        Item::new(10, 5, 0),
        Item::new(25, 6, 0),
        Item::new(40, 7, 0),
        Item::new(74, 8, 0),
    ];
    let armor = [
        None,
        Some(Item::new(13, 0, 1)),
        Some(Item::new(31, 0, 2)),
        Some(Item::new(53, 0, 3)),
        Some(Item::new(75, 0, 4)),
        Some(Item::new(102, 0, 5)),
    ];
    let rings = [
        None,
        Some(Item::new(25, 1, 0)),
        Some(Item::new(50, 2, 0)),
        Some(Item::new(100, 3, 0)),
        Some(Item::new(20, 0, 1)),
        Some(Item::new(40, 0, 2)),
        Some(Item::new(80, 0, 3)),
    ];

    Items {
        weapons,
        armor,
        rings,
    }
};

pub fn parse(input: &str) -> (usize, usize) {
    let re = Regex::new(r"^Hit Points: (\d+)\nDamage: (\d+)\nArmor: (\d+)$").unwrap();
    let caps = re.captures(input).unwrap();
    let boss = Player {
        hit_points: caps[1].parse().unwrap(),
        damage: caps[2].parse().unwrap(),
        armor: caps[3].parse().unwrap(),
    };

    let mut min_cost = usize::MAX;
    let mut max_cost = 0;

    for weapon in ITEMS.weapons {
        for armor in ITEMS.armor {
            for ring1 in ITEMS.rings {
                for ring2 in ITEMS.rings {
                    if let Some(ring1) = ring1 {
                        if let Some(ring2) = ring2 {
                            if ring1 == ring2 {
                                continue;
                            }
                        }
                    }

                    let player = Player {
                        hit_points: 100,
                        damage: weapon.damage
                            + ring1.map_or(0, |r| r.damage)
                            + ring2.map_or(0, |r| r.damage),
                        armor: armor.map_or(0, |r| r.armor)
                            + ring1.map_or(0, |r| r.armor)
                            + ring2.map_or(0, |r| r.armor),
                    };

                    let cost = weapon.cost
                        + armor.map_or(0, |r| r.cost)
                        + ring1.map_or(0, |r| r.cost)
                        + ring2.map_or(0, |r| r.cost);

                    if player_wins(&player, &boss) {
                        min_cost = min_cost.min(cost);
                    } else {
                        max_cost = max_cost.max(cost);
                    }
                }
            }
        }
    }
    (min_cost, max_cost)
}

pub fn player_wins(player: &Player, boss: &Player) -> bool {
    let player_damage = player.damage.saturating_sub(boss.armor).max(1);
    let boss_damage = boss.damage.saturating_sub(player.armor).max(1);
    let player_turns = boss.hit_points.div_ceil(player_damage);
    let boss_turns = player.hit_points.div_ceil(boss_damage);
    player_turns <= boss_turns
}

pub fn part1(&(min_cost, _): &(usize, usize)) -> usize {
    min_cost
}

pub fn part2(&(_, max_cost): &(usize, usize)) -> usize {
    max_cost
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_wins() {
        let player = Player {
            hit_points: 8,
            damage: 5,
            armor: 5,
        };
        let boss = Player {
            hit_points: 12,
            damage: 7,
            armor: 2,
        };
        assert!(player_wins(&player, &boss));
    }
}
