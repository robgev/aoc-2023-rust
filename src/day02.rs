use std::{fs, i32};

fn get_game_id(game: &str) -> i32 {
    let game_id_start = "Game ".len();
    let game_id_end = game.find(':').unwrap();
    let game_id: i32 = (&game[game_id_start..game_id_end]).parse().unwrap();

    game_id
}

fn get_max_counts(game: &str) -> [i32; 3] {
    let mut max_counts = [0, 0, 0];
    game
    .split(";")
    .for_each(|game| 
         game
         .split(',')
         .for_each(|counts| {
            let balls = counts.trim();
            let separator_idx = balls.find(' ').unwrap();
            let number: i32 = (&balls[..separator_idx]).parse().unwrap();
            let color = &balls[separator_idx + 1..];
            let index = match color {
                "red" => 0,
                "green" => 1,
                "blue" => 2,
                _ => 3,
            };
            if max_counts[index] < number {
                max_counts[index] = number;
            }
         })
    );

    max_counts
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should read the file");

    let answer: i32 = contents
        .lines()
        .map(|line| {
            let game_id_end = line.find(':').unwrap();
            let max_counts = get_max_counts(&line[game_id_end+1..]);
            max_counts[0] * max_counts[1] * max_counts[2]
        })
        .sum();

    println!("Part 2 Answer: {answer}");
}

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should read the file");

    let answer: i32 = contents
        .lines()
        .map(|line| {
            let game_id = get_game_id(line);
            let game_id_end = line.find(':').unwrap();
            let max_counts = get_max_counts(&line[game_id_end+1..]);

            if max_counts[0] < 13 && max_counts[1] < 14 && max_counts[2] < 15 {
                game_id
            }
            else { 0 }
        })
        .sum();

    println!("Part 1 Answer: {answer}");
}

pub fn solve() {
    self::solve_part_1();
    self::solve_part_2();
}

