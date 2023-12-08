use std::{fs, i32, collections::HashMap};

pub fn lcm(nums: &Vec<usize>) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..].to_vec());
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should read the file");
    let mut directions_map: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut start_points: Vec<&str> = Vec::new();
    let lines: Vec<&str> = contents.lines().collect();
    let instructions: Vec<char> = lines[0].chars().collect();
    let _ = (&lines[2..])
        .iter()
        .for_each(|entry| {
            let entry_point = &entry[0..3];
            let to_left = &entry[7..10];
            let to_right = &entry[12..15];
            if &entry_point[2..] == "A" {
                start_points.push(entry_point)
            }
            directions_map.insert(entry_point, (to_left, to_right));
        });

    let mut steps = vec![0; start_points.len()];

    start_points.clone().iter().enumerate().for_each(|(i, current_location)| {
        let mut current_steps = 0;
        let mut start_location = *current_location;

        while &start_location[2..] != "Z" {
            let direction = instructions[(current_steps % instructions.len()) as usize];
            let directions_from_here = directions_map.get(start_location).unwrap();
            current_steps += 1;
            if direction == 'L' {
                start_location = directions_from_here.0
            } else {
                start_location = directions_from_here.1
            }
        }

        steps[i] = current_steps;
    });

    let answer = lcm(&steps);
    

    println!("Part 2 Answer: {answer}");
}

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should read the file");
    let mut directions_map: HashMap<&str, (&str, &str)> = HashMap::new();
    let lines: Vec<&str> = contents.lines().collect();
    let instructions: Vec<char> = lines[0].chars().collect();
    let _ = (&lines[2..])
        .iter()
        .for_each(|entry| {
            let entry_point = &entry[0..3];
            let to_left = &entry[7..10];
            let to_right = &entry[12..15];
            directions_map.insert(entry_point, (to_left, to_right));
        });

    let mut steps = 0;
    let mut current_location = "AAA";

    while current_location != "ZZZ" {
        let direction = instructions[(steps % instructions.len()) as usize];
        let directions_from_here = directions_map.get(current_location).unwrap();
        steps += 1;
        if direction == 'L' {
            current_location = directions_from_here.0
        } else {
            current_location = directions_from_here.1
        }
    }

    println!("Part 1 Answer: {steps}");
}

pub fn solve() {
    // self::solve_part_1();
    self::solve_part_2();
}

