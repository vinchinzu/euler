// Project Euler 534 - Weak Queens
//
// Q(n, w) = ways to place n non-attacking queens on n x n board
// with limited attack range. Sum of Q(14, w) for w = 0..13.
// Profile DP for k <= 7, branchless incremental bitmask DFS with row-0 symmetry for k >= 8.
// Parallelized over flattened (k, c0) tasks sorted by workload.

use fxhash::FxHashMap;
use rayon::prelude::*;

const NN: usize = 14;

// DFS with incremental bitmasks for large k
fn dfs_inc(
    row: usize,
    cols: &mut [u32; NN],
    col_mask: u32,
    ld_mask: u32,
    rd_mask: u32,
    k: usize,
    count: &mut i64,
) {
    let full_mask = (1u32 << NN) - 1;
    let blocked = col_mask | ld_mask | rd_mask;
    let mut avail = full_mask & !blocked;
    if row == NN - 1 {
        *count += avail.count_ones() as i64;
        return;
    }
    if row >= k {
        let exp_bit = cols[row - k];
        let d = (k + 1) as u32;
        let exp_col_mask = !exp_bit;
        let exp_ld_mask = !(exp_bit >> d);
        let exp_rd_mask = !((exp_bit << d) & full_mask);

        while avail != 0 {
            let bit = avail & avail.wrapping_neg();
            avail &= avail - 1;
            if row < NN - k - 1 {
                cols[row] = bit;
            }

            let next_col = (col_mask | bit) & exp_col_mask;
            let next_ld = ((ld_mask | bit) >> 1) & exp_ld_mask;
            let next_rd = (((rd_mask | bit) << 1) & full_mask) & exp_rd_mask;

            dfs_inc(row + 1, cols, next_col, next_ld, next_rd, k, count);
        }
    } else {
        while avail != 0 {
            let bit = avail & avail.wrapping_neg();
            avail &= avail - 1;
            if row < NN - k - 1 {
                cols[row] = bit;
            }

            let next_col = col_mask | bit;
            let next_ld = (ld_mask | bit) >> 1;
            let next_rd = ((rd_mask | bit) << 1) & full_mask;

            dfs_inc(row + 1, cols, next_col, next_ld, next_rd, k, count);
        }
    }
}

#[derive(Clone, Copy)]
struct Config {
    key: u32,
    first_col: u8,
    blocked: u16,
}

fn gen_configs(
    k: usize,
    pos: usize,
    first_col: u8,
    key: u32,
    base: u32,
    col_mask: u32,
    ld_mask: u32,
    rd_mask: u32,
    configs: &mut Vec<Config>,
) {
    if pos == k {
        let blocked = (col_mask | ld_mask | rd_mask) as u16;
        configs.push(Config { key, first_col, blocked });
        return;
    }
    let full_mask = (1u32 << NN) - 1;
    let mut avail = full_mask & !(col_mask | ld_mask | rd_mask);
    while avail != 0 {
        let bit = avail & avail.wrapping_neg();
        avail &= avail - 1;
        let c = bit.trailing_zeros();
        let fc = if pos == 0 { c as u8 } else { first_col };
        gen_configs(
            k,
            pos + 1,
            fc,
            key + c * base,
            base * NN as u32,
            col_mask | bit,
            (ld_mask | bit) >> 1,
            ((rd_mask | bit) << 1) & full_mask,
            configs,
        );
    }
}

fn solve_dp(k: usize) -> i64 {
    if k == 0 {
        let mut r = 1i64;
        for _ in 0..NN { r *= NN as i64; }
        return r;
    }

    let mut configs = Vec::new();
    gen_configs(k, 0, 0, 0, 1, 0, 0, 0, &mut configs);
    let nconfigs = configs.len();

    let mut config_map = FxHashMap::with_capacity_and_hasher(nconfigs * 2, Default::default());
    for (i, cfg) in configs.iter().enumerate() {
        config_map.insert(cfg.key, i as u32);
    }

    // Build transition list
    let mut trans_target = Vec::with_capacity(nconfigs * 4);
    let mut trans_offset = vec![0u32; nconfigs + 1];

    let full_mask = (1u32 << NN) - 1;
    let pow_k = (NN as u32).pow((k - 1) as u32);

    for i in 0..nconfigs {
        trans_offset[i] = trans_target.len() as u32;
        let cfg = &configs[i];
        let base_key = (cfg.key - cfg.first_col as u32) / NN as u32;

        let mut avail = full_mask & !(cfg.blocked as u32);
        while avail != 0 {
            let bit = avail & avail.wrapping_neg();
            avail &= avail - 1;
            let c = bit.trailing_zeros();
            let new_key = base_key + c * pow_k;
            if let Some(&target) = config_map.get(&new_key) {
                trans_target.push(target);
            }
        }
    }
    trans_offset[nconfigs] = trans_target.len() as u32;

    let mut dp = vec![1i64; nconfigs];
    let mut dp2 = vec![0i64; nconfigs];

    for _ in k..NN {
        for x in dp2.iter_mut() { *x = 0; }
        for i in 0..nconfigs {
            let v = dp[i];
            if v == 0 { continue; }
            for t in (trans_offset[i] as usize)..(trans_offset[i + 1] as usize) {
                dp2[trans_target[t] as usize] += v;
            }
        }
        std::mem::swap(&mut dp, &mut dp2);
    }

    dp.iter().sum()
}

enum Task {
    Dp(usize),
    Dfs { k: usize, c0: usize },
}

fn run_task(task: &Task) -> i64 {
    match *task {
        Task::Dp(k) => solve_dp(k),
        Task::Dfs { k, c0 } => {
            let full_mask = (1u32 << NN) - 1;
            let mut cols = [0u32; NN];
            let bit = 1u32 << c0;
            if 0 < NN - k - 1 {
                cols[0] = bit;
            }
            let next_col = bit;
            let next_ld = bit >> 1;
            let next_rd = (bit << 1) & full_mask;
            let mut count = 0i64;
            dfs_inc(1, &mut cols, next_col, next_ld, next_rd, k, &mut count);
            count * 2
        }
    }
}

fn main() {
    let mut tasks = Vec::with_capacity(50);

    // Schedule heaviest tasks first to minimize thread starvation
    tasks.push(Task::Dp(7));
    for c0 in 0..NN / 2 {
        tasks.push(Task::Dfs { k: 8, c0 });
    }
    tasks.push(Task::Dp(6));
    for c0 in 0..NN / 2 {
        tasks.push(Task::Dfs { k: 9, c0 });
    }
    for c0 in 0..NN / 2 {
        tasks.push(Task::Dfs { k: 10, c0 });
    }
    for c0 in 0..NN / 2 {
        tasks.push(Task::Dfs { k: 11, c0 });
    }
    for c0 in 0..NN / 2 {
        tasks.push(Task::Dfs { k: 12, c0 });
    }
    for c0 in 0..NN / 2 {
        tasks.push(Task::Dfs { k: 13, c0 });
    }
    tasks.push(Task::Dp(5));
    tasks.push(Task::Dp(4));
    tasks.push(Task::Dp(3));
    tasks.push(Task::Dp(2));
    tasks.push(Task::Dp(1));
    tasks.push(Task::Dp(0));

    let ans: i64 = tasks.into_par_iter().map(|t| run_task(&t)).sum();
    println!("{ans}");
}
