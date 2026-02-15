// Project Euler 660 - Pandigital Triangles
// 120-degree triangles with pandigital sides in bases 9-18.

use euler_utils::gcd;
use std::collections::HashSet;

fn is_pandigital(a: i64, b: i64, c: i64, base: i64) -> bool {
    let mut used = vec![false; base as usize];
    let mut count = 0;
    for mut x in [a, b, c] {
        if x == 0 {
            if used[0] { return false; }
            used[0] = true; count += 1;
        }
        while x > 0 {
            let d = (x % base) as usize;
            if used[d] { return false; }
            used[d] = true; count += 1;
            x /= base;
        }
    }
    count == base
}

fn ceil_div(a: i32, b: i32) -> i32 { (a + b - 1) / b }

fn main() {
    let mut seen = HashSet::new();
    let mut total = 0i64;
    for base in 9..=18 {
        let e1 = ceil_div(base, 3);
        let mut limit = 1i64;
        for _ in 0..e1 { limit *= base as i64; }
        let mut limit2 = 1i64;
        for _ in 0..e1-1 { limit2 *= base as i64; }
        limit += limit2;
        for n in 1.. {
            if (n as i64) * (n as i64) > limit { break; }
            for m in n+1..2*n {
                let ls1 = (m as i64)*(m as i64) - (m as i64)*(n as i64) + (n as i64)*(n as i64);
                if ls1 > limit { break; }
                if (m + n) % 3 == 0 { continue; }
                if gcd(m as u64, n as u64) != 1 { continue; }
                for k in 1.. {
                    let c = k as i64 * ls1;
                    if c > limit { break; }
                    let a = k as i64 * ((m as i64)*(m as i64) - (n as i64)*(n as i64));
                    let b_val = k as i64 * (m as i64) * (2*n as i64 - m as i64);
                    if is_pandigital(a, b_val, c, base as i64) {
                        if seen.insert((a, b_val, c)) { total += c; }
                    }
                }
            }
        }
    }
    println!("{}", total);
}
