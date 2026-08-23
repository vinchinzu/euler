// Project Euler 544 - Chromatic Polynomial
//
// Compute sum_{k=1}^{N} f(R,C,k) mod M for an R x C grid graph using DP
// with Lagrange interpolation to extrapolate cumulative sum.

use fxhash::FxHashMap;

const R_VAL: usize = 9;
const C_VAL: usize = 10;
const MOD: i64 = 1_000_000_007;
const RC: usize = R_VAL * C_VAL;
const F_SIZE: usize = 2 * RC + 5;

fn encode_state(r: usize, last_colors: &[i32]) -> u64 {
    let mut key = r as u64;
    for &c in last_colors {
        key = key * (R_VAL as u64 + 2) + c as u64;
    }
    key = key * (R_VAL as u64 + 2) + last_colors.len() as u64;
    key
}

fn f_rec(
    r: usize,
    colors: &[i32],
    cache: &mut FxHashMap<u64, usize>,
    results: &mut Vec<[i32; F_SIZE]>,
) -> usize {
    let start = colors.len().saturating_sub(R_VAL);
    let mut mapping = [0i32; 2 * R_VAL + 2];
    let mut curr_max: i32 = 0;
    let mut last_colors = [0i32; R_VAL];
    let mut nlast = 0usize;

    for i in start..colors.len() {
        let c = colors[i] as usize;
        if mapping[c] == 0 {
            curr_max += 1;
            mapping[c] = curr_max;
        }
        last_colors[nlast] = mapping[c];
        nlast += 1;
    }
    let max_color = curr_max;

    let key = encode_state(r, &last_colors[..nlast]);
    if let Some(&idx) = cache.get(&key) {
        return idx;
    }

    if r == RC {
        let idx = results.len();
        results.push([1; F_SIZE]);
        cache.insert(key, idx);
        return idx;
    }

    let mut f_arr = [0i32; F_SIZE];
    let mut pass = [0i32; R_VAL + 1];
    pass[..nlast].copy_from_slice(&last_colors[..nlast]);

    for color in 1..=(max_color + 1) {
        if r % R_VAL != 0 && nlast > 0 && color == last_colors[nlast - 1] {
            continue;
        }
        if nlast >= R_VAL && color == last_colors[nlast - R_VAL] {
            continue;
        }

        pass[nlast] = color;
        let next_idx = f_rec(r + 1, &pass[..=nlast], cache, results);
        let next_f = &results[next_idx];
        let new_col = color == max_color + 1;
        for n_idx in 0..F_SIZE {
            let choices = if new_col {
                n_idx as i64 - color as i64 + 1
            } else {
                1
            };
            if choices <= 0 {
                continue;
            }
            // SAFETY: n_idx < F_SIZE; next_idx from cache/results
            let nf = unsafe { *next_f.get_unchecked(n_idx) } as i64;
            let slot = unsafe { f_arr.get_unchecked_mut(n_idx) };
            *slot = ((*slot as i64 + choices * nf) % MOD) as i32;
        }
    }

    let idx = results.len();
    results.push(f_arr);
    cache.insert(key, idx);
    idx
}

fn lagrange_extrapolation(values: &[i64], x: i64) -> i64 {
    let n = values.len();
    let mut prefix = vec![0i64; n + 1];
    let mut suffix = vec![0i64; n + 1];

    prefix[0] = 1;
    for i in 0..n {
        prefix[i + 1] = prefix[i] * (((x - (i as i64 + 1)) % MOD + MOD) % MOD) % MOD;
    }

    suffix[n] = 1;
    for i in (0..n).rev() {
        suffix[i] = suffix[i + 1] * (((x - (i as i64 + 1)) % MOD + MOD) % MOD) % MOD;
    }

    let mut fact = vec![0i64; n + 1];
    let mut inv_fact = vec![0i64; n + 1];
    fact[0] = 1;
    for i in 1..=n {
        fact[i] = fact[i - 1] * i as i64 % MOD;
    }

    let mut base = fact[n];
    let mut e = MOD - 2;
    let mut inv_n = 1i64;
    while e > 0 {
        if e & 1 == 1 {
            inv_n = inv_n * base % MOD;
        }
        base = base * base % MOD;
        e >>= 1;
    }
    inv_fact[n] = inv_n;
    for i in (0..n).rev() {
        inv_fact[i] = inv_fact[i + 1] * (i as i64 + 1) % MOD;
    }

    let mut result: i64 = 0;
    for i in 0..n {
        let num = prefix[i] * suffix[i + 1] % MOD;
        let mut denom = inv_fact[i] * inv_fact[n - 1 - i] % MOD;
        if (n - 1 - i) % 2 == 1 {
            denom = (MOD - denom) % MOD;
        }
        result = (result + values[i] % MOD * num % MOD * denom % MOD) % MOD;
    }

    (result % MOD + MOD) % MOD
}

fn main() {
    let n_target: i64 = 1_112_131_415;

    let mut cache: FxHashMap<u64, usize> = FxHashMap::default();
    let mut results: Vec<[i32; F_SIZE]> = Vec::new();
    let empty: [i32; 0] = [];
    let f_idx = f_rec(0, &empty, &mut cache, &mut results);
    let f_result = &results[f_idx];

    let n_points = RC + 2;
    let mut sum_values = vec![0i64; n_points];

    for k in 1..=n_points {
        let mut s: i64 = 0;
        for i in 0..=k.min(F_SIZE - 1) {
            s = (s + f_result[i] as i64) % MOD;
        }
        sum_values[k - 1] = s;
    }

    let ans = lagrange_extrapolation(&sum_values, n_target);
    println!("{ans}");
}
