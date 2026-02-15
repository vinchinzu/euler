// Project Euler 631 - Constrained Permutations
// Memoized DP for pattern-avoiding permutations with bounded descents

const K_VAL: usize = 40;
const MOD: i64 = 1_000_000_007;
const MAX_N: usize = 42;
const DIM_N: usize = MAX_N + 1;
const DIM_K: usize = K_VAL + 1;
const DIM_M: usize = MAX_N + 1;
const DIM_R: usize = MAX_N + 1;

fn main() {
    let n_val: i64 = 1_000_000_000_000_000_000;

    // Flat cache: [n][k][min_val][rise]
    let total = DIM_N * DIM_K * DIM_M * DIM_R;
    let mut cache = vec![-1i64; total];

    fn idx(n: usize, k: usize, m: usize, r: usize) -> usize {
        n * DIM_K * DIM_M * DIM_R + k * DIM_M * DIM_R + m * DIM_R + r
    }

    fn num_perms(n: usize, k: i32, min_val: usize, rise: usize, cache: &mut [i64]) -> i64 {
        if n == 0 { return 1; }
        if k < 0 { return 0; }
        let ku = k as usize;
        if n < DIM_N && ku < DIM_K && min_val < DIM_M && rise < DIM_R {
            let i = idx(n, ku, min_val, rise);
            if cache[i] >= 0 { return cache[i]; }
        }

        let mut result = 0i64;
        let limit = if n < rise { n } else { rise };
        for nv in 1..=limit {
            if nv - 1 > ku { continue; }
            let mut next_rise = rise;
            if nv < next_rise {
                if nv >= min_val { next_rise = nv; } else { next_rise -= 1; }
            }
            let new_min = if nv < min_val { nv } else { min_val };
            result = (result + num_perms(n - 1, k - (nv as i32 - 1), new_min, next_rise, cache)) % MOD;
        }

        if n < DIM_N && ku < DIM_K && min_val < DIM_M && rise < DIM_R {
            cache[idx(n, ku, min_val, rise)] = result;
        }
        result
    }

    let mut ans = 0i64;
    for n in 0..=K_VAL + 1 {
        ans = (ans + num_perms(n, K_VAL as i32, n, n, &mut cache)) % MOD;
    }

    if n_val > K_VAL as i64 + 1 {
        let base_count = num_perms(K_VAL + 2, K_VAL as i32, K_VAL + 2, K_VAL + 2, &mut cache);
        let remaining = (n_val - K_VAL as i64 - 1) % MOD;
        ans = (ans + (remaining as i128 * base_count as i128 % MOD as i128) as i64) % MOD;
    }

    println!("{}", ans);
}
