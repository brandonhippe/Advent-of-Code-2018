use relative_path::RelativePath;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> String {
    let mut tracks: HashMap<(i64, i64), HashMap<(i64, i64), (i64, i64)>> = HashMap::new();
    let mut carts: Vec<Cart> = Vec::new();
    let mut intersctions: HashSet<(i64, i64)> = HashSet::new();
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let mut dir_map: HashMap<(i64, i64), (i64, i64)> = HashMap::new();
            match c {
                '-' | '<' | '>' => {
                    dir_map.insert((1, 0), (1, 0));
                    dir_map.insert((-1, 0), (-1, 0));

                    if c == '<' {
                        carts.push(Cart {
                            x: x as i64,
                            y: y as i64,
                            dx: -1,
                            dy: 0,
                            intersection: 0,
                        });
                    } else if c == '>' {
                        carts.push(Cart {
                            x: x as i64,
                            y: y as i64,
                            dx: 1,
                            dy: 0,
                            intersection: 0,

                        });
                    }

                    tracks.insert((x as i64, y as i64), dir_map);
                }
                '|' | 'v' | '^' => {
                    dir_map.insert((0, 1), (0, 1));
                    dir_map.insert((0, -1), (0, -1));

                    if c == '^' {
                        carts.push(Cart {
                            x: x as i64,
                            y: y as i64,
                            dx: 0,
                            dy: -1,
                            intersection: 0,
                        });
                    } else if c == 'v' {
                        carts.push(Cart {
                            x: x as i64,
                            y: y as i64,
                            dx: 0,
                            dy: 1,
                            intersection: 0,
                        });
                    }

                    tracks.insert((x as i64, y as i64), dir_map);
                }
                '/' => {
                    dir_map.insert((0, -1), (1, 0));
                    dir_map.insert((-1, 0), (0, 1));
                    dir_map.insert((0, 1), (-1, 0));
                    dir_map.insert((1, 0), (0, -1));

                    tracks.insert((x as i64, y as i64), dir_map);
                }
                '\\' => {
                    dir_map.insert((0, -1), (-1, 0));
                    dir_map.insert((1, 0), (0, 1));
                    dir_map.insert((0, 1), (1, 0));
                    dir_map.insert((-1, 0), (0, -1));

                    tracks.insert((x as i64, y as i64), dir_map);
                }
                '+' => {
                    intersctions.insert((x as i64, y as i64));
                }
                _ => {}
            }
        }
    }

    loop {
        carts.sort_by(|a, b| a.y.cmp(&b.y).then(a.x.cmp(&b.x)));
        let mut cart_positions: HashSet<(i64, i64)> = HashSet::new();

        for cart in carts.iter_mut() {
            cart.move_cart(&tracks, &intersctions);

            if cart_positions.contains(&(cart.x, cart.y)) {
                return format!("{},{}", cart.x, cart.y);
            }

            cart_positions.insert((cart.x, cart.y));
        }
    }
}

