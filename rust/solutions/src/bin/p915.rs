// Project Euler 915 - Recursive Modulo Sequence
// Uses summatory Euler totient (Lucy DP) and periodic sequence H.
// sum_{g=1}^N H(g) * (2*S_Phi(floor(N/g)) - 1) mod 123456789

use fxhash::FxHashMap;

const MOD: i64 = 123456789;
const PRECOMPUTE_LIMIT: usize = 1_000_000;

#[inline(always)]
fn s_step_mod(x: i64, m: i64) -> i64 {
    let xm1 = ((x - 1) % m + m) % m;
    let xm1_i128 = xm1 as i128;
    let m_i128 = m as i128;
    let xm1_sq = xm1_i128 * xm1_i128 % m_i128;
    ((xm1_sq * xm1_i128 % m_i128) + 2) as i64 % m
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
        // SAFETY: i is in range [2, PRECOMPUTE_LIMIT] and phi_arr has size PRECOMPUTE_LIMIT+1
        if unsafe { *phi_arr.get_unchecked(i) } == i as i32 {
            // prime
            let mut j = i;
            while j <= PRECOMPUTE_LIMIT {
                // SAFETY: j is in range [i, PRECOMPUTE_LIMIT] stepping by i
                unsafe {
                    let val = *phi_arr.get_unchecked(j);
                    *phi_arr.get_unchecked_mut(j) = val - val / i as i32;
                }
                j += i;
            }
        }
    }

    let mut s_phi = vec![0i64; PRECOMPUTE_LIMIT + 1];
    let mut current: i64 = 0;
    for i in 1..=PRECOMPUTE_LIMIT {
        // SAFETY: i is in range [1, PRECOMPUTE_LIMIT]
        current += unsafe { *phi_arr.get_unchecked(i) } as i64;
        if current >= MOD {
            current -= MOD;
        }
        // SAFETY: i is in range [1, PRECOMPUTE_LIMIT]
        unsafe {
            *s_phi.get_unchecked_mut(i) = current;
        }
    }

    // Memoized S_Phi
    let mut memo: FxHashMap<i64, i64> = FxHashMap::default();

    fn s_phi_recursive(
        val: i64,
        s_phi_table: &[i64],
        memo: &mut FxHashMap<i64, i64>,
        inv2: i64,
    ) -> i64 {
        if val <= PRECOMPUTE_LIMIT as i64 {
            // SAFETY: val is checked to be in range
            return unsafe { *s_phi_table.get_unchecked(val as usize) };
        }
        if let Some(&v) = memo.get(&val) {
            return v;
        }

        let nm = val % MOD;
        let vp1m = (val + 1) % MOD;
        let term1 = (nm as i128 * vp1m as i128 % MOD as i128 * inv2 as i128 % MOD as i128) as i64;

        let mut sub_sum: i64 = 0;
        let mut l: i64 = 2;
        while l <= val {
            let v = val / l;
            let r = if v == 0 { val } else { val / v };
            let count = (r - l + 1) % MOD;
            let phi_v = s_phi_recursive(v, s_phi_table, memo, inv2);
            let term = (count as i128 * phi_v as i128 % MOD as i128) as i64;
            sub_sum = (sub_sum + term) % MOD;
            l = r + 1;
        }

        let res = (term1 - sub_sum + MOD) % MOD;
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
            // SAFETY: g is in [1,4] and s_mod_m has size limit_m+2
            let sg = unsafe { *s_mod_m.get_unchecked(g) } as usize;
            // SAFETY: sg is computed from s_mod_m which is bounded
            unsafe {
                *h_vals.get_unchecked_mut(g) = *s_mod_m.get_unchecked(sg);
            }
        } else {
            let eff_g = 3 + ((g as i64 - 3) % 420) as usize;
            // SAFETY: eff_g is in [3, 3+419] and s_mod_p1 has size limit_p1+2
            let s_g_mod_p1 = unsafe { *s_mod_p1.get_unchecked(eff_g) };
            let mut k = s_g_mod_p1;
            while k <= 53 {
                k += 33705;
            }
            // SAFETY: k is in valid range for s_mod_m
            unsafe {
                *h_vals.get_unchecked_mut(g) = *s_mod_m.get_unchecked(k as usize);
            }
        }
    }

    let mut h_prefix = vec![0i64; 1001];
    let mut curr: i64 = 0;
    for i in 1..=1000 {
        // SAFETY: i is in [1, 1000]
        curr += unsafe { *h_vals.get_unchecked(i) };
        if curr >= MOD {
            curr -= MOD;
        }
        unsafe {
            *h_prefix.get_unchecked_mut(i) = curr;
        }
    }

    // get_sum_H
    let get_sum_h = |nn: i64| -> i64 {
        if nn <= 0 {
            return 0;
        }
        if nn <= 1000 {
            // SAFETY: nn is checked to be in [1, 1000]
            return unsafe { *h_prefix.get_unchecked(nn as usize) };
        }

        // SAFETY: indices 4 and 424 are in bounds [0, 1000]
        let sum_pre = unsafe { *h_prefix.get_unchecked(4) };
        let count = nn - 4;
        let p: i64 = 420;
        let num_full = count / p;
        let rem = count % p;

        let h4p = unsafe { *h_prefix.get_unchecked(424) };
        let sum_period = (h4p - sum_pre + MOD) % MOD;

        let mut total = sum_pre;
        let nfm = num_full % MOD;
        total = (total + (nfm as i128 * sum_period as i128 % MOD as i128) as i64) % MOD;
        let h4r = unsafe { *h_prefix.get_unchecked((4 + rem) as usize) };
        let term_rem = (h4r - sum_pre + MOD) % MOD;
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
