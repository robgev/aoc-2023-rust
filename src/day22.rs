use std::{fs, i32, collections::{HashMap, VecDeque}};

fn to_num(num_str: &str) -> usize {
    let num: usize = num_str.trim().parse().unwrap();

    num
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should read the file");
    let mut brick_coords = Vec::new();
    let mut max_x = 0;
    let mut max_y = 0;

    contents.lines().for_each(|line| {
        let tilde_loc = line.find("~").unwrap();
        let coord1: Vec<usize> = (&line[0..tilde_loc])
            .split(",")
            .map(to_num)
            .collect();
        let coord2: Vec<usize> = (&line[tilde_loc+1..])
            .split(",")
            .map(to_num)
            .collect();
        let x = coord1[0].max(coord2[0]);
        let y = coord1[1].max(coord2[1]);
        brick_coords.push(((coord1[0], coord1[1], coord1[2]), (coord2[0], coord2[1], coord2[2])));
        if x > max_x {
            max_x = x;
        }
        if y > max_y {
            max_y = y;
        }
    });

    brick_coords.sort_by(|brick1, brick2| {
        let z1 = brick1.1.2;
        let z2 = brick2.1.2;

        z1.cmp(&z2)
    });

    let mut orders: Vec<Vec<usize>> = vec![vec![0; max_x + 1]; max_y + 1];
    let mut lone_sups: Vec<usize> = Vec::new();
    let mut is_supporting: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut is_supported_by: HashMap<usize, Vec<usize>> = HashMap::new();

    for i in 0..brick_coords.len() {
        let ((x1, y1, z1), (x2, y2, z2)) = brick_coords[i];
        let height = z2 - z1;
        let mut max_z = 0;
        let mut supports: Vec<usize> = Vec::new();
        is_supporting.insert(i + 1, Vec::new());
        is_supported_by.insert(i + 1, Vec::new());
        for x in x1..=x2 {
            for y in y1..=y2 {
                if orders[x][y] > 0 {
                    // Consider 0 as ground
                    let idx = orders[x][y] - 1;
                    let current_brick = brick_coords[idx];
                    if current_brick.1.2 > max_z {
                        supports = vec![(orders[x][y])];
                        max_z = current_brick.1.2;
                    } else if current_brick.1.2 == max_z {
                        if !supports.contains(&orders[x][y]) {
                            supports.push(orders[x][y]);
                        }
                    }
                }
                orders[x][y] = i + 1;
            }
        }

        for support in &supports {
            if supports.len() == 1 {
                if !lone_sups.contains(support) {
                    lone_sups.push(*support);
                }
            }
            is_supported_by
                .entry(i + 1)
                .and_modify(|supports| (*supports).push(*support));
            is_supporting
                .entry(*support)
                .and_modify(|is_supporting| (*is_supporting).push(i + 1));
        }

        brick_coords[i] = ((x1, y1, max_z + 1), (x2, y2, max_z + 1 + height));
    }

    let mut answer = 0;
    // For all the lone supports we need to find the number of 
    // bricks that will fall if we take that one out 
    lone_sups.iter().for_each(|root| {
        let mut total = 0;
        // We start from the lone brick 
        let mut q: VecDeque<usize> = VecDeque::from([*root]);
        let mut falling_bricks: Vec<usize> = Vec::new();

        while !q.is_empty() {
            let brick = q.pop_front().unwrap();
            falling_bricks.push(brick);
            total += 1;
            // Find all bricks that this brick supports
            let supported_by_brick = is_supporting.get(&brick).unwrap();
            for sup_brick in supported_by_brick {
                // If all the supports of the current brick will fall 
                // then this brick also will fall
                let supports_of_brick = is_supported_by.get(sup_brick).unwrap();
                let will_fall = supports_of_brick.iter().all(|brick| falling_bricks.contains(brick));
                // If this brick will fall, add it to the queue to check all the bricks
                // That this one supports
                if !falling_bricks.contains(sup_brick) && will_fall {
                    q.push_back(*sup_brick);
                }
            }
        }

        answer += total - 1;
    });

    println!("Part 2 Answer: {answer}");
}

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should read the file");
    let mut brick_coords = Vec::new();
    let mut max_x = 0;
    let mut max_y = 0;

    contents.lines().for_each(|line| {
        let tilde_loc = line.find("~").unwrap();
        let coord1: Vec<usize> = (&line[0..tilde_loc])
            .split(",")
            .map(to_num)
            .collect();
        let coord2: Vec<usize> = (&line[tilde_loc+1..])
            .split(",")
            .map(to_num)
            .collect();
        let x = coord1[0].max(coord2[0]);
        let y = coord1[1].max(coord2[1]);
        brick_coords.push(((coord1[0], coord1[1], coord1[2]), (coord2[0], coord2[1], coord2[2])));
        if x > max_x {
            max_x = x;
        }
        if y > max_y {
            max_y = y;
        }
    });

    brick_coords.sort_by(|brick1, brick2| {
        let z1 = brick1.1.2;
        let z2 = brick2.1.2;

        z1.cmp(&z2)
    });

    // Top down view of the highest bricks at each x/y coordinate.
    // Whatever brick falls at certain x/y range it will always sit on 
    // top of the highest brick at that x/y range
    let mut orders: Vec<Vec<usize>> = vec![vec![0; max_x + 1]; max_y + 1];
    let mut lone_sups: Vec<usize> = Vec::new();

    for i in 0..brick_coords.len() {
        let ((x1, y1, z1), (x2, y2, z2)) = brick_coords[i];
        let height = z2 - z1;
        let mut max_z = 0;
        let mut supports: Vec<usize> = Vec::new();
        // Find the highest brick in x/y range and use it's z as 
        // the "landing" of this brick.
        for x in x1..=x2 {
            for y in y1..=y2 {
                if orders[x][y] > 0 {
                    // Consider 0 as ground
                    let idx = orders[x][y] - 1;
                    let current_brick = brick_coords[idx];
                    if current_brick.1.2 > max_z {
                        supports = vec![(orders[x][y])];
                        max_z = current_brick.1.2;
                    } else if current_brick.1.2 == max_z {
                        if !supports.contains(&orders[x][y]) {
                            supports.push(orders[x][y]);
                        }
                    }
                }
                orders[x][y] = i + 1;
            }
        }

        for support in &supports {
            // If there is only one brick supporting 
            // this one, then that supporting brick 
            // cannot be disintegrated. Otherwise, 
            // the current brick we are investigating
            // will fall too. 
            if supports.len() == 1 {
                if !lone_sups.contains(support) {
                    lone_sups.push(*support);
                }
            }
        }

        brick_coords[i] = ((x1, y1, max_z + 1), (x2, y2, max_z + 1 + height));
    }

    // number of bricks we can disintegrate is the 
    // number of total minus the ones that are sole support 
    // for at least one other brick
    let answer = brick_coords.len() - lone_sups.len();
    println!("Part 1 Answer: {answer}");
}

pub fn solve() {
    self::solve_part_1();
    self::solve_part_2();
}

