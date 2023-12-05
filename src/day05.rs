use std::{fs, i32, usize, u64};

fn to_num(num_str: &str) -> u64 {
    let num: u64 = num_str.trim().parse().unwrap();

    num
}

fn find_mapped_number(map: Vec<Vec<u64>>, source_number: u64) -> u64 {
    let mut dest_number = source_number;
    for line in map {
        let src_start = line[1];
        if source_number >= src_start {
            let src_end = src_start + line[2];
            if source_number < src_end {
                let dest_start = line[0];
                dest_number = source_number + dest_start - src_start;
                return dest_number;
            }
        }
    }

    dest_number
}

fn find_mapped_ranges(map: Vec<Vec<u64>>, source_nums: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut dest_ranges: Vec<(u64, u64)> = Vec::new();
    let mut source_ranges = source_nums.clone();

    while source_ranges.len() > 0 {
        let (start, end) = source_ranges.pop().unwrap();
        let mut found = false;
        for line in &map {
            let src_start = line[1];
            let src_end = src_start + line[2];
            let overlap_start = u64::max(start, src_start);
            let overlap_end = u64::min(end, src_end);
            if overlap_start < overlap_end {
                found = true;
                let dest_start = overlap_start - src_start + line[0];
                let dest_end = overlap_end - src_start + line[0];
                dest_ranges.push((dest_start, dest_end));
                if overlap_start > start {
                    source_ranges.push((start, overlap_start));
                }
                if end > overlap_end {
                    source_ranges.push((overlap_end, end));
                }
                break;
            }
        }
        if !found {
            dest_ranges.push((start, end));
        }
    }

    dest_ranges
}

fn get_seed_numbers(seed_str: &str) -> Vec<(u64, u64)> {
    let seed_nums: Vec<u64> = seed_str
        .trim()
        .split(' ')
        .map(|seed_str| {
            let seed_number: u64 = seed_str.parse().unwrap();
            seed_number
        })
        .collect();

    let seed_ranges: Vec<(u64, u64)> = seed_nums.chunks(2).map(|chunk| (chunk[0], chunk[0] + chunk[1])).collect();

    seed_ranges
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should read the file");
    let map_names = [
        "seed-to-soil map:",
        "soil-to-fertilizer map:",
        "fertilizer-to-water map:",
        "water-to-light map:",
        "light-to-temperature map:",
        "temperature-to-humidity map:",
        "humidity-to-location map:",
        "",
    ];

    let seed_idx = contents.find("seeds:").unwrap() + "seeds:".len();
    let seed_end_idx = contents.find("seed-to-soil map:").unwrap();
    let seed_nums = get_seed_numbers(&contents[seed_idx..seed_end_idx]);
    let maps: Vec<Vec<Vec<u64>>> = map_names.windows(2).map(|map_name_pair| {
        let map_start_idx = contents.find(map_name_pair[0]).unwrap() + map_name_pair[0].len();
        let map_end_idx = contents.find(map_name_pair[1]).unwrap();
        let contents_ref = &contents;
        let map_str = if map_name_pair[1] == "" {
            contents_ref[map_start_idx..].trim()
        } else {
            contents_ref[map_start_idx..map_end_idx].trim()
        };

        let ans: Vec<Vec<u64>> = map_str.lines().map(|line| {
            let ans = line.split_whitespace().map(|num| { to_num(num) });
            ans.collect()
        }).collect();

        ans
    }).collect();

    let answer = maps
    .iter()
    .fold(seed_nums, |acc, map| {
        find_mapped_ranges(map.to_vec(), acc)
    })
    .iter()
    .map(|(range_min, _)| *range_min)
    .min()
    .unwrap();

    println!("Part 2 Answer: {:?}", answer);
}

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should read the file");
    let map_names = [
        "seed-to-soil map:",
        "soil-to-fertilizer map:",
        "fertilizer-to-water map:",
        "water-to-light map:",
        "light-to-temperature map:",
        "temperature-to-humidity map:",
        "humidity-to-location map:",
        "",
    ];

    let seed_idx = contents.find("seeds:").unwrap() + "seeds:".len();
    let seed_end_idx = contents.find("seed-to-soil map:").unwrap();
    let maps: Vec<Vec<Vec<u64>>> = map_names.windows(2).map(|map_name_pair| {
        let map_start_idx = contents.find(map_name_pair[0]).unwrap() + map_name_pair[0].len();
        let map_end_idx = contents.find(map_name_pair[1]).unwrap();
        let contents_ref = &contents;
        let map_str = if map_name_pair[1] == "" {
            contents_ref[map_start_idx..].trim()
        } else {
            contents_ref[map_start_idx..map_end_idx].trim()
        };

        let ans: Vec<Vec<u64>> = map_str.lines().map(|line| {
            let ans = line.split_whitespace().map(|num| { to_num(num) });
            ans.collect()
        }).collect();

        ans
    }).collect();
    let answer: u64 = (&contents[seed_idx..seed_end_idx])
        .trim()
        .split(' ')
        .map(|seed_str| {
            let seed_number: u64 = seed_str.parse().unwrap();
            let answer = maps.iter().fold(seed_number, |acc, map| {
                find_mapped_number(map.to_vec(), acc)
            });

            answer
        })
        .min()
        .unwrap();

    println!("Part 1 Answer: {answer}");
}

pub fn solve() {
    // self::solve_part_1();
    self::solve_part_2();
}

