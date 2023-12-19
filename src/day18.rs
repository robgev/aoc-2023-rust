use std::{fs, i64};

fn get_direction(dir: &str) -> (i64, i64) {
    match dir {
        "R" | "0" => (0, 1),
        "L" | "2" => (0, -1),
        "U" | "3" => (-1, 0),
        "D" | "1" => (1, 0),
        _ => (0, 0),
    }
}

fn to_num(num_str: &str) -> usize {
    let num: usize = num_str.trim().parse().unwrap();

    num
}

fn get_area(vertices: &Vec<(i64, i64)>) -> i64 {
    let area: i64 = vertices.windows(2).map(|v| {
        let (y1, x1) = v[0];
        let (y2, x2) = v[1];
        x1 * y2 - y1 * x2
    }).sum();

    let perimeter: i64 = vertices.windows(2).map(|v| {
        let (y1, x1) = v[0];
        let (y2, x2) = v[1];

        (x2 - x1).abs() + (y2 - y1).abs()
    }).sum();

    (area + perimeter) / 2 + 1
}

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should read the file");
    let mut vertices: Vec<(i64, i64)> = vec![(0, 0)];

    let _ = contents
        .lines()
        .for_each(|line| {
            let (last_y, last_x) = vertices.last().unwrap();
            let instruction: Vec<&str> = line.split_whitespace().collect();
            let (dir_y, dir_x) = get_direction(instruction[0]);
            let distance = to_num(instruction[1]) as i64;
            vertices.push((last_y + dir_y * distance, last_x + dir_x * distance));
        });

    vertices.push((0, 0));

    let answer = get_area(&vertices);
    
    println!("Part 1 Answer: {answer}");
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should read the file");
    let mut vertices: Vec<(i64, i64)> = vec![(0, 0)];

    let _ = contents
        .lines()
        .for_each(|line| {
            let (last_y, last_x) = vertices.last().unwrap();
            let instruction = line.split_whitespace().last().unwrap();
            let distance = i64::from_str_radix(&instruction[2..7], 16).unwrap();
            let (dir_y, dir_x) = get_direction(&instruction[7..8]);
            vertices.push((last_y + dir_y as i64 * distance, last_x + dir_x as i64 * distance));
        });

    vertices.push((0, 0));

    let answer = get_area(&vertices);
    
    println!("Part 2 Answer: {answer}");
}
pub fn solve() {
    self::solve_part_1();
    self::solve_part_2();
}

