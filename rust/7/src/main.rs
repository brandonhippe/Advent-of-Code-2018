use relative_path::RelativePath;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> String {
    let mut steps: HashMap<char, Vec<char>> = HashMap::new();
    let mut prereqs: HashMap<char, HashSet<char>> = HashMap::new();

    for line in contents.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let prereq = parts[1].chars().next().unwrap();
        let step = parts[7].chars().next().unwrap();

        steps.entry(step).or_insert(Vec::new());
        prereqs.entry(prereq).or_insert(HashSet::new());

        steps.entry(prereq).or_insert(Vec::new()).push(step);
        prereqs.entry(step).or_insert(HashSet::new()).insert(prereq);
    }

    let mut available: BinaryHeap<Reverse<char>> = BinaryHeap::new();
    for (step, prereq) in &prereqs {
        if prereq.len() == 0 {
            available.push(Reverse(*step));
        }
    }

    let mut finished: HashSet<char> = HashSet::new();
    let mut order = String::new();
    while !available.is_empty() {

        let step = available.pop().unwrap().0;
        order.push(step);
        finished.insert(step);

        for next in steps.get(&step).unwrap() {
            if finished.is_superset(&prereqs.get(next).unwrap()) {
                available.push(Reverse(*next));
            }
        }
    }

    return order;
}

fn part2(contents: String, num_workers: i64, base_time: i64) -> i64 {
    let mut steps: HashMap<char, Vec<char>> = HashMap::new();
    let mut prereqs: HashMap<char, HashSet<char>> = HashMap::new();

    for line in contents.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let prereq = parts[1].chars().next().unwrap();
        let step = parts[7].chars().next().unwrap();

        steps.entry(step).or_insert(Vec::new());
        prereqs.entry(prereq).or_insert(HashSet::new());

        steps.entry(prereq).or_insert(Vec::new()).push(step);
        prereqs.entry(step).or_insert(HashSet::new()).insert(prereq);
    }

    let mut available: BinaryHeap<Reverse<char>> = BinaryHeap::new();
    for (step, prereq) in &prereqs {
        if prereq.len() == 0 {
            available.push(Reverse(*step));
        }
    }

    let mut finished: HashSet<char> = HashSet::new();
    let mut curr_time: i64 = 0;
    let mut workers: BinaryHeap<Reverse<Worker>> = BinaryHeap::new();

    while available.len() > 0 || workers.len() > 0 {
        if workers.len() < num_workers as usize && available.len() > 0 {
            let step = available.pop().unwrap().0;
            workers.push(Reverse(Worker {
                time: curr_time + base_time + (step as i64 - 'A' as i64) + 1,
                step: step,
            }));
        } else {
            let finished_worker = workers.pop().unwrap().0;
            curr_time = finished_worker.time;
            finished.insert(finished_worker.step);

            for next in steps.get(&finished_worker.step).unwrap() {
                if finished.is_superset(&prereqs.get(next).unwrap()) {
                    available.push(Reverse(*next));
                }
            }
        }
    }

    return curr_time;
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Clone)]
struct Worker {
    time: i64,
    step: char,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), "CABDFE".to_string());
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents, 2, 0), 15);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2018".to_string();
    let day = "7".to_string();

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
        "\nPart 1:\nOrder of completion: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nTime to complete: {}\nRan in {:.5?}",
        part2(contents.clone(), 5, 60),
        part2_timer.elapsed()
    );
}