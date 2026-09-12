// Project Euler 557 - Cutting a Triangle
//
// A triangle has integer area S. A cevian and a line parallel to one side
// divide it into four regions with integer areas a, b, c, d.
// Find sum of S for all valid (a,b,c,d) with S <= 10000.
//
// Skip coprime (a,S) (no admissible d). Integer isqrt. Binary gcd.

use rayon::prelude::*;

#[inline(always)]
fn gcd(mut a: i64, mut b: i64) -> i64 {
    if a == 0 {
        return b.abs();
    }
    if b == 0 {
        return a.abs();
    }
    let shift = (a.abs() | b.abs()).trailing_zeros();
    a = a.abs() >> a.abs().trailing_zeros();
    b = b.abs();
    loop {
        b >>= b.trailing_zeros();
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        b -= a;
        if b == 0 {
            return a << shift;
        }
    }
}

#[inline(always)]
fn contrib(a: i64, n: i64) -> i64 {
    let mut ans: i64 = 0;
    let a2 = a * a;
    for s in (a + 3)..=n {
        // gcd(a^2, a+S) > 1 iff gcd(a, S) > 1.
        if gcd(a, s) == 1 {
            continue;
        }
        let aps = a + s;
        let g = gcd(a2, aps);
        let mult = aps / g;

        let sa = s - a;
        let base = a2 / g;
        let mut k_max = (sa - 2) / mult;
        if base > 0 {
            let k_disc = sa * sa / (4 * base);
            if k_disc < k_max {
                k_max = k_disc;
            }
        }
        if k_max <= 0 {
            continue;
        }

        let mut k = 1i64;
        while k <= k_max {
            let d = k * mult;
            let bc = base * k;
            let bpc = sa - d;
            if bpc < 2 {
                break;
            }
            let disc = bpc * bpc - 4 * bc;
            if disc >= 0 {
                let sq = disc.isqrt();
                if sq * sq == disc && ((bpc + sq) & 1) == 0 {
                    if (bpc - sq) >= 2 {
                        ans += s;
                    }
                }
            }
            k += 1;
        }
    }
    ans
}

fn main() {
    let n: i64 = 10_000;
    // a=1 does far more work than a near n; max_len=1 lets rayon steal.
    let ans: i64 = (1..n as usize)
        .into_par_iter()
        .with_max_len(1)
        .map(|a| contrib(a as i64, n))
        .sum();
    println!("{ans}");
}
