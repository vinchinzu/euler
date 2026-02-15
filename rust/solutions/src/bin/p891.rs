// Project Euler 891
// Ambiguous moments on a 3-hand clock.
// For each permutation of hand indices, solve linear congruences
// to find times where another distinct time produces the same hand
// configuration. Uses hash set for distinct rational times.

use std::collections::HashSet;

fn gcd_ll(mut a: i64, mut b: i64) -> i64 {
    if a < 0 { a = -a; }
    if b < 0 { b = -b; }
    while b != 0 { let t = b; b = a % b; a = t; }
    a
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

        let m11 = -c01;
        let m12 = b01;
        let m21 = -c02;
        let m22 = b02;

        let d = m11 * m22 - m12 * m21;
        if d == 0 { continue; }

        let d_abs = d.abs();

        for s in 0..d_abs {
            for sp in 0..d_abs {
                let k_num = s * m11 + m12 * sp;
                let l_num = m21 * s + m22 * sp;
                if k_num % d != 0 || l_num % d != 0 { continue; }

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

                // Canonicalize the rational u_num / d
                let mut num = u_num;
                let mut den = d;
                if den < 0 { num = -num; den = -den; }
                let g = gcd_ll(num, den);
                num /= g;
                den /= g;

                times.insert((num, den));
            }
        }
    }

    println!("{}", times.len());
}
