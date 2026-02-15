// Project Euler 383: Divisibility comparison between factorials
use std::collections::HashMap;

const BASE: i64 = 5;

fn floor_div(a: i32, b: i32) -> i32 {
    if a >= 0 { a / b } else { (a - b + 1) / b }
}

fn t(n: i64, diff: i32, total_diff: i32, cache: &mut HashMap<(i64, i32, i32), i64>) -> i64 {
    if n <= 0 {
        return if n == 0 && total_diff > 0 { 1 } else { 0 };
    }

    let key = (n, diff, total_diff);
    if let Some(&val) = cache.get(&key) { return val; }

    let mut result = 0i64;
    let n_mod = (n % BASE) as i32;

    for r in 0..BASE as i32 {
        let new_diff = -floor_div(2 * r - diff, BASE as i32);
        let n_next = n / BASE - if r > n_mod { 1 } else { 0 };
        result += t(n_next, new_diff, total_diff + new_diff, cache);
    }

    cache.insert(key, result);
    result
}

fn main() {
    let n: i64 = 1_000_000_000_000_000_000; // 10^18
    let mut cache = HashMap::new();
    let ans = t(n, 1, 0, &mut cache) - 1;
    println!("{}", ans);
}
