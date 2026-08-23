// Project Euler 542 - Geometric Progression with Alternating Sum
//
// S(n) = maximum sum of a geometric progression with >= 3 distinct terms <= n.
// Find sum_{k=4}^{10^17} (-1)^k S(k) using divide-and-conquer.

use rayon::join;

/// Binary exponentiation; 0 is the overflow/invalid sentinel (p^e is never 0 for p>=2).
#[inline(always)]
fn ipow(mut base: u64, mut exp: u32) -> u64 {
    let mut acc = 1u64;
    loop {
        if exp & 1 != 0 {
            let (v, ov) = acc.overflowing_mul(base);
            if ov {
                return 0;
            }
            acc = v;
        }
        exp >>= 1;
        if exp == 0 {
            return acc;
        }
        let (v, ov) = base.overflowing_mul(base);
        if ov {
            return 0;
        }
        base = v;
    }
}

#[inline(always)]
fn iroot(n: u64, k: u32) -> u64 {
    match k {
        1 => n,
        2 => n.isqrt(),
        _ => {
            if n < 2 {
                return n;
            }
            let mut x = (n as f64).powf(1.0 / k as f64) as u64;
            if x < 1 {
                x = 1;
            }
            loop {
                let p = ipow(x, k);
                if p != 0 && p <= n {
                    break;
                }
                x -= 1;
                if x == 0 {
                    return 0;
                }
            }
            loop {
                let p = ipow(x + 1, k);
                if p != 0 && p <= n {
                    x += 1;
                } else {
                    break;
                }
            }
            x
        }
    }
}

fn s(n: i64) -> i64 {
    if n < 3 {
        return 0;
    }
    let n = n as u64;
    let mut max_s = 0u64;
    let max_e = 63 - n.leading_zeros();

    for e in (2..=max_e).rev() {
        let p_max = iroot(n, e);
        if p_max < 2 {
            continue;
        }
        // Length-(e+1) GP sum is < min(e+1, p_max)*n.
        let ub = (e as u64 + 1).min(p_max);
        if ub.saturating_mul(n) <= max_s {
            if p_max >= e as u64 + 1 {
                break;
            }
            continue;
        }

        let mut prev_pe = 1u64;
        for p in 2..=p_max {
            let pe = ipow(p, e);
            if pe == 0 || pe > n {
                break;
            }
            let r = n / pe;
            let diff = p as u128 * pe as u128 - (p - 1) as u128 * prev_pe as u128;
            let sum_val = (diff * r as u128) as u64;
            if sum_val > max_s {
                max_s = sum_val;
            }
            prev_pe = pe;
        }
    }
    max_s as i64
}

const JOIN_SPAN: i64 = 4096;

fn t(low: i64, high: i64, s_low: i64, s_high: i64) -> i64 {
    if s_low == s_high {
        let count = high - low;
        if count & 1 == 0 {
            0
        } else if low & 1 == 0 {
            s_low
        } else {
            -s_low
        }
    } else if high - low == 1 {
        if low & 1 == 0 { s_low } else { -s_low }
    } else {
        let mid = (low + high) >> 1;
        let s_mid = s(mid);
        if high - low >= JOIN_SPAN {
            let (a, b) = join(
                || t(low, mid, s_low, s_mid),
                || t(mid, high, s_mid, s_high),
            );
            a + b
        } else {
            t(low, mid, s_low, s_mid) + t(mid, high, s_mid, s_high)
        }
    }
}

fn main() {
    let n: i64 = 100_000_000_000_000_000; // 10^17
    println!("{}", t(4, n + 1, s(4), s(n + 1)));
}
