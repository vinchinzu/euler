// Problem 943 - Self Describing Sequences
//
// Compute sum of T(a, b, N) for all pairs (a, b) with 2 <= a, b <= 223, a != b,
// where N = 22332223332233, modulo 2233222333.
//
// Uses a recursive Kolakoski-like solver with memoization, parallelized over pairs.

use rayon::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Copy)]
struct CalcResult {
    count_a: u64,
    count_b: u64,
    next_state: u64,
}

impl CalcResult {
    fn total(self) -> u64 {
        self.count_a + self.count_b
    }
}

struct KolakoskiSolver {
    a: u64,
    b: u64,
    cache: HashMap<u64, CalcResult>,
}

impl KolakoskiSolver {
    fn new(a: u64, b: u64) -> Self {
        Self {
            a,
            b,
            cache: HashMap::new(),
        }
    }

    fn calc(&mut self, state: u64, level: u32, maxlen: u64) -> CalcResult {
        if maxlen == 0 {
            return CalcResult {
                count_a: 0,
                count_b: 0,
                next_state: state,
            };
        }

        let length_bit = 2u64.wrapping_shl(level);
        let bit = state & length_bit;
        let run_len = if bit != 0 { self.b } else { self.a };
        let count = if run_len < maxlen { run_len } else { maxlen };

        if level == 0 {
            if (state & 1) == 0 {
                return CalcResult {
                    count_a: count,
                    count_b: 0,
                    next_state: state ^ 1,
                };
            } else {
                return CalcResult {
                    count_a: 0,
                    count_b: count,
                    next_state: state ^ 1,
                };
            }
        }

        let mut produced_a: u64 = 0;
        let mut produced_b: u64 = 0;
        let mut substate = state ^ bit;

        for _ in 0..count {
            let child_key = substate.wrapping_add(2u64.wrapping_shl(level));

            let child = if let Some(cached) = self.cache.get(&child_key) {
                let child_total = cached.total();
                if produced_a + produced_b + child_total <= maxlen {
                    *cached
                } else {
                    self.calc(substate, level - 1, maxlen - (produced_a + produced_b))
                }
            } else {
                self.calc(substate, level - 1, maxlen - (produced_a + produced_b))
            };

            produced_a += child.count_a;
            produced_b += child.count_b;
            substate = child.next_state;
        }

        let res = CalcResult {
            count_a: produced_a,
            count_b: produced_b,
            next_state: substate ^ bit ^ (1u64 << level),
        };

        let cache_key = state.wrapping_add(4u64.wrapping_shl(level));
        self.cache.insert(cache_key, res);
        res
    }
}

fn evaluate_counts(a: u64, b: u64, limit: u64) -> (u64, u64) {
    let mut solver = KolakoskiSolver::new(a, b);
    let mut res = CalcResult {
        count_a: 0,
        count_b: 0,
        next_state: 0,
    };

    for level in 0..64u32 {
        res = solver.calc(0, level, limit);
        if res.total() >= limit {
            break;
        }
    }

    (res.count_a, res.count_b)
}

fn compute_t(a: u64, b: u64, limit: u64) -> u64 {
    let (count_a, count_b) = evaluate_counts(a, b, limit);
    count_a * a + count_b * b
}

fn main() {
    const MOD: u64 = 2_233_222_333;
    const N: u64 = 22_332_223_332_233;

    // Verify test cases
    debug_assert_eq!(compute_t(2, 3, 10), 25);
    debug_assert_eq!(compute_t(4, 2, 10_000), 30_004);

    // Build work units: all (a, b) pairs with 2 <= a, b <= 223, a != b
    let pairs: Vec<(u64, u64)> = (2..=223u64)
        .flat_map(|a| (2..=223u64).filter(move |&b| b != a).map(move |b| (a, b)))
        .collect();

    let total: u64 = pairs
        .par_iter()
        .map(|&(a, b)| compute_t(a, b, N) % MOD)
        .reduce(|| 0, |acc, x| (acc + x) % MOD);

    println!("{}", total % MOD);
}
