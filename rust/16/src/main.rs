use cached::proc_macro::cached;
use regex::Regex;
use relative_path::RelativePath;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let mut examples: Vec<Vec<String>> = Vec::new();
    let mut example: Vec<String> = Vec::new();
    for line in contents.lines() {
        if line.is_empty() {
            if example.is_empty() {
                break;
            }
            examples.push(example.clone());
            example.clear();
        } else {
            example.push(line.to_string());
        }
    }

    return determine_opcodes(examples).0;
}

fn part2(contents: String) -> i64 {
    let mut examples: Vec<Vec<String>> = Vec::new();
    let mut example: Vec<String> = Vec::new();
    for line in contents.lines() {
        if line.is_empty() {
            if example.is_empty() {
                break;
            }

            examples.push(example.clone());
            example.clear();
        } else {
            example.push(line.to_string());
        }
    }

    let opcodes = determine_opcodes(examples).1;

    let mut registers: Vec<i64> = vec![0, 0, 0, 0];
    let num_regex = Regex::new(r"\d+").unwrap();

    for line in contents.split("\n\n\n\n").nth(1).unwrap().lines() {
        let instruction: Vec<usize> = num_regex
            .find_iter(&line)
            .map(|x| x.as_str().parse().unwrap())
            .collect();

        let (opcode, r1, r2, r_out) = (
            instruction[0],
            instruction[1],
            instruction[2],
            instruction[3],
        );

        match opcodes.get(&(opcode as i64)).unwrap().as_str() {
            "addr" => registers[r_out] = registers[r1] + registers[r2],
            "addi" => registers[r_out] = registers[r1] + r2 as i64,
            "mulr" => registers[r_out] = registers[r1] * registers[r2],
            "muli" => registers[r_out] = registers[r1] * r2 as i64,
            "banr" => registers[r_out] = registers[r1] & registers[r2],
            "bani" => registers[r_out] = registers[r1] & r2 as i64,
            "borr" => registers[r_out] = registers[r1] | registers[r2],
            "bori" => registers[r_out] = registers[r1] | r2 as i64,
            "setr" => registers[r_out] = registers[r1],
            "seti" => registers[r_out] = r1 as i64,
            "gtir" => registers[r_out] = (r1 as i64 > registers[r2]) as i64,
            "gtri" => registers[r_out] = (registers[r1] > r2 as i64) as i64,
            "gtrr" => registers[r_out] = (registers[r1] > registers[r2]) as i64,
            "eqir" => registers[r_out] = (r1 as i64 == registers[r2]) as i64,
            "eqri" => registers[r_out] = (registers[r1] == r2 as i64) as i64,
            "eqrr" => registers[r_out] = (registers[r1] == registers[r2]) as i64,
            _ => panic!("Invalid instruction"),
        }
    }

    return registers[0];
}

#[cached]
fn determine_opcodes(examples: Vec<Vec<String>>) -> (i64, HashMap<i64, String>) {
    let mut possible_opcodes: HashMap<i64, HashSet<&str>> = HashMap::new();
    let all_instructions: HashSet<&str> = HashSet::from([
        "addr", "addi", "mulr", "muli", "banr", "bani", "borr", "bori", "setr", "seti", "gtir",
        "gtri", "gtrr", "eqir", "eqri", "eqrr",
    ]);

    for i in 0..all_instructions.len() {
        possible_opcodes.insert(i as i64, all_instructions.clone());
    }

    let mut act_as_three: i64 = 0;

    let num_regex = Regex::new(r"\d+").unwrap();
    for ex in examples {
        let before: Vec<i64> = num_regex
            .find_iter(&ex[0])
            .map(|x| x.as_str().parse().unwrap())
            .collect();
        let instruction: Vec<usize> = num_regex
            .find_iter(&ex[1])
            .map(|x| x.as_str().parse().unwrap())
            .collect();
        let after: Vec<i64> = num_regex
            .find_iter(&ex[2])
            .map(|x| x.as_str().parse().unwrap())
            .collect();

        let (opcode, r1, r2, r_out) = (
            instruction[0],
            instruction[1],
            instruction[2],
            instruction[3],
        );
        let possible_instructions: HashSet<&str> = all_instructions
            .iter()
            .filter(|ins| match ins {
                &&"addr" => before[r1] + before[r2] == after[r_out],
                &&"addi" => before[r1] + r2 as i64 == after[r_out],
                &&"mulr" => before[r1] * before[r2] == after[r_out],
                &&"muli" => before[r1] * r2 as i64 == after[r_out],
                &&"banr" => before[r1] & before[r2] == after[r_out],
                &&"bani" => before[r1] & r2 as i64 == after[r_out],
                &&"borr" => before[r1] | before[r2] == after[r_out],
                &&"bori" => before[r1] | r2 as i64 == after[r_out],
                &&"setr" => before[r1] == after[r_out],
                &&"seti" => r1 as i64 == after[r_out],
                &&"gtir" => (r1 as i64 > before[r2]) as i64 == after[r_out],
                &&"gtri" => (before[r1] > r2 as i64) as i64 == after[r_out],
                &&"gtrr" => (before[r1] > before[r2]) as i64 == after[r_out],
                &&"eqir" => (r1 as i64 == before[r2]) as i64 == after[r_out],
                &&"eqri" => (before[r1] == r2 as i64) as i64 == after[r_out],
                &&"eqrr" => (before[r1] == before[r2]) as i64 == after[r_out],
                _ => panic!("Invalid instruction"),
            })
            .map(|ins| *ins)
            .collect();

        act_as_three += (possible_instructions.len() >= 3) as i64;
        let mut pos_ops = possible_opcodes.get(&(opcode as i64)).unwrap().clone();
        pos_ops = pos_ops
            .intersection(&possible_instructions)
            .map(|x| *x)
            .collect();
        possible_opcodes.insert(opcode as i64, pos_ops.clone());
    }

    let mut final_opcodes: HashMap<i64, String> = HashMap::new();
    while final_opcodes.len() < all_instructions.len() {
        for (opcode, possible) in &mut possible_opcodes {
            if possible.len() == 1 {
                final_opcodes.insert(*opcode, possible.iter().next().unwrap().to_string());
            } else {
                for (_, val) in final_opcodes.iter() {
                    possible.remove(val.as_str());
                }
            }
        }
    }

    return (act_as_three, final_opcodes);
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2018".to_string();
    let day = "16".to_string();

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
        "\nPart 1:\nNumber of examples that act like 3 or more opcodes: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nRegister 0 after running program: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}