use std::{fs, i32, collections::HashMap};

fn count_energized(tiles: &Vec<Vec<char>>, initial_beam: (usize, usize, char)) -> i32 {
    let mut visited: HashMap<(usize, usize), bool> = HashMap::new();
    let mut processed_splitters: HashMap<(usize, usize), bool> = HashMap::new();
    let mut beams_stack: Vec<(usize, usize, char)> = vec![initial_beam];
    let mut answer = 0;

    while beams_stack.len() > 0 {
        let current_idx = beams_stack.len() - 1;
        let mut finished = false;
        let (row, col, mut dir) = beams_stack[current_idx];
        let tile = tiles[row][col];
        if tile == '\\' {
            dir = match dir {
                'r' => 'd',
                'l' => 'u',
                'u' => 'l',
                'd' => 'r',
                _ => '0',
            };
        } else if tile == '/' {
            dir = match dir {
                'r' => 'u',
                'l' => 'd',
                'u' => 'r',
                'd' => 'l',
                _ => '0',
            }
        } else if tile == '|' && (dir == 'r' || dir == 'l') {
            if !processed_splitters.contains_key(&(row, col)) {
                processed_splitters.insert((row, col), true);
                dir = 'u';
                beams_stack.insert(0, (row, col, 'd'));
            } else {
                beams_stack.pop();
                finished = true;
            }
        } else if tile == '-' && (dir == 'u' || dir == 'd') {
            if !processed_splitters.contains_key(&(row, col)) {
                processed_splitters.insert((row, col), true);
                dir = 'r';
                beams_stack.insert(0, (row, col, 'l'));
            } else {
                beams_stack.pop();
                finished = true;
            }
        }

        if !visited.contains_key(&(row, col)) {
            answer += 1;
            visited.insert((row, col), true);
        }

        if beams_stack.len() > 0 && !finished {
            let last_idx = beams_stack.len() - 1;
            match dir {
                'u' => {
                    if row > 0 {
                        beams_stack[last_idx] = (row - 1, col, dir);
                    } else {
                        beams_stack.pop();
                    }
                },
                'd' => {
                    if row < tiles.len() - 1 {
                        beams_stack[last_idx] = (row + 1, col, dir);
                    } else {
                        beams_stack.pop();
                    }
                },
                'l' => {
                    if col > 0 {
                        beams_stack[last_idx] = (row, col - 1, dir);
                    } else {
                        beams_stack.pop();
                    }
                },
                'r' => {
                    if col < tiles[0].len() - 1 {
                        beams_stack[last_idx] = (row, col + 1, dir);
                    } else {
                        beams_stack.pop();
                    }
                },
                _ => {},
            }
        }
    }

    answer
}

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should read the file");

    let tiles: Vec<Vec<char>> = contents
        .lines()
        .map(|line| {
            line.chars().collect()
        })
        .collect();
    
    let answer = count_energized(&tiles, (0, 0, 'r'));

    println!("Part 1 Answer: {answer}");
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should read the file");

    let tiles: Vec<Vec<char>> = contents
        .lines()
        .map(|line| {
            line.chars().collect()
        })
        .collect();

    let mut answer: i32 = 0;

    for row in 0..tiles.len() {
        let last_col = tiles[0].len() - 1;
        let right_edge_count = count_energized(&tiles, (row, 0, 'r'));
        let left_edge_count = count_energized(&tiles, (row, last_col, 'l'));
        let max_of_two = if right_edge_count > left_edge_count { right_edge_count } else { left_edge_count };
        if max_of_two > answer {
            answer = max_of_two;
        }
    }

    for col in 0..tiles[0].len() {
        let last_row = tiles.len() - 1;
        let top_edge_count = count_energized(&tiles, (0, col, 'd'));
        let bottom_edge_count = count_energized(&tiles, (last_row, col, 'u'));
        let max_of_two = if top_edge_count > bottom_edge_count { top_edge_count } else { bottom_edge_count };
        if max_of_two > answer {
            answer = max_of_two;
        }
    }

    println!("Part 2 Answer: {answer}");
}


pub fn solve() {
    // self::solve_part_1();
    self::solve_part_2();
}

