// Project Euler 556 - Squarefree Gaussian Integers
//
// Count proper squarefree Gaussian integers a+bi with a^2+b^2 <= N.
// Uses Mobius-like recursion with hash table memoization.

use std::collections::HashMap;

const N: i64 = 100_000_000_000_000; // 10^14

fn isqrt(n: i64) -> i64 {
    if n <= 0 { return 0; }
    let mut r = (n as f64).sqrt() as i64;
    while r > 0 && r * r > n { r -= 1; }
    while (r + 1) * (r + 1) <= n { r += 1; }
    r
}

fn count_gauss(n: i64) -> i64 {
    if n <= 0 { return 0; }
    let mut total: i64 = 0;
    let sq = isqrt(n);
    for a in 1..=sq {
        let rem = n - a * a;
        if rem < 0 { break; }
        total += isqrt(rem) + 1;
    }
    total
}

fn f(n: i64, cache: &mut HashMap<i64, i64>) -> i64 {
    if n == 0 { return 0; }
    if let Some(&v) = cache.get(&n) { return v; }

    let mut result = count_gauss(n);
    let sn = isqrt(n);
    let fourth = isqrt(sn);

    for a in 1..=fourth {
        let a_sq = a * a;
        if a_sq * a_sq > n { break; }
        let max_b_sq = sn - a_sq;
        if max_b_sq < 0 { break; }
        let max_b = isqrt(max_b_sq);

        for b in 0..=max_b {
            let z_norm_sq = a_sq + b * b;
            if z_norm_sq <= 1 { continue; }
            let z4 = z_norm_sq * z_norm_sq;
            if z4 > n { break; }
            let arg = n / z4;
            if arg == 0 { continue; }
            result -= f(arg, cache);
        }
    }

    cache.insert(n, result);
    result
}

fn main() {
    let mut cache: HashMap<i64, i64> = HashMap::new();
    println!("{}", f(N, &mut cache));
}
