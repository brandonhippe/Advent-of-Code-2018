use relative_path::RelativePath;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let mut immune_system: Vec<Group> = Vec::new();
    let mut infection: Vec<Group> = Vec::new();

    for (ix, group) in contents.split("\n\n").enumerate() {
        for line in group.lines().skip(1) {
            match ix {
                0 => immune_system.push(Group::new(line)),
                1 => infection.push(Group::new(line)),
                _ => panic!("Shouldn't have gotten here"),
            }
        }
    }

    while immune_system.len() > 0 && infection.len() > 0 {
        fight(&mut immune_system, &mut infection);
    }

    return immune_system.iter().map(|x| x.units).sum::<i64>()
        + infection.iter().map(|x| x.units).sum::<i64>();
}

fn part2(contents: String) -> i64 {
    let mut immune_system: Vec<Group> = Vec::new();
    let mut infection: Vec<Group> = Vec::new();

    for (ix, group) in contents.split("\n\n").enumerate() {
        for line in group.lines().skip(1) {

            match ix {
                0 => immune_system.push(Group::new(line)),
                1 => infection.push(Group::new(line)),
                _ => panic!("Shouldn't have gotten here"),
            }
        }
    }

    let mut lower_bound: i32 = 0;
    let mut upper_bound: i32 = i32::MAX;

    while lower_bound < upper_bound - 1 {
        let mut immune_system = immune_system.clone();
        let mut infection = infection.clone();

        let boost = (upper_bound + lower_bound) / 2;
        for group in immune_system.iter_mut() {
            group.damage += boost as i64;
        }

        let mut steps: i64 = 0;
        while immune_system.len() > 0 && infection.len() > 0 && steps < 5000 {
            fight(&mut immune_system, &mut infection);
            steps += 1;
        }

        if infection.len() == 0 {
            upper_bound = boost;
        } else {
            lower_bound = boost;
        }
    }

    let mut immune_system = immune_system.clone();
    let mut infection = infection.clone();
    for group in immune_system.iter_mut() {
        group.damage += upper_bound as i64;
    }

    while immune_system.len() > 0 && infection.len() > 0 {
        fight(&mut immune_system, &mut infection);
    }

    return immune_system.iter().map(|x| x.units).sum::<i64>()
        + infection.iter().map(|x| x.units).sum::<i64>();
}

fn fight(immune_system: &mut Vec<Group>, infection: &mut Vec<Group>) {
    let mut order: Vec<(i64, usize)> = Vec::new();
    let mut available: HashSet<(i64, usize)> = HashSet::new();
    for i in 0..immune_system.len() {
        order.push((0, i));
        available.insert((0, i));
    }

    for i in 0..infection.len() {
        order.push((1, i));
        available.insert((1, i));
    }

    order.sort_by(|a, b| {
        let a = if a.0 == 0 {
            &immune_system[a.1]
        } else {
            &infection[a.1]
        };

        let b = if b.0 == 0 {
            &immune_system[b.1]
        } else {
            &infection[b.1]
        };

        return (b.units * b.damage)
            .cmp(&(a.units * a.damage))
            .then(b.initiative.cmp(&a.initiative));
    });

    let mut targets: HashMap<(i64, usize), (i64, usize)> = HashMap::new();
    for unit in order.iter() {
        let group = if unit.0 == 0 {
            &immune_system[unit.1]
        } else {
            &infection[unit.1]
        };

        let mut most_damage = 0;
        let mut targeted = (0, 0);

        for enemies in available.iter().filter(|x| x.0 != unit.0) {
            let enemy = if enemies.0 == 0 {
                &immune_system[enemies.1]
            } else {
                &infection[enemies.1]
            };

            let damage = if enemy.immunities.contains(&group.damage_type) {
                0
            } else if enemy.weaknesses.contains(&group.damage_type) {
                group.damage * 2
            } else {
                group.damage
            };

            if damage > most_damage {
                most_damage = damage;
                targeted = *enemies;
            } else if damage == most_damage {
                let target = if targeted.0 == 0 {
                    &immune_system[targeted.1]
                } else {
                    &infection[targeted.1]
                };

                if enemy.units * enemy.damage > target.units * target.damage {
                    most_damage = damage;
                    targeted = *enemies;
                } else if enemy.units * enemy.damage == target.units * target.damage {
                    if enemy.initiative > target.initiative {
                        most_damage = damage;
                        targeted = *enemies;
                    }
                }
            }
        }

        if most_damage > 0 {
            targets.insert(*unit, targeted);
            available.remove(&targeted);
        }
    }

    order = targets.keys().map(|x| *x).collect();
    order.sort_by(|a, b| {
        let a = if a.0 == 0 {
            &immune_system[a.1]
        } else {
            &infection[a.1]
        };

        let b = if b.0 == 0 {
            &immune_system[b.1]
        } else {
            &infection[b.1]
        };

        return b.initiative.cmp(&a.initiative);
    });

    for unit in order.iter() {
        if unit.0 == 0 {
            let group = &immune_system[unit.1];

            if group.units <= 0 {
                continue;
            }

            let target = &mut infection[targets[unit].1];
            let damage = if target.immunities.contains(&group.damage_type) {
                0
            } else if target.weaknesses.contains(&group.damage_type) {
                group.damage * 2
            } else {
                group.damage
            } * group.units;

            let killed = damage / target.hp;
            target.units -= killed;
        } else {
            let group = &infection[unit.1];

            if group.units <= 0 {
                continue;
            }

            let target = &mut immune_system[targets[unit].1];
            let damage = if target.immunities.contains(&group.damage_type) {
                0
            } else if target.weaknesses.contains(&group.damage_type) {
                group.damage * 2
            } else {
                group.damage
            } * group.units;

            let killed = damage / target.hp;
            target.units -= killed;
        }
    }

    immune_system.retain(|x| x.units > 0);
    infection.retain(|x| x.units > 0);
}

