use relative_path::RelativePath;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let mut two_count: i64 = 0;
    let mut three_count: i64 = 0;

    for line in contents.lines() {
        let letter_counts: HashMap<char, i64> = line.chars().fold(HashMap::new(), |acc, c| {
            let mut new_acc = acc.clone();
            *new_acc.entry(c).or_insert(0) += 1;
            new_acc
        });

        two_count += letter_counts.values().any(|&x| x == 2) as i64;
        three_count += letter_counts.values().any(|&x| x == 3) as i64;
    }

    return two_count * three_count;
}

fn part2(contents: String) -> String {
    for (i, line) in contents.lines().enumerate() {
        for other_line in contents.lines().skip(i + 1) {
            let mut diff_count = 0;
            let mut diff_index = 0;

            for (j, (c1, c2)) in line.chars().zip(other_line.chars()).enumerate() {
                if c1 != c2 {
                    diff_count += 1;
                    diff_index = j;

                }
            }

            if diff_count == 1 {
                return line
                    .chars()
                    .enumerate()
                    .filter_map(|(i, c)| if i != diff_index { Some(c) } else { None })
                    .collect();
            }
        }
    }

    return "".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("p1_example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 12);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("p2_example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), "fgij".to_string());
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2018".to_string();
    let day = "2".to_string();

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
        "\nPart 1:\nChecksum: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nCommon letters: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}