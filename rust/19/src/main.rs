use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let mut registers: Vec<i64> = vec![0; 6];

    let ip = contents
        .lines()
        .nth(0)
        .unwrap()
        .split(" ")
        .nth(1)
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let instructions: Vec<&str> = contents.lines().skip(1).collect();
    run(&mut registers, ip, instructions);

    return factor(registers[5]).iter().sum();
}

fn part2(contents: String) -> i64 {
    let mut registers: Vec<i64> = vec![0; 6];
    registers[0] = 1;

    let ip = contents
        .lines()
        .nth(0)
        .unwrap()
        .split(" ")
        .nth(1)
        .unwrap()
        .parse::<usize>()

        .unwrap();
    let instructions: Vec<&str> = contents.lines().skip(1).collect();
    run(&mut registers, ip, instructions);

    return factor(registers[5]).iter().sum();
}

fn run(registers: &mut Vec<i64>, ip: usize, instructions: Vec<&str>) {
    while registers[ip] < instructions.len() as i64 && registers[ip] >= 0 && registers[ip] != 1 {
        let line = instructions[registers[ip] as usize];

        let op = line.split(" ").nth(0).unwrap();
        let a = line.split(" ").nth(1).unwrap().parse::<i64>().unwrap();
        let b = line.split(" ").nth(2).unwrap().parse::<i64>().unwrap();
        let c = line.split(" ").nth(3).unwrap().parse::<i64>().unwrap();

        match op {
            "addr" => registers[c as usize] = registers[a as usize] + registers[b as usize],
            "addi" => registers[c as usize] = registers[a as usize] + b,
            "mulr" => registers[c as usize] = registers[a as usize] * registers[b as usize],
            "muli" => registers[c as usize] = registers[a as usize] * b,
            "banr" => registers[c as usize] = registers[a as usize] & registers[b as usize],
            "bani" => registers[c as usize] = registers[a as usize] & b,
            "borr" => registers[c as usize] = registers[a as usize] | registers[b as usize],
            "bori" => registers[c as usize] = registers[a as usize] | b,
            "setr" => registers[c as usize] = registers[a as usize],
            "seti" => registers[c as usize] = a,
            "gtir" => registers[c as usize] = if a > registers[b as usize] { 1 } else { 0 },
            "gtri" => registers[c as usize] = if registers[a as usize] > b { 1 } else { 0 },
            "gtrr" => {
                registers[c as usize] = if registers[a as usize] > registers[b as usize] {
                    1
                } else {
                    0
                }
            }
            "eqir" => registers[c as usize] = if a == registers[b as usize] { 1 } else { 0 },
            "eqri" => registers[c as usize] = if registers[a as usize] == b { 1 } else { 0 },
            "eqrr" => {
                registers[c as usize] = if registers[a as usize] == registers[b as usize] {
                    1
                } else {
                    0
                }
            }
            _ => panic!("Invalid opcode"),
        }

        registers[ip] += 1;
    }

    registers[ip] -= 1;
}

fn factor(n: i64) -> Vec<i64> {
    let mut factors = Vec::new();
    let upper_bound: i64 = (n as f64).sqrt() as i64;
    for i in 1..=upper_bound {
        if n % i == 0 {
            factors.push(i);
            factors.push(n / i);
        }
    }

    factors
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2018".to_string();
    let day = "19".to_string();

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
        "\nPart 1:\nValue in register 0: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nValue in register 0: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}