#[derive(Debug, Clone)]
struct Group {
    units: i64,
    hp: i64,
    damage: i64,
    damage_type: String,
    initiative: i64,
    weaknesses: Vec<String>,
    immunities: Vec<String>,
}

impl Group {
    fn new(line: &str) -> Group {
        let line = &line
            .replace("(", "")
            .replace(")", "")
            .replace(",", "")
            .replace(";", "");
        let mut parts = line.split_whitespace();
        let units = parts.next().unwrap().parse::<i64>().unwrap();
        let hp = parts.nth(3).unwrap().parse::<i64>().unwrap();
        let mut weaknesses = Vec::new();
        let mut immunities = Vec::new();
        let mut damage = 0;
        let mut damage_type = String::new();
        let mut initiative = 0;

        while let Some(part) = parts.next() {
            match part {
                "immune" | "weak" => {
                    let mut current: &mut Vec<String> = if part == "immune" {
                        &mut immunities
                    } else {
                        &mut weaknesses
                    };

                    let mut next = parts.next().unwrap();
                    while next != "with" && next != "at" {
                        if next == "immune" {
                            current = &mut immunities;
                        } else if next == "weak" {
                            current = &mut weaknesses;
                        } else if next != "to" {
                            current.push(next.to_string());
                        }

                        next = parts.next().unwrap();
                    }
                }
                "does" => {
                    damage = parts.next().unwrap().parse::<i64>().unwrap();
                    damage_type = parts.next().unwrap().to_string();
                }
                "initiative" => {
                    initiative = parts.next().unwrap().parse::<i64>().unwrap();
                }
                _ => {}
            }
        }

        return Group {
            units,
            hp,
            damage,
            damage_type,
            initiative,
            weaknesses,
            immunities,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 5216);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 51);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2018".to_string();
    let day = "24".to_string();

    let root = env::current_dir().unwrap();
    let path_str = if args.len() > 1 {
        args[1].clone()
    } else if root.ends_with(format!("{}", day)) {
        format!("../../../Inputs/{}_{}.txt", year, day)
    } else {
        format!("/Inputs/{}_{}.txt", year, day)
    };


    let contents = fs::read_to_string(if args.len() > 1 {path_str} else {RelativePath::new(&path_str).to_path(&root).display().to_string()})
        .expect("Should have been able to read the file");

    let part1_timer = Instant::now();
    println!(
        "\nPart 1:\nNumber of remaining units: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nRemaining units after boosting immune system: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}