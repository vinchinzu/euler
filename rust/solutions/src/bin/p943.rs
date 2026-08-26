// Problem 943 - Self Describing Sequences
//
// Compute sum of T(a, b, N) for all pairs (a, b) with 2 <= a, b <= 223, a != b,
// where N = 22332223332233, modulo 2233222333.
//
// Uses a recursive Kolakoski-like solver with memoization, parallelized over pairs.
// (2,3) and (3,2) prefill a dense Vec of complete low-level blocks.

use rayon::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Copy)]
struct CalcResult {
    count_a: u64,
    count_b: u64,
    next_state: u64,
}

impl CalcResult {
    #[inline(always)]
    fn total(self) -> u64 {
        self.count_a + self.count_b
    }
}

struct KolakoskiSolver {
    a: u64,
    b: u64,
    dense: Vec<CalcResult>,
    cache: HashMap<u64, CalcResult>,
}

impl KolakoskiSolver {
    fn new(a: u64, b: u64) -> Self {
        let mut solver = Self {
            a,
            b,
            dense: Vec::new(),
            cache: HashMap::new(),
        };
        // Only (2,3) and (3,2) have ~2M distinct block states.
        if a * b == 6 {
            solver.prefill(1 << 21);
        }
        solver
    }

    fn prefill(&mut self, cap: usize) {
        self.dense.clear();
        self.dense.resize(
            cap,
            CalcResult {
                count_a: 0,
                count_b: 0,
                next_state: 0,
            },
        );
        let capu = cap as u64;
        let a = self.a;
        let b = self.b;
        let dense = self.dense.as_mut_slice();

        for level in 1u32..64 {
            let base = 4u64.wrapping_shl(level);
            if base >= capu {
                break;
            }
            let end = (8u64.wrapping_shl(level)).min(capu);
            let length_bit = 2u64.wrapping_shl(level);
            let flip = 1u64 << level;

            if level == 1 {
                for key in base..end {
                    let state = key - base;
                    let bit = state & length_bit;
                    let run_len = if bit != 0 { b } else { a };
                    let mut produced_a = 0u64;
                    let mut produced_b = 0u64;
                    let mut substate = state ^ bit;
                    for _ in 0..run_len {
                        let l0_run = if (substate & 2) != 0 { b } else { a };
                        if (substate & 1) == 0 {
                            produced_a += l0_run;
                        } else {
                            produced_b += l0_run;
                        }
                        substate ^= 1;
                    }
                    dense[key as usize] = CalcResult {
                        count_a: produced_a,
                        count_b: produced_b,
                        next_state: substate ^ bit ^ flip,
                    };
                }
            } else {
                let child_add = 2u64.wrapping_shl(level);
                for key in base..end {
                    let state = key - base;
                    let bit = state & length_bit;
                    let run_len = if bit != 0 { b } else { a };
                    let mut produced_a = 0u64;
                    let mut produced_b = 0u64;
                    let mut substate = state ^ bit;
                    for _ in 0..run_len {
                        let child = dense[(substate + child_add) as usize];
                        produced_a += child.count_a;
                        produced_b += child.count_b;
                        substate = child.next_state;
                    }
                    dense[key as usize] = CalcResult {
                        count_a: produced_a,
                        count_b: produced_b,
                        next_state: substate ^ bit ^ flip,
                    };
                }
            }
        }
    }

    #[inline(always)]
    fn cache_get(&self, key: u64) -> Option<CalcResult> {
        let i = key as usize;
        if i < self.dense.len() {
            // Prefill occupies every slot in [8, cap). Slot 0 is unused/zero.
            if i >= 8 {
                Some(self.dense[i])
            } else {
                None
            }
        } else {
            self.cache.get(&key).copied()
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

            let child = if let Some(cached) = self.cache_get(child_key) {
                let child_total = cached.total();
                if produced_a + produced_b + child_total <= maxlen {
                    cached
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
        if cache_key >= self.dense.len() as u64 {
            self.cache.insert(cache_key, res);
        }
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

    debug_assert_eq!(compute_t(2, 3, 10), 25);
    debug_assert_eq!(compute_t(4, 2, 10_000), 30_004);

    let pairs: Vec<(u64, u64)> = (2..=223u64)
        .flat_map(|a| (2..=223u64).filter(move |&b| b != a).map(move |b| (a, b)))
        .collect();

    let total: u64 = pairs
        .par_iter()
        .map(|&(a, b)| compute_t(a, b, N) % MOD)
        .reduce(|| 0, |acc, x| (acc + x) % MOD);

    println!("{}", total % MOD);
}
