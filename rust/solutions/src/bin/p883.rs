// Project Euler 883
// Lattice point counting with GCD constraints and hexagonal geometry.

use rayon::prelude::*;

const N: i64 = 1_000_000;
const TWO_N: i32 = 2_000_000;
// n_val <= HEAVY is split into geometric md chunks (n_val=1 is ~14% of work).
const HEAVY: i32 = 256;

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

#[inline(always)]
fn isqrt(n: i64) -> i64 {
    (n as u64).isqrt() as i64
}

/// Sum of y-counts for x=1..=m in the disc >= 3x-2 region (max_y = 2x-1).
#[inline(always)]
fn phase1_sum(m: i64, mod3: i32, mult: i64) -> i64 {
    if m <= 0 {
        return 0;
    }
    let s = if mod3 == 0 {
        let min_sum = if m % 2 == 0 {
            let t = m / 2;
            t * (t - 1)
        } else {
            let t = (m - 1) / 2;
            t * t
        };
        m * m - min_sum
    } else {
        if m % 2 == 0 {
            let t = m / 2;
            t * t
        } else {
            let t = (m - 1) / 2;
            t * (t + 1)
        }
    };
    s * mult
}

/// Largest x in 1..=x_max with isqrt(q - 3x^2) >= 3x - 2.
#[inline(always)]
fn x_phase1(q: i64, x_max: i64) -> i64 {
    if q < 4 || x_max <= 0 {
        return 0;
    }
    let lim = q - 4;
    let mut x = isqrt(lim / 12) + 1;
    if x > x_max {
        x = x_max;
    }
    while x > 0 && 12 * x * (x - 1) > lim {
        x -= 1;
    }
    while x < x_max && 12 * (x + 1) * x <= lim {
        x += 1;
    }
    x
}

fn process_md_range(n_val: i32, md_lo: i32, md_hi: i32, num: i64) -> i64 {
    let mut ans = 0i64;
    for md in md_lo..=md_hi {
        if n_val > 1 && gcd(n_val, md) != 1 {
            continue;
        }
        let m = n_val + md;
        let den = n_val as i64 * md as i64;
        let q = num / (den * den);
        if q < 3 {
            continue;
        }
        let x_max = isqrt(q / 3);
        let mult: i64 = if m + n_val == 3 { 2 } else { 6 };
        let mod3 = (n_val + m) % 3;
        let x1 = x_phase1(q, x_max);
        ans += phase1_sum(x1, mod3, mult);

        if x1 >= x_max {
            continue;
        }
        let mut x = x1 + 1;
        let mut disc = isqrt(q - 3 * x * x);
        while x <= x_max {
            let disc_sq = q - 3 * x * x;
            while disc * disc > disc_sq {
                disc -= 1;
            }
            // Phase 2: disc < 3x-2 ⇒ max_y = (x+disc)/2, min_y = (x-1)/2.
            let mut min_y = (x - 1) / 2;
            let mut max_y = (x + disc) / 2;
            if mod3 > 0 {
                min_y = (x + min_y) / 3;
                max_y = (x + max_y) / 3;
            }
            if max_y > min_y {
                ans += (max_y - min_y) * mult;
            }
            x += 1;
        }
    }
    ans
}

fn main() {
    let num = 16 * N * N;

    let (heavy, light) = rayon::join(
        || {
            let mut units = Vec::new();
            for n_val in 1..=HEAVY {
                let max_md = TWO_N / n_val;
                let mut md = 1i32;
                while md <= max_md {
                    let md_hi = md.saturating_mul(2).saturating_sub(1).min(max_md);
                    units.push((n_val, md, md_hi));
                    md = md_hi + 1;
                }
            }
            units
                .into_par_iter()
                .map(|(n_val, lo, hi)| process_md_range(n_val, lo, hi, num))
                .sum::<i64>()
        },
        || {
            (HEAVY + 1..TWO_N + 1)
                .into_par_iter()
                .map(|n_val| process_md_range(n_val, 1, TWO_N / n_val, num))
                .sum::<i64>()
        },
    );
    println!("{}", heavy + light);
}
