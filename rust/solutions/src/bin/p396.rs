// Project Euler 396 - Weak Goodstein Sequence
// Last 9 digits of sum(G(n)) for 1 <= n <= 15.

use euler_utils::{mod_inv, mod_pow};

const MOD: i64 = 1_000_000_000;

fn solve_mod(b: i64) -> i64 {
    let mut pow5 = [0i64; 10];
    pow5[0] = 1;
    for i in 1..=9 {
        pow5[i] = pow5[i - 1] * 5;
    }

    // Track iter mod 5^k for k=1..9
    let mut five_mods = [0i64; 10];
    for k in 1..=9 {
        let mk = pow5[k];
        five_mods[k] = ((b + 1).rem_euclid(mk)) * mod_pow(2, (b + 1) as u64, mk as u64) as i64 % mk;
    }

    // Iterate B times
    for _ in 0..b {
        let mut new_five_mods = [0i64; 10];
        for k in 1..=9 {
            let mk = pow5[k];
            let iter_mod_mk = five_mods[k];

            let exp_mod = if k == 1 {
                0i64
            } else {
                let m_km1 = pow5[k - 1];
                let r = five_mods[k - 1];
                let inv4 = mod_inv(4u64, m_km1 as u64).unwrap() as i64;
                let t = (r as i128 * inv4 as i128 % m_km1 as i128) as i64;
                4 * t
            };

            let pow2 = mod_pow(2, exp_mod as u64, mk as u64) as i64;
            new_five_mods[k] = (iter_mod_mk as i128 * pow2 as i128 % mk as i128) as i64;
        }
        five_mods = new_five_mods;
    }

    // CRT: combine mod 512 (=0) and mod 5^9
    let m9 = pow5[9];
    let mod_5_9 = five_mods[9];
    let inv512 = mod_inv(512u64, m9 as u64).unwrap() as i64;
    let k_val = (mod_5_9 as i128 * inv512 as i128 % m9 as i128) as i64;
    (512i64 * k_val) % MOD
}

fn main() {
    let mut total: i64 = 0;

    // n=1..3: direct values
    let direct = [1i64, 3, 5];
    for &v in &direct {
        total = (total + v) % MOD;
    }

    // n=4..7: 3-digit binary
    for n in 4..=7 {
        let d1 = (n >> 1) & 1;
        let d0 = n & 1;
        let f_val = ((1i64 << d1) - 1) * (3 + d0) + d0;
        let c0 = 3 + f_val;
        let c1_mod = (c0 % MOD * mod_pow(2, c0 as u64, MOD as u64) as i64) % MOD;
        let gn = (c1_mod - 3 + MOD) % MOD;
        total = (total + gn) % MOD;
    }

    // n=8..15: 4-digit binary
    for n in 8..=15 {
        let d2 = (n >> 2) & 1;
        let d1 = (n >> 1) & 1;
        let d0 = n & 1;

        let sub3 = if d2 == 0 {
            if d1 == 0 {
                d0
            } else {
                ((1i64 << d1) - 1) * (3 + d0) + d0
            }
        } else {
            let f_val = ((1i64 << d1) - 1) * (3 + d0) + d0;
            let c0 = 3 + f_val;
            c0 * (1i64 << c0) - 3
        };

        let b = 2 + sub3;
        let iter_b_mod = solve_mod(b);
        let gn = (iter_b_mod - 3 + MOD) % MOD;
        total = (total + gn) % MOD;
    }

    println!("{}", total);
}
