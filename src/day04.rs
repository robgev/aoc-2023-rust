use std::{fs, i32, collections::HashMap};

fn find_card_number(line: &str) -> i32 {
    let card = "Card ".len();
    let colon_idx = line.find(':').unwrap();
    let card_num: i32 = (&line[card..colon_idx]).trim().parse().unwrap();

    card_num
}

fn find_winning_nums_count(line: &str) -> u32 {
    let colon_idx = line.find(':').unwrap();
    let bar_idx = line.find('|').unwrap();
    let mut winning_nums_map: HashMap<&str, bool> = HashMap::new();
    let mut my_winning_nums = 0;
    // TODO: looks like should fold?
    let _ = (&line[(colon_idx + 1)..(bar_idx - 1)])
        .trim()
        .split_whitespace()
        .for_each(|num| {
            winning_nums_map.insert(num, true);
        });

    let _ = (&line[(bar_idx + 1)..])
        .trim()
        .split_whitespace()
        .for_each(|num| {
            if winning_nums_map.contains_key(num) {
                my_winning_nums += 1;
            }
        });

    my_winning_nums
}

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should read the file");

    let answer: i32 = contents
        .lines()
        .map(|line| {
            let base: i32 = 2;
            let my_winning_nums = find_winning_nums_count(line);
            if my_winning_nums > 0 {
                base.pow(my_winning_nums - 1)
            }
            else { 0 }
        })
        .sum();

    println!("Part 1 Answer: {answer}");
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should read the file");

    let mut ticket_counts: Vec<i32> = contents.lines().map(|_| 1).collect();

    let _ = contents
        .lines()
        .for_each(|line| {
            let card_num = find_card_number(line);
            let my_winning_nums = find_winning_nums_count(line);

            for i in card_num..(card_num + (my_winning_nums as i32)) {
                if i > 0 {
                    ticket_counts[i as usize] += ticket_counts[(card_num - 1) as usize];
                }
            }
        });

    let sum: i32 = ticket_counts.iter().sum();
    println!("Part 2 Answer: {sum}");
}

pub fn solve() {
    self::solve_part_1();
    self::solve_part_2();
}
