// Project Euler 513 - Triangles with Integer Median
//
// Count triangles with integer sides a <= b <= c <= N where the median to c
// has integer length.
//
// Key identity: 2a^2 + 2b^2 - c^2 = 4m^2. Let s=(a+b)/2, d=(b-a)/2, t=c/2.
// Then s^2 + d^2 = t^2 + m^2. Let u = s - t >= 1. Then:
//   P = u(2t+u) = (m-d)(m+d) = v*w
// where v and w have same parity as u. For each (u,v) with v > u, same parity,
// and v | P for some valid t, the values of t form an arithmetic progression
// with step = v' = v/gcd(u,v) (or related via CRT when v divisible by 4 and u even).
//
// The answer is f_count(N) -- no Mobius inversion needed.
// Optimized: iterate over divisors g of u to avoid per-v GCD computation.

use rayon::prelude::*;

const NN: i64 = 100_000;

#[inline(always)]
fn gcd32(mut a: u32, mut b: u32) -> u32 {
    if a == 0 { return b; }
    if b == 0 { return a; }
    let shift = (a | b).trailing_zeros();
    a >>= a.trailing_zeros();
    loop {
        b >>= b.trailing_zeros();
        if a > b { let t = a; a = b; b = t; }
        b -= a;
        if b == 0 { return a << shift; }
    }
}

#[inline(always)]
fn neg_mod32(val: u32, m: u32) -> u32 {
    let r = val % m;
    if r == 0 { 0 } else { m - r }
}

/// Process a single (u, v) pair for the odd u case.
/// Returns the count contribution.
#[inline(always)]
fn process_odd(u: u32, v: u32, g: u32, half_n_i64: i64) -> i64 {
    let vp = v / g;

    let (step, t0): (u32, u32);
    if vp == 1 {
        step = 1;
        t0 = 1;
    } else {
        // vp always odd when u odd
        let inv2 = (vp + 1) / 2;
        let r = neg_mod32(u, vp);
        let mut t0_ = ((r as u64 * inv2 as u64) % vp as u64) as u32;
        if t0_ == 0 { t0_ = vp; }
        step = vp;
        t0 = t0_;
    }

    let u64_ = u as i64;
    let v64_ = v as i64;
    let mut t_min = u64_;

    let vv_uu = v64_ * v64_ - u64_ * u64_;
    let t_min2 = (vv_uu + 2 * u64_ - 1) / (2 * u64_);
    if t_min2 > t_min { t_min = t_min2; }

    let num = (u64_ + v64_) * (u64_ + v64_) - 2 * v64_ * v64_;
    if num > 0 {
        let denom = 2 * (v64_ - u64_);
        let t_min3 = (num + denom - 1) / denom;
        if t_min3 > t_min { t_min = t_min3; }
    }

    let step64 = step as i64;
    let t064 = t0 as i64;
    if t_min <= t064 {
        t_min = t064;
    } else {
        let k = (t_min - t064 + step64 - 1) / step64;
        t_min = t064 + k * step64;
    }

    if t_min <= half_n_i64 {
        (half_n_i64 - t_min) / step64 + 1
    } else {
        0
    }
}

