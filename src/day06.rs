use std::{fs, u64};

fn to_num(num_str: &str) -> u64 {
    let num: u64 = num_str.trim().parse().unwrap();

    num
}

fn calculate_win_cases(time: u64, distance: u64) -> u64 {
    let mut win_cases = 0;
    for button_hold in 0..(time/2 + 1) {
        if button_hold * (time - button_hold) > distance {
            win_cases += 1
        }
    }
    
    if time % 2 == 0 {
        win_cases * 2 - 1
    } else {
        win_cases * 2
    }

}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should read the file");

    let time_idx = contents.find("Time:").unwrap() + "Time:".len();
    let distance_start_idx = contents.find("Distance:").unwrap();
    let distance_end_idx = distance_start_idx + "Distance:".len();

    let time_str: String = (&contents[time_idx..distance_start_idx])
        .trim()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    let distance_str: String = (&contents[distance_end_idx..])
        .trim()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    println!("{distance_str}");
    let time = to_num(&time_str);
    let distance = to_num(&distance_str);
    let answer = calculate_win_cases(time, distance);
    println!("Part 2 Answer: {answer}");
}

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should read the file");

    let time_idx = contents.find("Time:").unwrap() + "Time:".len();
    let distance_start_idx = contents.find("Distance:").unwrap();
    let distance_end_idx = distance_start_idx + "Distance:".len();
    let times: Vec<u64> = (&contents[time_idx..distance_start_idx])
        .trim()
        .split_whitespace()
        .map(|num| to_num(num))
        .collect();
    let distances: Vec<u64> = (&contents[distance_end_idx..])
        .trim()
        .split_whitespace()
        .map(|num| to_num(num))
        .collect();

    let answer: u64 = times
        .iter()
        .enumerate()
        .map(|(i, time)| {
            let distance = distances[i];
            calculate_win_cases(*time, distance)
        })
        .fold(1, |acc, num| acc * num);

    println!("Part 1 Answer: {answer}");
}

pub fn solve() {
    self::solve_part_1();
    self::solve_part_2();
}

