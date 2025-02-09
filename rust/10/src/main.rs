use cached::proc_macro::cached;
use regex::Regex;
use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> String {
    let points = determine_message(contents).0;
    let min_x = points.iter().map(|p| p.x).min().unwrap();
    let max_x = points.iter().map(|p| p.x).max().unwrap();
    let min_y = points.iter().map(|p| p.y).min().unwrap();
    let max_y = points.iter().map(|p| p.y).max().unwrap();

    let mut message = String::new();
    for y in min_y..=max_y {
        message.push('\n');

        for x in min_x..=max_x {
            if points.iter().any(|p| p.x == x && p.y == y) {
                message.push('█');
            } else {
                message.push(' ');
            }
        }
    }

    return message;
}

fn part2(contents: String) -> i64 {
    return determine_message(contents).1;
}

#[derive(Debug, Clone)]

struct Point {
    x: i64,
    y: i64,
    x_v: i64,
    y_v: i64,
}

impl Point {
    fn new(x: i64, y: i64, x_v: i64, y_v: i64) -> Point {
        Point { x, y, x_v, y_v }
    }

    fn step(&mut self) {
        self.x += self.x_v;
        self.y += self.y_v;
    }

    fn step_back(&mut self) {
        self.x -= self.x_v;
        self.y -= self.y_v;
    }
}

#[cached]
fn determine_message(contents: String) -> (Vec<Point>, i64) {
    let int_re: Regex = Regex::new(r"-?\d+").unwrap();
    let mut points: Vec<Point> = contents
        .lines()
        .map(|line| {
            let mut nums: Vec<i64> = int_re
                .find_iter(line)
                .map(|x| x.as_str().parse().unwrap())
                .collect();
            Point::new(
                nums.remove(0),
                nums.remove(0),
                nums.remove(0),
                nums.remove(0),
            )
        })
        .collect();

    let mut p_area = i64::MAX;
    let mut area = (points.iter().map(|p| p.x).max().unwrap()
        - points.iter().map(|p| p.x).min().unwrap())
        * (points.iter().map(|p| p.y).max().unwrap() - points.iter().map(|p| p.y).min().unwrap());
    let mut steps: i64 = 0;

    while area < p_area {
        p_area = area;

        for point in points.iter_mut() {
            point.step();
        }

        let min_x = points.iter().map(|p| p.x).min().unwrap();
        let max_x = points.iter().map(|p| p.x).max().unwrap();
        let min_y = points.iter().map(|p| p.y).min().unwrap();
        let max_y = points.iter().map(|p| p.y).max().unwrap();
        area = (max_x - min_x) * (max_y - min_y);
        steps += 1;
    }

    for point in points.iter_mut() {
        point.step_back();
    }

    return (points, steps - 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 3);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2018".to_string();
    let day = "10".to_string();

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
        "\nPart 1:\nMessage: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nSteps for message to appear: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}