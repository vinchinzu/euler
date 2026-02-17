// Project Euler 735 - Divisors of 2n^2
//
// Mobius function sieve + counting lattice points.
// All work is flattened into fine-grained chunks and processed in one rayon pool.

use rayon::prelude::*;

#[inline(always)]
fn isq(n: i64) -> i64 { n * n }
#[inline(always)]
fn icb(n: i64) -> i64 { n * n * n }

fn isqrt_f(n: i64) -> i64 {
    if n <= 0 { return 0; }
    let mut sq = (n as f64).sqrt() as i64;
    if sq < 0 { sq = 0; }
    while sq * sq > n { sq -= 1; }
    while (sq + 1) * (sq + 1) <= n { sq += 1; }
    sq
}

fn cbrt_f(n: i64) -> i64 {
    if n <= 0 { return 0; }
    let mut c = (n as f64).cbrt() as i64;
    if c < 0 { c = 0; }
    while icb(c + 1) <= n { c += 1; }
    while icb(c) > n { c -= 1; }
    c
}

// Compute all 6 sub-loop sums sequentially for small n_val
fn compute_inner(n_val: i64) -> i64 {
    let mut res: i64 = 0;

    {
        let mut x: i64 = 1;
        while icb(x) <= n_val {
            let sq_nox = isqrt_f(n_val / x);
            let mut y = x + 1;
            while y <= sq_nox { res += n_val / (x * y) - y; y += 1; }
            x += 1;
        }
        x = 1;
        while icb(x) <= n_val {
            let sq_nox = isqrt_f(n_val / x);
            let mut z = x + 1;
            while z <= sq_nox { res += n_val / (x * z) - (z - 1); z += 1; }
            x += 1;
        }
        let mut z: i64 = 1;
        while icb(z) <= n_val {
            let sq_noz = isqrt_f(n_val / z);
            let mut x = z;
            while x <= sq_noz { res += n_val / (x * z) - x; x += 1; }
            z += 1;
        }
    }
    {
        let mut x: i64 = 1;
        while icb(x) <= n_val {
            let sq_nox = isqrt_f(n_val / x);
            let mut y = 2 * x + 1;
            while y <= sq_nox { res += n_val / (x * y) - y; y += 1; }
            x += 1;
        }
        x = 1;
        while icb(x) <= n_val {
            let sq_nox = isqrt_f(n_val / x);
            let mut z = x + 1;
            while z <= sq_nox {
                if 2 * z * isq(x) > n_val { break; }
                res += n_val / (x * z) - (2 * x).max(z - 1);
                z += 1;
            }
            x += 1;
        }
        let mut z: i64 = 1;
        while icb(z) <= n_val {
            let mut x = z;
            while 2 * z * isq(x) <= n_val { res += n_val / (x * z) - 2 * x; x += 1; }
            z += 1;
        }
    }
    res
}

// Work unit: (sign: i8, loop_id: u8, n_val: i64, outer: i64, inner_lo: i64, inner_hi: i64)
// loop_id 255 = run compute_inner on n_val (outer/inner_lo/inner_hi unused)
type WorkUnit = (i8, u8, i64, i64, i64, i64);

const CHUNK: i64 = 20_000;

fn build_sub_loop_units(sign: i8, n_val: i64, work: &mut Vec<WorkUnit>) {
    let cbrt_n = cbrt_f(n_val);

    // Sub-loop 0: outer=x, inner=y in (x+1)..sqrt(n/x)
    for x in 1..=cbrt_n {
        let sq = isqrt_f(n_val / x);
        let lo = x + 1;
        if lo > sq { continue; }
        let mut y_lo = lo;
        while y_lo <= sq {
            let y_hi = (y_lo + CHUNK).min(sq + 1);
            work.push((sign, 0, n_val, x, y_lo, y_hi));
            y_lo = y_hi;
        }
    }

    // Sub-loop 1: outer=x, inner=z in (x+1)..sqrt(n/x)
    for x in 1..=cbrt_n {
        let sq = isqrt_f(n_val / x);
        let lo = x + 1;
        if lo > sq { continue; }
        let mut z_lo = lo;
        while z_lo <= sq {
            let z_hi = (z_lo + CHUNK).min(sq + 1);
            work.push((sign, 1, n_val, x, z_lo, z_hi));
            z_lo = z_hi;
        }
    }

    // Sub-loop 2: outer=z, inner=x in z..sqrt(n/z)
    for z in 1..=cbrt_n {
        let sq = isqrt_f(n_val / z);
        if z > sq { continue; }
        let mut x_lo = z;
        while x_lo <= sq {
            let x_hi = (x_lo + CHUNK).min(sq + 1);
            work.push((sign, 2, n_val, z, x_lo, x_hi));
            x_lo = x_hi;
        }
    }

    // Sub-loop 3: outer=x, inner=y in (2x+1)..sqrt(n/x)
    for x in 1..=cbrt_n {
        let sq = isqrt_f(n_val / x);
        let lo = 2 * x + 1;
        if lo > sq { continue; }
        let mut y_lo = lo;
        while y_lo <= sq {
            let y_hi = (y_lo + CHUNK).min(sq + 1);
            work.push((sign, 3, n_val, x, y_lo, y_hi));
            y_lo = y_hi;
        }
    }

    // Sub-loop 4: outer=x, inner=z in (x+1)..min(sqrt(n/x), n/(2x^2))
    for x in 1..=cbrt_n {
        let sq = isqrt_f(n_val / x);
        let z_max_cap = n_val / (2 * isq(x));
        let hi_bound = sq.min(z_max_cap);
        let lo = x + 1;
        if lo > hi_bound { continue; }
        let mut z_lo = lo;
        while z_lo <= hi_bound {
            let z_hi = (z_lo + CHUNK).min(hi_bound + 1);
            work.push((sign, 4, n_val, x, z_lo, z_hi));
            z_lo = z_hi;
        }
    }

    // Sub-loop 5: outer=z, inner=x in z..sqrt(n/(2z))
    for z in 1..=cbrt_n {
        let x_max = isqrt_f(n_val / (2 * z));
        if z > x_max { continue; }
        let mut x_lo = z;
        while x_lo <= x_max {
            let x_hi = (x_lo + CHUNK).min(x_max + 1);
            work.push((sign, 5, n_val, z, x_lo, x_hi));
            x_lo = x_hi;
        }
    }
}

