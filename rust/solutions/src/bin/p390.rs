// Project Euler 390: Triangles with Non Rational Sides and Integral Area
//
// For each even a, find (v,t) with v^2 = (a^2+1)*t^2 - a^2, t even.
// Hybrid approach:
//   - Small a (a <= PELL_THRESH): Use Pell equation to enumerate solutions.
//     Find fundamental solutions with even t, then apply Pell recurrence.
//   - Large a (a > PELL_THRESH): Brute-force iterate t values (all u64).
// Modular quadratic residue filtering skips most non-squares cheaply.
// Rayon for parallelism.

use rayon::prelude::*;

fn isqrt64(n: u64) -> u64 {
    if n == 0 { return 0; }
    let mut x = (n as f64).sqrt() as u64;
    loop {
        if x == 0 { return 0; }
        let q = n / x;
        if x <= q + 1 { break; }
        x = (x + q) / 2;
    }
    while x * x > n { x -= 1; }
    if (x + 1) * (x + 1) <= n { x += 1; }
    x
}

fn isqrt128(n: u128) -> u64 {
    if n == 0 { return 0; }
    if n <= u64::MAX as u128 { return isqrt64(n as u64); }
    let mut x = (n as f64).sqrt() as u64;
    for _ in 0..10 {
        if x == 0 { break; }
        let x128 = x as u128;
        let next = (x128 + n / x128) / 2;
        if next >= x128 && next - x128 <= 1 { break; }
        if x128 >= next && x128 - next <= 1 { break; }
        x = next as u64;
    }
    while (x as u128) * (x as u128) > n { x -= 1; }
    while ((x + 1) as u128) * ((x + 1) as u128) <= n { x += 1; }
    x
}

fn main() {
    let big_n: u64 = 20_000_000_000;
    let big_n128 = big_n as u128;

    // Pell threshold: use Pell for a <= this, brute force for larger a.
    // Pell fundamental search is O(a^2), brute force is O(N/(a^2+1)).
    // Crossover at a ~ N^{1/4} ~ 376.
    let pell_thresh: u64 = 400;

    let a_max = isqrt64(big_n - 1);

    // Part 1: Pell equation for small a values
    // For each even a, D = a^2+1, fundamental Pell solution (x1,y1) = (2a^2+1, 2a).
    // Find all fundamental solutions (v0, t0) to v^2 - D*t^2 = -a^2 with t even,
    // by searching t in 2,4,...,t_fund_max where t_fund_max ≈ a*x1/sqrt(D).
    // Then generate all solutions via recurrence.
    let ans_pell: u128 = (1..=(std::cmp::min(pell_thresh, a_max) / 2))
        .into_par_iter()
        .map(|half_a| {
            let a = 2 * half_a;
            let a128 = a as u128;
            let a2_128 = a128 * a128;
            let d = a2_128 + 1;
            let x1 = 2 * a2_128 + 1;
            let y1 = 2 * a128;

            // Upper bound for fundamental solution search:
            // t_fund_max = a * x1 / sqrt(D) ≈ a * (2a^2+1) / sqrt(a^2+1)
            // Conservative bound: use a * (2a^2+1) / a = 2a^2+1
            let t_fund_max = (2 * a * a + 1) as u64;
            // But also cap at the maximum useful t
            let t_upper = (big_n128 / d) as u64;
            let t_fund_max = std::cmp::min(t_fund_max, t_upper);

            let mut local_sum: u128 = 0;

            for t0 in (2..=t_fund_max).step_by(2) {
                let t0_128 = t0 as u128;
                let s = d * t0_128 * t0_128;
                if s < a2_128 { continue; }
                let s = s - a2_128;
                let v0 = isqrt128(s);
                if (v0 as u128) * (v0 as u128) != s { continue; }

                // Verify this is a fundamental solution (no predecessor with positive v and smaller t)
                // Predecessor: v_prev = x1*v0 - D*y1*t0, must be <= 0 for fundamental
                let v0_128 = v0 as u128;
                let v_prev_pos = x1 * v0_128;
                let v_prev_neg = d * y1 * t0_128;
                if v_prev_pos > v_prev_neg {
                    // v_prev > 0, so this is NOT a fundamental - it can be reduced
                    continue;
                }

                // Generate all solutions from this fundamental via Pell recurrence
                let mut v = v0_128;
                let mut t = t0_128;

                loop {
                    if t % 2 == 0 {
                        let b = a128 * t + v;
                        let n_val = a128 * b + t;
                        if n_val > big_n128 { break; }
                        local_sum += n_val / 2;
                    }

                    // Pell recurrence: (v', t') = (x1*v + D*y1*t, y1*v + x1*t)
                    let v_new = x1 * v + d * y1 * t;
                    let t_new = y1 * v + x1 * t;
                    v = v_new;
                    t = t_new;

                    if t > big_n128 { break; }
                }
            }

            local_sum
        })
        .sum();

    // Part 2: Brute force for larger a (all u64 arithmetic)
    let t_chunk: u64 = 500_000;

    // QR tables for filtering
    let qr63: [bool; 63] = {
        let mut arr = [false; 63];
        for i in 0..63u64 { arr[(i * i % 63) as usize] = true; }
        arr
    };
    let qr65: [bool; 65] = {
        let mut arr = [false; 65];
        for i in 0..65u64 { arr[(i * i % 65) as usize] = true; }
        arr
    };
    let qr11: [bool; 11] = {
        let mut arr = [false; 11];
        for i in 0..11u64 { arr[(i * i % 11) as usize] = true; }
        arr
    };

    let bf_start = if pell_thresh + 2 <= a_max { pell_thresh + 2 } else { a_max + 2 };
    let bf_start = if bf_start % 2 != 0 { bf_start + 1 } else { bf_start };

    let mut work_units: Vec<(u64, u64, u64)> = Vec::new();
    let mut a = bf_start;
    while a <= a_max {
        let a2 = a * a;
        let d = a2 + 1;
        let upper = big_n / d;
        if upper >= 2 {
            let mut t_start: u64 = 2;
            while t_start <= upper {
                let t_end = std::cmp::min(t_start + 2 * t_chunk - 2, upper);
                work_units.push((a, t_start, t_end));
                t_start = t_end + 2;
            }
        }
        a += 2;
    }

    let ans_bf: u128 = work_units
        .par_iter()
        .map(|&(a, t_start, t_end)| {
            let a2 = a * a;
            let d = a2 + 1;
            let mut local_sum: u128 = 0;
            let mut t = t_start;
            while t <= t_end {
                let t2 = t * t;
                let s = d * t2 - a2;
                let s63 = (s % 63) as usize;
                let s65 = (s % 65) as usize;
                let s11 = (s % 11) as usize;
                // SAFETY: s63 < 63, s65 < 65, s11 < 11 by modular arithmetic
                if unsafe { *qr63.get_unchecked(s63) && *qr65.get_unchecked(s65) && *qr11.get_unchecked(s11) } {
                    let v = isqrt64(s);
                    if v * v == s {
                        let b = a as u128 * t as u128 + v as u128;
                        let n_val = a as u128 * b + t as u128;
                        if n_val <= big_n128 {
                            local_sum += n_val / 2;
                        }
                    }
                }
                t += 2;
            }
            local_sum
        })
        .sum();

    println!("{}", ans_pell + ans_bf);
}
