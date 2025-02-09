use relative_path::RelativePath;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let mut plants: HashSet<i64> = HashSet::from_iter(
        contents
            .lines()
            .nth(0)
            .unwrap()
            .split(" ")
            .nth(2)
            .unwrap()
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == '#')
            .map(|(i, _)| i as i64),
    );
    let rules: HashSet<String> = HashSet::from_iter(
        contents
            .lines()
            .skip(2)
            .filter(|l| l.split(" => ").nth(1).unwrap() == "#")
            .map(|l| l.split(" =>").nth(0).unwrap().to_string()),
    );

    for _ in 0..20 {
        let mut new_plants: HashSet<i64> = HashSet::new();
        let min = *plants.iter().min().unwrap() - 2;
        let max = *plants.iter().max().unwrap() + 2;
        let mut pos_str: String = (-2..=2)
            .map(|i| {
                if plants.contains(&(min + i)) {

                    '#'
                } else {
                    '.'
                }
            })
            .collect();

        for i in min..=max {
            if rules.contains(&pos_str) {
                new_plants.insert(i);
            }

            pos_str = pos_str.chars().skip(1).collect::<String>()
                + if plants.contains(&(i + 3)) { "#" } else { "." };
        }

        plants = new_plants;
    }

    return plants.iter().sum();
}

fn part2(contents: String) -> i64 {
    let mut plants: HashSet<i64> = HashSet::from_iter(
        contents
            .lines()
            .nth(0)
            .unwrap()
            .split(" ")
            .nth(2)
            .unwrap()
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == '#')
            .map(|(i, _)| i as i64),
    );
    let rules: HashSet<String> = HashSet::from_iter(
        contents
            .lines()
            .skip(2)
            .filter(|l| l.split(" => ").nth(1).unwrap() == "#")
            .map(|l| l.split(" =>").nth(0).unwrap().to_string()),
    );

    let mut steps: i64 = 0;
    let mut deltas: Vec<i64> = Vec::new();
    let mut p_sum: i64 = plants.iter().sum();
    let check_back: usize = 3;

    while steps < 50000000000 {
        let mut new_plants: HashSet<i64> = HashSet::new();
        let min = *plants.iter().min().unwrap() - 2;
        let max = *plants.iter().max().unwrap() + 2;
        let mut pos_str: String = (-2..=2)
            .map(|i| {
                if plants.contains(&(min + i)) {
                    '#'
                } else {
                    '.'
                }
            })
            .collect();

        for i in min..=max {
            if rules.contains(&pos_str) {
                new_plants.insert(i);
            }

            pos_str = pos_str.chars().skip(1).collect::<String>()
                + if plants.contains(&(i + 3)) { "#" } else { "." };
        }

        plants = new_plants;
        steps += 1;

        let sum = plants.iter().sum();
        deltas.push(sum - p_sum);

        p_sum = sum;

        if deltas.len() > check_back
            && (0..check_back)
                .map(|i| deltas[deltas.len() - 1 - i])
                .collect::<HashSet<i64>>()
                .len()
                == 1
        {
            break;
        }
    }

    return p_sum + (50000000000 - steps) * deltas[deltas.len() - 1];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 325);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2018".to_string();
    let day = "12".to_string();

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
        "\nPart 1:\nSum of plant pot numbers after 20 steps: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nSum of plant pot numbers after 50000000000 steps: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}