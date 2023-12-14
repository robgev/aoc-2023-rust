use std::{fs, i32, collections::HashMap};

fn roll_horizontal(platform: &mut Vec<Vec<char>>, dir: char) {
    let mut free_locs: Vec<usize> = Vec::new();
    for row in 0..platform.len() {
        for c in 0..platform[0].len() {
            let col = if dir == 'w' { c } else { platform[0].len() - 1 - c };
            if platform[row][col] == '.' {
                free_locs.insert(0, col);
            } else if platform[row][col] == 'O' {
                if !free_locs.is_empty() {
                    let loc = free_locs.pop().unwrap();
                    platform[row][loc] = 'O';
                    platform[row][col] = '.';
                    free_locs.insert(0, col);
                }
            } else if platform[row][col] == '#' {
                free_locs = Vec::new();
            }
        }
        free_locs = Vec::new();
    }
}

fn roll_vertical(platform: &mut Vec<Vec<char>>, dir: char) {
    let mut free_locs: Vec<usize> = Vec::new();
    for col in 0..platform[0].len() {
        for r in 0..platform.len() {
            let row = if dir == 'n' { r } else { platform.len() - 1 - r };
            if platform[row][col] == '.' {
                free_locs.insert(0, row);
            } else if platform[row][col] == 'O' {
                if !free_locs.is_empty() {
                    let loc = free_locs.pop().unwrap();
                    platform[loc][col] = 'O';
                    platform[row][col] = '.';
                    free_locs.insert(0, row);
                }
            } else if platform[row][col] == '#' {
                free_locs = Vec::new();
            }
        }
        free_locs = Vec::new();
    }
}

fn cycle(platform: &mut Vec<Vec<char>>) {
    roll_vertical(platform, 'n');
    roll_horizontal(platform, 'w');
    roll_vertical(platform, 's');
    roll_horizontal(platform, 'e');
}

fn score(platform: &Vec<Vec<char>>) -> usize {
    let mut result = 0;
    for col in 0..platform[0].len() {
        for row in 0..platform.len() {
            if platform[row][col] == 'O' {
                result += platform.len() - row;
            }
        }
    }

    result
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should read the file");
    let mut platform: Vec<Vec<char>> = contents
        .lines()
        .map(|line| {
            line.chars().collect()
        })
        .collect();

    let mut i = 0;
    let mut seen_configs: HashMap<String, i32> = HashMap::new();
    let iters = 1000000000;
    while i < iters {
        cycle(&mut platform);
        i += 1;
        let platform_str = String::from_iter(platform.iter().map(|line| String::from_iter(line.iter()) + "\n"));
        if seen_configs.contains_key(&platform_str) {
            let first_seen_iteration = seen_configs.get(&platform_str).unwrap();
            let config_cycle_length = i - first_seen_iteration;
            let number_of_cycles = (iters - i) / config_cycle_length;
            i += number_of_cycles * config_cycle_length;
        } else {
            seen_configs.insert(platform_str, i);
        }
    }

    let answer: usize = score(&platform);

    println!("Part 2 Answer: {answer}");
}

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should read the file");
    let platform: Vec<Vec<char>> = contents
        .lines()
        .map(|line| {
            line.chars().collect()
        })
        .collect();
    let mut answer: usize = 0;
    let mut free_locs: Vec<usize> = Vec::new();

    for col in 0..platform[0].len() {
        for row in 0..platform.len() {
            if platform[row][col] == '.' {
                free_locs.insert(0, platform.len() - row);
            } else if platform[row][col] == 'O' {
                if !free_locs.is_empty() {
                    let loc = free_locs.pop().unwrap();
                    answer += loc;
                    free_locs.insert(0, platform.len() - row);
                } else {
                    answer += platform.len() - row;
                }
            } else if platform[row][col] == '#' {
                free_locs = Vec::new();
            }
        }
        free_locs = Vec::new();
    }


    println!("Part 1 Answer: {answer}");
}

pub fn solve() {
    // self::solve_part_1();
    self::solve_part_2();
}

