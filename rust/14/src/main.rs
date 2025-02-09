use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let mut recipes = vec![3, 7];
    let mut elf1 = 0;
    let mut elf2 = 1;

    for _ in 0..contents.parse::<usize>().unwrap() + 10 {
        recipes = gen_recipes(recipes, &mut elf1, &mut elf2);
    }

    return recipes
        .iter()
        .skip(contents.parse::<usize>().unwrap())
        .take(10)
        .fold(0, |acc, x| acc * 10 + *x as i64);
}

fn part2(contents: String) -> i64 {
    let mut recipes = vec![3, 7];
    let mut elf1 = 0;
    let mut elf2 = 1;

    let goal_vec: Vec<u8> = contents
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u8)
        .collect();

    loop {
        recipes = gen_recipes(recipes, &mut elf1, &mut elf2);

        if recipes.len() >= goal_vec.len()

            && recipes[recipes.len() - goal_vec.len()..] == goal_vec[..]
        {
            return recipes.len() as i64 - goal_vec.len() as i64;
        } else if recipes.len() > goal_vec.len()
            && recipes[recipes.len() - goal_vec.len() - 1..recipes.len() - 1] == goal_vec[..]
        {
            return recipes.len() as i64 - goal_vec.len() as i64 - 1;
        }
    }
}

fn gen_recipes(existing: Vec<u8>, elf1: &mut usize, elf2: &mut usize) -> Vec<u8> {
    let mut recipes = existing;

    let sum = recipes[*elf1] + recipes[*elf2];
    if sum >= 10 {
        recipes.push(sum / 10);
        recipes.push(sum % 10);
    } else {
        recipes.push(sum);
    }

    *elf1 = (*elf1 + recipes[*elf1] as usize + 1) % recipes.len();
    *elf2 = (*elf2 + recipes[*elf2] as usize + 1) % recipes.len();

    recipes
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2018".to_string();
    let day = "14".to_string();

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
        "\nPart 1:\nTen recipes after input num: {}\nRan in {:.5?}",
        part1(contents.clone().lines().next().unwrap().to_string()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nRecipes before input sequence: {}\nRan in {:.5?}",
        part2(contents.clone().lines().next().unwrap().to_string()),
        part2_timer.elapsed()
    );
}