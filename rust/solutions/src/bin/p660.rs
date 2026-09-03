// Project Euler 660 - Pandigital Triangles
// 120-degree triangles with pandigital sides in bases 9-18.
// Optimized: const-generic fast digit extraction (zero division instructions via compiler magic)
// and fine-grained rayon parallelism over (base, n) subtasks.

use rayon::prelude::*;
use std::collections::HashSet;

#[inline(always)]
fn gcd32(mut a: u32, mut b: u32) -> u32 {
    if a == 0 { return b; }
    if b == 0 { return a; }
    let shift = (a | b).trailing_zeros();
    a >>= a.trailing_zeros();
    loop {
        b >>= b.trailing_zeros();
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        b -= a;
        if b == 0 { break; }
    }
    a << shift
}

#[inline(always)]
fn add_digits<const BASE: u32>(mut x: u32, mask: &mut u32) -> bool {
    if x == 0 {
        if *mask & 1 != 0 { return false; }
        *mask |= 1;
        return true;
    }
    while x > 0 {
        let d = x % BASE;
        let bit = 1u32 << d;
        if *mask & bit != 0 { return false; }
        *mask |= bit;
        x /= BASE;
    }
    true
}

#[inline(always)]
fn is_pandigital_const<const BASE: u32>(a: u32, b_val: u32, c: u32) -> bool {
    let mut mask: u32 = 0;
    add_digits::<BASE>(a, &mut mask)
        && add_digits::<BASE>(b_val, &mut mask)
        && add_digits::<BASE>(c, &mut mask)
        && mask == (1u32 << BASE) - 1
}

#[inline(always)]
fn check_pandigital(base: u32, a: u32, b_val: u32, c: u32) -> bool {
    match base {
        9 => is_pandigital_const::<9>(a, b_val, c),
        10 => is_pandigital_const::<10>(a, b_val, c),
        11 => is_pandigital_const::<11>(a, b_val, c),
        12 => is_pandigital_const::<12>(a, b_val, c),
        13 => is_pandigital_const::<13>(a, b_val, c),
        14 => is_pandigital_const::<14>(a, b_val, c),
        15 => is_pandigital_const::<15>(a, b_val, c),
        16 => is_pandigital_const::<16>(a, b_val, c),
        17 => is_pandigital_const::<17>(a, b_val, c),
        18 => is_pandigital_const::<18>(a, b_val, c),
        _ => false,
    }
}

fn ceil_div(a: i32, b: i32) -> i32 { (a + b - 1) / b }

fn main() {
    let all_results: Vec<(i64, i64, i64)> = (9..=18u32)
        .into_par_iter()
        .flat_map(|base| {
            let e1 = ceil_div(base as i32, 3);
            let mut limit = 1i64;
            for _ in 0..e1 { limit *= base as i64; }
            let mut limit2 = 1i64;
            for _ in 0..e1 - 1 { limit2 *= base as i64; }
            limit += limit2;

            let max_n = (limit as f64).sqrt() as i64;
            (1..max_n + 1).into_par_iter().flat_map_iter(move |n| {
                let mut results = Vec::new();
                let n2 = n * n;
                for m in n + 1..2 * n {
                    let ls1 = m * m - m * n + n2;
                    if ls1 > limit { break; }
                    if (m + n) % 3 == 0 { continue; }
                    if gcd32(m as u32, n as u32) != 1 { continue; }

                    let m2 = m * m;
                    let diff = m2 - n2;
                    let m_2n_m = m * (2 * n - m);

                    for k in 1.. {
                        let c = k * ls1;
                        if c > limit { break; }
                        let a = k * diff;
                        let b_val = k * m_2n_m;
                        if check_pandigital(base, a as u32, b_val as u32, c as u32) {
                            results.push((a, b_val, c));
                        }
                    }
                }
                results
            })
        })
        .collect();

    let mut seen = HashSet::new();
    let mut total = 0i64;
    for (a, b, c) in all_results {
        if seen.insert((a, b, c)) {
            total += c;
        }
    }
    println!("{}", total);
}
