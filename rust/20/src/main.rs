use cached::proc_macro::cached;
use relative_path::RelativePath;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    return contents
        .lines()
        .map(|line| {
            let (room_connections, mut furthest_candidates) = examine_rooms(line.to_string());

            let mut visited = HashSet::new();
            let mut queue: VecDeque<(i64, (i64, i64))> = VecDeque::new();
            queue.push_back((0, (0, 0)));

            let mut furthest: i64 = -1;

            while let Some((distance, room)) = queue.pop_front() {
                if visited.contains(&room) {
                    continue;
                }

                visited.insert(room);

                if furthest_candidates.contains(&room) {
                    if furthest_candidates.len() == 1 {
                        furthest = distance;
                    }

                    furthest_candidates.remove(&room);
                }


                for (dest, _) in room_connections
                    .get(&room)
                    .unwrap_or(&HashMap::new())
                    .iter()
                {
                    queue.push_back((distance + 1, dest.clone()));
                }
            }

            furthest
        })
        .sum::<i64>();
}

fn part2(contents: String) -> i64 {
    return contents
        .lines()
        .map(|line| {
            let room_connections = examine_rooms(line.to_string()).0;

            let mut visited = HashSet::new();
            let mut queue: VecDeque<(i64, (i64, i64))> = VecDeque::new();
            queue.push_back((0, (0, 0)));

            let mut over_1000: i64 = 0;

            while let Some((distance, room)) = queue.pop_front() {
                if visited.contains(&room) {
                    continue;
                }

                visited.insert(room);

                if distance >= 1000 {
                    over_1000 += 1;
                }

                for (dest, _) in room_connections
                    .get(&room)
                    .unwrap_or(&HashMap::new())
                    .iter()
                {
                    queue.push_back((distance + 1, dest.clone()));
                }
            }

            over_1000
        })
        .sum::<i64>();
}

#[cached]
fn examine_rooms(
    room_regex: String,
) -> (
    HashMap<(i64, i64), HashMap<(i64, i64), i64>>,
    HashSet<(i64, i64)>,
) {
    let mut room_connections: HashMap<(i64, i64), HashMap<(i64, i64), i64>> = HashMap::new();
    let starts = vec![(0, 0)];
    let furthest_candidates: HashSet<(i64, i64)> =
        follow_regex(room_regex, starts, &mut room_connections);

    return (room_connections, furthest_candidates);
}

fn follow_regex(
    room_regex: String,
    starts: Vec<(i64, i64)>,
    room_connections: &mut HashMap<(i64, i64), HashMap<(i64, i64), i64>>,
) -> HashSet<(i64, i64)> {
    let mut current_rooms = starts.clone();
    let mut opened_parentheses = 0;
    let mut sub_regex = String::new();

    let mut dests = HashSet::new();
    for c in room_regex.chars() {
        if opened_parentheses > 0 {
            if c == '(' {
                opened_parentheses += 1;
            } else if c == ')' {
                opened_parentheses -= 1;
                if opened_parentheses == 0 {
                    current_rooms =
                        follow_regex(sub_regex.clone(), current_rooms.clone(), room_connections)
                            .iter()
                            .cloned()
                            .collect();
                }
            }

            sub_regex.push(c);
        } else {
            match c {
                'N' => {
                    for room in current_rooms.iter_mut() {
                        if room_connections.get(room).is_none() {
                            room_connections.insert(room.clone(), HashMap::new());
                        }

                        room_connections
                            .get_mut(room)
                            .unwrap_or(&mut HashMap::new())
                            .insert((room.0, room.1 - 1), 1);
                        room.1 -= 1;

                        if room_connections.get(room).is_none() {
                            room_connections.insert(room.clone(), HashMap::new());
                        }

                        room_connections
                            .get_mut(room)
                            .unwrap_or(&mut HashMap::new())
                            .insert((room.0, room.1 + 1), 1);
                    }
                }
                'S' => {
                    for room in current_rooms.iter_mut() {
                        if room_connections.get(room).is_none() {
                            room_connections.insert(room.clone(), HashMap::new());
                        }

                        room_connections
                            .get_mut(room)
                            .unwrap_or(&mut HashMap::new())
                            .insert((room.0, room.1 + 1), 1);
                        room.1 += 1;

                        if room_connections.get(room).is_none() {
                            room_connections.insert(room.clone(), HashMap::new());
                        }

                        room_connections
                            .get_mut(room)
                            .unwrap_or(&mut HashMap::new())
                            .insert((room.0, room.1 - 1), 1);
                    }
                }
                'E' => {
                    for room in current_rooms.iter_mut() {
                        if room_connections.get(room).is_none() {
                            room_connections.insert(room.clone(), HashMap::new());
                        }

                        room_connections
                            .get_mut(room)
                            .unwrap_or(&mut HashMap::new())
                            .insert((room.0 + 1, room.1), 1);
                        room.0 += 1;

                        if room_connections.get(room).is_none() {
                            room_connections.insert(room.clone(), HashMap::new());
                        }

                        room_connections
                            .get_mut(room)
                            .unwrap_or(&mut HashMap::new())
                            .insert((room.0 - 1, room.1), 1);
                    }
                }
                'W' => {
                    for room in current_rooms.iter_mut() {
                        if room_connections.get(room).is_none() {
                            room_connections.insert(room.clone(), HashMap::new());
                        }

                        room_connections
                            .get_mut(room)
                            .unwrap_or(&mut HashMap::new())
                            .insert((room.0 - 1, room.1), 1);
                        room.0 -= 1;

                        if room_connections.get(room).is_none() {
                            room_connections.insert(room.clone(), HashMap::new());
                        }

                        room_connections
                            .get_mut(room)
                            .unwrap_or(&mut HashMap::new())
                            .insert((room.0 + 1, room.1), 1);
                    }
                }
                '|' => {
                    dests.extend(current_rooms.iter().cloned());
                    current_rooms = starts.clone();
                }
                '(' => {
                    opened_parentheses += 1;
                    sub_regex = String::new();
                }
                _ => {}
            };
        }
    }

    dests.extend(current_rooms.iter().cloned());
    return dests;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 85);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2018".to_string();
    let day = "20".to_string();

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
        "\nPart 1:\nDistance to furthest room: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nRooms at least 1000 away from start: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}