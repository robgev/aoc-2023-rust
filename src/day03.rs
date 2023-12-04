use std::{fs, i32, collections::HashMap};

fn is_symbol(chr: char) -> bool {
    chr != '.'
}

fn get_char_at(schema: &Vec<Vec<char>>, row: i32, col: i32) -> char {
    let schema_len: i32 = schema.len().try_into().unwrap();
    let row_len: i32 = schema[0].len().try_into().unwrap();
    if row < 0 || col < 0 || row >= schema_len || col >= row_len {
        '.'
    } else {
        schema[row as usize][col as usize]
    }
}

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should read the file");

    let lines: Vec<String> = contents
        .lines()
        .map(String::from)
        .collect();
    let schema: Vec<Vec<char>> = contents
        .lines()
        .map(|line| {
            line.chars().collect()
        })
        .collect();


    let mut sum = 0;
    let mut row: i32 = 0;
    let mut col: i32 = 0;
    // TODO: Need to learn to think the rust way
    let schema_len: i32 = schema.len().try_into().unwrap();
    let row_len: i32 = schema[0].len().try_into().unwrap();

    while row < schema_len {
        while col < row_len {
            // found a numeric character in row
            if schema[row as usize][col as usize].is_numeric() {
                let start = col;
                let mut is_part = false;
                while get_char_at(&schema, row, col).is_numeric() {
                    let top_char = get_char_at(&schema, row - 1, col);
                    let bot_char = get_char_at(&schema, row + 1, col); 
                    if is_symbol(top_char) || is_symbol(bot_char) {
                        is_part = true;
                    }

                    col += 1;
                }

                is_part = is_part 
                    || is_symbol(get_char_at(&schema, row, start - 1))
                    || is_symbol(get_char_at(&schema, row, col))
                    || is_symbol(get_char_at(&schema, row - 1, start - 1))
                    || is_symbol(get_char_at(&schema, row - 1, col))
                    || is_symbol(get_char_at(&schema, row + 1, start - 1))
                    || is_symbol(get_char_at(&schema, row + 1, col));

                let line = &lines[row as usize];
                if is_part {
                    let number: i32 = (&line[(start as usize)..(col as usize)]).parse().unwrap();
                    sum += number;
                }
            } else {
                col += 1;
            }
        }

        col = 0;
        row += 1;
    }

    println!("{sum}");
}

fn is_gear(chr: char) -> bool {
    chr == '*'
}

fn find_gear(schema: &Vec<Vec<char>>, row: i32, start: i32, end: i32) -> (i32, i32) {
    for rows in (row - 1)..(row + 2) {
        for cols in (start - 1)..(end + 1) {
            if is_gear(get_char_at(schema, rows, cols)) {
                return (rows, cols)
            }
        }
    }

    (-1, -1)
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should read the file");

    let lines: Vec<String> = contents
        .lines()
        .map(String::from)
        .collect();
    let schema: Vec<Vec<char>> = contents
        .lines()
        .map(|line| {
            line.chars().collect()
        })
        .collect();


    let mut sum = 0;
    let mut row: i32 = 0;
    let mut col: i32 = 0;
    let mut gears: HashMap<(i32, i32), i32> = HashMap::new();
    // TODO: Need to learn to think the rust way
    let schema_len: i32 = schema.len().try_into().unwrap();
    let row_len: i32 = schema[0].len().try_into().unwrap();

    while row < schema_len {
        while col < row_len {
            // found a numeric character in row
            if schema[row as usize][col as usize].is_numeric() {
                let start = col;
                while get_char_at(&schema, row, col).is_numeric() {
                    col += 1;
                }

                let gear = find_gear(&schema, row, start, col);

                let line = &lines[row as usize];
                if gear.0 > 0 && gear.1 > 0 {
                    let number: i32 = (&line[(start as usize)..(col as usize)]).parse().unwrap();
                    if gears.contains_key(&gear) {
                        sum += gears.get(&gear).unwrap() * number;
                    } else {
                        gears.insert(gear, number);
                    }
                }
            } else {
                col += 1;
            }
        }

        col = 0;
        row += 1;
    }

    println!("{sum}");
}

pub fn solve() {
    self::solve_part_1();
    self::solve_part_2();
}
