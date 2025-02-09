use relative_path::RelativePath;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let mut area: HashMap<(i64, i64), i64> = HashMap::new();
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let value = match c {
                '.' => 1,
                '|' => 1 << 4,
                '#' => 1 << 8,
                _ => panic!("Invalid character"),
            };
            area.insert((x as i64, y as i64), value);
        }
    }

    for _ in 0..10 {
        iterate_area(&mut area);
    }

    let wooded = area.values().filter(|v| *v & (1 << 4) > 0).count() as i64;
    let lumber = area.values().filter(|v| *v & (1 << 8) > 0).count() as i64;

    return wooded * lumber;
}

fn part2(contents: String) -> i64 {
    let mut area: HashMap<(i64, i64), i64> = HashMap::new();
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let value = match c {

                '.' => 1,
                '|' => 1 << 4,
                '#' => 1 << 8,
                _ => panic!("Invalid character"),
            };
            area.insert((x as i64, y as i64), value);
        }
    }

    let mut area_history: HashMap<Vec<i64>, i64> = HashMap::new();
    let mut t = 0;
    let mut wooded = 0;
    let mut lumber = 0;

    while t < 1000000000 {
        if let Some(&prev_t) = area_history.get(&(get_area_state(area.clone()))) {
            let cycle_length = t - prev_t;
            let remaining = 1000000000 - t;
            let remaining_t = remaining % cycle_length;

            wooded = area_history
                .iter()
                .find(|(_, &v)| v == prev_t + remaining_t)
                .unwrap()
                .0
                .iter()
                .filter(|v| **v & (1 << 4) > 0)
                .count() as i64;
            lumber = area_history
                .iter()
                .find(|(_, &v)| v == prev_t + remaining_t)
                .unwrap()
                .0
                .iter()
                .filter(|v| **v & (1 << 8) > 0)
                .count() as i64;

            break;
        }

        area_history.insert(get_area_state(area.clone()), t);
        iterate_area(&mut area);
        t += 1;
    }

    if wooded == 0 || lumber == 0 {
        wooded = area.values().filter(|v| *v & (1 << 4) > 0).count() as i64;
        lumber = area.values().filter(|v| *v & (1 << 8) > 0).count() as i64;
    }

    return wooded * lumber;
}

fn iterate_area(area: &mut HashMap<(i64, i64), i64>) {
    let mut new_area: HashMap<(i64, i64), i64> = HashMap::new();
    for (x, y) in area.keys() {
        let mut adjacent = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                if let Some(value) = area.get(&(x + dx, y + dy)) {
                    adjacent += value;
                }
            }
        }
        let value = area.get(&(*x, *y)).unwrap();
        let new_value = match value {
            1 => {
                if (adjacent & 0xf0) >> 4 >= 3 {
                    1 << 4
                } else {
                    1
                }
            }
            16 => {
                if (adjacent & 0xf00) >> 8 >= 3 {
                    1 << 8
                } else {
                    1 << 4
                }
            }
            256 => {
                if (adjacent & 0xf00) >> 8 >= 1 && (adjacent & 0xf0) >> 4 >= 1 {
                    1 << 8
                } else {
                    1
                }
            }
            _ => panic!("Invalid value"),
        };
        new_area.insert((*x, *y), new_value);
    }

    *area = new_area;
}

fn get_area_state(area: HashMap<(i64, i64), i64>) -> Vec<i64> {
    let mut state: Vec<((i64, i64), i64)> = area.iter().map(|(k, v)| (*k, *v)).collect();
    state.sort_by(|a, b| a.0.cmp(&b.0));

    return state.iter().map(|(_, v)| *v).collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 1147);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2018".to_string();
    let day = "18".to_string();

    let root = env::current_dir().unwrap();
    let path_str = if args.len() > 1 {
        args[1].clone()
    } else if root.ends_with(format!("rust_{}_{}", year, day)) {
        format!("../../../Inputs/{}_{}.txt", year, day)
    } else {
        format!("/Inputs/{}_{}.txt", year, day)
    };


    let contents = fs::read_to_string(if args.len() > 1 {path_str} else {RelativePath::new(&path_str).to_path(&root).display().to_string()})
        .expect("Should have been able to read the file");

    let part1_timer = Instant::now();
    println!(
        "\nPart 1:\nResource value after 10 minutes: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nResource value after 1000000000 minutes: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}