fn exec_work_unit(wu: &WorkUnit) -> i64 {
    let &(sign, loop_id, n_val, outer, lo, hi) = wu;
    if loop_id == 255 {
        return sign as i64 * compute_inner(n_val);
    }
    let mut r = 0i64;
    match loop_id {
        0 => { let x = outer; let mut y = lo; while y < hi { r += n_val / (x * y) - y; y += 1; } }
        1 => { let x = outer; let mut z = lo; while z < hi { r += n_val / (x * z) - (z - 1); z += 1; } }
        2 => { let z = outer; let mut x = lo; while x < hi { r += n_val / (x * z) - x; x += 1; } }
        3 => { let x = outer; let mut y = lo; while y < hi { r += n_val / (x * y) - y; y += 1; } }
        4 => { let x = outer; let mut z = lo; while z < hi { if 2 * z * isq(x) > n_val { break; } r += n_val / (x * z) - (2 * x).max(z - 1); z += 1; } }
        5 => { let z = outer; let mut x = lo; while x < hi { if 2 * z * isq(x) > n_val { break; } r += n_val / (x * z) - 2 * x; x += 1; } }
        _ => {}
    }
    sign as i64 * r
}

fn main() {
    let big_n: i64 = 1_000_000_000_000;
    let mut l = (big_n as f64).sqrt() as i64;
    if l * l > big_n { l -= 1; }
    while (l + 1) * (l + 1) <= big_n { l += 1; }

    // Sieve Mobius
    let lim = l as usize;
    let mut mobius = vec![1i32; lim + 1];
    let mut is_prime = vec![true; lim + 1];
    is_prime[0] = false;
    if lim >= 1 { is_prime[1] = false; }

    for i in 2..=lim {
        if is_prime[i] {
            for j in (i..=lim).step_by(i) {
                if j != i { is_prime[j] = false; }
                mobius[j] *= -1;
            }
            let sq = i as u64 * i as u64;
            if sq <= lim as u64 {
                let mut j = sq as usize;
                while j <= lim {
                    mobius[j] = 0;
                    j += sq as usize;
                }
            }
        }
    }

    // Split threshold for fine-grained parallelism
    let split_threshold: i64 = 10_000_000; // 10^7

    // Build all work units
    let mut work: Vec<WorkUnit> = Vec::new();

    for g in 1..=lim {
        if mobius[g] == 0 { continue; }
        let g_sq = isq(g as i64);
        if g_sq >= big_n { break; }
        let sign: i8 = if mobius[g] > 0 { 1 } else { -1 };
        let mut t = 0u32;
        while g_sq <= (big_n >> t) {
            let n_val = (big_n / g_sq) >> t;
            if n_val < 1 { t += 1; continue; }

            let parity: i8 = if t % 2 == 0 { 1 } else { -1 };
            let combined_sign: i8 = sign * parity;

            if n_val >= split_threshold {
                build_sub_loop_units(combined_sign, n_val, &mut work);
            } else {
                work.push((combined_sign, 255, n_val, 0, 0, 0));
            }
            t += 1;
        }
    }

    let parallel_sum: i64 = work.par_iter().map(|wu| exec_work_unit(wu)).sum();

    let ans = big_n + parallel_sum;
    println!("{}", ans);
}
