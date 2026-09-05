// Project Euler 735 - Divisors of 2n^2
//
// Mobius function sieve + counting lattice points.
// Loops 0+1+3+4 share outer x and isqrt(n/x); loops 2+5 share outer z.
// Each pair/group is fused so the inner division runs once.

use rayon::prelude::*;

#[inline(always)]
fn isq(n: i64) -> i64 {
    n * n
}
#[inline(always)]
fn icb(n: i64) -> i64 {
    n * n * n
}

#[inline(always)]
fn isqrt_f(n: i64) -> i64 {
    if n <= 0 {
        0
    } else {
        (n as u64).isqrt() as i64
    }
}

fn cbrt_f(n: i64) -> i64 {
    if n <= 0 {
        return 0;
    }
    let mut c = (n as f64).cbrt() as i64;
    if c < 0 {
        c = 0;
    }
    while icb(c + 1) <= n {
        c += 1;
    }
    while icb(c) > n {
        c -= 1;
    }
    c
}

/// Fused loops 0,1,3,4 over y in [y_lo, y_hi).
/// 0: n/(x y) - y
/// 1: n/(x y) - (y-1)
/// 3: n/(x y) - y          if y >= 2x+1
/// 4: n/(x y) - max(2x, y-1) if y <= n/(2 x^2)
#[inline(always)]
fn fused_x_range(n_val: i64, x: i64, y_lo: i64, y_hi: i64) -> i64 {
    if y_lo >= y_hi {
        return 0;
    }
    let two_x = 2 * x;
    let z4 = n_val / (2 * isq(x));
    let split_3 = two_x + 1; // loop 3 starts
    let split_4 = z4 + 1; // loop 4 ends (exclusive)

    let mut r = 0i64;
    let mut y = y_lo;

    // y < 2x+1 and y <= z4: loops 0+1+4 → 3q - 2y + 1 - 2x
    let e = y_hi.min(split_3).min(split_4);
    while y < e {
        let q = n_val / (x * y);
        r += 3 * q - 2 * y + 1 - two_x;
        y += 1;
    }
    // y < 2x+1 and y > z4: loops 0+1 → 2q - 2y + 1
    let e = y_hi.min(split_3);
    while y < e {
        let q = n_val / (x * y);
        r += 2 * q - 2 * y + 1;
        y += 1;
    }
    // y >= 2x+1 and y <= z4: loops 0+1+3+4 → 4q - 4y + 2
    let e = y_hi.min(split_4);
    while y < e {
        let q = n_val / (x * y);
        r += 4 * q - 4 * y + 2;
        y += 1;
    }
    // y >= 2x+1 and y > z4: loops 0+1+3 → 3q - 3y + 1
    while y < y_hi {
        let q = n_val / (x * y);
        r += 3 * q - 3 * y + 1;
        y += 1;
    }
    r
}

/// Fused loops 2+5 over x in [x_lo, x_hi).
/// 2: n/(x z) - x
/// 5: n/(x z) - 2x  if x <= sqrt(n/(2z))
#[inline(always)]
fn fused_z_range(n_val: i64, z: i64, x_lo: i64, x_hi: i64) -> i64 {
    if x_lo >= x_hi {
        return 0;
    }
    let x_max5 = isqrt_f(n_val / (2 * z));
    let mut r = 0i64;
    let mut x = x_lo;
    let e = x_hi.min(x_max5 + 1);
    while x < e {
        let q = n_val / (x * z);
        r += 2 * q - 3 * x;
        x += 1;
    }
    while x < x_hi {
        let q = n_val / (x * z);
        r += q - x;
        x += 1;
    }
    r
}

fn compute_inner(n_val: i64) -> i64 {
    let mut res: i64 = 0;
    let cbrt_n = cbrt_f(n_val);

    for x in 1..=cbrt_n {
        let sq_nox = isqrt_f(n_val / x);
        let lo = x + 1;
        if lo <= sq_nox {
            res += fused_x_range(n_val, x, lo, sq_nox + 1);
        }
    }

    for z in 1..=cbrt_n {
        let sq_noz = isqrt_f(n_val / z);
        if z <= sq_noz {
            res += fused_z_range(n_val, z, z, sq_noz + 1);
        }
    }
    res
}

// Work unit: (sign, loop_id, n_val, outer, inner_lo, inner_hi)
// loop_id 0 = fused x-outer, 2 = fused z-outer, 255 = compute_inner
type WorkUnit = (i8, u8, i64, i64, i64, i64);

const CHUNK: i64 = 50_000;

fn build_sub_loop_units(sign: i8, n_val: i64, work: &mut Vec<WorkUnit>) {
    let cbrt_n = cbrt_f(n_val);

    for x in 1..=cbrt_n {
        let sq = isqrt_f(n_val / x);
        let lo = x + 1;
        if lo > sq {
            continue;
        }
        let mut y_lo = lo;
        while y_lo <= sq {
            let y_hi = (y_lo + CHUNK).min(sq + 1);
            work.push((sign, 0, n_val, x, y_lo, y_hi));
            y_lo = y_hi;
        }
    }

    for z in 1..=cbrt_n {
        let sq = isqrt_f(n_val / z);
        if z > sq {
            continue;
        }
        let mut x_lo = z;
        while x_lo <= sq {
            let x_hi = (x_lo + CHUNK).min(sq + 1);
            work.push((sign, 2, n_val, z, x_lo, x_hi));
            x_lo = x_hi;
        }
    }
}

fn exec_work_unit(wu: &WorkUnit) -> i64 {
    let &(sign, loop_id, n_val, outer, lo, hi) = wu;
    let r = match loop_id {
        255 => compute_inner(n_val),
        0 => fused_x_range(n_val, outer, lo, hi),
        2 => fused_z_range(n_val, outer, lo, hi),
        _ => 0,
    };
    sign as i64 * r
}

fn main() {
    let big_n: i64 = 1_000_000_000_000;
    let l = isqrt_f(big_n);

    // Sieve Mobius
    let lim = l as usize;
    let mut mobius = vec![1i32; lim + 1];
    let mut is_prime = vec![true; lim + 1];
    is_prime[0] = false;
    if lim >= 1 {
        is_prime[1] = false;
    }

    for i in 2..=lim {
        if is_prime[i] {
            for j in (i..=lim).step_by(i) {
                if j != i {
                    is_prime[j] = false;
                }
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

    let split_threshold: i64 = 10_000_000;

    let mut work: Vec<WorkUnit> = Vec::with_capacity(1 << 20);

    for g in 1..=lim {
        if mobius[g] == 0 {
            continue;
        }
        let g_sq = isq(g as i64);
        if g_sq >= big_n {
            break;
        }
        let sign: i8 = if mobius[g] > 0 { 1 } else { -1 };
        let mut t = 0u32;
        while g_sq <= (big_n >> t) {
            let n_val = (big_n / g_sq) >> t;
            if n_val < 1 {
                t += 1;
                continue;
            }

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
