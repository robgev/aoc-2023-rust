use std::{fs, i32};

fn extrapolate_val(dataset: Vec<i32>) -> i32 {
    if dataset.iter().all(|v| *v == 0) {
        0
    } else {
        let last = dataset.last().unwrap();
        let differences = dataset.windows(2).map(|vals| vals[1] - vals[0]).collect();
        last + extrapolate_val(differences)
    }

}

fn extrapolate_backwards(dataset: Vec<i32>) -> i32 {
    if dataset.iter().all(|v| *v == 0) {
        0
    } else {
        let first = dataset.first().unwrap();
        let differences = dataset.windows(2).map(|vals| vals[1] - vals[0]).collect();
        first - extrapolate_backwards(differences)
    }

}

fn to_num(num_str: &str) -> i32 {
    let num: i32 = num_str.trim().parse().unwrap();

    num
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should read the file");

    let answer: i32 = contents
        .lines()
        .map(|line| {
            let nums = line
                .split_whitespace()
                .map(|num_str| to_num(num_str))
                .collect();
            extrapolate_backwards(nums)
        })
        .sum();

    println!("Part 2 Answer: {answer}");
}

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should read the file");

    let answer: i32 = contents
        .lines()
        .map(|line| {
            let nums = line
                .split_whitespace()
                .map(|num_str| to_num(num_str))
                .collect();
            extrapolate_val(nums)
        })
        .sum();

    println!("Part 1 Answer: {answer}");
}

pub fn solve() {
    self::solve_part_1();
    self::solve_part_2();
}

