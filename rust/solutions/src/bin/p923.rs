// Problem 923 - Young's Game B
//
// Optimized: replaced HashMaps with Vec for bounded integer keys (CLAUDE.md Rule #2).
// Computes S(8, 64) mod 10^9+7.

const MOD: u64 = 1_000_000_007;

const INT_OFFSET: usize = 62;
const INT_SIZE: usize = 124;
const HOT_T_SIZE: usize = 61;
const HOT_R_OFFSET: usize = 48;
const HOT_R_SIZE: usize = 49;
const SUM_OFFSET: usize = 500;
const SUM_SIZE: usize = 1300;

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

fn counts_for_w(w: i64) -> (Vec<u64>, Vec<Vec<u64>>) {
    let mut ints = vec![0u64; INT_SIZE];
    let mut hots = vec![vec![0u64; HOT_R_SIZE]; HOT_T_SIZE];

    for a in 1..w - 1 {
        for b in 1..w - a {
            let max_k = w - a - b;
            if max_k < 1 {
                continue;
            }
            for k in 1..=max_k {
                match classify_staircase(a, b, k) {
                    Classification::Int(v) => {
                        let idx = (v + INT_OFFSET as i64) as usize;
                        ints[idx] += 1;
                    }
                    Classification::Hot(t, r) => {
                        let t_idx = t as usize;
                        let r_idx = (r + HOT_R_OFFSET as i64) as usize;
                        hots[t_idx][r_idx] += 1;
                    }
                }
            }
        }
    }
    (ints, hots)
}

fn solve(m: usize, w: i64) -> u64 {
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

    let mut dp_hot: Vec<[Vec<u64>; 2]> = Vec::with_capacity(m + 1);
    for _ in 0..=m {
        dp_hot.push([vec![0u64; SUM_SIZE], vec![0u64; SUM_SIZE]]);
    }
    dp_hot[0][0][SUM_OFFSET] = 1;

    let mut hot_types: Vec<(usize, usize, u64)> = Vec::new();
    for t in 0..HOT_T_SIZE {
        for r_idx in 0..HOT_R_SIZE {
            let c = hots[t][r_idx];
            if c > 0 {
                hot_types.push((t, r_idx, c));
            }
        }
    }
    hot_types.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));

    for &(t, r_idx, c) in &hot_types {
        let r_val = r_idx as i64 - HOT_R_OFFSET as i64;
        let t_val = t as i64;
        
        let mut poly = vec![0u64; m + 1];
        poly[0] = 1;
        let mut p = 1u64;
        let c_mod = c % MOD;
        for k in 1..=m {
            p = p * c_mod % MOD;
            poly[k] = p * invfact[k] % MOD;
        }

        let mut new_dp: Vec<[Vec<u64>; 2]> = Vec::with_capacity(m + 1);
        for _ in 0..=m {
            new_dp.push([vec![0u64; SUM_SIZE], vec![0u64; SUM_SIZE]]);
        }

        for used in 0..=m {
            for parity in 0..2 {
                let cur = &dp_hot[used][parity];
                for s_idx in 0..SUM_SIZE {
                    let coeff = cur[s_idx];
                    if coeff == 0 {
                        continue;
                    }
                    let s = s_idx as i64 - SUM_OFFSET as i64;
                    for k in 0..=(m - used) {
                        let mult = poly[k];
                        if mult == 0 {
                            continue;
                        }
                        let right_turns = ((k as i64) + 1 - parity as i64) / 2;
                        let delta = k as i64 * r_val + right_turns * t_val;
                        let nu = used + k;
                        let np = parity ^ (k & 1);
                        let ns = s + delta;
                        let ns_idx = (ns + SUM_OFFSET as i64) as usize;
                        new_dp[nu][np][ns_idx] = (new_dp[nu][np][ns_idx] + coeff * mult) % MOD;
                    }
                }
            }
        }
        dp_hot = new_dp;
    }

    let mut dp_int: Vec<Vec<u64>> = Vec::with_capacity(m + 1);
    for _ in 0..=m {
        dp_int.push(vec![0u64; SUM_SIZE]);
    }
    dp_int[0][SUM_OFFSET] = 1;

    for v_idx in 0..INT_SIZE {
        let c = ints[v_idx];
        if c == 0 {
            continue;
        }
        let v = v_idx as i64 - INT_OFFSET as i64;
        
        let mut poly = vec![0u64; m + 1];
        poly[0] = 1;
        let mut p = 1u64;
        let c_mod = c % MOD;
        for k in 1..=m {
            p = p * c_mod % MOD;
            poly[k] = p * invfact[k] % MOD;
        }

        let mut new_dp: Vec<Vec<u64>> = Vec::with_capacity(m + 1);
        for _ in 0..=m {
            new_dp.push(vec![0u64; SUM_SIZE]);
        }

        for used in 0..=m {
            let cur = &dp_int[used];
            for s_idx in 0..SUM_SIZE {
                let coeff = cur[s_idx];
                if coeff == 0 {
                    continue;
                }
                let s = s_idx as i64 - SUM_OFFSET as i64;
                for k in 0..=(m - used) {
                    let mult = poly[k];
                    if mult == 0 {
                        continue;
                    }
                    let nu = used + k;
                    let ns = s + k as i64 * v;
                    let ns_idx = (ns + SUM_OFFSET as i64) as usize;
                    new_dp[nu][ns_idx] = (new_dp[nu][ns_idx] + coeff * mult) % MOD;
                }
            }
        }
        dp_int = new_dp;
    }

    let mut multiset_count = 0u64;
    for j in 0..=m {
        let rem = m - j;
        for parity in 0..2 {
            let hot_map = &dp_hot[j][parity];
            let int_map = &dp_int[rem];
            
            for s_hot_idx in 0..SUM_SIZE {
                let ch = hot_map[s_hot_idx];
                if ch == 0 {
                    continue;
                }
                let s_hot = s_hot_idx as i64 - SUM_OFFSET as i64;
                
                for s_int_idx in 0..SUM_SIZE {
                    let ci = int_map[s_int_idx];
                    if ci == 0 {
                        continue;
                    }
                    let s_int = s_int_idx as i64 - SUM_OFFSET as i64;
                    let total = s_hot + s_int;
                    if total > 0 || (total == 0 && parity == 1) {
                        multiset_count = (multiset_count + ch * ci) % MOD;
                    }
                }
            }
        }
    }

    multiset_count * fact[m] % MOD
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
