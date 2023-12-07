use std::{fs, i32, cmp::Ordering, collections::HashMap, usize};

fn to_num(num_str: &str) -> u32 {
    let num: u32 = num_str.trim().parse().unwrap();

    num
}

fn maxes_to_value(maxes: (u32, u32)) -> u32 {
    match maxes {
        (5, 0) => 7,
        (4, 1) => 6,
        (3, 2) => 5,
        (3, 1) => 4,
        (2, 2) => 3,
        (2, 1) => 2,
        (1, 1) => 1,
        _ => 0,
    }
}

fn card_to_val(card: char) -> u32 {
    match card {
        'A' => 13,
        'K' => 12,
        'Q' => 11,
        'J' => 10,
        'T' => 9,
        n => n.to_digit(10).unwrap() - 1,
    }
}

fn card_to_val_2(card: char) -> u32 {
    match card {
        'A' => 13,
        'K' => 12,
        'Q' => 11,
        'J' => 1,
        'T' => 10,
        n => n.to_digit(10).unwrap(),
    }
}
fn get_hand_type(hand: &str) -> u32 {
    let mut seen_cards = [0; 13];
    let mut maxes = (0, 0);
    let _ = hand
        .chars()
        .for_each(|card| {
            let idx = (card_to_val(card) - 1) as usize;
            seen_cards[idx] += 1;
        });

    let _ = seen_cards.iter().for_each(|card_count| {
        if *card_count > maxes.0 {
            maxes.1 = maxes.0;
            maxes.0 = *card_count;
        } else if *card_count > maxes.1 {
            maxes.1 = *card_count;
        }
    });

    maxes_to_value(maxes)
}

fn get_hand_type2(hand: &str) -> u32 {
    let mut seen_cards = [0; 13];
    let mut maxes = (0, 0);
    let _ = hand
        .chars()
        .for_each(|card| {
            let idx = (card_to_val_2(card) - 1) as usize;
            seen_cards[idx] += 1;
        });

    let _ = seen_cards.iter().enumerate().for_each(|(i, card_count)| {
        if i > 0 {
            if *card_count > maxes.0 {
                maxes.1 = maxes.0;
                maxes.0 = *card_count;
            } else if *card_count > maxes.1 {
                maxes.1 = *card_count;
            }
        } 
    });

    maxes.0 += seen_cards[0];

    maxes_to_value(maxes)
}

fn compare_high_cards2(hand1: &str, hand2: &str) -> Ordering {
    let cards1: Vec<char> = hand1.chars().collect();
    let cards2: Vec<char> = hand2.chars().collect();

    for i in  0..cards1.len() {
        if card_to_val_2(cards1[i]) > card_to_val_2(cards2[i]) {
            return Ordering::Greater
        } else if card_to_val_2(cards1[i]) < card_to_val_2(cards2[i]) {
            return Ordering::Less
        }
    }

    Ordering::Equal
}

fn compare_high_cards(hand1: &str, hand2: &str) -> Ordering {
    let cards1: Vec<char> = hand1.chars().collect();
    let cards2: Vec<char> = hand2.chars().collect();

    for i in  0..cards1.len() {
        if card_to_val(cards1[i]) > card_to_val(cards2[i]) {
            return Ordering::Greater
        } else if card_to_val(cards1[i]) < card_to_val(cards2[i]) {
            return Ordering::Less
        }
    }

    Ordering::Equal
}

fn compare_hands_2(line1: &str, line2: &str) -> Ordering {
    let line_and_val1: Vec<&str> = line1.split_whitespace().collect();
    let line_and_val2: Vec<&str> = line2.split_whitespace().collect();
    let hand1 = line_and_val1[0];
    let hand2 = line_and_val2[0];
    let hand_type1 = get_hand_type2(hand1);
    let hand_type2 = get_hand_type2(hand2);

    if hand_type1 > hand_type2 {
        Ordering::Greater
    } else if hand_type1 < hand_type2 {
        Ordering::Less
    } else {
        compare_high_cards2(hand1, hand2)
    }
}

fn compare_hands(line1: &str, line2: &str) -> Ordering {
    let line_and_val1: Vec<&str> = line1.split_whitespace().collect();
    let line_and_val2: Vec<&str> = line2.split_whitespace().collect();
    let hand1 = line_and_val1[0];
    let hand2 = line_and_val2[0];
    let hand_type1 = get_hand_type(hand1);
    let hand_type2 = get_hand_type(hand2);

    if hand_type1 > hand_type2 {
        Ordering::Greater
    } else if hand_type1 < hand_type2 {
        Ordering::Less
    } else {
        compare_high_cards(hand1, hand2)
    }
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should read the file");

    let mut lines: Vec<&str> = contents
        .lines()
        .collect();

    let _ = lines
        .sort_by(|line1, line2| compare_hands_2(line1, line2));

    let mut answer = 0;

    for i in 0..lines.len() {
        let line = lines[i];
        let rank = (i + 1) as u32;
        let line_vec: Vec<&str> = line.split_whitespace().collect();
        let bid = to_num(line_vec[1]);

        answer += rank * bid;
    }

    println!("{answer}")
}

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should read the file");

    let mut lines: Vec<&str> = contents
        .lines()
        .collect();

    let _ = lines
        .sort_by(|line1, line2| compare_hands(line1, line2));

    let mut answer = 0;

    for i in 0..lines.len() {
        let line = lines[i];
        let rank = (i + 1) as u32;
        let line_vec: Vec<&str> = line.split_whitespace().collect();
        let bid = to_num(line_vec[1]);

        answer += rank * bid;
    }

    println!("{answer}")
}

pub fn solve() {
    self::solve_part_1();
    self::solve_part_2();
}

