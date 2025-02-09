use relative_path::RelativePath;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    return contents
        .split("\n\n")
        .map(|game| game_result(game, false))
        .sum();
}

fn part2(contents: String) -> i64 {
    return contents
        .split("\n\n")
        .map(|game| game_result(game, true))
        .sum();
}

fn game_result(game: &str, increase_elf_strength: bool) -> i64 {
    let mut top_bound = 200;
    let mut bottom_bound = 3;
    let mut round_results: HashMap<i64, i64> = HashMap::new();

    loop {
        let elf_attack = if increase_elf_strength {
            (top_bound + bottom_bound) / 2
        } else {
            3
        };
        let mut area: HashSet<(i64, i64)> = HashSet::new();
        let mut units: Vec<Unit> = Vec::new();


        for (y, line) in game.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => {}
                    '.' => {
                        area.insert((x as i64, y as i64));
                    }
                    _ => {
                        if c != '.' {
                            let unit = Unit::new(
                                x as i64,
                                y as i64,
                                200,
                                if c == 'E' { elf_attack } else { 3 },
                                c == 'E',
                            );
                            units.push(unit);
                        }
                    }
                };
            }
        }

        let total_elves: usize = units.iter().filter(|u| u.elf).count();
        let mut rounds = 0;

        while game_round(&mut area, &mut units) {
            rounds += 1;
        }

        if !increase_elf_strength {
            return rounds * units.iter().map(|u| u.hp).filter(|hp| *hp > 0).sum::<i64>();
        }

        round_results.insert(
            elf_attack,
            rounds * units.iter().map(|u| u.hp).filter(|hp| *hp > 0).sum::<i64>(),
        );

        if units[0].elf && units.len() == total_elves {
            // Elves win
            top_bound = elf_attack;
        } else {
            // Goblins win
            bottom_bound = elf_attack;
        }

        if top_bound - bottom_bound <= 1 {
            let min_required = top_bound.max(bottom_bound);
            return *round_results.get(&min_required).unwrap();
        }
    }
}

fn game_round(area: &mut HashSet<(i64, i64)>, units: &mut Vec<Unit>) -> bool {
    let mut game_result: bool = true;

    let elf_indecies: Vec<usize> = (0..units.len()).filter(|&i| units[i].elf).collect();
    let goblin_indecies: Vec<usize> = (0..units.len()).filter(|&i| !units[i].elf).collect();

    for i in 0..units.len() {
        if units[i].hp <= 0 {
            continue;
        }

        let enemies_indices: Vec<usize> = if units[i].elf {
            goblin_indecies
                .iter()
                .filter(|&j| units[*j].hp > 0)
                .map(|j| *j)
                .collect()
        } else {
            elf_indecies
                .iter()
                .filter(|&j| units[*j].hp > 0)
                .map(|j| *j)
                .collect()
        };

        if enemies_indices.is_empty() {
            game_result = false;
            break;
        }

        let enemies: Vec<Unit> = enemies_indices.iter().map(|&j| units[j].clone()).collect();
        let mut closest_points: Vec<(i64, i64)> =
            get_closest_points(&area, (units[i].x, units[i].y), enemies)
                .iter()
                .map(|p| *p)
                .collect();
        if closest_points.is_empty() {
            continue;
        }

        closest_points.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
        let shortest_path = get_shortest_path(&area, (units[i].x, units[i].y), closest_points[0]);

        if shortest_path.len() > 1 {
            // Move the unit
            let new_pos = shortest_path[1];
            area.insert((units[i].x, units[i].y));
            units[i].x = new_pos.0;
            units[i].y = new_pos.1;
            area.remove(&new_pos);
        }

        if shortest_path.len() <= 2 {
            let close_enemies: Vec<usize> = enemies_indices
                .iter()
                .filter(|&j| {
                    let enemy = &units[*j];
                    (enemy.x - units[i].x).abs() + (enemy.y - units[i].y).abs() == 1
                })
                .cloned()
                .collect();

            let min_hp = close_enemies.iter().map(|ix| units[*ix].hp).min().unwrap();

            let target_ix: usize = *close_enemies
                .iter()
                .filter(|ix| units[**ix].hp == min_hp)
                .next()
                .unwrap();
            units[target_ix].hp -= units[i].attack;

            if units[target_ix].hp <= 0 {
                area.insert((units[target_ix].x, units[target_ix].y));
            }
        }
    }

    for i in (0..units.len()).rev() {
        if units[i].hp <= 0 {
            units.remove(i);
        }
    }
    units.sort_by(|a, b| a.y.cmp(&b.y).then(a.x.cmp(&b.x)));

    return game_result;
}

fn get_closest_points(
    area: &HashSet<(i64, i64)>,
    start: (i64, i64),
    ends: Vec<Unit>,
) -> HashSet<(i64, i64)> {
    let mut open_list: Vec<(i64, i64)> = vec![start];
    let mut visited: HashSet<(i64, i64)> = HashSet::new();
    let mut closest: HashSet<(i64, i64)> = HashSet::new();

    while closest.len() == 0 && open_list.len() > 0 {
        let mut new_open: HashSet<(i64, i64)> = HashSet::new();

        for pos in open_list.iter() {
            if visited.contains(pos) {
                continue;
            }

            visited.insert(*pos);
            for new_pos in [
                (pos.0, pos.1 - 1),
                (pos.0 - 1, pos.1),
                (pos.0 + 1, pos.1),
                (pos.0, pos.1 + 1),
            ]
            .iter()
            {
                if ends
                    .iter()
                    .filter(|u| u.x == new_pos.0 && u.y == new_pos.1)
                    .count()
                    > 0
                {
                    closest.insert(*pos);
                    continue;
                }

                if !area.contains(new_pos) || visited.contains(new_pos) {
                    continue;
                }

                new_open.insert(*new_pos);
            }
        }

        open_list = new_open.iter().map(|v| *v).collect();
    }

    return closest;
}

fn get_shortest_path(
    area: &HashSet<(i64, i64)>,
    start: (i64, i64),
    end: (i64, i64),
) -> Vec<(i64, i64)> {
    let mut open_list: VecDeque<Vec<(i64, i64)>> = VecDeque::new();
    open_list.push_back(vec![start]);
    let mut visited: HashSet<(i64, i64)> = HashSet::new();

    while let Some(path) = open_list.pop_front() {
        let last = path.last().unwrap();
        if visited.contains(last) {
            continue;
        }

        if *last == end {
            return path;
        }

        visited.insert(*last);
        for next in [
            (last.0, last.1 - 1),
            (last.0 - 1, last.1),
            (last.0 + 1, last.1),
            (last.0, last.1 + 1),
        ]
        .iter()
        {
            if !area.contains(next) || visited.contains(next) {
                continue;
            }

            let mut new_path = path.clone();
            new_path.push(*next);
            open_list.push_back(new_path);
        }
    }

    return vec![];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Unit {
    x: i64,
    y: i64,
    hp: i64,
    attack: i64,
    elf: bool,
}

impl Unit {
    fn new(x: i64, y: i64, hp: i64, attack: i64, elf: bool) -> Self {
        Unit {
            x,
            y,
            hp,
            attack,
            elf,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 142683);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 47364);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2018".to_string();
    let day = "15".to_string();

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
        "\nPart 1:\nScore of combat: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nScore of combat where all elves survive: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}