use cached::proc_macro::cached;
use relative_path::RelativePath;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let depth: i64 = contents
        .lines()
        .nth(0)
        .unwrap()
        .split("depth: ")
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();

    let target: Vec<i64> = contents
        .lines()
        .nth(1)
        .unwrap()
        .split("target: ")
        .nth(1)
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    let mut sum: i64 = 0;
    for x in 0..=target[0] {
        for y in 0..=target[1] {
            sum += erosion_level(x, y, depth, target.clone()) % 3;
        }

    }

    return sum;
}

fn part2(contents: String) -> i64 {
    let depth: i64 = contents
        .lines()
        .nth(0)
        .unwrap()
        .split("depth: ")
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();

    let target: Vec<i64> = contents
        .lines()
        .nth(1)
        .unwrap()
        .split("target: ")
        .nth(1)
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    let mut visited: HashSet<(i64, i64, i64)> = HashSet::new();
    let mut queue: BinaryHeap<(i64, i64, i64, i64)> = BinaryHeap::new();

    // Rocky: 0, Wet: 1, Narrow: 2
    // Neither: 0, Torch: 1, Climbing: 2
    queue.push((0, 0, 0, 1));

    while let Some((time, x, y, eqip)) = queue.pop() {
        if x == target[0] && y == target[1] && eqip == 1 {
            return -time;
        }

        if visited.contains(&(x, y, eqip)) {
            continue;
        }

        visited.insert((x, y, eqip));
        let p_region = erosion_level(x, y, depth, target.clone()) % 3;

        for (nx, ny) in [(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)].iter() {
            if *nx < 0 || *ny < 0 {
                continue;
            }

            let region = erosion_level(*nx, *ny, depth, target.clone()) % 3;
            for new_eqip in 0..3 {
                if region == new_eqip || p_region == new_eqip {
                    continue;
                }

                if new_eqip == eqip {
                    queue.push((time - 1, *nx, *ny, new_eqip));
                } else {
                    queue.push((time - 8, *nx, *ny, new_eqip));
                }
            }
        }
    }

    return -1;
}

#[cached]
fn erosion_level(x: i64, y: i64, depth: i64, target: Vec<i64>) -> i64 {
    if (x == 0 && y == 0) || (x == target[0] && y == target[1]) {
        return (0 + depth) % 20183;
    } else if y == 0 {
        return (x * 16807 + depth) % 20183;
    } else if x == 0 {
        return (y * 48271 + depth) % 20183;
    } else {
        return (erosion_level(x - 1, y, depth, target.clone())
            * erosion_level(x, y - 1, depth, target.clone())
            + depth)
            % 20183;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents = "depth: 510\ntarget: 10,10\n".to_string();

        assert_eq!(part1(contents), 114);
    }

    #[test]
    fn p2_test() {
        let contents = "depth: 510\ntarget: 10,10\n".to_string();

        assert_eq!(part2(contents), 45);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2018".to_string();
    let day = "22".to_string();

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
        "\nPart 1:\nSum of region types in target area: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nShortest path to target: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}