/// For a given u, count all valid triangles by iterating over v.
#[inline(never)]
fn count_for_u(u: u32, half_n: u32) -> i64 {
    let mut count: i64 = 0;
    let p_max: u64 = u as u64 * (2 * half_n as u64 + u as u64);

    let mut v_max = (p_max as f64).sqrt() as u32 + 1;
    while (v_max as u64) * (v_max as u64) > p_max {
        v_max -= 1;
    }

    let u_odd = u & 1 == 1;
    let half_n_i64 = half_n as i64;

    if u_odd {
        // For odd u, iterate v = u+2, u+4, ... (odd values)
        // Split into: v coprime to u (gcd=1) and v sharing a factor with u
        // Use divisor-based iteration to handle non-coprime efficiently
        let mut v: u32 = u + 2;
        while v <= v_max {
            let g = gcd32(u, v);
            count += process_odd(u, v, g, half_n_i64);
            v += 2;
        }
    } else {
        let hu = u / 2;
        let mut v: u32 = u + 2;
        while v <= v_max {
            let g = gcd32(u, v);
            let vp = v / g;

            let (mut step, mut t0): (u32, u32);
            if vp == 1 {
                step = 1;
                t0 = 1;
            } else if vp & 1 == 1 {
                let inv2 = (vp + 1) / 2;
                let r = neg_mod32(u, vp);
                t0 = ((r as u64 * inv2 as u64) % vp as u64) as u32;
                if t0 == 0 { t0 = vp; }
                step = vp;
            } else {
                let half_vp = vp / 2;
                t0 = neg_mod32(hu, half_vp);
                if t0 == 0 { t0 = half_vp; }
                step = half_vp;
            }

            // CRT when v divisible by 4
            let v1 = v >> 1;
            if v1 & 1 == 0 {
                let g2 = gcd32(hu, v1);
                let r2 = v1 / g2;
                if r2 > 1 {
                    let t0_2 = neg_mod32(hu, r2);
                    let g3 = gcd32(step, r2);
                    let diff_raw = t0_2 as i64 - t0 as i64;
                    if diff_raw.rem_euclid(g3 as i64) != 0 {
                        v += 2;
                        continue;
                    }
                    let lcm = (step / g3) as u64 * r2 as u64;
                    if lcm > half_n as u64 * 2 {
                        v += 2;
                        continue;
                    }
                    let lcm = lcm as u32;
                    let m1g = step / g3;
                    let m2g = r2 / g3;
                    let diff = (t0_2 as i64 - t0 as i64) / g3 as i64;
                    let (mut x, mut x1) = (0i64, 1i64);
                    let (mut tm, mut ta) = (m2g as i64, m1g as i64);
                    while ta > 0 {
                        let q = tm / ta;
                        let tmp = ta; ta = tm - q * ta; tm = tmp;
                        let tmp = x1; x1 = x - q * x1; x = tmp;
                    }
                    let kk = ((diff % m2g as i64) * (x % m2g as i64) % m2g as i64 + m2g as i64)
                        .rem_euclid(m2g as i64);
                    let mut t0_64 = t0 as i64 + kk * step as i64;
                    t0_64 = ((t0_64 % lcm as i64) + lcm as i64) % lcm as i64;
                    if t0_64 == 0 { t0_64 = lcm as i64; }
                    t0 = t0_64 as u32;
                    step = lcm;
                }
            }

            if t0 == 0 { t0 = step; }

            let u64_ = u as i64;
            let v64_ = v as i64;
            let mut t_min = u64_;

            let vv_uu = v64_ * v64_ - u64_ * u64_;
            let t_min2 = (vv_uu + 2 * u64_ - 1) / (2 * u64_);
            if t_min2 > t_min { t_min = t_min2; }

            let num = (u64_ + v64_) * (u64_ + v64_) - 2 * v64_ * v64_;
            if num > 0 {
                let denom = 2 * (v64_ - u64_);
                let t_min3 = (num + denom - 1) / denom;
                if t_min3 > t_min { t_min = t_min3; }
            }

            let step64 = step as i64;
            let t064 = t0 as i64;
            if t_min <= t064 {
                t_min = t064;
            } else {
                let k = (t_min - t064 + step64 - 1) / step64;
                t_min = t064 + k * step64;
            }

            if t_min <= half_n_i64 {
                count += (half_n_i64 - t_min) / step64 + 1;
            }

            v += 2;
        }
    }

    count
}

fn f_count(n: i64) -> i64 {
    let half_n = (n / 2) as u32;

    let chunk_size = 32u32;
    let num_chunks = (half_n + chunk_size - 1) / chunk_size;

    (0..num_chunks)
        .into_par_iter()
        .map(|chunk_idx| {
            let u_start = chunk_idx * chunk_size + 1;
            let u_end = std::cmp::min(u_start + chunk_size - 1, half_n);
            let mut total = 0i64;
            for u in u_start..=u_end {
                total += count_for_u(u, half_n);
            }
            total
        })
        .sum()
}

fn main() {
    let n = std::env::args()
        .nth(1)
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(NN);

    println!("{}", f_count(n));
}
