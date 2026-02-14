// Project Euler 151: Paper sheets of standard sizes
// Expected number of times the envelope contains a single sheet (excluding first/last).

use std::collections::HashMap;

fn solve(c: [u8; 5], memo: &mut HashMap<[u8; 5], f64>) -> f64 {
    let num_sheets: u8 = c.iter().sum();
    if num_sheets == 0 { return 0.0; }
    if num_sheets == 1 && c[4] == 1 { return 0.0; }

    if let Some(&v) = memo.get(&c) {
        return v;
    }

    let contribution = if num_sheets == 1 { 1.0 } else { 0.0 };
    let mut future = 0.0;

    for idx in 0..5 {
        if c[idx] > 0 {
            let prob = c[idx] as f64 / num_sheets as f64;
            let mut nc = c;
            nc[idx] -= 1;
            for k in (idx + 1)..5 {
                nc[k] += 1;
            }
            future += prob * solve(nc, memo);
        }
    }

    let result = contribution + future;
    memo.insert(c, result);
    result
}

fn main() {
    let mut memo = HashMap::new();
    let raw = solve([1, 0, 0, 0, 0], &mut memo);
    let result = raw - 1.0;
    println!("{:.6}", result);
}