fn part2(contents: String) -> String {
    let mut tracks: HashMap<(i64, i64), HashMap<(i64, i64), (i64, i64)>> = HashMap::new();
    let mut carts: VecDeque<Cart> = VecDeque::new();
    let mut intersctions: HashSet<(i64, i64)> = HashSet::new();
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let mut dir_map: HashMap<(i64, i64), (i64, i64)> = HashMap::new();
            match c {
                '-' | '<' | '>' => {
                    dir_map.insert((1, 0), (1, 0));
                    dir_map.insert((-1, 0), (-1, 0));

                    if c == '<' {
                        carts.push_back(Cart {
                            x: x as i64,
                            y: y as i64,
                            dx: -1,
                            dy: 0,
                            intersection: 0,
                        });
                    } else if c == '>' {
                        carts.push_back(Cart {
                            x: x as i64,
                            y: y as i64,
                            dx: 1,
                            dy: 0,
                            intersection: 0,
                        });
                    }

                    tracks.insert((x as i64, y as i64), dir_map);
                }
                '|' | 'v' | '^' => {
                    dir_map.insert((0, 1), (0, 1));
                    dir_map.insert((0, -1), (0, -1));

                    if c == '^' {
                        carts.push_back(Cart {
                            x: x as i64,
                            y: y as i64,
                            dx: 0,
                            dy: -1,
                            intersection: 0,
                        });
                    } else if c == 'v' {
                        carts.push_back(Cart {
                            x: x as i64,
                            y: y as i64,
                            dx: 0,
                            dy: 1,
                            intersection: 0,
                        });
                    }

                    tracks.insert((x as i64, y as i64), dir_map);
                }
                '/' => {
                    dir_map.insert((0, -1), (1, 0));
                    dir_map.insert((-1, 0), (0, 1));
                    dir_map.insert((0, 1), (-1, 0));
                    dir_map.insert((1, 0), (0, -1));

                    tracks.insert((x as i64, y as i64), dir_map);
                }
                '\\' => {
                    dir_map.insert((0, -1), (-1, 0));
                    dir_map.insert((1, 0), (0, 1));
                    dir_map.insert((0, 1), (1, 0));
                    dir_map.insert((-1, 0), (0, -1));

                    tracks.insert((x as i64, y as i64), dir_map);
                }
                '+' => {
                    intersctions.insert((x as i64, y as i64));
                }
                _ => {}
            }
        }
    }

    while carts.len() > 1 {
        let mut new_carts: VecDeque<Cart> = VecDeque::new();

        while carts.len() > 0 {
            let mut cart = carts.pop_front().unwrap();
            cart.move_cart(&tracks, &intersctions);

            let mut crash = false;
            for (i, c) in carts.iter().enumerate() {
                if c.x == cart.x && c.y == cart.y {
                    carts.remove(i);
                    crash = true;
                    break;
                }
            }

            for c in new_carts.iter() {
                if c.x == cart.x && c.y == cart.y {
                    new_carts.retain(|x| x.x != cart.x || x.y != cart.y);
                    crash = true;
                    break;
                }
            }

            if !crash {
                new_carts.push_back(cart);

                let ix = new_carts.len() - 1;
                while ix > 0 && new_carts[ix] < new_carts[ix - 1] {
                    new_carts.swap(ix, ix - 1);
                }
            }
        }

        carts = new_carts;
    }

    return format!("{},{}", carts[0].x, carts[0].y);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Cart {
    y: i64,
    x: i64,
    dy: i64,
    dx: i64,
    intersection: i64,
}

impl Cart {
    fn move_cart(
        &mut self,
        tracks: &HashMap<(i64, i64), HashMap<(i64, i64), (i64, i64)>>,
        intersctions: &HashSet<(i64, i64)>,
    ) {
        self.x += self.dx;
        self.y += self.dy;

        if let Some(_) = intersctions.get(&(self.x, self.y)) {
            match self.intersection {
                0 => {
                    self.intersection = 1;
                    self.turn_left();
                }
                1 => {
                    self.intersection = 2;
                }
                2 => {
                    self.intersection = 0;
                    self.turn_right();
                }
                _ => {}
            }

            return;
        }

        if let Some(track) = tracks.get(&(self.x, self.y)) {
            if let Some((dx, dy)) = track.get(&(self.dx, self.dy)) {
                self.dx = *dx;
                self.dy = *dy;
            }
        }
    }

    fn turn_left(&mut self) {
        let temp = self.dx;
        self.dx = self.dy;
        self.dy = -temp;
    }

    fn turn_right(&mut self) {
        let temp = self.dx;
        self.dx = -self.dy;
        self.dy = temp;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("p1_example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), "7,3".to_string());
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("p2_example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), "6,4".to_string());
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2018".to_string();
    let day = "13".to_string();

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
        "\nPart 1:\nLocation of first crash: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nPosition of last remaining cart: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}