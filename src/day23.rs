use std::{fs, i32, collections::BinaryHeap, i64};

fn dfs(seen_crossroads: &mut Vec<usize>, adj_matrix: &Vec<Vec<usize>>, current_idx: usize, end_idx: usize) -> usize {
    if current_idx == end_idx {
        0
    } else {
        let mut max_steps = i64::MIN;
        if !seen_crossroads.contains(&current_idx) {
            seen_crossroads.push(current_idx);
        }

        adj_matrix[current_idx].iter().enumerate().for_each(|(idx, n_steps)| {
            if *n_steps > 0 && !seen_crossroads.contains(&idx) {
                let steps = dfs(seen_crossroads, adj_matrix, idx, end_idx) + n_steps;
                if steps as i64 > max_steps {
                    max_steps = steps as i64
                }
            }
        });

        for i in 0..seen_crossroads.len() {
            if seen_crossroads[i] == current_idx {
                seen_crossroads.remove(i);
            }
        }
        
        max_steps as usize
    }
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should read the file");
    let map: Vec<Vec<char>> = contents
        .lines()
        .map(|line| { line.chars().collect() })
        .collect();
    let mut start_col = 0;
    let mut end_col = 0;
    for j in 0..map[0].len() {
        if map[0][j] == '.' {
            start_col = j;
        }
        if map[map[0].len() - 1][j] == '.' {
            end_col = j;
        }
    }
    let mut crossroads: Vec<(usize, usize)> = vec![(0, start_col), (map[0].len() - 1, end_col)];
    let mut adj_matrix: Vec<Vec<usize>> = Vec::new();

    // Since it's a maze with one way pathing most of the time 
    // we do edge contraction
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] != '#' {
                let mut ways_to_go = 0;
                for (d_row, d_col) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    let next_row = (row as i32) + d_row;
                    let next_col = (col as i32) + d_col;
                    if next_row >= 0 && (next_row as usize) < map.len() && next_col >= 0 && (next_col as usize) < map[0].len() {
                        if map[next_row as usize][next_col as usize] != '#' {
                            ways_to_go += 1;
                        }
                    }
                }

                if ways_to_go >= 3 {
                    crossroads.push((row, col));
                }
            }
        }
    };

    for _ in &crossroads {
        adj_matrix.push(vec![0; crossroads.len()]);
    }

    for i in 0..crossroads.len() {
        let (sr, sc) = crossroads[i];
        let mut stack = vec![(sr, sc, 0)];
        let mut seen: Vec<(usize, usize)> = vec![(sr, sc)];
         
        while let Some((row, col, steps)) = stack.pop() {
           if crossroads.contains(&(row, col)) && steps != 0 {
               let neighbor_index = crossroads
                   .iter()
                   .position(|cr| *cr == (row, col))
                   .unwrap();
               adj_matrix[i][neighbor_index] = steps;
           } else {
               for (d_row, d_col) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    let next_row = (row as i32) + d_row;
                    let next_col = (col as i32) + d_col;
                    let nr = next_row as usize;
                    let nc = next_col as usize;
                    if next_row >= 0 
                        && (nr) < map.len() 
                        && next_col >= 0 
                        && (nc) < map[0].len() 
                        && map[nr][nc] != '#'
                        && !seen.contains(&(nr, nc))
                    {
                        if map[nr][nc] != '#' {
                            stack.push((nr, nc, steps + 1));
                            seen.push((nr, nc));
                        }
                    }
               }
           }
        }
    }

    let start_idx = 0;
    let end_idx = 1;
    let mut seen_crossroads: Vec<usize> = Vec::new();
    let answer = dfs(&mut seen_crossroads, &adj_matrix, start_idx, end_idx);

    println!("Part 2 Answer: {answer}");
}

fn get_dirs_from(loc: char) -> Vec<(i32, i32)> {
    match loc {
        '>' => vec![(0, 1)],
        '<' => vec![(0, -1)],
        '^' => vec![(-1, 0)],
        'v' => vec![(1, 0)],
        '.' => vec![(0, 1), (0, -1), (1, 0), (-1, 0)],
        _ => vec![(0, 1), (0, -1), (1, 0), (-1, 0)],
    }
}

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should read the file");
    let map: Vec<Vec<char>> = contents
        .lines()
        .map(|line| { line.chars().collect() })
        .collect();
    let mut start_col = 0;
    let mut end_col = 0;
    for j in 0..map[0].len() {
        if map[0][j] == '.' {
            start_col = j;
        }
        if map[map[0].len() - 1][j] == '.' {
            end_col = j;
        }
    }
    let mut crossroads: Vec<(usize, usize)> = vec![(0, start_col), (map[0].len() - 1, end_col)];
    let mut adj_matrix: Vec<Vec<usize>> = Vec::new();

    // Since it's a maze with one way pathing most of the time 
    // we do edge contraction
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] != '#' {
                let mut ways_to_go = 0;
                for (d_row, d_col) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    let next_row = (row as i32) + d_row;
                    let next_col = (col as i32) + d_col;
                    if next_row >= 0 && (next_row as usize) < map.len() && next_col >= 0 && (next_col as usize) < map[0].len() {
                        if map[next_row as usize][next_col as usize] != '#' {
                            ways_to_go += 1;
                        }
                    }
                }

                if ways_to_go >= 3 {
                    crossroads.push((row, col));
                }
            }
        }
    };

    for _ in &crossroads {
        adj_matrix.push(vec![0; crossroads.len()]);
    }

    for i in 0..crossroads.len() {
        let (sr, sc) = crossroads[i];
        let mut stack = vec![(sr, sc, 0)];
        let mut seen: Vec<(usize, usize)> = vec![(sr, sc)];
         
        while let Some((row, col, steps)) = stack.pop() {
           if crossroads.contains(&(row, col)) && steps != 0 {
               let neighbor_index = crossroads
                   .iter()
                   .position(|cr| *cr == (row, col))
                   .unwrap();
               adj_matrix[i][neighbor_index] = steps;
           } else {
               for (d_row, d_col) in get_dirs_from(map[row][col]) {
                    let next_row = (row as i32) + d_row;
                    let next_col = (col as i32) + d_col;
                    let nr = next_row as usize;
                    let nc = next_col as usize;
                    if next_row >= 0 
                        && (nr) < map.len() 
                        && next_col >= 0 
                        && (nc) < map[0].len() 
                        && map[nr][nc] != '#'
                        && !seen.contains(&(nr, nc))
                    {
                        if map[nr][nc] != '#' {
                            stack.push((nr, nc, steps + 1));
                            seen.push((nr, nc));
                        }
                    }
               }
           }
        }
    }

    let start_idx = 0;
    let end_idx = 1;
    let mut answer = 0;
    let mut pq = BinaryHeap::from([(start_idx, 0)]);
    while let Some((steps, crossroad_idx)) = pq.pop() {
        if crossroad_idx == end_idx {
            if steps > answer {
                answer = steps; 
            }
        }
        adj_matrix[crossroad_idx].iter().enumerate().for_each(|(idx, n_steps)| {
            if *n_steps > 0 {
                pq.push((steps + n_steps, idx))
            }
        })
    }

    println!("Part 1 Answer: {answer}");
}

pub fn solve() {
    self::solve_part_1();
    self::solve_part_2();
}

