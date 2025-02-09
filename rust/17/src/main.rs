use cached::proc_macro::cached;
use relative_path::RelativePath;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    return make_water(contents).len() as i64;
}

fn part2(contents: String) -> i64 {
    return make_water(contents).iter().filter(|(_, &v)| v).count() as i64;
}

#[cached]
fn make_water(contents: String) -> HashMap<(i64, i64), bool> {
    let mut walls: HashSet<(i64, i64)> = HashSet::new();
    for line in contents.lines() {
        let constant: i64 = line
            .split("=")
            .nth(1)
            .unwrap()
            .split(",")
            .nth(0)
            .unwrap()
            .parse()
            .unwrap();
        let start: i64 = line
            .split("=")
            .nth(2)
            .unwrap()
            .split("..")

            .nth(0)
            .unwrap()
            .parse()
            .unwrap();
        let end: i64 = line
            .split("=")
            .nth(2)
            .unwrap()
            .split("..")
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();
        let axis: char = line.chars().nth(0).unwrap();

        for i in start..=end {
            match axis {
                'x' => walls.insert((constant, i)),
                'y' => walls.insert((i, constant)),
                _ => panic!("Invalid axis"),
            };
        }
    }

    let max_y = walls.iter().map(|(_, y)| y).max().unwrap();

    let mut water: HashMap<(i64, i64), bool> = HashMap::new();
    let mut falling: VecDeque<(i64, i64)> = VecDeque::new();
    falling.push_back((500, 0));

    while let Some(next) = falling.pop_front() {
        let mut pos = next;
        if water.contains_key(&pos) {
            continue;
        }

        while (pos.1 <= *max_y) && !walls.contains(&pos) && !water.contains_key(&pos) {
            water.insert(pos, false);
            pos = (pos.0, pos.1 + 1);
        }

        if pos.1 > *max_y || !*water.get(&pos).unwrap_or(&true) {
            continue;
        }

        loop {
            pos = (pos.0, pos.1 - 1);

            let mut left = pos;
            while walls.contains(&(left.0, left.1 + 1))
                || (water.get(&(left.0, left.1 + 1)) == Some(&true))
            {
                if walls.contains(&left) {
                    break;
                }

                water.insert(left, false);
                left = (left.0 - 1, left.1);
            }

            let mut right = pos;
            while walls.contains(&(right.0, right.1 + 1))
                || (water.get(&(right.0, right.1 + 1)) == Some(&true))
            {
                if walls.contains(&right) {
                    break;
                }

                water.insert(right, false);
                right = (right.0 + 1, right.1);
            }

            if !(walls.contains(&(left.0, left.1 + 1))
                || (water.get(&(left.0, left.1 + 1)) == Some(&true)))
                || !(walls.contains(&(right.0, right.1 + 1))
                    || (water.get(&(right.0, right.1 + 1)) == Some(&true)))
            {
                if !(walls.contains(&(left.0, left.1 + 1))
                    || (water.get(&(left.0, left.1 + 1)) == Some(&true)))
                {
                    falling.push_back(left);
                }

                if !(walls.contains(&(right.0, right.1 + 1))
                    || (water.get(&(right.0, right.1 + 1)) == Some(&true)))
                {
                    falling.push_back(right);
                }

                break;
            }

            for x in left.0 + 1..right.0 {
                water.insert((x, pos.1), true);
            }
        }
    }

    let min_y = walls.iter().map(|(_, y)| y).min().unwrap();
    for key in water.clone().keys().filter(|(_, y)| y < min_y) {
        water.remove(key);
    }

    return water;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 57);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 29);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2018".to_string();
    let day = "17".to_string();

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
        "\nPart 1:\nSquares occupied by water: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nSquares with water at rest: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}