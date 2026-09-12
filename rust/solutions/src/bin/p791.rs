// Project Euler 791 - Average and Variance
// S(n) = sum of (a+b+c+d) over ordered quadruples where average = 2*variance.
// O(sqrt(N)) algorithm with modular arithmetic.

use rayon::prelude::*;

const MOD: u64 = 433_494_437; // MOD^2 < 2^64
const N: i64 = 100_000_000;

fn powmod(mut base: u64, mut exp: u64) -> u64 {
    let mut result = 1u64;
    base %= MOD;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % MOD;
        }
        base = base * base % MOD;
        exp >>= 1;
    }
    result
}

#[inline]
fn isqrt(n: i64) -> i64 {
    (n as u64).isqrt() as i64
}

#[inline]
fn closed_sum(g: i64, h_val: i64, inv3: u64) -> u64 {
    if h_val < 0 {
        return 0;
    }
    let gm = g as u64;
    let hm = h_val as u64;
    let hp1 = hm + 1;
    let t1 = 2 * gm % MOD * gm % MOD * hp1 % MOD * hp1 % MOD;
    let t2 = hm * hp1 % MOD * (2 * hm + 1) % MOD * (2 * hm + 3) % MOD * inv3 % MOD;
    (t1 + t2) % MOD
}

#[inline]
fn sum_sq_to(nn: i64, inv6: u64) -> u64 {
    if nn < 0 {
        return 0;
    }
    let nm = nn as u64;
    nm * (nm + 1) % MOD * (2 * nm + 1) % MOD * inv6 % MOD
}

#[inline]
fn sum_sq_range(a: i64, b: i64, inv6: u64) -> u64 {
    if a > b {
        return 0;
    }
    if a >= 0 {
        (sum_sq_to(b, inv6) + MOD - sum_sq_to(a - 1, inv6)) % MOD
    } else if b < 0 {
        (sum_sq_to(-a, inv6) + MOD - sum_sq_to(-b - 1, inv6)) % MOD
    } else {
        (sum_sq_to(b, inv6) + sum_sq_to(-a, inv6)) % MOD
    }
}

fn contrib_g(g: i64, inv3: u64, inv6: u64) -> u64 {
    let g2 = g * g;
    let t_full = 2 * N - g2 - g;
    if t_full < 0 {
        return 0;
    }

    let mut h_full = (-1 + isqrt(1 + 2 * t_full)) / 2;
    if h_full > g {
        h_full = g;
    }

    let mut ans = closed_sum(g, h_full, inv3);

    let mut h_any = (-1 + isqrt(1 + 4 * t_full)) / 2;
    if h_any > g {
        h_any = g;
    }

    for h in (h_full + 1)..=h_any {
        let h2 = h * h;
        let t_val = 2 * N - g2 - h2 - g - h;
        if t_val < 0 {
            break;
        }

        let s = isqrt(1 + 4 * t_val);
        let mut r_hi = (-1 + s) / 2;
        if r_hi > h {
            r_hi = h;
        }

        let r_neg_max = (1 + s) / 2;
        let mut r_lo = -h;
        if r_lo < -r_neg_max {
            r_lo = -r_neg_max;
        }

        if r_lo > r_hi {
            continue;
        }

        let cnt = (r_hi - r_lo + 1) as u64;
        let sr = sum_sq_range(r_lo, r_hi, inv6);
        let gh2 = (g2 + h2) as u64;
        let contrib = (2 * sr + 2 * cnt % MOD * gh2) % MOD;
        ans += contrib;
    }

    ans
}

fn main() {
    let inv3 = powmod(3, MOD - 2);
    let inv6 = powmod(6, MOD - 2);

    let mut g_max = isqrt(2 * N);
    while g_max * (g_max + 1) > 2 * N {
        g_max -= 1;
    }

    // RangeInclusive is not IndexedParallelIterator; use 0..g_max+1.
    let ans: u64 = (0..g_max as usize + 1)
        .into_par_iter()
        .map(|g| contrib_g(g as i64, inv3, inv6))
        .sum();

    let ans = (ans % MOD + MOD - 12) % MOD;
    println!("{}", ans);
}
