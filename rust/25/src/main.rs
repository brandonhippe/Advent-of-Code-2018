use relative_path::RelativePath;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let mut points = HashSet::new();
    for line in contents.lines() {
        let mut coords = line.split(",").map(|x| x.parse::<i64>().unwrap());
        points.insert((
            coords.next().unwrap(),
            coords.next().unwrap(),
            coords.next().unwrap(),
            coords.next().unwrap(),
        ));
    }

    let mut constellations: i64 = 0;
    while !points.is_empty() {
        let mut constellation = HashSet::new();
        let mut to_visit = vec![points.iter().next().unwrap().clone()];
        while !to_visit.is_empty() {
            let point = to_visit.pop().unwrap();
            if constellation.insert(point.clone()) {
                points.remove(&point);
                for other in points.iter() {
                    if manhattan_distance(point, *other) <= 3 {
                        to_visit.push(other.clone());
                    }
                }
            }
        }

        constellations += 1;

    }

    return constellations;
}

fn part2(_contents: String) -> String {
    return "Christmas has been saved!".to_string();
}

fn manhattan_distance(a: (i64, i64, i64, i64), b: (i64, i64, i64, i64)) -> i64 {
    return (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs() + (a.3 - b.3).abs();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 8);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2018".to_string();
    let day = "25".to_string();

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
        "\nPart 1:\nNumber of constellations: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\n{}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}