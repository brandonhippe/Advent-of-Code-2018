use regex::Regex;
use relative_path::RelativePath;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let area_re = Regex::new(r"(-?\d+),(-?\d+): (\d+)x(\d+)").unwrap();

    let mut claimed: HashSet<(i64, i64)> = HashSet::new();
    let mut overlaps: HashSet<(i64, i64)> = HashSet::new();
    for line in contents.lines() {
        let caps = area_re.captures(line).unwrap();
        for y in (caps[2].parse::<i64>().unwrap())
            ..(caps[2].parse::<i64>().unwrap() + caps[4].parse::<i64>().unwrap())
        {
            for x in (caps[1].parse::<i64>().unwrap())
                ..(caps[1].parse::<i64>().unwrap() + caps[3].parse::<i64>().unwrap())
            {
                if !claimed.insert((x, y)) {
                    overlaps.insert((x, y));
                }
            }
        }
    }

    return overlaps.len() as i64;
}

fn part2(contents: String) -> i64 {
    let area_re = Regex::new(r"(-?\d+),(-?\d+): (\d+)x(\d+)").unwrap();
    let mut areas: Vec<(i64, i64, i64, i64)> = Vec::new();
    for line in contents.lines() {
        let caps = area_re.captures(line).unwrap();

        areas.push((
            caps[1].parse::<i64>().unwrap(),
            caps[2].parse::<i64>().unwrap(),
            caps[3].parse::<i64>().unwrap(),
            caps[4].parse::<i64>().unwrap(),
        ));
    }

    for i in 0..areas.len() {
        let mut intersection = false;
        for j in 0..areas.len() {
            if i != j && intersects(areas[i], areas[j]) {
                intersection = true;
                break;
            }
        }

        if !intersection {
            return i as i64 + 1;
        }
    }

    return -1;
}

fn intersects(a: (i64, i64, i64, i64), b: (i64, i64, i64, i64)) -> bool {
    return a.0 < b.0 + b.2 && a.0 + a.2 > b.0 && a.1 < b.1 + b.3 && a.1 + a.3 > b.1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 4);
    }

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
    let day = "3".to_string();

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
        "\nPart 1:\nArea of overlapping claims: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nID of claim that doesn't overlap: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}