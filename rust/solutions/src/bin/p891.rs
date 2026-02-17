// Project Euler 891 - Ambiguous moments on a 3-hand clock.
//
// Optimized solution using modular arithmetic to avoid O(D^2) iteration.

use std::collections::HashSet;

fn gcd_ll(mut a: i64, mut b: i64) -> i64 {
    if a < 0 {
        a = -a;
    }
    if b < 0 {
        b = -b;
    }
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

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

fn main() {
    let a_coeff: [i64; 3] = [1, 12, 720];
    const T_CYCLE: i64 = 43200;

    let perms: [[usize; 3]; 6] = [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ];

    let mut times: HashSet<(i64, i64)> = HashSet::new();

    for perm in &perms {
        let c01 = a_coeff[0] - a_coeff[1];
        let c02 = a_coeff[0] - a_coeff[2];
        let b01 = a_coeff[perm[0]] - a_coeff[perm[1]];
        let b02 = a_coeff[perm[0]] - a_coeff[perm[2]];

        let m11 = -c01; // = 11
        let m12 = b01;
        let m21 = -c02; // = 719
        let m22 = b02;

        let d = m11 * m22 - m12 * m21;
        if d == 0 {
            continue;
        }

        let d_abs = d.abs();

        // Use the modular approach: iterate over sp, solve for s
        // From eq1: m11*s + m12*sp ≡ 0 (mod d)
        // So: m11*s ≡ -m12*sp (mod d)
        let g1 = gcd_ll(m11, d_abs);

        for sp in 0..d_abs {
            let target = (-m12 * sp).rem_euclid(d_abs);

            if target % g1 != 0 {
                continue;
            }

            // Solve m11*s ≡ target (mod d_abs)
            let d1 = d_abs / g1;
            let m11_red = m11 / g1;
            let target_red = target / g1;

            let inv_m11 = mod_inv(m11_red.rem_euclid(d1), d1);
            let s0 = (target_red * inv_m11).rem_euclid(d1);

            // All solutions: s = s0 + k*d1 for k = 0, 1, ..., g1-1
            for k in 0..g1 {
                let s = s0 + k * d1;

                // Verify eq2: m21*s + m22*sp ≡ 0 (mod d)
                // Use rem_euclid to handle negative values correctly
                let eq2 = m21 * s + m22 * sp;
                if eq2.rem_euclid(d_abs) != 0 {
                    continue;
                }

                // Compute k_val and l (the integer quotients, not to be confused with loop variable k)
                let k_val = (m11 * s + m12 * sp) / d;
                let l_val = eq2 / d;

                // Compute times
                let u_num = T_CYCLE * (k_val * m22 - l_val * m12);
                let up_num = T_CYCLE * (-k_val * m21 + l_val * m11);

                // Normalize to positive denominator, reduce mod T_CYCLE
                let den = d_abs;
                let u_n = if d > 0 { u_num } else { -u_num };
                let up_n = if d > 0 { up_num } else { -up_num };
                let cycle = T_CYCLE * den;
                let u_r = u_n.rem_euclid(cycle);
                let up_r = up_n.rem_euclid(cycle);

                if u_r == up_r {
                    continue;
                }

                // Store as reduced fraction for canonical dedup across perms
                let g1 = gcd_ll(u_r, den);
                times.insert((u_r / g1, den / g1));
                let g2 = gcd_ll(up_r, den);
                times.insert((up_r / g2, den / g2));
            }
        }
    }

    println!("{}", times.len());
}
