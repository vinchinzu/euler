// Project Euler 932 — 2025
// 2025 = (20+25)^2. ab is a "2025-number" if ab = (a+b)^2.
// T(n) = sum of all 2025-numbers with n digits or less. T(4) = 5131.
// Find T(16).
// Expected: 72673459417881349

use rayon::prelude::*;

#[inline(always)]
fn isqrt64(n: u64) -> u64 {
    if n < 2 { return n; }
    if n < (1u64 << 52) {
        (n as f64).sqrt() as u64
    } else {
        let mut x = (n as f64).sqrt() as u64;
        x = (x + n / x) >> 1;
        x
    }
}

/// Process a band of b values where all b in [b_lo, b_hi) have exactly `n` digits.
/// pow10n = 10^n, pow10_2n = 10^(2n), max_digits is the overall digit limit.
fn process_band(b_lo: u64, b_hi: u64, _n: u32, pow10n: u64, pow10_2n: u64, max_digits: u32) -> u64 {
    let pow10n_minus1 = pow10n - 1;
    let max_val = 10u64.pow(max_digits);
    let four_times = pow10n_minus1 << 2;

    let mut total: u64 = 0;

    for b in b_lo..b_hi {
        let term = b * four_times;
        if term >= pow10_2n { continue; }
        let disc = pow10_2n - term;

        let v = isqrt64(disc);
        if v * v != disc { continue; }

        let twice_b = b << 1;
        let base = pow10n - twice_b;

        let num1 = base + v;
        if num1 & 1 == 0 {
            let a = num1 >> 1;
            if a > 0 {
                let val = a * pow10n + b;
                if val < max_val {
                    total += val;
                }
            }
        }

        if v <= base {
            let num2 = base - v;
            if num2 & 1 == 0 {
                let a = num2 >> 1;
                if a > 0 {
                    let val = a * pow10n + b;
                    if val < max_val {
                        total += val;
                    }
                }
            }
        }
    }

    total
}

fn compute(max_digits: u32) -> u64 {
    let half_digits = max_digits / 2;

    // Build work units: one per digit-band, chunked for parallelism
    let chunk_size: u64 = 500_000; // ~500K per chunk for good load balancing
    let mut work_units: Vec<(u64, u64, u32, u64, u64)> = Vec::new();

    for n in 1..=half_digits {
        let pow10n = 10u64.pow(n);
        let pow10_2n = pow10n * pow10n;
        let b_lo = if n == 1 { 1 } else { 10u64.pow(n - 1) };
        let b_hi = pow10n;

        let mut start = b_lo;
        while start < b_hi {
            let end = (start + chunk_size).min(b_hi);
            work_units.push((start, end, n, pow10n, pow10_2n));
            start = end;
        }
    }

    work_units.par_iter()
        .map(|&(b_lo, b_hi, n, pow10n, pow10_2n)| {
            process_band(b_lo, b_hi, n, pow10n, pow10_2n, max_digits)
        })
        .sum()
}

fn main() {
    debug_assert_eq!(compute(4), 5131);
    println!("{}", compute(16));
}
