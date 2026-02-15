// Project Euler 815 - Grouping Cards
// K=4 copies of N=60 values, find expected max distinct values present

use std::collections::HashMap;

const N: usize = 60;
const K: usize = 4;

fn ncr(n: usize, r: usize, table: &[[i64; K + 2]]) -> i64 {
    if r > n || r > K + 1 { return 0; }
    table[n][r]
}

fn index_from_counts(c: &[usize; K + 1], table: &[[i64; K + 2]]) -> usize {
    let mut total: i32 = -1;
    let mut idx: usize = 0;
    for i in 0..K {
        total += c[i] as i32 + 1;
        if total >= (i + 1) as i32 {
            idx += table[total as usize][i + 1] as usize;
        }
    }
    idx
}

fn main() {
    // Precompute nCr
    let mut ncr_table = vec![[0i64; K + 2]; N + K + 1];
    for n in 0..=N + K {
        ncr_table[n][0] = 1;
        for r in 1..=K + 1 {
            if r <= n {
                ncr_table[n][r] = ncr_table[n - 1][r - 1] + ncr_table[n - 1][r];
            }
        }
    }

    // Max index = C(64, 4) = 635376
    const MAX_INDEX: usize = 635376;
    const MAX_VAL: usize = N + 1;

    let mut cache: Vec<Vec<f64>> = vec![vec![f64::NAN; MAX_VAL]; MAX_INDEX];

    // Iterative DFS to avoid stack overflow
    fn solve(
        c: &mut [usize; K + 1],
        max_val: usize,
        cache: &mut Vec<Vec<f64>>,
        ncr_table: &[[i64; K + 2]],
    ) -> f64 {
        let idx = index_from_counts(c, ncr_table);
        if !cache[idx][max_val].is_nan() {
            return cache[idx][max_val];
        }

        let mut remaining = 0usize;
        for i in 0..K {
            remaining += (K - i) * c[i];
        }
        if remaining == 0 {
            cache[idx][max_val] = max_val as f64;
            return max_val as f64;
        }

        let mut result = 0.0;
        for t in 0..K {
            if c[t] > 0 {
                let count = c[t];
                c[t] -= 1;
                c[t + 1] += 1;
                let distinct = N - c[0] - c[K];
                let new_max = max_val.max(distinct);
                result += solve(c, new_max, cache, ncr_table) * (K - t) as f64 * count as f64;
                c[t] += 1;
                c[t + 1] -= 1;
            }
        }

        cache[idx][max_val] = result / remaining as f64;
        cache[idx][max_val]
    }

    let mut c = [0usize; K + 1];
    c[0] = N;

    let result = solve(&mut c, 0, &mut cache, &ncr_table);
    println!("{:.8}", result);
}
