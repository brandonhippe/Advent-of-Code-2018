use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> String {
    let serial: i64 = contents.trim().parse().unwrap();
    let power_levels: Vec<Vec<i64>> = Vec::from_iter(
        (1..=300).map(|y| {
            Vec::from_iter(
                (1..=300).map(|x| {
                    let rack_id = x + 10;
                    let mut power = rack_id * y;
                    power += serial;
                    power *= rack_id;
                    power = (power / 100) % 10;
                    power -= 5;
                    power
                })
            )
        })
    );

    let mut max_power = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    for y in 0..298 {
        for x in 0..298 {
            let mut total_power = 0;
            for i in 0..3 {
                for j in 0..3 {
                    total_power += power_levels[y + j][x + i];
                }
            }

            if total_power > max_power {
                max_power = total_power;
                max_x = x + 1;
                max_y = y + 1;
            }
        }
    }

    return format!("{},{}", max_x, max_y);
}

fn part2(contents: String) -> String {
    let serial: i64 = contents.trim().parse().unwrap();
    let check_back: i64 = 1;

    let power_levels: Vec<Vec<i64>> = Vec::from_iter(
        (1..=300).map(|y| {
            Vec::from_iter(
                (1..=300).map(|x| {
                    let rack_id = x + 10;
                    let mut power = rack_id * y;
                    power += serial;
                    power *= rack_id;
                    power = (power / 100) % 10;
                    power -= 5;
                    power
                })
            )
        })
    );

    let mut max_power = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_size = 0;
    let mut increased: Vec<bool> = vec![];

    for size in 3..=300 {
        for y in 0..(300 - size) {
            for x in 0..(300 - size) {
                let mut total_power = 0;
                for i in 0..size {
                    for j in 0..size {
                        total_power += power_levels[y + j][x + i];
                    }
                }

                if total_power > max_power {
                    max_power = total_power;
                    max_x = x + 1;
                    max_y = y + 1;
                    max_size = size;
                    increased.push(true);
                }
            }
        }

        if increased.len() == size as usize - 3 {
            increased.push(false);
        }

        if increased[increased.len() - check_back as usize..increased.len()]
            .iter()
            .all(|&x| x == false)
        {
            break;
        }
    }

    return format!("{},{},{}", max_x, max_y, max_size);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let mut contents =
            fs::read_to_string("example1.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), "33,45".to_string());

        contents =
            fs::read_to_string("example2.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), "21,61".to_string());
    }

    #[test]
    fn p2_test() {
        let mut contents =
            fs::read_to_string("example1.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), "90,269,16".to_string());

        contents =
            fs::read_to_string("example2.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), "232,251,12".to_string());
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2018".to_string();
    let day = "11".to_string();

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
        "\nPart 1:\nLargest power: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nLargest power: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}