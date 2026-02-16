// Project Euler 730 - Shifted Pythagorean Triples
//
// DFS using Barning matrices on primitive triples, parallelized over k.

use rayon::prelude::*;

const LIMIT: i64 = 100_000_000;
const K_MAX: usize = 100;
const L: usize = 200;

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn count_for_k(k: usize) -> i64 {
    let mut used = [false; L * L]; // flat 2D on stack (40KB)
    let mut count: i64 = 0;
    let mut stack: Vec<(i64, i64, i64)> = Vec::new();

    // Find seed triples for this k
    for p in 1..L as i64 {
        for q in p..L as i64 {
            let r2 = p * p + q * q + k as i64;
            let mut r = (r2 as f64).sqrt() as i64;
            while r * r < r2 {
                r += 1;
            }
            if r * r != r2 {
                continue;
            }
            if p + q + r > LIMIT {
                continue;
            }
            if gcd(gcd(p, q), r) != 1 {
                continue;
            }
            stack.push((p, q, r));
        }
    }

    // DFS with explicit stack
    while let Some((a_raw, b_raw, c)) = stack.pop() {
        if a_raw + b_raw + c > LIMIT {
            continue;
        }

        let (a, b) = if a_raw <= b_raw {
            (a_raw, b_raw)
        } else {
            (b_raw, a_raw)
        };

        if (a as usize) < L && (b as usize) < L {
            let idx = a as usize * L + b as usize;
            // SAFETY: a < L and b < L guaranteed by check above, so idx < L*L
            unsafe {
                if *used.get_unchecked(idx) {
                    continue;
                }
                *used.get_unchecked_mut(idx) = true;
            }
        }

        count += 1;

        // Barning matrix transformations (generate 3 children)
        stack.push((
            a - 2 * b + 2 * c,
            2 * a - b + 2 * c,
            2 * a - 2 * b + 3 * c,
        ));
        stack.push((
            a + 2 * b + 2 * c,
            2 * a + b + 2 * c,
            2 * a + 2 * b + 3 * c,
        ));
        if a != b {
            stack.push((
                -a + 2 * b + 2 * c,
                -2 * a + b + 2 * c,
                -2 * a + 2 * b + 3 * c,
            ));
        }
    }

    count
}

fn main() {
    let total: i64 = (0..=K_MAX).into_par_iter().map(|k| count_for_k(k)).sum();
    println!("{}", total);
}
