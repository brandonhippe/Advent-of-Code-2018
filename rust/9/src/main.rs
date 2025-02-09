use relative_path::RelativePath;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    return contents
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let players = parts.next().unwrap().parse::<i64>().unwrap();
            let last_marble = parts.nth(5).unwrap().parse::<i64>().unwrap();
            max_score(players, last_marble)
        })
        .sum::<i64>();
}

fn part2(contents: String) -> i64 {
    return contents
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let players = parts.next().unwrap().parse::<i64>().unwrap();
            let last_marble = parts.nth(5).unwrap().parse::<i64>().unwrap();
            max_score(players, last_marble * 100)
        })
        .sum::<i64>();
}

fn max_score(players: i64, last_marble: i64) -> i64 {
    let mut scores = vec![0; players as usize];
    let mut circle = VecDeque::new();
    circle.push_back(0);


    let mut max_score = 0;

    for marble in 1..=last_marble {
        if marble % 23 == 0 {
            for _ in 0..7 {
                let temp = circle.pop_back().unwrap();
                circle.push_front(temp);
            }

            scores[(marble % players) as usize] += marble + circle.pop_front().unwrap();
            max_score = std::cmp::max(max_score, scores[(marble % players) as usize]);
        } else {
            for _ in 0..2 {
                let temp = circle.pop_front().unwrap();
                circle.push_back(temp);
            }

            circle.push_front(marble);
        }
    }

    return max_score;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 249477);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2018".to_string();
    let day = "9".to_string();

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
        "\nPart 1:\nHighest score: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nHighest score: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}