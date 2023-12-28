use std::{fs, i32, collections::{HashMap, VecDeque}};

fn solve_part_1() {
    // This will probably do but it takes too long 
    // so I just went ahead and used a graph solving lib 
    // in Python for the submission :D 
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should read the file");
    let mut connections = HashMap::new();
    contents
        .lines()
        .for_each(|line| {
            let colon_loc = line.find(":").unwrap();
            let source = &line[0..colon_loc];
            let map_entry = connections.entry(source).or_insert(Vec::new());
            let dests: Vec<&str> = (&line[colon_loc+1..])
                .trim()
                .split_whitespace()
                .collect();
            for &key in &dests {
                map_entry.push(key);
            }

            for key in &dests {
                connections
                    .entry(key)
                    .and_modify(|con| (*con).push(source))
                    .or_insert(Vec::new());
            }
        });

    let mut frequencies = HashMap::new();

    for &start in connections.keys() {
        let mut q = VecDeque::new();
        q.push_back(start);

        let mut seen = Vec::new();

        while let Some(pos) = q.pop_front() {
            seen.push(start);
            let edges = connections.get(pos).unwrap();
            for &next in edges {
                if !seen.contains(&next) {
                    seen.push(next);
                    let key = if pos < next { [pos, next] } else { [next, pos] };

                    frequencies
                        .entry(key)
                        .and_modify(|entry| *entry += 1)
                        .or_insert(0);

                    q.push_back(next);
                }
            }
        }
    }

    dbg!(frequencies.clone());

    let mut order: Vec<_> = frequencies.iter().collect();
    order.sort_unstable_by_key(|e| e.1);
    order.reverse();

    let cut: Vec<_> = order.iter().take(3).map(|p| *p.0).collect();
    let start = *connections.keys().next().unwrap();
    let mut size = 1;

    let mut q = VecDeque::new();
    q.push_back(start);

    let mut seen = Vec::new();
    seen.push(start);

    while let Some(pos) = q.pop_front() {
        let edges = connections.get(pos).unwrap();
        for &next in edges {
            let key = if pos < next { [pos, next] } else { [next, pos] };

            if !cut.contains(&key) {
                if !seen.contains(&next) {
                    seen.push(next);
                    size += 1;
                    q.push_back(next);
                }
            }
        }
    }

    dbg!(size);
    let answer = size * (connections.len() - size);

    println!("Part 1 Answer: {answer}");
}

pub fn solve() {
    self::solve_part_1();
}

