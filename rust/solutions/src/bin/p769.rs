// Project Euler 769 - Binary Quadratic Form
// Count representations of z^2 as x^2+5xy+3y^2 with z <= N using Mobius function.

use rayon::prelude::*;

const N: i64 = 100_000_000_000_000; // 10^14

const INV_SQRT3: u128 = 10650232656628343401u128;

/// floor(n / sqrt(3)) = isqrt(n^2 / 3) for n > 0.
#[inline(always)]
fn floor_n_div_sqrt3(n: i64) -> i64 {
    ((n as u128 * INV_SQRT3) >> 64) as i64
}

#[inline(always)]
fn fast_isqrt(x: u64) -> i64 {
    let mut s = (x as f64).sqrt() as u64;
    if s * s > x {
        s -= 1;
    } else if (s + 1) * (s + 1) <= x {
        s += 1;
    }
    s as i64
}

/// term1 <= term2 iff 25 (n g)^4 <= 3 (h N - 2 (n g)^2)^2.
#[inline(always)]
fn t1_le_t2(n: i64, g: i64, h: i64) -> bool {
    let ng = n * g;
    let ng2 = ng * ng;
    let rhs_in = h * N - 2 * ng2;
    if rhs_in <= 0 {
        return false;
    }
    let ng2 = ng2 as i128;
    25 * ng2 * ng2 <= 3 * (rhs_in as i128) * (rhs_in as i128)
}

/// Last n in 0..=n_max for which floor(n/sqrt(3)) <= term2.
#[inline(always)]
fn n_cross(g: i64, h: i64, n_max: i64) -> i64 {
    // n_cross / n_max ≈ sqrt((5*sqrt(3)-6)/13) ≈ 0.452365
    let mut n = ((n_max as f64) * 0.452365_f64) as i64;
    if n > n_max {
        n = n_max;
    }
    while n < n_max && t1_le_t2(n + 1, g, h) {
        n += 1;
    }
    while n > 0 && !t1_le_t2(n, g, h) {
        n -= 1;
    }
    n
}

/// MODE 0: sum max_m
/// MODE 1: sum max_m - (max_m + (3n)%13) / 13
/// MODE 2: sum (max_m + (3n)%13) / 13
#[inline(always)]
fn add_mode<const MODE: u8>(acc: &mut i64, max_m: i64, rem: i64) {
    if MODE == 0 {
        *acc += max_m;
    } else if MODE == 1 {
        *acc += max_m - (max_m + rem) / 13;
    } else {
        *acc += (max_m + rem) / 13;
    }
}

fn inner<const MODE: u8>(g: i64, h: i64) -> i64 {
    let g_sq = g * g;
    let n_max = ((h * N) as u64 / g_sq as u64).isqrt() as i64;
    if n_max <= 0 {
        return 0;
    }

    let split = n_cross(g, h, n_max);
    let mut acc = 0i64;

    // n = 1..=split: max_m = floor(n / sqrt(3))
    if MODE == 0 {
        for n in 1..=split {
            acc += floor_n_div_sqrt3(n);
        }
    } else {
        let mut rem = 3i64; // (3*n) % 13 at n = 1
        for n in 1..=split {
            add_mode::<MODE>(&mut acc, floor_n_div_sqrt3(n), rem);
            rem += 3;
            if rem >= 13 {
                rem -= 13;
            }
        }
    }

    // n = split+1..=n_max: max_m = floor((sqrt(13 (ng)^2 + 12 h N) - 5 ng) / (6 g))
    let c = 12 * h * N;
    let six_g = 6 * g;
    let mut rem = (3 * (split + 1)).rem_euclid(13);
    let mut ng = (split + 1) * g;
    for _n in (split + 1)..=n_max {
        let s = fast_isqrt((13 * ng * ng + c) as u64);
        let t2 = (s - 5 * ng) / six_g;
        add_mode::<MODE>(&mut acc, t2, rem);
        rem += 3;
        if rem >= 13 {
            rem -= 13;
        }
        ng += g;
    }
    acc
}

fn process_g(g: i64, mu: i64) -> i64 {
    if g % 13 == 0 {
        mu * inner::<0>(g, 13)
    } else {
        mu * (inner::<1>(g, 1) + inner::<2>(g, 13))
    }
}

fn main() {
    let sqrt_n = (N as u64).isqrt() as usize;

    let mut mobius = vec![1i8; sqrt_n + 1];
    let mut is_prime = vec![true; sqrt_n + 1];
    is_prime[0] = false;
    if sqrt_n >= 1 {
        is_prime[1] = false;
    }

    for i in 2..=sqrt_n {
        if is_prime[i] {
            for j in (i..=sqrt_n).step_by(i) {
                if j > i {
                    is_prime[j] = false;
                }
                if (j / i) % i == 0 {
                    mobius[j] = 0;
                } else {
                    mobius[j] = -mobius[j];
                }
            }
        }
    }
    drop(is_prime);

    // Iterate g from large to small so work-stealing splits off the expensive small-g tail.
    let ans: i64 = (0..sqrt_n)
        .into_par_iter()
        .with_min_len(1)
        .map(|i| {
            let g = sqrt_n - i;
            // SAFETY: g is in 1..=sqrt_n
            let mu = unsafe { *mobius.get_unchecked(g) } as i64;
            if mu == 0 { 0 } else { process_g(g as i64, mu) }
        })
        .sum();

    println!("{}", ans);
}
