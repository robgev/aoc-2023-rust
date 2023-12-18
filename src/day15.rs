use std::{fs, i32, collections::HashMap};

fn to_num(num_str: &str) -> usize {
    let num: usize = num_str.trim().parse().unwrap();

    num
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should read the file");
    let mut boxes: Vec<Vec<&str>>  = Vec::new();
    for _ in 0..256 {
        boxes.push(Vec::new())
    }
    let mut lenses_map: HashMap<&str, usize> = HashMap::new();

    contents
        .trim()
        .split(",")
        .for_each(|step| {
            if let Some(minus_loc) = step.find("-") {
                let label = &step[0..minus_loc];
                let label_box = hash(label);
                lenses_map
                    .entry(label)
                    .and_modify(|focal_length| *focal_length = 0)
                    .or_insert(0);
                boxes[label_box].retain(|lense_label| *lense_label != label);
            } else if let Some(equal_loc) = step.find("=") {
                let label = &step[0..equal_loc];
                let new_focal_length = to_num(&step[equal_loc + 1..]);
                let label_box = hash(label);
                lenses_map
                    .entry(label)
                    .and_modify(|focal_length| *focal_length = new_focal_length)
                    .or_insert(new_focal_length);
                if !boxes[label_box].contains(&label) {
                    boxes[label_box].push(label);
                }
            }
        });

    let answer = boxes.iter().enumerate().fold(0, |acc, (box_num, box_content)| {
        let box_total = box_content.iter().enumerate().fold(0, |box_total, (lens_slot, label)| {
            let focal_length = lenses_map.get(label).unwrap();
            box_total + (box_num + 1) * (lens_slot + 1) * focal_length
        });
        acc + box_total
    });
    println!("Part 2 Answer: {answer}");
}

fn hash(label: &str) -> usize {
    label.chars().fold(0, |acc, step_char| {
        ((acc + step_char as usize) % 256) * 17 % 256
    })
}

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should read the file");

    let answer: usize = contents
        .trim()
        .split(",")
        .map(|step| {
            hash(step)
        })
        .sum();

    println!("Part 1 Answer: {answer}");
}

pub fn solve() {
    // self::solve_part_1();
    self::solve_part_2();
}

