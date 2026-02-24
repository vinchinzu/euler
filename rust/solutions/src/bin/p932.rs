// Project Euler 932 — 2025
// 2025 = (20+25)^2. ab is a "2025-number" if ab = (a+b)^2.
// T(n) = sum of all 2025-numbers with n digits or less. T(4) = 5131.
// Find T(16).
// Expected: 72673459417881349

use rayon::prelude::*;

#[inline(always)]
fn isqrt64(n: u64) -> u64 {
    if n == 0 { return 0; }
    let mut x = (n as f64).sqrt() as u64;
    // Newton's refinement
    loop {
        let x1 = (x + n / x) / 2;
        if x1 >= x { break; }
        x = x1;
    }
    // Ensure exact
    while x * x > n { x -= 1; }
    while (x + 1) * (x + 1) <= n { x += 1; }
    x
}

/// Process a band of b values where all b in [b_lo, b_hi) have exactly `n` digits.
/// pow10n = 10^n, pow10_2n = 10^(2n), max_digits is the overall digit limit.
fn process_band(b_lo: u64, b_hi: u64, _n: u32, pow10n: u64, pow10_2n: u64, max_digits: u32) -> u64 {
    let pow10n_minus1 = pow10n - 1;
    let max_val = 10u64.pow(max_digits); // values must be < 10^max_digits

    let mut total: u64 = 0;

    for b in b_lo..b_hi {
        // disc = 10^(2n) - 4*b*(10^n - 1)
        let term = 4 * b * pow10n_minus1;
        if pow10_2n <= term { continue; }
        let disc = pow10_2n - term;

        let v = isqrt64(disc);
        if v * v != disc { continue; }

        // a1 = (10^n - 2*b + v) / 2
        let num1 = pow10n - 2 * b + v;
        if num1 % 2 == 0 {
            let a = num1 / 2;
            if a > 0 {
                let val = a * pow10n + b;
                if val < max_val {
                    total += val;
                }
            }
        }

        // a2 = (10^n - 2*b - v) / 2
        if pow10n >= 2 * b + v {
            let num2 = pow10n - 2 * b - v;
            if num2 % 2 == 0 {
                let a = num2 / 2;
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
