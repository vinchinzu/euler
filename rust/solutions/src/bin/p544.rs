// Project Euler 544 - Chromatic Polynomial
//
// Compute sum_{k=1}^{N} f(R,C,k) mod M for an R x C grid graph using DP
// with Lagrange interpolation to extrapolate cumulative sum.

use std::collections::HashMap;

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
    cache: &mut HashMap<u64, Vec<i64>>,
) -> Vec<i64> {
    // Normalize: keep only last R colors, relabel from 1
    let start = if colors.len() > R_VAL { colors.len() - R_VAL } else { 0 };
    let mut mapping = vec![0i32; 2 * R_VAL as i32 as usize + 2];
    let mut curr_max: i32 = 0;
    let mut last_colors = Vec::with_capacity(R_VAL);

    for i in start..colors.len() {
        let c = colors[i] as usize;
        if mapping[c] == 0 {
            curr_max += 1;
            mapping[c] = curr_max;
        }
        last_colors.push(mapping[c]);
    }
    let max_color = curr_max;

    let key = encode_state(r, &last_colors);
    if let Some(v) = cache.get(&key) {
        return v.clone();
    }

    let nlast = last_colors.len();
    let mut f_arr = vec![0i64; F_SIZE];

    if r == RC {
        for v in f_arr.iter_mut() {
            *v = 1;
        }
        cache.insert(key, f_arr.clone());
        return f_arr;
    }

    for color in 1..=(max_color + 1) {
        // Check horizontal neighbor
        if r % R_VAL != 0 && nlast > 0 && color == last_colors[nlast - 1] {
            continue;
        }
        // Check vertical neighbor
        if nlast >= R_VAL && color == last_colors[nlast - R_VAL] {
            continue;
        }

        let mut pass_colors = last_colors.clone();
        pass_colors.push(color);

        let next_f = f_rec(r + 1, &pass_colors, cache);

        for n_idx in 0..F_SIZE {
            let choices = if color == max_color + 1 {
                n_idx as i64 - color as i64 + 1
            } else {
                1
            };
            if choices <= 0 { continue; }
            f_arr[n_idx] = (f_arr[n_idx] + choices % MOD * next_f[n_idx] % MOD) % MOD;
        }
    }

    cache.insert(key, f_arr.clone());
    f_arr
}

fn lagrange_extrapolation(values: &[i64], x: i64) -> i64 {
    let n = values.len();
    let mut prefix = vec![0i64; n + 1];
    let mut suffix = vec![0i64; n + 1];

    prefix[0] = 1;
    for i in 0..n {
        prefix[i + 1] = (prefix[i] as i128 * (((x - (i as i64 + 1)) % MOD + MOD) % MOD) as i128 % MOD as i128) as i64;
    }

    suffix[n] = 1;
    for i in (0..n).rev() {
        suffix[i] = (suffix[i + 1] as i128 * (((x - (i as i64 + 1)) % MOD + MOD) % MOD) as i128 % MOD as i128) as i64;
    }

    let mut fact = vec![0i64; n + 1];
    let mut inv_fact = vec![0i64; n + 1];
    fact[0] = 1;
    for i in 1..=n {
        fact[i] = (fact[i - 1] as i128 * i as i128 % MOD as i128) as i64;
    }

    // Fermat inverse of fact[n]
    let mut base = fact[n] as i128;
    let mut e = MOD - 2;
    let mut inv_n: i128 = 1;
    while e > 0 {
        if e & 1 == 1 { inv_n = inv_n * base % MOD as i128; }
        base = base * base % MOD as i128;
        e >>= 1;
    }
    inv_fact[n] = inv_n as i64;
    for i in (0..n).rev() {
        inv_fact[i] = (inv_fact[i + 1] as i128 * (i as i128 + 1) % MOD as i128) as i64;
    }

    let mut result: i64 = 0;
    for i in 0..n {
        let num = (prefix[i] as i128 * suffix[i + 1] as i128 % MOD as i128) as i64;
        let mut denom = (inv_fact[i] as i128 * inv_fact[n - 1 - i] as i128 % MOD as i128) as i64;
        if (n - 1 - i) % 2 == 1 {
            denom = (MOD - denom) % MOD;
        }
        result = (result as i128 + (values[i] % MOD) as i128 * num as i128 % MOD as i128 * denom as i128 % MOD as i128) as i64 % MOD;
    }

    ((result % MOD) + MOD) % MOD
}

fn main() {
    let n_target: i64 = 1_112_131_415;

    let mut cache: HashMap<u64, Vec<i64>> = HashMap::new();
    let empty: Vec<i32> = vec![];
    let f_result = f_rec(0, &empty, &mut cache);

    let n_points = RC + 2;
    let mut sum_values = vec![0i64; n_points];

    for k in 1..=n_points {
        let mut s: i64 = 0;
        for i in 0..=k.min(F_SIZE - 1) {
            s = (s + f_result[i]) % MOD;
        }
        sum_values[k - 1] = s;
    }

    let ans = lagrange_extrapolation(&sum_values, n_target);
    println!("{ans}");
}
