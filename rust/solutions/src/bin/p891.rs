// Project Euler 891 - Ambiguous moments on a 3-hand clock.
//
// Optimized solution using modular arithmetic to avoid O(D^2) iteration.

use fxhash::FxHashSet;
use rayon::prelude::*;

#[inline(always)]
fn gcd_ll(mut a: i64, mut b: i64) -> i64 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

#[inline]
fn mod_inv(a: i64, m: i64) -> i64 {
    let (mut old_r, mut r) = (a.rem_euclid(m), m);
    let (mut old_s, mut s) = (1i64, 0i64);
    while r != 0 {
        let q = old_r / r;
        let tmp = r;
        r = old_r - q * r;
        old_r = tmp;
        let tmp = s;
        s = old_s - q * s;
        old_s = tmp;
    }
    old_s.rem_euclid(m)
}

#[inline]
fn times_for_perm(m11: i64, m12: i64, m21: i64, m22: i64, d: i64) -> FxHashSet<(i64, i64)> {
    const T_CYCLE: i64 = 43200;
    let d_abs = d.abs();
    let g1 = gcd_ll(m11, d_abs);
    let d1 = d_abs / g1;
    let m11_red = m11 / g1;
    let inv_m11 = mod_inv(m11_red.rem_euclid(d1), d1);
    let cycle = T_CYCLE * d_abs;
    let d_pos = d > 0;

    let mut times: FxHashSet<(i64, i64)> = FxHashSet::with_capacity_and_hasher(1024, Default::default());
    for sp in 0..d_abs {
        let target = (-m12 * sp).rem_euclid(d_abs);
        if target % g1 != 0 {
            continue;
        }

        let target_red = target / g1;
        let s0 = (target_red * inv_m11).rem_euclid(d1);

        for k in 0..g1 {
            let s = s0 + k * d1;
            let eq2 = m21 * s + m22 * sp;
            if eq2 % d_abs != 0 {
                continue;
            }

            let k_val = (m11 * s + m12 * sp) / d;
            let l_val = eq2 / d;
            let u_num = T_CYCLE * (k_val * m22 - l_val * m12);
            let up_num = T_CYCLE * (-k_val * m21 + l_val * m11);
            
            let u_n = if d_pos { u_num } else { -u_num };
            let up_n = if d_pos { up_num } else { -up_num };
            
            let u_r = u_n.rem_euclid(cycle);
            let up_r = up_n.rem_euclid(cycle);
            
            if u_r != up_r {
                let gu = gcd_ll(u_r, d_abs);
                times.insert((u_r / gu, d_abs / gu));
                let gup = gcd_ll(up_r, d_abs);
                times.insert((up_r / gup, d_abs / gup));
            }
        }
    }
    times
}

fn main() {
    let a_coeff: [i64; 3] = [1, 12, 720];

    let perms: [[usize; 3]; 6] = [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ];
    
    let perm_sets: Vec<FxHashSet<(i64, i64)>> = perms
        .par_iter()
        .filter_map(|perm| {
            let b01 = a_coeff[perm[0]] - a_coeff[perm[1]];
            let b02 = a_coeff[perm[0]] - a_coeff[perm[2]];

            let m11 = a_coeff[1] - a_coeff[0]; // 11
            let m12 = b01;
            let m21 = a_coeff[2] - a_coeff[0]; // 719
            let m22 = b02;

            let d = m11 * m22 - m12 * m21;
            if d == 0 {
                return None;
            }
            Some(times_for_perm(m11, m12, m21, m22, d))
        })
        .collect();

    let mut times: FxHashSet<(i64, i64)> = FxHashSet::with_capacity_and_hasher(2048, Default::default());
    for s in perm_sets {
        times.extend(s);
    }

    println!("{}", times.len());
}
