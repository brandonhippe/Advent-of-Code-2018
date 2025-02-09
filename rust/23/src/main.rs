use regex::Regex;
use relative_path::RelativePath;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let int_regex = Regex::new(r"(-?\d+)").unwrap();
    let mut bots: Vec<Vec<i64>> = vec![];

    for line in contents.lines() {
        let mut bot: Vec<i64> = vec![];
        for cap in int_regex.captures_iter(line) {
            bot.push(cap[0].parse().unwrap());
        }
        bots.push(bot);
    }

    bots.sort_by(|a, b| Reverse(a[3]).cmp(&Reverse(b[3])));

    return bots
        .iter()
        .filter(|bot| manhattan_distance(bots[0][0..3].to_vec(), bot[0..3].to_vec()) <= bots[0][3])
        .count() as i64;
}

fn part2(contents: String) -> i64 {
    let int_regex = Regex::new(r"(-?\d+)").unwrap();
    let mut queue: BinaryHeap<(i64, i64)> = BinaryHeap::new();

    for line in contents.lines() {
        let mut bot: Vec<i64> = vec![];
        for cap in int_regex.captures_iter(line) {

            bot.push(cap[0].parse().unwrap());
        }

        let distance = manhattan_distance(vec![0, 0, 0], bot[0..3].to_vec());
        queue.push((-(distance - bot[3]).max(0), 1));
        queue.push((-(distance + bot[3] + 1), -1));
    }

    let mut count = 0;
    let mut max_count = 0;
    let mut result = 0;

    while let Some((distance, delta)) = queue.pop() {
        count += delta;
        if count > max_count {
            max_count = count;
            result = -distance;
        }
    }

    return result;
}

fn manhattan_distance(a: Vec<i64>, b: Vec<i64>) -> i64 {
    return a.iter().zip(b.iter()).map(|(x, y)| (x - y).abs()).sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("p1_example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 7);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("p2_example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 36);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2018".to_string();
    let day = "23".to_string();

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
        "\nPart 1:\nNanobots within range of nanobot with largest range: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nShortest distance to point within range of the most nanobots: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}