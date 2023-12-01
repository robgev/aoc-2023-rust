use std::{fs, i32};
use regex::Regex;

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should read the file");
    let first_digit_re = Regex::new(r"^.*?(\d{1}|one|two|three|four|five|six|seven|eight|nine|zero).*$").unwrap();
    let second_digit_re = Regex::new(r"^.*(\d{1}|one|two|three|four|five|six|seven|eight|nine|zero).*$").unwrap();

    let answer: i32 = contents
        .trim()
        .split('\n')
        .map(|line| {
            let Some(first_digit_caps) = first_digit_re.captures(line) else { return 0 };
            let Some(second_digit_caps) = second_digit_re.captures(line) else { return 0 };

            let first_digit: i32 = match &first_digit_caps[1] {
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                "zero" => 0,
                num => num.parse::<i32>().unwrap(),
            };

            let second_digit: i32 = match &second_digit_caps[1] {
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                "zero" => 0,
                num => num.parse::<i32>().unwrap(),
            };

            println!("{line}, {first_digit}, {second_digit}");


            (10 * first_digit + second_digit) as i32
        })
        .sum();

    println!("Part 2 Answer: {answer} \n");
}

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should read the file");

    let answer: i32 = contents
        .trim()
        .split('\n')
        .map(|line| {
            let chars: Vec<char> = line.chars().collect();
            let mut start = 0;
            let mut end = line.len() - 1;

            while start <= end && (!chars[start].is_numeric() || !chars[end].is_numeric()) {
                if !chars[start].is_numeric() {
                    start += 1;
                }

                if !chars[end].is_numeric() {
                    end -= 1;
                } 
            }

            (10 * chars[start].to_digit(10).unwrap() + chars[end].to_digit(10).unwrap()) as i32
        })
        .sum();

    println!("Part 1 Answer: {answer}");
}

pub fn solve() {
    self::solve_part_1();
    self::solve_part_2();
}
