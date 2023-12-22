use std::{fs, i32, collections::{BinaryHeap, HashMap}};

fn find_start_loc(maze: &Vec<Vec<char>>) -> (usize, usize) {
    for i in 0..maze.len() {
        for j in 0..maze[0].len() {
            if maze[i][j] == 'S' {
                return (i, j)
            }
        }
    }

    return (0, 0);
}

fn traverse(garden: &Vec<Vec<char>>, start: (usize, usize), target_steps: i32) -> usize {
    let (sr, sc) = start;
    let mut pq: BinaryHeap<(i32, (i32, i32))> = BinaryHeap::from([(0, (sr as i32, sc as i32))]);
    let mut answer = 0;
    let mut seen_plots: HashMap<(i32, i32), bool> = HashMap::new();
    while let Some((steps, pos)) = pq.pop() {
        if !seen_plots.contains_key(&pos) {
            seen_plots.insert(pos, true);
            if (target_steps - (-steps)) % 2 == 0 {
                answer += 1;
            }
            let (row, col) = pos;
            for (d_row, d_col) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let next_row = row + d_row;
                let next_col = col + d_col;
                if next_row >= 0 && (next_row as usize) < garden.len() && next_col >= 0 && (next_col as usize) < garden[0].len() {
                    if (garden[next_row as usize][next_col as usize] == '.' 
                        || garden[next_row as usize][next_col as usize] == 'S') && -steps < target_steps
                    {
                        pq.push((steps - 1, (next_row, next_col)));
                    }
                }
            }
        }
    }

    answer
} 

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should read the file");
    let garden: Vec<Vec<char>> = contents
        .lines()
        .map(|line| { line.chars().collect() })
        .collect();
    let target_steps = 26501365;
    let start_loc = find_start_loc(&garden);

    // Apparently the number of steps given is a multiplier of 
    // grid size. Plus, the number of steps is exactly 202300
    // times the grid length. Additionally, the grid is square!
    // Moreover, we start exactly in the middle of the grid and 
    // middle row and column are fully free. This is done for a 
    // purpose so the bigger grid that is made from these grids
    // is actually a grid itself! So instead of traversing from S
    // to all points we can calculate the number of variants in 
    // those grids and add together. The final shape is a diamond 
    // like shape which is exactly what we get in the first part as well
    // Cute :) 
    let size = garden.len();
    let width = target_steps / size - 1;
    let mut answer = 0;

    // Calculate the whole garden repetition counts and amounts of plots we can reach in those
    let odd_gardens = (if width % 2 == 0 { width } else { width - 1 } + 1).pow(2) as usize;
    let even_gardens = (if width % 2 == 0 { width } else { width + 1 }).pow(2) as usize;
    let plots_on_ods = traverse(&garden, start_loc, size as i32 * 2 + 1);
    let plots_on_evens = traverse(&garden, start_loc, size as i32 * 2);

    answer += odd_gardens * plots_on_ods + even_gardens * plots_on_evens;

    // Handled the whole grids and can reach the edges of the edge grids
    // That's why need to know where we can go in exactly size - 1 steps
    let (start_row, start_col) = start_loc;
    let top_corner = traverse(&garden, (size - 1, start_col), (size - 1) as i32);
    let right_corner = traverse(&garden, (start_row, 0), (size - 1) as i32);
    let left_corner = traverse(&garden, (start_row, size - 1), (size - 1) as i32);
    let bottom_corner = traverse(&garden, (0, start_col), (size - 1) as i32);

    answer += top_corner + right_corner + left_corner + bottom_corner;

    // Handle small segments between the diagonal lines
    let top_right_small = traverse(&garden, (size - 1, 0), size as i32 / 2 - 1);
    let bottom_right_small = traverse(&garden, (0, 0), size as i32 / 2 - 1);
    let top_left_small = traverse(&garden, (size - 1, size - 1), size as i32 / 2 - 1);
    let bottom_left_small = traverse(&garden, (0, size - 1), size as i32 / 2 - 1);

    answer += (width + 1) * (top_right_small + bottom_left_small + bottom_right_small + top_left_small);

    // Big segments are the same starting point but we have size more steps
    let top_right_big = traverse(&garden, (size - 1, 0), (size * 3 / 2 - 1) as i32);
    let bottom_right_big = traverse(&garden, (0, 0), (size * 3 / 2 - 1) as i32);
    let top_left_big = traverse(&garden, (size - 1, size - 1), (size * 3 / 2 - 1) as i32);
    let bottom_left_big = traverse(&garden, (0, size - 1), (size * 3 / 2 - 1) as i32);

    answer += (width) * (top_right_big + top_left_big + bottom_right_big + bottom_left_big);

    println!("Part 2 Answer: {answer}");
}

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should read the file");
    let garden: Vec<Vec<char>> = contents
        .lines()
        .map(|line| { line.chars().collect() })
        .collect();
    let start_loc = find_start_loc(&garden);
    let answer = traverse(&garden, start_loc, 64);

    println!("Part 1 Answer: {answer}");
}

pub fn solve() {
    self::solve_part_1();
    self::solve_part_2();
}

