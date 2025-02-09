use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::HashMap;

fn part1(contents: String) -> i64 {
    let mut lines = contents.lines().collect::<Vec<&str>>();
    lines.sort_by(|a, b| {
        let date_a = DateTime::new(a);
        let date_b = DateTime::new(b);
        
        return date_a.cmp(&date_b);
    });

    let mut guards: HashMap<i64, Guard> = HashMap::new();
    let mut current_guard: i64 = 0;
    for line in lines {
        let date = DateTime::new(line);
        if line.contains("Guard") {
            let id = line.split_whitespace().collect::<Vec<&str>>()[3].replace("#", "").parse::<i64>().unwrap();
            if !guards.contains_key(&id) {
                guards.insert(id, Guard::new(id));
            }

            current_guard = id;
        } else if line.contains("falls asleep") {
            let start = DateTime::new(line);
            let end = DateTime::new(line);
            guards.get_mut(&current_guard).unwrap().sleep_times.push((start, end));
        } else if line.contains("wakes up") {
            let end = DateTime::new(line);
            guards.get_mut(&current_guard).unwrap().sleep_times.last_mut().unwrap().1 = end;
        }
    }


    let max_guard = guards.iter().max_by_key(|x| x.1.total_sleep()).unwrap().1;
    
    return max_guard.id * max_guard.most_asleep() as i64;
}

fn part2(contents: String) -> i64 {
    let mut lines = contents.lines().collect::<Vec<&str>>();
    lines.sort_by(|a, b| {
        let date_a = DateTime::new(a);
        let date_b = DateTime::new(b);
        
        return date_a.cmp(&date_b);
    });

    let mut guards: HashMap<i64, Guard> = HashMap::new();
    let mut current_guard: i64 = 0;
    for line in lines {
        let date = DateTime::new(line);
        if line.contains("Guard") {
            let id = line.split_whitespace().collect::<Vec<&str>>()[3].replace("#", "").parse::<i64>().unwrap();
            if !guards.contains_key(&id) {
                guards.insert(id, Guard::new(id));
            }

            current_guard = id;
        } else if line.contains("falls asleep") {
            let start = DateTime::new(line);
            let end = DateTime::new(line);
            guards.get_mut(&current_guard).unwrap().sleep_times.push((start, end));
        } else if line.contains("wakes up") {
            let end = DateTime::new(line);
            guards.get_mut(&current_guard).unwrap().sleep_times.last_mut().unwrap().1 = end;
        }
    }

    let mut minute_counts: HashMap<i64, HashMap<i64, i64>> = HashMap::new();
    let mut most_minute = 0;
    let mut most_guard = 0;
    let mut most_times = 0;

    for guard in guards.values() {
        for (start, end) in &guard.sleep_times {
            for i in start.minute..end.minute {
                *minute_counts.entry(i).or_insert(HashMap::new()).entry(guard.id).or_insert(0) += 1;
                if *minute_counts.get(&i).unwrap().get(&guard.id).unwrap() > most_times {
                    most_times = *minute_counts.get(&i).unwrap().get(&guard.id).unwrap();
                    most_minute = i;
                    most_guard = guard.id;
                }
            }
        }
    }
    
    return most_minute * most_guard;
}

#[derive(Debug, Ord, PartialOrd, PartialEq, Eq, Hash, Clone, Copy)]
struct DateTime {
    year: i64,
    month: i64,
    day: i64,
    hour: i64,
    minute: i64,
}

impl DateTime {
    fn new(date_str: &str) -> DateTime {
        let datestr = date_str.replace("[", "").replace("]", "");
        let date = datestr.split_whitespace().collect::<Vec<&str>>()[0];
        let time = datestr.split_whitespace().collect::<Vec<&str>>()[1];

        let year = date.split("-").collect::<Vec<&str>>()[0].parse::<i64>().unwrap();
        let month = date.split("-").collect::<Vec<&str>>()[1].parse::<i64>().unwrap();
        let day = date.split("-").collect::<Vec<&str>>()[2].parse::<i64>().unwrap();

        let hour = time.split(":").collect::<Vec<&str>>()[0].parse::<i64>().unwrap();
        let minute = time.split(":").collect::<Vec<&str>>()[1].parse::<i64>().unwrap();

        DateTime {
            year,
            month,
            day,
            hour,
            minute,
        }
    }
}

#[derive(Debug, Ord, PartialOrd, PartialEq, Eq, Hash, Clone)]
struct Guard {
    id: i64,
    sleep_times: Vec<(DateTime, DateTime)>,
}

impl Guard {
    fn new(id: i64) -> Guard {
        Guard {
            id,
            sleep_times: Vec::new(),
        }
    }

    fn total_sleep(&self) -> i64 {
        return self.sleep_times.iter().map(|x| x.1.minute - x.0.minute).sum();
    }

    fn most_asleep(&self) -> i64 {
        let mut minutes: HashMap<i64, i64> = HashMap::new();
        for (start, end) in &self.sleep_times {
            for i in start.minute..end.minute {
                *minutes.entry(i).or_insert(0) += 1;
            }
        }

        return *minutes.iter().max_by_key(|x| x.1).unwrap().0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 240);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 4455);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2018".to_string();
    let day = "4".to_string();

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
        "\nPart 1:\nGuard ID * Minute: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nGuard ID * Minute: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}