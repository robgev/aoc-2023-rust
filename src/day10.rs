use std::{fs, i32, collections::HashMap};

fn find_start_loc(maze: &Vec<Vec<char>>) -> (usize, usize) {
    for i in 0..maze.len() {
        for j in 0..maze[0].len() {
            if maze[i][j] == 'S' {
                return (i, j)
            }
        }
    }

    return (0, 0);
}

fn get_char_at(schema: &Vec<Vec<char>>, loc: (i32, i32)) -> char {
    let row = loc.0;
    let col = loc.1;
    let schema_len: i32 = schema.len().try_into().unwrap();
    let row_len: i32 = schema[0].len().try_into().unwrap();
    if row < 0 || col < 0 || row >= schema_len || col >= row_len {
        '.'
    } else {
        schema[row as usize][col as usize]
    }
}

fn traverse(maze: &Vec<Vec<char>>, start_loc: (usize, usize), start_dir: char, dirs_map: &HashMap<(char, char), char>) -> usize {
    let mut loc = start_loc;
    let mut dir = start_dir;
    let mut visited: HashMap<(usize, usize), bool> = HashMap::new();
    let mut steps = 0;
    while !visited.contains_key(&(loc.0, loc.1)) {
        visited.insert(loc, true);
        let row = loc.0 as i32;
        let col = loc.1 as i32;
        let next_loc = match dir {
            'u' => (row - 1, col),
            'd' => (row + 1, col),
            'l' => (row, col - 1),
            'r' => (row, col + 1),
            _ => (row, col),
        };
        let pipe = get_char_at(maze, next_loc);
        if dirs_map.contains_key(&(pipe, dir))  {
            steps += 1;
            loc = (next_loc.0 as usize, next_loc.1 as usize);
            dir = *dirs_map.get(&(pipe, dir)).unwrap();
        }

    }

    steps
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should read the file");

    let dirs_map = HashMap::from([
        (('|', 'u'), 'u'),
        (('|', 'd'), 'd'),
        (('-', 'r'), 'r'),
        (('-', 'l'), 'l'),
        (('L', 'l'), 'u'),
        (('L', 'd'), 'r'),
        (('J', 'r'), 'u'),
        (('J', 'd'), 'l'),
        (('7', 'r'), 'd'),
        (('7', 'u'), 'l'),
        (('F', 'u'), 'r'),
        (('F', 'l'), 'd'),
    ]);

    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should read the file");

    let mut maze: Vec<Vec<char>> = contents
        .lines()
        .map(|line| { line.chars().collect() })
        .collect();
    let start_loc = find_start_loc(&maze);
    let start_dir = 'd';

    let mut loc = start_loc;
    let mut dir = start_dir;
    let mut visited: HashMap<(usize, usize), bool> = HashMap::new();
    while !visited.contains_key(&(loc.0, loc.1)) {
        visited.insert(loc, true);
        let row = loc.0 as i32;
        let col = loc.1 as i32;
        let next_loc = match dir {
            'u' => (row - 1, col),
            'd' => (row + 1, col),
            'l' => (row, col - 1),
            'r' => (row, col + 1),
            _ => (row, col),
        };
        let pipe = get_char_at(&maze, next_loc);
        if dirs_map.contains_key(&(pipe, dir))  {
            loc = (next_loc.0 as usize, next_loc.1 as usize);
            dir = *dirs_map.get(&(pipe, dir)).unwrap();
        }
    }

    maze[start_loc.0][start_loc.1] = match (start_dir, dir) {
        ('u', 'u') | ('d', 'd') => '|',
        ('l', 'l') | ('r', 'r') => '-',
        ('r', 'u') | ('d', 'l') => 'F',
        ('r', 'd') | ('u', 'l') => 'L',
        ('l', 'd') | ('u', 'r')=> 'J',
        ('l', 'u') | ('d', 'r') => '7',
        _ => '.'
    };
    let mut answer = 0;

    maze.iter().enumerate().for_each(|(r, row)| {
        let mut ray_hits = 0;
        let mut changed_dir = false;
        row.iter().enumerate().for_each(|(c, pipe_or_ground)| {
            if visited.contains_key(&(r, c)) {
                match pipe_or_ground {
                    '|' => ray_hits += 1,
                    'F' => changed_dir = false,
                    'L' => changed_dir = true,
                    '7' => if changed_dir  { ray_hits += 1; changed_dir = false },
                    'J' => if !changed_dir { ray_hits += 1; changed_dir = false },
                    _ => {},
                }
            } else if ray_hits % 2 == 1 {
                answer += 1;
            }

        })
    });

    println!("Part 2 Answer: {answer}");
}

fn solve_part_1() {
    let dirs_map = HashMap::from([
        (('|', 'u'), 'u'),
        (('|', 'd'), 'd'),
        (('-', 'r'), 'r'),
        (('-', 'l'), 'l'),
        (('L', 'l'), 'u'),
        (('L', 'd'), 'r'),
        (('J', 'r'), 'u'),
        (('J', 'd'), 'l'),
        (('7', 'r'), 'd'),
        (('7', 'u'), 'l'),
        (('F', 'u'), 'r'),
        (('F', 'l'), 'd'),
    ]);

    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should read the file");

    let maze: Vec<Vec<char>> = contents
        .lines()
        .map(|line| { line.chars().collect() })
        .collect();

    let start_loc = find_start_loc(&maze);
    let dirs = ['u', 'd', 'r', 'l'];
    let mut answer = 0;
    for dir in dirs {
        let steps = (traverse(&maze, start_loc, dir, &dirs_map) + 1) / 2;
        if steps > answer {
            answer = steps;
        }
    }

    println!("Part 1 Answer: {:?}", answer);
}

pub fn solve() {
    // self::solve_part_1();
    self::solve_part_2();
}

