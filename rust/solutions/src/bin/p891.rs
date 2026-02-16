// Project Euler 891
// Ambiguous moments on a 3-hand clock.
// For each non-identity permutation of hand indices, solve linear congruences
// to find times where another distinct time produces the same hand
// configuration (up to rotation). Insert BOTH times from each pair.

use std::collections::HashSet;

fn gcd_ll(mut a: i64, mut b: i64) -> i64 {
    if a < 0 { a = -a; }
    if b < 0 { b = -b; }
    while b != 0 { let t = b; b = a % b; a = t; }
    a
}

fn mod_inv(a: i64, m: i64) -> i64 {
    let (mut old_r, mut r) = (a.rem_euclid(m), m);
    let (mut old_s, mut s) = (1i64, 0i64);
    while r != 0 {
        let q = old_r / r;
        let tmp = r; r = old_r - q * r; old_r = tmp;
        let tmp = s; s = old_s - q * s; old_s = tmp;
    }
    old_s.rem_euclid(m)
}

fn canonicalize(num: i64, den: i64) -> (i64, i64) {
    let mut n = num;
    let mut d = den;
    if d < 0 { n = -n; d = -d; }
    let g = gcd_ll(n.abs(), d);
    (n / g, d / g)
}

fn main() {
    let a_coeff: [i64; 3] = [1, 12, 720];

    let perms: [[usize; 3]; 6] = [
        [0, 1, 2], [0, 2, 1], [1, 0, 2],
        [1, 2, 0], [2, 0, 1], [2, 1, 0],
    ];

    let mut times: HashSet<(i64, i64)> = HashSet::new();

    for perm in &perms {
        let c01 = a_coeff[0] - a_coeff[1];
        let c02 = a_coeff[0] - a_coeff[2];
        let b01 = a_coeff[perm[0]] - a_coeff[perm[1]];
        let b02 = a_coeff[perm[0]] - a_coeff[perm[2]];

        let m11 = -c01; // always 11
        let m12 = b01;
        let m21 = -c02; // always 719
        let m22 = b02;

        let d = m11 * m22 - m12 * m21;
        if d == 0 { continue; }

        let d_abs = d.abs();

        // For each sp, solve m11*s + m12*sp ≡ 0 (mod d_abs) for s
        let g1 = gcd_ll(m11, d_abs);
        let d_red = d_abs / g1;
        let m11_red = m11 / g1;
        let inv_m11 = if d_red > 1 { mod_inv(m11_red, d_red) } else { 0 };

        for sp in 0..d_abs {
            let target = (-m12 * sp).rem_euclid(d_abs);

            if target % g1 != 0 { continue; }

            let target_red = target / g1;
            let s0 = if d_red > 1 {
                (target_red * inv_m11).rem_euclid(d_red)
            } else { 0 };

            for s_idx in 0..g1 {
                let s = s0 + s_idx * d_red;

                // Verify eq2: m21*s + m22*sp ≡ 0 (mod d_abs)
                let l_num = m21 * s + m22 * sp;
                if l_num % d_abs != 0 { continue; }

                let k_num = m11 * s + m12 * sp;
                let k = k_num / d;
                let l = l_num / d;

                let u_num = 43200i64 * (k * m22 - l * m12);
                let up_num = 43200i64 * (-k * m21 + l * m11);

                if d > 0 {
                    if u_num < 0 || u_num >= 43200i64 * d { continue; }
                    if up_num < 0 || up_num >= 43200i64 * d { continue; }
                } else {
                    if u_num > 0 || u_num <= 43200i64 * d { continue; }
                    if up_num > 0 || up_num <= 43200i64 * d { continue; }
                }

                if u_num == up_num { continue; }

                // Insert BOTH t and t' as ambiguous times
                times.insert(canonicalize(u_num, d));
                times.insert(canonicalize(up_num, d));
            }
        }
    }

    println!("{}", times.len());
}
