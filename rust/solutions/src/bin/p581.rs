// Project Euler 581 - 47-smooth Triangular Numbers
//
// Find the sum of all n such that T(n) = n(n+1)/2 is 47-smooth.
// Uses Stormer's theorem: solve Pell equations x^2 - 2q*y^2 = 1
// for each product q of a subset of odd primes <= 47.

use std::collections::HashSet;

const PRIMES: [i64; 15] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];

fn is_square(n: i64) -> bool {
    if n < 0 { return false; }
    let r = (n as f64).sqrt() as i64;
    for s in (r - 1).max(0)..=r + 1 {
        if s * s == n { return true; }
    }
    false
}

fn is_smooth(mut n: i128) -> bool {
    if n <= 1 { return n == 1; }
    for &p in &PRIMES {
        let p = p as i128;
        while n % p == 0 { n /= p; }
    }
    n == 1
}

fn solve_pell(d: i64) -> Vec<i128> {
    if is_square(d) { return vec![]; }

    let a0 = (d as f64).sqrt() as i64;
    let (mut m, mut den, mut a): (i64, i64, i64) = (0, 1, a0);
    let (mut h_prev, mut k_prev): (i128, i128) = (1, 0);
    let (mut h, mut k): (i128, i128) = (a0 as i128, 1);

    let mut x0: i128 = 0;
    let mut y0: i128 = 0;
    let mut found = false;

    for _ in 0..100000 {
        if h * h - (d as i128) * k * k == 1 {
            x0 = h;
            y0 = k;
            found = true;
            break;
        }
        m = den * a - m;
        den = (d as i64 - m * m) / den;
        a = (a0 + m) / den;

        let h_next = (a as i128) * h + h_prev;
        let k_next = (a as i128) * k + k_prev;
        h_prev = h; k_prev = k;
        h = h_next; k = k_next;
    }

    if !found { return vec![]; }

    let limit: i128 = 1i128 << 64;
    let mut results = vec![x0];
    let (mut x, mut y) = (x0, y0);

    for _ in 1..24 {
        let x_next = x0 * x + (d as i128) * y0 * y;
        let y_next = x0 * y + y0 * x;
        x = x_next;
        y = y_next;
        if x > limit { break; }
        results.push(x);
    }

    results
}

fn main() {
    let mut found: HashSet<i128> = HashSet::new();

    for subset in 0..(1u32 << 15) {
        if subset == 1 { continue; } // skip subset containing only 2

        let mut q: i64 = 1;
        let mut overflow = false;
        for i in 0..15 {
            if subset & (1 << i) != 0 {
                q *= PRIMES[i];
                if q > 1_000_000_000_000_000 {
                    overflow = true;
                    break;
                }
            }
        }
        if overflow { continue; }

        let d = 2 * q;
        let xs = solve_pell(d);

        for x in xs {
            let b = x >> 1;
            if b > 0 && is_smooth(b) && is_smooth(b + 1) {
                found.insert(b);
            }
        }
    }

    let total: i128 = found.iter().sum();
    println!("{}", total);
}
