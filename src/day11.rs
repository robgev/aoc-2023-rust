use std::fs;

fn find_distances(lines: Vec<&str>, expansion_rate: usize) -> usize {
    // Compute the prefix sum of empty rows and cols 
    // This way it's easier to know how many empty rows are between 
    // each rows and columns
    let mut empty_rows_prefix: Vec<usize> = vec![0; lines.len()];
    // Keep track of empty columns: Assume all columns are empty
    // When finding a star, mark the star's column as non empty
    // Used 0/1 instead of boolean for easier prefix computation
    let mut empty_cols: Vec<usize> = vec![1; lines[0].len()];

    // Save the start locations for final computation
    let mut star_locations: Vec<(usize, usize)> = Vec::new();

    let _ = lines
        .iter()
        .enumerate()
        .for_each(|(row, line)| {
            // Assume the row is empty
            let mut is_empty = 1;
            line
                .chars()
                .enumerate()
                .for_each(|(col, cchar)| {
                    if cchar == '#' {
                        // Found a star, so the col and
                        // row are not empty
                        empty_cols[col] = 0;
                        is_empty = 0;
                        star_locations.push((row, col));
                    }
                });

            // Compute the prefix sum value of the current row
            if row > 0 {
                empty_rows_prefix[row] = empty_rows_prefix[row - 1] + is_empty;
            } else {
                empty_rows_prefix[row] = is_empty;
            }
        });

    // Now when we have empty cols data, compute the prefix some of cols
    let mut empty_cols_prefix = vec![0; empty_cols.len()];
    empty_rows_prefix[0] = if empty_cols[0] == 1 { 1 } else { 0 };
    for i in 1..empty_cols.len() {
        empty_cols_prefix[i] = empty_cols_prefix[i - 1] + empty_cols[i];
    }

    let mut answer = 0;
    // Compute distances between each pair of stars
    for i in 0..star_locations.len() {
        for j in i..star_locations.len() {
            let (row1, col1) = star_locations[i];
            let (row2, col2) = star_locations[j];
            answer += 
                // Row diff
                row2.abs_diff(row1) + 
                // Col diff
                col2.abs_diff(col1) + 
                // Number of empty rows in between these 2 stars
                // 1 row gets multiplied expansion_rate times and we already counted the original
                // so we need to add expansion_rate - 1 (aka the original row/col) empty row/cols
                empty_rows_prefix[row1].abs_diff(empty_rows_prefix[row2]) * (expansion_rate - 1) +
                // Number of empty cols in between these 2 stars
                empty_cols_prefix[col1].abs_diff(empty_cols_prefix[col2]) * (expansion_rate - 1);
        }
    }

    answer
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should read the file");
    let lines: Vec<&str> = contents
        .lines()
        .collect();

    let answer = find_distances(lines, 1000000);

    println!("Part 2 Answer: {answer}");
}

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should read the file");
    let lines: Vec<&str> = contents
        .lines()
        .collect();

    let answer = find_distances(lines, 2);

    println!("Part 1 Answer: {answer}");
}

pub fn solve() {
    self::solve_part_1();
    self::solve_part_2();
}

