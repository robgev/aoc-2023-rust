use std::{fs, i32, collections::HashMap, usize};

fn to_num(num_str: &str) -> usize {
    let num: usize = num_str.trim().parse().unwrap();

    num
}

fn chop_ranges(ranges: &HashMap<char, (usize, usize)>, name: &str, rules_map: &HashMap<&str, Vec<(&str, &str)>>) -> usize {
    if name == "R" {
        0
    } else if name == "A" {
        ranges.values().fold(1, |acc, (min, max)| acc * (max - min + 1))
    } else {
        let mut total = 0;
        let mut ranges_clone = ranges.clone();
        for (comparison, dest) in rules_map.get(name).unwrap() { 
            if *comparison != "" {
                let comparison_key = &comparison[0..1].chars().last().unwrap();
                let comp_sign = &comparison[1..2];
                let comp_val = to_num(&comparison[2..]);
                let (min, max) = ranges_clone.get(comparison_key).unwrap(); 
                let mut true_part = (*min, *max);
                let mut false_part = (*min, *max);
                if comp_sign == "<" {
                    true_part = (*min, comp_val - 1);
                    false_part = (comp_val, *max);
                } else {
                    true_part = (comp_val + 1, *max);
                    false_part = (*min, comp_val);
                }

                if true_part.0 <= true_part.1 {
                    let mut new_ranges = ranges_clone.clone();
                    new_ranges.entry(*comparison_key).and_modify(|range| *range = true_part);
                    total += chop_ranges(&new_ranges, dest, rules_map)
                }

                if false_part.0 <= false_part.1 {
                    ranges_clone.entry(*comparison_key).and_modify(|range| *range = false_part);
                } else {
                    break;
                }
            } else {
                total += chop_ranges(&ranges_clone, dest, rules_map)
            }
        }

        total
    }
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should read the file");

    let content_parts: Vec<&str> = contents.split("\n\n")
        .collect();
    let mut rules_map: HashMap<&str, Vec<(&str, &str)>> = HashMap::new();
    content_parts[0]
        .lines()
        .for_each(|rule| {
            let brace_loc = rule.find("{").unwrap();
            let brace_close_loc = rule.find("}").unwrap();
            let name = &rule[0..brace_loc];
            let rules: Vec<(&str, &str)> = rule[brace_loc+1..brace_close_loc]
                .split(",")
                .map(|check| {
                    if let Some(colon_loc) = check.find(":") {
                        let comparison = &check[0..colon_loc];
                        let dest = &check[colon_loc+1..];
                        (comparison, dest)
                    } else {
                        ("", check)
                    }
                })
                .collect();
            rules_map.insert(name, rules);
        });

    let ranges: HashMap<char, (usize, usize)> = HashMap::from([('x', (1, 4000)), ('m', (1, 4000)), ('a', (1, 4000)), ('s', (1, 4000))]);
    let answer = chop_ranges(&ranges, "in", &rules_map);

    println!("Part 2 Answer: {answer}");
}

fn comparator(check: &str, part: &HashMap<char, usize>) -> bool {
    let comparison_key = &check[0..1].chars().last().unwrap();
    let comp_sign = &check[1..2];
    let comp_val = to_num(&check[2..]);
    let check_val = part.get(comparison_key).unwrap();
    match comp_sign {
        "<" => check_val < &comp_val,
        ">" => check_val > &comp_val,
        _ => false,
    }
}

fn get_next(rules: &Vec<(&str, &str)>, part: &HashMap<char, usize>) -> String {
    let passed_rule = rules.iter().position(|&(check, _)| check == "" || comparator(check, part)).unwrap();

    rules[passed_rule].1.to_string()
}

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should read the file");

    let content_parts: Vec<&str> = contents.split("\n\n")
        .collect();
    let mut rules_map: HashMap<&str, Vec<(&str, &str)>> = HashMap::new();
    let mut parts_map: Vec<HashMap<char, usize>> = Vec::new();
    content_parts[0]
        .lines()
        .for_each(|rule| {
            let brace_loc = rule.find("{").unwrap();
            let brace_close_loc = rule.find("}").unwrap();
            let name = &rule[0..brace_loc];
            let rules: Vec<(&str, &str)> = rule[brace_loc+1..brace_close_loc]
                .split(",")
                .map(|check| {
                    if let Some(colon_loc) = check.find(":") {
                        let comparison = &check[0..colon_loc];
                        let dest = &check[colon_loc+1..];
                        (comparison, dest)
                    } else {
                        ("", check)
                    }
                })
                .collect();
            rules_map.insert(name, rules);
        });

    content_parts[1]
        .lines()
        .for_each(|part| {
            let x_loc = part.find("x").unwrap();
            let m_loc = part.find("m").unwrap();
            let a_loc = part.find("a").unwrap();
            let s_loc = part.find("s").unwrap();
            let close_brace = part.find("}").unwrap();
            let x = to_num(&part[x_loc + 2..m_loc - 1]);
            let m = to_num(&part[m_loc + 2..a_loc - 1]);
            let a = to_num(&part[a_loc + 2..s_loc - 1]);
            let s = to_num(&part[s_loc + 2..close_brace]);
            let t = x + m + a + s;
            let map = HashMap::from([('x', x), ('m', m), ('a', a), ('s', s), ('t', t)]);
            parts_map.push(map);
        });

    let mut answer = 0;
    parts_map.iter().for_each(|part| {
        let mut loc: String = "in".to_string();
        while loc != "A" && loc != "R" {
            loc = get_next(&rules_map.get(&loc.as_str()).unwrap(), &part);
        }

        if loc == "A" {
            answer += part.get(&'t').unwrap();
        }
    });

    println!("Part 1 Answer: {answer}");
}

pub fn solve() {
    // self::solve_part_1();
    self::solve_part_2();
}

