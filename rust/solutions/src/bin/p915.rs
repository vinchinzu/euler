// Project Euler 915 - Recursive Modulo Sequence
// Uses summatory Euler totient (Lucy DP) and periodic sequence H.
// sum_{g=1}^N H(g) * (2*S_Phi(floor(N/g)) - 1) mod 123456789

use std::collections::HashMap;

const MOD: i64 = 123456789;
const PRECOMPUTE_LIMIT: usize = 1_000_000;

fn s_step_mod(x: i64, m: i64) -> i64 {
    let xm1 = ((x - 1) % m + m) % m;
    (xm1 as i128 * xm1 as i128 % m as i128 * xm1 as i128 % m as i128 + 2) as i64 % m
}

fn main() {
    let n: i64 = 100_000_000; // 10^8
    let inv2: i64 = (MOD + 1) / 2;

    // Compute phi sieve and prefix sums
    let mut phi_arr = vec![0i32; PRECOMPUTE_LIMIT + 1];
    for i in 0..=PRECOMPUTE_LIMIT {
        phi_arr[i] = i as i32;
    }
    for i in 2..=PRECOMPUTE_LIMIT {
        if phi_arr[i] == i as i32 {
            // prime
            let mut j = i;
            while j <= PRECOMPUTE_LIMIT {
                phi_arr[j] -= phi_arr[j] / i as i32;
                j += i;
            }
        }
    }

    let mut s_phi = vec![0i64; PRECOMPUTE_LIMIT + 1];
    let mut current: i64 = 0;
    for i in 1..=PRECOMPUTE_LIMIT {
        current = (current + phi_arr[i] as i64) % MOD;
        s_phi[i] = current;
    }

    // Memoized S_Phi
    let mut memo: HashMap<i64, i64> = HashMap::new();

    fn s_phi_recursive(
        val: i64,
        s_phi_table: &[i64],
        memo: &mut HashMap<i64, i64>,
        inv2: i64,
    ) -> i64 {
        if val <= PRECOMPUTE_LIMIT as i64 {
            return s_phi_table[val as usize];
        }
        if let Some(&v) = memo.get(&val) {
            return v;
        }

        let nm = val % MOD;
        let term1 = nm * ((val + 1) % MOD) % MOD * inv2 % MOD;

        let mut sub_sum: i64 = 0;
        let mut l: i64 = 2;
        while l <= val {
            let v = val / l;
            let r = if v == 0 { val } else { val / v };
            let count = (r - l + 1) % MOD;
            let term = count * s_phi_recursive(v, s_phi_table, memo, inv2) % MOD;
            sub_sum = (sub_sum + term) % MOD;
            l = r + 1;
        }

        let res = (term1 - sub_sum % MOD + MOD) % MOD;
        memo.insert(val, res);
        res
    }

    // Compute H period
    // s mod MOD has preperiod 53, period 33705
    // s mod 33705 has preperiod 2, period 420
    let limit_m = 53 + 33705 + 100;
    let mut s_mod_m = vec![0i64; limit_m + 2];
    s_mod_m[1] = 1;
    for i in 2..=limit_m {
        s_mod_m[i] = s_step_mod(s_mod_m[i - 1], MOD);
    }

    let limit_p1 = 2 + 420 + 100;
    let mut s_mod_p1 = vec![0i64; limit_p1 + 2];
    s_mod_p1[1] = 1;
    for i in 2..=limit_p1 {
        s_mod_p1[i] = s_step_mod(s_mod_p1[i - 1], 33705);
    }

    let mut h_vals = vec![0i64; 1001];
    for g in 1..=1000 {
        if g <= 4 {
            let sg = s_mod_m[g] as usize;
            h_vals[g] = s_mod_m[sg];
        } else {
            let eff_g = 3 + ((g as i64 - 3) % 420) as usize;
            let s_g_mod_p1 = s_mod_p1[eff_g];
            let mut k = s_g_mod_p1;
            while k <= 53 {
                k += 33705;
            }
            h_vals[g] = s_mod_m[k as usize];
        }
    }

    let mut h_prefix = vec![0i64; 1001];
    let mut curr: i64 = 0;
    for i in 1..=1000 {
        curr = (curr + h_vals[i]) % MOD;
        h_prefix[i] = curr;
    }

    // get_sum_H
    let get_sum_h = |nn: i64| -> i64 {
        if nn <= 0 {
            return 0;
        }
        if nn <= 1000 {
            return h_prefix[nn as usize];
        }

        let sum_pre = h_prefix[4];
        let count = nn - 4;
        let p: i64 = 420;
        let num_full = count / p;
        let rem = count % p;

        let sum_period = (h_prefix[4 + p as usize] - h_prefix[4] + MOD) % MOD;

        let mut total = sum_pre;
        total = (total + (num_full % MOD) * sum_period) % MOD;
        let term_rem = (h_prefix[4 + rem as usize] - h_prefix[4] + MOD) % MOD;
        total = (total + term_rem) % MOD;
        total
    };

    // Main summation
    let mut total_sum: i64 = 0;
    let mut l: i64 = 1;
    while l <= n {
        let val = n / l;
        let r = if val == 0 { n } else { n / val };

        let sum_h_range = (get_sum_h(r) - get_sum_h(l - 1) + MOD) % MOD;
        let phi_val = s_phi_recursive(val, &s_phi, &mut memo, inv2);
        let weight = (2 * phi_val - 1 + MOD) % MOD;
        let term = sum_h_range * weight % MOD;
        total_sum = (total_sum + term) % MOD;

        l = r + 1;
    }

    println!("{}", total_sum);
}
