use std::{fs, i32};

fn smudges(pattern: Vec<&str>, reflection: Vec<&str>) -> i32 {
    let mut smudges = 0;
    for r in 0..pattern.len() {
        let row = pattern[r];
        let reflection_row_idx = pattern.len() - 1 - r;
        let reflection_row = reflection[reflection_row_idx];
        if row != reflection_row {
            let row_chars: Vec<char> = row.chars().collect();
            let reflection_chars: Vec<char> = reflection_row.chars().collect(); 
            for c in 0..row_chars.len() {
                if row_chars[c] != reflection_chars[c] {
                    smudges += 1;
                }
            }
        }
    }

    smudges
}

fn smudges_vert(pattern: Vec<&str>, reflection: Vec<&str>) -> i32 {
    let mut smudges = 0;
    for r in 0..pattern.len() {
        let row = pattern[r];
        let reflection_row = reflection[r];
        if row != reflection_row {
            let row_chars: Vec<char> = row.chars().collect();
            let reflection_chars: Vec<char> = reflection_row.chars().collect(); 
            for c in 0..row_chars.len() {
                if row_chars[c] != reflection_chars[c] {
                    smudges += 1;
                }
            }
        }
    }

    smudges
}

fn find_smudge_reflection(pattern: &str) -> usize {
    let lines: Vec<&str> = pattern.lines().collect();
    for i in 0..(lines.len() - 1) {
        if smudges(vec![lines[i]], vec![lines[i + 1]]) <= 1 { 
            let reflection_length = usize::min(lines.len() - (i + 1), i + 1);
            let side1 = lines[(i + 1 - reflection_length)..(i + 1)].to_vec();
            let side2 = lines[(i + 1)..(i + reflection_length + 1)].to_vec();
            if smudges(side1, side2) == 1 {
                return (i + 1) * 100
            }
        }
    }

    let chars: Vec<Vec<char>> = lines
        .iter()
        .map(|line| 
             line
             .chars()
             .collect()
        )
        .collect();

    for col in 0..(chars[0].len() - 1) {
        let current_col_str = String::from_iter(chars.iter().map(|row| row[col]));
        let current_col: Vec<&str> = current_col_str.lines().collect();
        let next_col_str = String::from_iter(chars.iter().map(|row| row[col + 1]));
        let next_col: Vec<&str> = next_col_str.lines().collect();
        if smudges_vert(current_col, next_col) <= 1 {
            let reflection_length = usize::min(chars[0].len() - (col + 1), col + 1);
            let side1: String = chars.iter().map(|row| {
                format!("{:?}\n", &row[(col + 1 - reflection_length)..col + 1])
            }).collect();
            let side2: String = chars.iter().map(|row| {
                let mut reversed = row.clone()[(col + 1)..(col + reflection_length + 1)].to_vec();
                reversed.reverse();
                format!("{:?}\n", reversed)
            }).collect();
            let side1pat: Vec<&str> = side1.lines().collect();
            let side2pat: Vec<&str> = side2.lines().collect();
            if smudges_vert(side1pat, side2pat) == 1 {
                return col + 1;
            }
        }
    }

    0
}

fn find_reflection(pattern: &str) -> usize {
    let lines: Vec<&str> = pattern.lines().collect();
    for i in 0..(lines.len() - 1) {
        if lines[i] == lines[i+1] { 
            let reflection_length = usize::min(lines.len() - (i + 1), i + 1);
            let side1 = &lines[(i + 1 - reflection_length)..(i + 1)].join("\n");
            let mut side2_rev: Vec<&str> = lines.clone()[(i + 1)..(i + reflection_length + 1)].to_vec();
            side2_rev.reverse();
            let side2 = side2_rev.join("\n");
            if side1 == &side2 {
                return (i + 1) * 100
            }
        }
    }

    let chars: Vec<Vec<char>> = lines
        .iter()
        .map(|line| 
             line
             .chars()
             .collect()
        )
        .collect();

    for col in 0..(chars[0].len() - 1) {
        let current_col = String::from_iter(chars.iter().map(|row| row[col]));
        let next_col = String::from_iter(chars.iter().map(|row| row[col + 1]));
        if current_col == next_col {
            let reflection_length = usize::min(chars[0].len() - (col + 1), col + 1);
            let side1: String = chars.iter().map(|row| {
                format!("{:?}\n", &row[(col + 1 - reflection_length)..col + 1])
            }).collect();
            let side2: String = chars.iter().map(|row| {
                let mut reversed = row.clone()[(col + 1)..(col + reflection_length + 1)].to_vec();
                reversed.reverse();
                format!("{:?}\n", reversed)
            }).collect();
            if side1 == side2 {
                return col + 1;
            }
        }
    }

    0
}

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should read the file");

    let answer: usize = contents
        .split("\n\n")
        .map(|pattern| {
            find_reflection(pattern)
        })
        .sum();

    println!("Part 1 Answer: {answer}");
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should read the file");

    let answer: usize = contents
        .split("\n\n")
        .map(|pattern| {
            find_smudge_reflection(pattern)
        })
        .sum();

    println!("Part 2 Answer: {answer}");
}

pub fn solve() {
    self::solve_part_1();
    self::solve_part_2();
}

