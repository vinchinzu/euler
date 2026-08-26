// Project Euler 880 - Nested radicals (Fermat cubes)

use rayon::prelude::*;

const N: u64 = 1_000_000_000_000_000; // 10^15
const M: u64 = 1_095_912_793; // 1031^3 + 2

#[inline(always)]
fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

#[inline(always)]
fn icbrt(n: u64) -> u64 {
    if n < 8 {
        return u64::from(n != 0);
    }
    let mut x = (n as f64).cbrt() as u64;
    loop {
        let x3 = x * x * x;
        if x3 > n {
            x -= 1;
            continue;
        }
        let y = x + 1;
        if y * y * y <= n {
            x = y;
            continue;
        }
        return x;
    }
}

fn iroot4(n: u64) -> u64 {
    let mut r = n.isqrt().isqrt();
    loop {
        let p2 = r.saturating_mul(r);
        let p4 = p2.saturating_mul(p2);
        if p4 > n {
            r -= 1;
            continue;
        }
        let n1 = r + 1;
        let q2 = n1.saturating_mul(n1);
        let q4 = q2.saturating_mul(q2);
        if q4 <= n {
            r = n1;
            continue;
        }
        return r;
    }
}

/// Cube-free kernel: product p^(v_p(n) mod 3).
fn cube_free_table(limit: usize) -> Vec<u32> {
    let mut spf: Vec<u32> = (0..=limit as u32).collect();
    let mut p = 2usize;
    while p * p <= limit {
        if spf[p] == p as u32 {
            let mut m = p * p;
            while m <= limit {
                if spf[m] == m as u32 {
                    spf[m] = p as u32;
                }
                m += p;
            }
        }
        p += 1;
    }
    let mut cf = vec![1u32; limit + 1];
    for n in 2..=limit {
        let p = spf[n] as usize;
        let mut m = n / p;
        let mut e = 1u32;
        while m % p == 0 {
            m /= p;
            e += 1;
        }
        cf[n] = match e % 3 {
            0 => cf[m],
            1 => cf[m] * p as u32,
            _ => cf[m] * p as u32 * p as u32,
        };
    }
    cf
}

#[inline(always)]
fn pair_contrib(x: u64, y: u64) -> u64 {
    if y == 0 || y > N {
        return 0;
    }
    let maxc = if x > y { x } else { y };
    let t = (N / maxc).isqrt();
    if t == 0 {
        return 0;
    }
    let t128 = t as u128;
    let sg = t128 * (t128 + 1) * (2 * t128 + 1) / 6;
    ((x + y) as u128 * sg % M as u128) as u64
}

fn process_odd_b(b: u64, cf_b: u32, cf4: &[u32]) -> u64 {
    let cb = icbrt(N / b);
    if cb <= b {
        return 0;
    }
    let a_limit = (cb - b) / 4;
    if a_limit == 0 {
        return 0;
    }
    let b32 = b as u32;
    let mut acc = 0u64;
    for a in 1..=a_limit {
        if gcd(a as u32, b32) != 1 || cf_b == cf4[a as usize] {
            continue;
        }
        let x_base = b + 4 * a;
        let x = x_base * x_base * x_base * b;
        let yb = (a as i64 - 2 * b as i64).unsigned_abs();
        let y = 4 * a * yb * yb * yb;
        acc += pair_contrib(x, y);
    }
    acc
}

fn process_even_b(b: u64, cf_2b: u32, cf: &[u32]) -> u64 {
    let half = b / 2;
    let cb = icbrt(N / (2 * b));
    if cb <= half {
        return 0;
    }
    let a_limit = (cb - half) / 2;
    if a_limit == 0 {
        return 0;
    }
    let b32 = b as u32;
    let mut acc = 0u64;
    let mut a = 1u64;
    while a <= a_limit {
        if gcd(a as u32, b32) == 1 && cf_2b != cf[a as usize] {
            let x_base = half + 2 * a;
            let x = 2 * b * x_base * x_base * x_base;
            let yb = (a as i64 - 2 * b as i64).unsigned_abs();
            let y = a * yb * yb * yb;
            acc += pair_contrib(x, y);
        }
        a += 2;
    }
    acc
}

fn main() {
    let b_limit = iroot4(4 * N);
    let max_odd_a = {
        let c = icbrt(N);
        if c > 1 { (c - 1) / 4 } else { 0 }
    };
    let max_even_a = {
        let c = icbrt(N / 4);
        if c > 1 { (c - 1) / 2 } else { 0 }
    };
    let cf_limit = (4 * max_odd_a)
        .max(max_even_a)
        .max(2 * b_limit) as usize;
    let cf = cube_free_table(cf_limit);

    let mut cf4 = vec![0u32; max_odd_a as usize + 1];
    for a in 1..=max_odd_a as usize {
        cf4[a] = cf[4 * a];
    }

    let n_odd = ((b_limit + 1) / 2) as usize;
    let n_even = (b_limit / 2) as usize;

    let (odd, even) = rayon::join(
        || {
            (0..n_odd)
                .into_par_iter()
                .with_min_len(1)
                .map(|i| {
                    let b = 2 * i as u64 + 1;
                    process_odd_b(b, cf[b as usize], &cf4)
                })
                .sum::<u64>()
        },
        || {
            (1..n_even + 1)
                .into_par_iter()
                .with_min_len(1)
                .map(|i| {
                    let b = 2 * i as u64;
                    process_even_b(b, cf[2 * b as usize], &cf)
                })
                .sum::<u64>()
        },
    );

    println!("{}", (odd + even) % M);
}
