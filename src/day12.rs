use std::{fs, i32, collections::HashMap};

fn to_num(num_str: &str) -> usize {
    let num: usize = num_str.trim().parse().unwrap();

    num
}

struct ConfigSolver {
    cache: HashMap<(String, Vec<usize>), usize>,
}

impl ConfigSolver {
    pub fn count_cases(&mut self, remaining_line: &str, nums: &Vec<usize>) -> usize {
        if self.cache.contains_key(&(remaining_line.to_string(), nums.to_vec())) {
            return *self.cache.get(&(remaining_line.to_string(), nums.to_vec())).unwrap();
        }

        if remaining_line.len() == 0 {
            return if nums.len() == 0 { 1 } else { 0 }
        }

        if nums.len() == 0 {
            return match remaining_line.find('#') {
                None => 1,
                _ => 0,
            }
        }

        let first_symbol = &remaining_line[0..1];

        if first_symbol == "." {
            return self.count_cases(&remaining_line[1..], nums);
        }

        let mut result = 0;

        if first_symbol == "?" {
            // Treat the ? as dot, count the cases for that
            let if_dot = self.count_cases(&remaining_line[1..], nums);
            result += if_dot
        }

        // At this point we should treat first question mark as #
        if first_symbol == "#" || first_symbol == "?" {
            let current_num = nums[0];
            if  current_num <= remaining_line.len() {
                // We have enough symbols left to check
                let contiguous_bangs = &remaining_line[0..current_num];
                if contiguous_bangs.find('.') == None {
                    // Chunk away the length of continuous bangs.
                    // Consider the edge cases of the end of the string 
                    // And the fact that after the bangs we also need to have one dot
                    if current_num == remaining_line.len() {
                        result += self.count_cases("", &nums[1..].to_vec());
                    } else if &remaining_line[current_num..current_num + 1] != "#" {
                        // Cut the dot or ? at + 1 coz it always should be a dot
                        result += self.count_cases(&remaining_line[current_num + 1..], &nums[1..].to_vec());
                    }
                }
            }
        }

        self.cache.insert((remaining_line.to_string(), nums.to_vec()), result);
        result

    }
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should read the file");

    let answer: usize = contents
        .lines()
        .map(|line| {
            let config_and_nums: Vec<&str> = line.split_whitespace().collect();
            let springs = config_and_nums[0];
            let nums_str = config_and_nums[1];
            let remaining_line = format!("{springs}?{springs}?{springs}?{springs}?{springs}");
            let all_nums = format!("{nums_str},{nums_str},{nums_str},{nums_str},{nums_str}");
            let nums: Vec<usize> = all_nums.split(',').map(|num_str| to_num(num_str)).collect();
            let mut config_solver = ConfigSolver { cache: HashMap::new() };
            config_solver.count_cases(&remaining_line, &nums)
        })
        .sum();

    println!("Part 2 Answer: {answer}");
}

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should read the file");

    let answer: usize = contents
        .lines()
        .map(|line| {
            let config_and_nums: Vec<&str> = line.split_whitespace().collect();
            let remaining_line = config_and_nums[0];
            let nums: Vec<usize> = config_and_nums[1].split(',').map(|num_str| to_num(num_str)).collect();
            let mut config_solver = ConfigSolver { cache: HashMap::new() };
            config_solver.count_cases(&remaining_line, &nums)
        })
        .sum();

    println!("Part 1 Answer: {answer}");
}

pub fn solve() {
    // self::solve_part_1();
    self::solve_part_2();
}

