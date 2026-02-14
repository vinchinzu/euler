// Project Euler 93: Arithmetic expressions
// Find the set of 4 digits {a,b,c,d} that maximizes consecutive
// positive integers obtainable via arithmetic expressions.

use itertools::Itertools;

fn apply_op(a: f64, op: u8, b: f64) -> f64 {
    match op {
        0 => a + b,
        1 => a - b,
        2 => a * b,
        3 => if b != 0.0 { a / b } else { 1e18 },
        _ => 1e18,
    }
}

fn is_pos_int(val: f64) -> bool {
    val.is_finite() && val > 0.0 && (val - val.round()).abs() < 1e-7
}

fn evaluate_all(p: &[u8; 4], results: &mut Vec<bool>) {
    let a = p[0] as f64;
    let b = p[1] as f64;
    let c = p[2] as f64;
    let d = p[3] as f64;

    for op0 in 0..4u8 {
        for op1 in 0..4u8 {
            for op2 in 0..4u8 {
                // ((a op0 b) op1 c) op2 d
                let v = apply_op(apply_op(apply_op(a, op0, b), op1, c), op2, d);
                if is_pos_int(v) {
                    let idx = v.round() as usize;
                    if idx < results.len() { results[idx] = true; }
                }

                // (a op0 (b op1 c)) op2 d
                let v = apply_op(apply_op(a, op0, apply_op(b, op1, c)), op2, d);
                if is_pos_int(v) {
                    let idx = v.round() as usize;
                    if idx < results.len() { results[idx] = true; }
                }

                // a op0 ((b op1 c) op2 d)
                let v = apply_op(a, op0, apply_op(apply_op(b, op1, c), op2, d));
                if is_pos_int(v) {
                    let idx = v.round() as usize;
                    if idx < results.len() { results[idx] = true; }
                }

                // a op0 (b op1 (c op2 d))
                let v = apply_op(a, op0, apply_op(b, op1, apply_op(c, op2, d)));
                if is_pos_int(v) {
                    let idx = v.round() as usize;
                    if idx < results.len() { results[idx] = true; }
                }

                // (a op0 b) op1 (c op2 d)
                let v = apply_op(apply_op(a, op0, b), op1, apply_op(c, op2, d));
                if is_pos_int(v) {
                    let idx = v.round() as usize;
                    if idx < results.len() { results[idx] = true; }
                }
            }
        }
    }
}

fn main() {
    let mut max_n = 0;
    let mut best = [0u8; 4];

    for combo in (0u8..=9).combinations(4) {
        let digits = [combo[0], combo[1], combo[2], combo[3]];
        let mut results = vec![false; 10000];

        // Try all permutations
        let perm = digits;
        for p in perm.iter().copied().collect::<Vec<_>>().into_iter().permutations(4) {
            let arr = [p[0], p[1], p[2], p[3]];
            evaluate_all(&arr, &mut results);
        }

        let mut n = 0;
        while results.get(n + 1).copied().unwrap_or(false) {
            n += 1;
        }
        if n > max_n {
            max_n = n;
            best = digits;
        }
    }

    println!("{}{}{}{}", best[0], best[1], best[2], best[3]);
}
