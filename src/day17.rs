use std::{fs, i32, collections::{BinaryHeap, HashMap}};

fn find_min_loss(grid: &Vec<Vec<i32>>) -> i32 {
    let mut pq: BinaryHeap<(i32, (i32, i32, i32, i32, i32))> = BinaryHeap::from([(0, (0, 0, 0, 0, 0))]);
    let mut seen_points: HashMap<(i32, i32, i32, i32, i32), bool> = HashMap::new();

    while let Some((hl, key)) = pq.pop() {
        if (key.0 as usize) == grid.len() - 1 && (key.1 as usize) == grid[0].len() - 1 {
            return -hl;
        }
        if !seen_points.contains_key(&key) {
            seen_points.insert(key, true);
            let (row, col, d_row, d_col, straight_steps) = key;
            if straight_steps < 3 && (d_row, d_col) != (0, 0) {
                let next_row = row + d_row;
                let next_col = col + d_col;
                if next_row >= 0 && (next_row as usize) < grid.len() && next_col >= 0 && (next_col as usize) < grid[0].len() {
                    pq.push((hl - grid[next_row as usize][next_col as usize], (next_row, next_col, d_row, d_col, straight_steps + 1)));
                }
            }

            for (new_d_row, new_d_col) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                if (new_d_row, new_d_col) != (d_row, d_col) && (new_d_row, new_d_col) != (-d_row, -d_col) {
                    let next_row = row + new_d_row;
                    let next_col = col + new_d_col;
                    if next_row >= 0 && (next_row as usize) < grid.len() && next_col >= 0 && (next_col as usize) < grid[0].len() {
                        pq.push((hl - grid[next_row as usize][next_col as usize], (next_row, next_col, new_d_row, new_d_col, 1)));
                    }
                }
            }
        }
    }

    0
}

fn find_min_loss_ultra(grid: &Vec<Vec<i32>>) -> i32 {
    let mut pq: BinaryHeap<(i32, (i32, i32, i32, i32, i32))> = BinaryHeap::from([(0, (0, 0, 0, 0, 0))]);
    let mut seen_points: HashMap<(i32, i32, i32, i32, i32), bool> = HashMap::new();

    while let Some((hl, key)) = pq.pop() {
        let (row, col, d_row, d_col, straight_steps) = key;
        if (row as usize) == grid.len() - 1 && (col as usize) == grid[0].len() - 1 && straight_steps >= 4 {
            return -hl;
        }
        if !seen_points.contains_key(&key) {
            seen_points.insert(key, true);
            if straight_steps < 10 && (d_row, d_col) != (0, 0) {
                let next_row = row + d_row;
                let next_col = col + d_col;
                if next_row >= 0 && (next_row as usize) < grid.len() && next_col >= 0 && (next_col as usize) < grid[0].len() {
                    pq.push((hl - grid[next_row as usize][next_col as usize], (next_row, next_col, d_row, d_col, straight_steps + 1)));
                }
            }

            if straight_steps >= 4 || (d_row, d_col) == (0, 0) {
                for (new_d_row, new_d_col) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    if (new_d_row, new_d_col) != (d_row, d_col) && (new_d_row, new_d_col) != (-d_row, -d_col) {
                        let next_row = row + new_d_row;
                        let next_col = col + new_d_col;
                        if next_row >= 0 && (next_row as usize) < grid.len() && next_col >= 0 && (next_col as usize) < grid[0].len() {
                            pq.push((hl - grid[next_row as usize][next_col as usize], (next_row, next_col, new_d_row, new_d_col, 1)));
                        }
                    }
                }
            }
        }
    }

    0
}

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should read the file");

    let grid: Vec<Vec<i32>> = 
        contents
        .lines()
        .map(|line| 
             line
             .chars()
             .map(|c| c.to_digit(10).unwrap_or(0) as i32)
             .collect()
        )
        .collect();

    let answer = find_min_loss(&grid);
    println!("Part 1 Answer: {answer}");
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should read the file");

    let grid: Vec<Vec<i32>> = 
        contents
        .lines()
        .map(|line| 
             line
             .chars()
             .map(|c| c.to_digit(10).unwrap_or(0) as i32)
             .collect()
        )
        .collect();

    let answer = find_min_loss_ultra(&grid);
    println!("Part 2 Answer: {answer}");
}

pub fn solve() {
    // self::solve_part_1();
    self::solve_part_2();
}

