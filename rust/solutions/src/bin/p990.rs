// Problem 990: Addition Equations
// Ported from the Python reference: digit DP with signed carries for A(50) mod 10^9+7.

use std::collections::HashMap;

const MOD: i64 = 1_000_000_007;
const MAX_N: usize = 50;
const MAX_TERMS: usize = (MAX_N + 1) / 2;
const MAX_CARRY: i32 = 25;
const CARRY_MIN: i32 = -MAX_CARRY;
const CARRY_MAX: i32 = MAX_CARRY;

fn build_binom(limit: usize) -> Vec<Vec<i64>> {
    let mut comb = vec![vec![0i64; limit + 1]; limit + 1];
    for n in 0..=limit {
        comb[n][0] = 1;
        if n <= limit {
            comb[n][n] = 1;
        }
        for k in 1..n {
            comb[n][k] = (comb[n - 1][k - 1] + comb[n - 1][k]) % MOD;
        }
    }
    comb
}

fn convolve_small(poly: &[i64], width: usize) -> Vec<i64> {
    let mut out = vec![0i64; poly.len() + width - 1];
    for (i, &value) in poly.iter().enumerate() {
        if value == 0 {
            continue;
        }
        for digit in 0..width {
            out[i + digit] = (out[i + digit] + value) % MOD;
        }
    }
    out
}

/// F[p][q][m] = ways to write m as sum of p vars in [0,9] and q vars in [0,8].
fn build_sum_tables(limit: usize) -> Vec<Vec<Vec<i64>>> {
    let mut ways_0_to_9: Vec<Vec<i64>> = vec![Vec::new(); limit + 1];
    ways_0_to_9[0] = vec![1];
    for p in 1..=limit {
        ways_0_to_9[p] = convolve_small(&ways_0_to_9[p - 1], 10);
    }

    let mut tables = vec![vec![Vec::new(); limit + 1]; limit + 1];
    for p in 0..=limit {
        tables[p][0] = ways_0_to_9[p].clone();
        for q in 1..=limit {
            tables[p][q] = convolve_small(&tables[p][q - 1], 9);
        }
    }
    tables
}

type Transition = (usize, usize, i32, i64);

fn transitions(
    active_left: usize,
    active_right: usize,
    carry: i32,
    binom: &[Vec<i64>],
    sum_tables: &[Vec<Vec<i64>>],
    cache: &mut HashMap<(usize, usize, i32), Vec<Transition>>,
) -> Vec<Transition> {
    let key = (active_left, active_right, carry);
    if let Some(cached) = cache.get(&key) {
        return cached.clone();
    }

    if active_left == 0 && active_right == 0 {
        cache.insert(key, Vec::new());
        return Vec::new();
    }

    let mut result = Vec::new();

    for next_left in 0..=active_left {
        let choose_left = binom[active_left][next_left];
        let ending_left = active_left - next_left;

        for next_right in 0..=active_right {
            let choose_terms = (choose_left * binom[active_right][next_right]) % MOD;
            let continuing = next_left + next_right;
            let ending = (active_left - next_left) + (active_right - next_right);
            let counts = &sum_tables[continuing][ending];

            let base = -carry - (ending_left as i32) + 9 * (active_right as i32);

            for next_carry in CARRY_MIN..=CARRY_MAX {
                let index = 10 * next_carry + base;
                if index >= 0 && (index as usize) < counts.len() {
                    let ways = counts[index as usize];
                    if ways != 0 {
                        let weight = (choose_terms * ways) % MOD;
                        result.push((next_left, next_right, next_carry, weight));
                    }
                }
            }
        }
    }

    cache.insert(key, result.clone());
    result
}

fn solve(limit: usize) -> i64 {
    let binom = build_binom(MAX_TERMS);
    let sum_tables = build_sum_tables(2 * MAX_TERMS);
    let mut trans_cache: HashMap<(usize, usize, i32), Vec<Transition>> = HashMap::new();

    // dp[length][(a, b, c)] = ways using exactly 'length' characters so far.
    let mut dp: Vec<HashMap<(usize, usize, i32), i64>> = vec![HashMap::new(); limit + 1];

    for left_terms in 1..=MAX_TERMS {
        for right_terms in 1..=MAX_TERMS {
            let base_length = left_terms + right_terms - 1; // plus signs + '='
            if base_length <= limit {
                let state = (left_terms, right_terms, 0i32);
                let entry = dp[base_length].entry(state).or_insert(0);
                *entry = (*entry + 1) % MOD;
            }
        }
    }

    let mut answer = 0i64;
    for used_length in 0..=limit {
        let current: Vec<((usize, usize, i32), i64)> =
            dp[used_length].iter().map(|(&k, &v)| (k, v)).collect();
        if current.is_empty() {
            continue;
        }

        if let Some(&v) = dp[used_length].get(&(0, 0, 0)) {
            answer = (answer + v) % MOD;
        }

        for ((active_left, active_right, carry), ways_so_far) in current {
            if ways_so_far == 0 || (active_left == 0 && active_right == 0) {
                continue;
            }

            let next_length = used_length + active_left + active_right;
            if next_length > limit {
                continue;
            }

            let trans = transitions(
                active_left,
                active_right,
                carry,
                &binom,
                &sum_tables,
                &mut trans_cache,
            );
            for (next_left, next_right, next_carry, weight) in trans {
                let state = (next_left, next_right, next_carry);
                let entry = dp[next_length].entry(state).or_insert(0);
                *entry = (*entry + ways_so_far * weight) % MOD;
            }
        }
    }

    answer
}

fn main() {
    debug_assert_eq!(solve(3), 9);
    debug_assert_eq!(solve(5), 171);
    debug_assert_eq!(solve(7), 4878);
    println!("{}", solve(50));
}
