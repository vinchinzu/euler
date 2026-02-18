// Problem 923 - Young's Game B
//
// Port of the Python solution embedded in the original stub.
// Computes S(8, 64) mod 10^9+7.

use std::collections::HashMap;

const MOD: u64 = 1_000_000_007;

fn ceil_div(a: i64, b: i64) -> i64 {
    (a + b - 1) / b
}

fn reduced_hook(a: i64, b: i64, k: i64) -> (i64, i64) {
    // Compute Durfee size d
    let mut d: i64 = 0;
    for j in 0..k {
        let row_len = (k - j) * b;
        let start = j * a + 1;
        if row_len < start {
            continue;
        }
        let end = (j + 1) * a;
        let cand = if end < row_len { end } else { row_len };
        if cand > d {
            d = cand;
        }
    }
    let block_of_row_d = (d - 1) / a;
    let lambda_d = (k - block_of_row_d) * b;
    let m_val = lambda_d - d + 1;

    // Column height at column d
    let need_blocks = ceil_div(d, b);
    let last_block = k - need_blocks;
    let col_height = (last_block + 1) * a;
    let n_val = col_height - d + 1;

    (m_val, n_val)
}

#[derive(Debug)]
enum Classification {
    Int(i64),
    Hot(i64, i64), // (t, R)
}

fn classify_staircase(a: i64, b: i64, k: i64) -> Classification {
    let (m_val, n_val) = reduced_hook(a, b, k);
    if n_val == 1 {
        return Classification::Int(m_val - 1);
    }
    if m_val == 1 {
        return Classification::Int(-(n_val - 1));
    }
    let l = m_val - 2;
    let r = -(n_val - 2);
    let t = l - r; // = m_val + n_val - 4
    Classification::Hot(t, r)
}

fn counts_for_w(w: i64) -> (HashMap<i64, u64>, HashMap<(i64, i64), u64>) {
    let mut ints: HashMap<i64, u64> = HashMap::new();
    let mut hots: HashMap<(i64, i64), u64> = HashMap::new();

    for a in 1..w - 1 {
        for b in 1..w - a {
            let max_k = w - a - b;
            if max_k < 1 {
                continue;
            }
            for k in 1..=max_k {
                match classify_staircase(a, b, k) {
                    Classification::Int(v) => {
                        *ints.entry(v).or_insert(0) += 1;
                    }
                    Classification::Hot(t, r) => {
                        *hots.entry((t, r)).or_insert(0) += 1;
                    }
                }
            }
        }
    }
    (ints, hots)
}

fn solve(m: usize, w: i64) -> u64 {
    // Factorials and inverse factorials mod MOD
    let mut fact = vec![1u64; m + 1];
    for i in 1..=m {
        fact[i] = fact[i - 1] * i as u64 % MOD;
    }
    let mut invfact = vec![1u64; m + 1];
    invfact[m] = mod_pow(fact[m], MOD - 2);
    for i in (1..=m).rev() {
        invfact[i - 1] = invfact[i] * i as u64 % MOD;
    }

    let (ints, hots) = counts_for_w(w);

    // DP over hot components in descending temperature.
    // State: dp_hot[used][parity] -> HashMap<sum_value, coeff>
    // parity 0 = Right to move, 1 = Down to move
    let mut dp_hot: Vec<[HashMap<i64, u64>; 2]> = Vec::with_capacity(m + 1);
    for _ in 0..=m {
        dp_hot.push([HashMap::new(), HashMap::new()]);
    }
    dp_hot[0][0].insert(0, 1);

    let mut hot_types: Vec<(i64, i64, u64)> = hots
        .iter()
        .map(|(&(t, r), &c)| (t, r, c))
        .collect();
    hot_types.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));

    for &(t, r_val, c) in &hot_types {
        // poly[k] = c^k / k! mod MOD
        let mut poly = vec![0u64; m + 1];
        poly[0] = 1;
        let mut p = 1u64;
        for k in 1..=m {
            p = p * (c % MOD) % MOD;
            poly[k] = p % MOD * invfact[k] % MOD;
        }

        let mut new_dp: Vec<[HashMap<i64, u64>; 2]> = Vec::with_capacity(m + 1);
        for _ in 0..=m {
            new_dp.push([HashMap::new(), HashMap::new()]);
        }

        for used in 0..=m {
            for parity in 0..2usize {
                let cur = &dp_hot[used][parity];
                if cur.is_empty() {
                    continue;
                }
                for (&s, &coeff) in cur.iter() {
                    for k in 0..=(m - used) {
                        let mult = poly[k];
                        if mult == 0 {
                            continue;
                        }
                        let right_turns = ((k as i64) + 1 - parity as i64) / 2;
                        let delta = k as i64 * r_val + right_turns * t;
                        let nu = used + k;
                        let np = parity ^ (k & 1);
                        let ns = s + delta;
                        let entry = new_dp[nu][np].entry(ns).or_insert(0);
                        *entry = (*entry + coeff % MOD * mult % MOD) % MOD;
                    }
                }
            }
        }
        dp_hot = new_dp;
    }

    // DP over integer-valued components
    // State: dp_int[used] -> HashMap<sum_value, coeff>
    let mut dp_int: Vec<HashMap<i64, u64>> = Vec::with_capacity(m + 1);
    for _ in 0..=m {
        dp_int.push(HashMap::new());
    }
    dp_int[0].insert(0, 1);

    for (&v, &c) in &ints {
        let mut poly = vec![0u64; m + 1];
        poly[0] = 1;
        let mut p = 1u64;
        for k in 1..=m {
            p = p * (c % MOD) % MOD;
            poly[k] = p % MOD * invfact[k] % MOD;
        }

        let mut new_dp: Vec<HashMap<i64, u64>> = Vec::with_capacity(m + 1);
        for _ in 0..=m {
            new_dp.push(HashMap::new());
        }

        for used in 0..=m {
            let cur = &dp_int[used];
            if cur.is_empty() {
                continue;
            }
            for (&s, &coeff) in cur.iter() {
                for k in 0..=(m - used) {
                    let mult = poly[k];
                    if mult == 0 {
                        continue;
                    }
                    let nu = used + k;
                    let ns = s + k as i64 * v;
                    let entry = new_dp[nu].entry(ns).or_insert(0);
                    *entry = (*entry + coeff % MOD * mult % MOD) % MOD;
                }
            }
        }
        dp_int = new_dp;
    }

    // Combine hot and integer parts
    let mut multiset_count = 0u64;
    for j in 0..=m {
        let rem = m - j;
        for parity in 0..2usize {
            let hot_map = &dp_hot[j][parity];
            if hot_map.is_empty() {
                continue;
            }
            let int_map = &dp_int[rem];
            if int_map.is_empty() {
                continue;
            }
            for (&s_hot, &ch) in hot_map.iter() {
                for (&s_int, &ci) in int_map.iter() {
                    let total = s_hot + s_int;
                    if total > 0 || (total == 0 && parity == 1) {
                        multiset_count = (multiset_count + ch % MOD * (ci % MOD) % MOD) % MOD;
                    }
                }
            }
        }
    }

    // Convert EGF to ordered sequences
    multiset_count % MOD * fact[m] % MOD
}

fn mod_pow(mut base: u64, mut exp: u64) -> u64 {
    let mut result = 1u64;
    base %= MOD;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % MOD;
        }
        base = base * base % MOD;
        exp >>= 1;
    }
    result
}

fn main() {
    // Debug: verify test cases
    debug_assert_eq!(solve(2, 4), 7);
    debug_assert_eq!(solve(3, 9), 315319);

    println!("{}", solve(8, 64));
}
