// Project Euler 534 - Weak Queens
//
// Q(n, w) = ways to place n non-attacking queens on n x n board
// with limited attack range. Sum of Q(14, w) for w = 0..13.
// Profile DP for small k, DFS with bitmask pruning for large k.

use std::collections::HashMap;

const NN: usize = 14;

// DFS for large k
fn dfs_fast(row: usize, cols: &mut [usize; NN], k: usize, count: &mut i64) {
    if row == NN {
        *count += 1;
        return;
    }
    let mut blocked = 0u32;
    let start = if row >= k { row - k } else { 0 };
    for r in start..row {
        let d = row - r;
        let c = cols[r];
        blocked |= 1 << c;
        if c + d < NN { blocked |= 1 << (c + d); }
        if c >= d { blocked |= 1 << (c - d); }
    }
    let mut avail = ((1u32 << NN) - 1) & !blocked;
    while avail != 0 {
        let bit = avail & avail.wrapping_neg();
        avail &= avail - 1;
        cols[row] = bit.trailing_zeros() as usize;
        dfs_fast(row + 1, cols, k, count);
    }
}

fn check_valid(cols: &[usize], pos: usize, c: usize) -> bool {
    for j in 0..pos {
        let dr = pos - j;
        let dc = (c as isize - cols[j] as isize).unsigned_abs();
        if dc == 0 || dc == dr { return false; }
    }
    true
}

fn gen_configs(k: usize, pos: usize, tmp: &mut [usize; NN], configs: &mut Vec<[usize; NN]>) {
    if pos == k {
        configs.push(*tmp);
        return;
    }
    for c in 0..NN {
        if check_valid(&tmp[..pos], pos, c) {
            tmp[pos] = c;
            gen_configs(k, pos + 1, tmp, configs);
        }
    }
}

fn encode_config(cols: &[usize], len: usize) -> u64 {
    let mut key = 0u64;
    let mut base = 1u64;
    for i in 0..len {
        key += cols[i] as u64 * base;
        base *= NN as u64;
    }
    key
}

fn solve_dp(k: usize) -> i64 {
    if k == 0 {
        let mut r = 1i64;
        for _ in 0..NN { r *= NN as i64; }
        return r;
    }

    let mut configs = Vec::new();
    let mut tmp = [0usize; NN];
    gen_configs(k, 0, &mut tmp, &mut configs);
    let nconfigs = configs.len();

    let mut config_map = HashMap::new();
    for (i, cfg) in configs.iter().enumerate() {
        config_map.insert(encode_config(cfg, k), i);
    }

    // Build transition list
    let mut trans_target = Vec::new();
    let mut trans_offset = vec![0usize; nconfigs + 1];

    for i in 0..nconfigs {
        trans_offset[i] = trans_target.len();
        let cols = &configs[i];
        for c in 0..NN {
            let mut ok = true;
            for j in 0..k {
                let dr = k - j;
                let dc = (c as isize - cols[j] as isize).unsigned_abs();
                if dc == 0 || dc == dr { ok = false; break; }
            }
            if ok {
                let mut new_cols = [0usize; NN];
                for j in 1..k { new_cols[j - 1] = cols[j]; }
                new_cols[k - 1] = c;
                if let Some(&target) = config_map.get(&encode_config(&new_cols, k)) {
                    trans_target.push(target);
                }
            }
        }
    }
    trans_offset[nconfigs] = trans_target.len();

    let mut dp = vec![1i64; nconfigs];

    for _ in k..NN {
        let mut dp2 = vec![0i64; nconfigs];
        for i in 0..nconfigs {
            if dp[i] == 0 { continue; }
            for t in trans_offset[i]..trans_offset[i + 1] {
                dp2[trans_target[t]] += dp[i];
            }
        }
        dp = dp2;
    }

    dp.iter().sum()
}

fn main() {
    let mut ans: i64 = 0;
    for k in 0..NN {
        let q = if k <= 8 {
            solve_dp(k)
        } else {
            let mut cols = [0usize; NN];
            let mut count = 0i64;
            dfs_fast(0, &mut cols, k, &mut count);
            count
        };
        ans += q;
    }
    println!("{ans}");
}
