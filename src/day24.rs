use std::{fs, i32, i64};
use z3::{Config, Context, Solver};
use z3::ast::{Ast, Int};

fn to_num(num_str: &str) -> f32 {
    let num: f32 = num_str.trim().parse().unwrap();

    num
}

fn to_int(num_str: &str) -> i64 {
    let num: i64 = num_str.trim().parse().unwrap();

    num
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should read the file");

    let params: Vec<[i64; 6]> = contents
        .lines()
        .map(|line| {
            let mut ans: [i64; 6] = [0; 6];
            let nums_line = line.replace("@", ",");
            nums_line
                .split(",")
                .enumerate()
                .for_each(|(i, num_str)| {
                    ans[i] = to_int(num_str);
                });
            ans
        })
        .collect();
    
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let sx = Int::new_const(&ctx, "sx");
    let sy = Int::new_const(&ctx, "sy");
    let sz = Int::new_const(&ctx, "sz");
    let vx = Int::new_const(&ctx, "vx");
    let vy = Int::new_const(&ctx, "vy");
    let vz = Int::new_const(&ctx, "vz");

    for param in params {
        let [sxh, syh, szh, vxh, vyh, vzh] = param;
        let sxn = Int::from_i64(&ctx, sxh);
        let syn = Int::from_i64(&ctx, syh);
        let szn = Int::from_i64(&ctx, szh);
        let vxn = Int::from_i64(&ctx, vxh);
        let vyn = Int::from_i64(&ctx, vyh);
        let vzn = Int::from_i64(&ctx, vzh);
        let tn = Int::fresh_const(&ctx, "t");

        solver.assert(&(&sxn + &vxn * &tn)._eq(&(&sx + &vx * &tn)));
        solver.assert(&(&syn + &vyn * &tn)._eq(&(&sy + &vy * &tn)));
        solver.assert(&(&szn + &vzn * &tn)._eq(&(&sz + &vz * &tn)));
    }

    solver.check();
    let model = solver.get_model().unwrap();
    let x = model.get_const_interp(&sx).unwrap().as_i64().unwrap();
    let y = model.get_const_interp(&sy).unwrap().as_i64().unwrap();
    let z = model.get_const_interp(&sz).unwrap().as_i64().unwrap();
    let answer = x + y + z;

    println!("Part 2 Answer: {answer}");
}

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should read the file");

    let params: Vec<[f32; 6]> = contents
        .lines()
        .map(|line| {
            let mut ans: [f32; 6] = [0.0; 6];
            let nums_line = line.replace("@", ",");
            nums_line
                .split(",")
                .enumerate()
                .for_each(|(i, num_str)| {
                    ans[i] = to_num(num_str);
                });
            ans
        })
        .collect();
    
    let mut answer = 0;

    for i in 0..params.len() {
        let [sx1, sy1, _, vx1, vy1, _] = params[i];
        let a1 = vy1;
        let b1 = -vx1;
        let c1 = sx1 * vy1 - sy1 * vx1;
        for j in 0..i {
            let [sx2, sy2, _, vx2, vy2, _] = params[j];
            let a2 = vy2;
            let b2 = -vx2;
            let c2 = sx2 * vy2 - sy2 * vx2;
            if a1 * b2 != a2 * b1 {
                let x = (c1 * b2 - c2 * b1) / (a1 * b2 - a2 * b1);
                let y = (c2 * a1 - c1 * a2) / (a1 * b2 - a2 * b1);
                let is_in_future1 = (x - sx1) * vx1 >= 0.0 && (y - sy1) * vy1 >= 0.0;
                let is_in_future2 = (x - sx2) * vx2 >= 0.0 && (y - sy2) * vy2 >= 0.0;
                if x >= 200000000000000.0 && x <= 400000000000000.0 && y >= 200000000000000.0 && y <= 400000000000000.0 && is_in_future1 && is_in_future2 {
                    answer += 1;
                }
            }
        }
    }

    println!("Part 1 Answer: {answer}");
}

pub fn solve() {
    // self::solve_part_1();
    self::solve_part_2();
}
