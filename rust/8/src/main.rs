use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    return sum_metadata(
        &contents
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect(),
        0,
    )
    .0;
}

fn part2(contents: String) -> i64 {
    return evaluate(
        &contents
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect(),
        0,
    )
    .0;
}

fn sum_metadata(nums: &Vec<i64>, index: usize) -> (i64, usize) {
    let num_children = nums[index];
    let num_metadata = nums[index + 1];

    let mut sum = 0;
    let mut i = index + 2;
    for _ in 0..num_children {
        let (child_sum, new_i) = sum_metadata(nums, i);

        sum += child_sum;
        i = new_i;
    }

    for _ in 0..num_metadata {
        sum += nums[i];
        i += 1;
    }

    return (sum, i);
}

fn evaluate(nums: &Vec<i64>, index: usize) -> (i64, usize) {
    let num_children = nums[index];
    if num_children == 0 {
        return sum_metadata(nums, index);
    }

    let num_metadata = nums[index + 1];
    let mut children_vals = vec![];
    let mut i = index + 2;

    for _ in 0..num_children {
        let (child_val, new_i) = evaluate(nums, i);
        children_vals.push(child_val);
        i = new_i;
    }

    let mut sum = 0;
    for _ in 0..num_metadata {
        let metadata = nums[i];
        if metadata > 0 && metadata <= num_children {
            sum += children_vals[(metadata - 1) as usize];
        }
        i += 1;
    }

    return (sum, i);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 138);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 66);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2018".to_string();
    let day = "8".to_string();

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
        "\nPart 1:\nSum of metadata entries: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nValue of root node: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}