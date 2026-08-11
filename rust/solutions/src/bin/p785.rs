// Project Euler 785 - Binary Quadratic Diophantine
// Parameterization with two cases based on (m+n) mod 3.
// Optimized: rayon over outer m loops.

use rayon::prelude::*;

#[inline(always)]
fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

#[inline(always)]
fn case1_m(m: i64, n: i64) -> i64 {
    if 95 * m * m > n {
        return 0;
    }
    let mut ans = 0i64;
    let mut nv = m + 1;
    loop {
        let z = (14 * m + 5 * nv) * (4 * m + nv);
        if z > n {
            break;
        }
        if m % 19 != nv % 19 && gcd(m, nv) == 1 && (m + nv) % 3 != 0 {
            ans += 8 * (13 * m * m + 5 * m * nv + nv * nv);
        }
        nv += 1;
    }
    ans
}

#[inline(always)]
fn case2_m(m: i64, n: i64) -> i64 {
    if 95 * m * m > 9 * n {
        return 0;
    }
    let mut ans = 0i64;
    let rem = (3 - (2 * m + 1) % 3) % 3;
    let mut nv = m + 1 + rem;
    loop {
        let z = (14 * m + 5 * nv) * (4 * m + nv);
        if z > 9 * n {
            break;
        }
        if m % 19 != nv % 19 && gcd(m, nv) == 1 {
            ans += 8 * (13 * m * m + 5 * m * nv + nv * nv) / 9;
        }
        nv += 3;
    }
    ans
}

fn main() {
    let n: i64 = 1_000_000_000;

    let m_max1 = ((n as f64 / 95.0).sqrt() as i64) + 1;
    let part1: i64 = (1..=m_max1).into_par_iter().map(|m| case1_m(m, n)).sum();

    let m_max2 = ((9.0 * n as f64 / 95.0).sqrt() as i64) + 1;
    let part2: i64 = (1..=m_max2).into_par_iter().map(|m| case2_m(m, n)).sum();

    println!("{}", part1 + part2);
}
