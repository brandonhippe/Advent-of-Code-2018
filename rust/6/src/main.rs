use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::HashMap;

fn part1(contents: String) -> i64 {
    let mut max_x: i64 = i64::MIN;
    let mut max_y: i64 = i64::MIN;
    let mut min_x: i64 = i64::MAX;
    let mut min_y: i64 = i64::MAX;

    let mut points: Vec<(i64, i64)> = Vec::new();

    for line in contents.lines() {
        let parts: Vec<&str> = line.split(", ").collect();
        let x: i64 = parts[0].parse().unwrap();
        let y: i64 = parts[1].parse().unwrap();

        if x > max_x {
            max_x = x;
        }
        if x < min_x {
            min_x = x;
        }
        if y > max_y {
            max_y = y;
        }
        if y < min_y {
            min_y = y;
        }

        points.push((x, y));
    }


    let mut area_sizes: HashMap<(i64, i64), (bool, i64)> = HashMap::from_iter(points.iter().map(|&p| (p, (true, 0))).collect::<Vec<_>>());

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let mut min_dist: i64 = i64::MAX;
            let mut min_point: (i64, i64) = (0, 0);
            let mut is_tie: bool = false;

            for point in &points {
                let dist = (x - point.0).abs() + (y - point.1).abs();
                if dist < min_dist {
                    min_dist = dist;
                    min_point = *point;
                    is_tie = false;
                } else if dist == min_dist {
                    is_tie = true;
                }
            }

            if !is_tie {
                if x == min_x || x == max_x || y == min_y || y == max_y {
                    area_sizes.insert(min_point, (false, 0));
                } else {
                    let (inside, size) = area_sizes.get(&min_point).unwrap();
                    if *inside {
                        area_sizes.insert(min_point, (true, size + 1));
                    }
                }
            }
        }
    }

    return area_sizes.iter().filter(|(_, v)| v.0).map(|(_, v)| v.1).max().unwrap();
}

fn part2(contents: String, max_total_dist: i64) -> i64 {
    let mut max_x: i64 = i64::MIN;
    let mut max_y: i64 = i64::MIN;
    let mut min_x: i64 = i64::MAX;
    let mut min_y: i64 = i64::MAX;

    let mut points: Vec<(i64, i64)> = Vec::new();

    for line in contents.lines() {
        let parts: Vec<&str> = line.split(", ").collect();
        let x: i64 = parts[0].parse().unwrap();
        let y: i64 = parts[1].parse().unwrap();

        if x > max_x {
            max_x = x;
        }
        if x < min_x {
            min_x = x;
        }
        if y > max_y {
            max_y = y;
        }
        if y < min_y {
            min_y = y;
        }

        points.push((x, y));
    }

    let mut region_size: i64 = 0;
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let total_dist: i64 = points.iter().map(|p| (x - p.0).abs() + (y - p.1).abs()).sum();
            if total_dist < max_total_dist {
                region_size += 1;
            }
        }
    }

    return region_size;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 17);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents, 32), 16);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2018".to_string();
    let day = "6".to_string();

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
        "\nPart 1:\nLargest finite area: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nRegion within 10000 total units of all points: {}\nRan in {:.5?}",
        part2(contents.clone(), 10000),
        part2_timer.elapsed()
    );
}