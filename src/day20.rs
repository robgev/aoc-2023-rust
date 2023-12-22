use std::{fs, i32, collections::{HashMap, VecDeque}};

pub fn lcm(nums: &Vec<usize>) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..].to_vec());
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should read the file");
    let mut broadcast_names: Vec<&str> = Vec::new();
    let mut mem: HashMap<&str, HashMap<&str, bool>> = HashMap::new();
    let mut outputs_map: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut types: HashMap<&str, char> = HashMap::new();

    // Brute-force clearly isn't working so we need to have couple of assumptions
    // Seems like only one channel feeds the "rx" and that channel is a conjuction
    // Let's find the name of that module
    let mut rx_feed = "";

    contents.lines().for_each(|line| {
        let arrow_loc = line.find("->").unwrap();
        let module_name = &line[0..arrow_loc - 1];
        let targets: Vec<&str> = (&line[arrow_loc + 2..]).trim().split(", ").collect();
        if module_name == "broadcaster" {
            broadcast_names = targets;
        } else {
            let m_type = module_name.chars().nth(0).unwrap();
            let name = &module_name[1..];
            if m_type == '%' || m_type == '&' {
                types.insert(name, m_type);
                outputs_map.insert(name, targets);
                if m_type == '%' {
                    mem.insert(name, HashMap::from([("", false)]));
                } else {
                    mem.insert(name, HashMap::new());
                }
            }        
        }
    });

    outputs_map.iter().for_each(|(name, outputs)| {
        outputs.iter().for_each(|output| {
            if *output == "rx" {
                rx_feed = name;
            }

            if types.contains_key(output) {
                let m_type = types.get(output).unwrap();
                if *m_type == '&' {
                    mem.entry(output).and_modify(|mem_map| {
                        mem_map.insert(name, false);
                    });
                }
            }
        })
    });

    // We assume that the all conjuctions that feed to rx will 
    // have cyclic intervals to sending a high pulse
    // we need to find the lcm of all those intervals for 
    // the low pulse to rx
    let mut intervals_map = HashMap::new();
    outputs_map
        .iter()
        .for_each(|(name, outputs)| {
            if (*outputs).contains(&rx_feed) {
                intervals_map.insert(*name, 0);
            }
        });
    let mut intervals: Vec<usize> = Vec::new();

    let mut i = 0;

    // When we found all cycle lengths we can find the answer 
    while intervals_map.values().len() != intervals.len() {
        i += 1;
        let mut signals_q: VecDeque<(&str, &str, bool)> = VecDeque::new();
        for item in &broadcast_names {
            signals_q.push_back(("broadcaster", *item, false));
        }
        while !signals_q.is_empty() {
            let (origin, destination, pulse) = signals_q.pop_front().unwrap();

            if types.contains_key(destination) {
                let m_type = types.get(destination).unwrap();
                let outputs = outputs_map.get(destination).unwrap();
                if *m_type == '%' {
                    if !pulse {
                        let mut is_on = false;
                        mem.entry(destination).and_modify(|map| {
                            map.entry("").and_modify(|mem| {
                                is_on = !*mem;
                                *mem = !*mem;
                            });
                        });
                        outputs.iter().for_each(|output| {
                           signals_q.push_back((destination, output, is_on)) 
                        }) 
                    }
                } else if *m_type == '&' {
                    if destination == rx_feed && pulse {
                        let first_occurence = *(intervals_map.get(origin).unwrap());
                        if first_occurence != 0 {
                            intervals.push(i - first_occurence);
                        } else {
                            intervals_map.entry(origin).and_modify(|curr| *curr = i);
                        }
                    }
                    mem.entry(destination).and_modify(|conj_mem| {
                        conj_mem.entry(origin).and_modify(|mem| *mem = pulse);
                    });
                    let dest_mem = mem.get(destination).unwrap();
                    let is_all_high = dest_mem.values().all(|pulse| *pulse);
                    outputs.iter().for_each(|output| {
                        signals_q.push_back((destination, output, !is_all_high));
                    })
                }
            }
        }
    }

    let answer = lcm(&intervals);
    println!("Part 2 Answer: {answer}");
}

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should read the file");
    let mut broadcast_names: Vec<&str> = Vec::new();
    let mut mem: HashMap<&str, HashMap<&str, bool>> = HashMap::new();
    let mut outputs_map: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut types: HashMap<&str, char> = HashMap::new();
    contents.lines().for_each(|line| {
        let arrow_loc = line.find("->").unwrap();
        let module_name = &line[0..arrow_loc - 1];
        let targets: Vec<&str> = (&line[arrow_loc + 2..]).trim().split(", ").collect();
        if module_name == "broadcaster" {
            broadcast_names = targets;
        } else {
            let m_type = module_name.chars().nth(0).unwrap();
            let name = &module_name[1..];
            if m_type == '%' || m_type == '&' {
                types.insert(name, m_type);
                outputs_map.insert(name, targets);
                if m_type == '%' {
                    mem.insert(name, HashMap::from([("", false)]));
                } else {
                    mem.insert(name, HashMap::new());
                }
            }        
        }
    });

    outputs_map.iter().for_each(|(name, outputs)| {
        outputs.iter().for_each(|output| {
            if types.contains_key(output) {
                let m_type = types.get(output).unwrap();
                if *m_type == '&' {
                    mem.entry(output).and_modify(|mem_map| {
                        mem_map.insert(name, false);
                    });
                }
            }
        })
    });

    let mut low_pulses = 0;
    let mut high_pulses = 0;
    for _ in 0..1000 {
        // button to broadcaser;
        low_pulses += 1;
        let mut signals_q: VecDeque<(&str, &str, bool)> = VecDeque::new();
        for item in &broadcast_names {
            signals_q.push_back(("broadcaster", *item, false));
        }

        while !signals_q.is_empty() {
            let (origin, destination, pulse) = signals_q.pop_front().unwrap();
            if !pulse {
                low_pulses += 1;
            } else {
                high_pulses += 1;
            }

            if types.contains_key(destination) {
                let m_type = types.get(destination).unwrap();
                let outputs = outputs_map.get(destination).unwrap();
                if *m_type == '%' {
                    if !pulse {
                        let mut is_on = false;
                        mem.entry(destination).and_modify(|map| {
                            map.entry("").and_modify(|mem| {
                                is_on = !*mem;
                                *mem = !*mem;
                            });
                        });
                        outputs.iter().for_each(|output| {
                           signals_q.push_back((destination, output, is_on)) 
                        }) 
                    }
                } else if *m_type == '&' {
                    mem.entry(destination).and_modify(|conj_mem| {
                        conj_mem.entry(origin).and_modify(|mem| *mem = pulse);
                    });
                    let dest_mem = mem.get(destination).unwrap();
                    let is_all_high = dest_mem.values().all(|pulse| *pulse);
                    outputs.iter().for_each(|output| {
                        signals_q.push_back((destination, output, !is_all_high));
                    })
                }
            }
        }
    }

    let answer = low_pulses * high_pulses;
    println!("Part 1 Answer: {answer}");
}

pub fn solve() {
    self::solve_part_1();
    self::solve_part_2();
}

