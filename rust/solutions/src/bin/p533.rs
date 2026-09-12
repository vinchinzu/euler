// Project Euler 533 - Carmichael Lambda Function
//
// Sieve-based computation of lambda values mod 10^9.
// Segmented into disjoint chunks processed in parallel with Rayon.

use rayon::prelude::*;

const N: usize = 20_000_000;
const MOD: u64 = 1_000_000_000;

#[derive(Clone, Copy)]
struct PrimeData {
    p: u32,
    d: u32,
    logp: f64,
}

fn main() {
    // Sieve odd primes up to N
    let n_odds = (N + 1) / 2;
    let mut odd = vec![true; n_odds];
    odd[0] = false;
    let limit = ((N as f64).sqrt() as usize) | 1;
    let limit_idx = limit / 2;
    for i in 1..=limit_idx {
        if odd[i] {
            let p = 2 * i + 1;
            let mut j = (p * p) / 2;
            while j < n_odds {
                odd[j] = false;
                j += p;
            }
        }
    }

    let mut odd_primes = Vec::with_capacity(1_300_000);
    for i in 1..n_odds {
        if odd[i] {
            let p = (2 * i + 1) as u32;
            odd_primes.push(PrimeData {
                p,
                d: p - 1,
                logp: (p as f64).ln(),
            });
        }
    }
    drop(odd);

    let log2v = 2.0f64.ln();
    const NUM_CHUNKS: usize = 128;
    let chunk_size = (N - 1 + NUM_CHUNKS - 1) / NUM_CHUNKS;

    let (best_log, best_mod) = (0..NUM_CHUNKS)
        .into_par_iter()
        .map(|chunk_idx| {
            let start = 1 + chunk_idx * chunk_size;
            let end = (start + chunk_size).min(N);
            if start >= end {
                return (-1.0f64, 0u32);
            }
            let len = end - start;

            let mut logs = vec![0.0f64; len];
            let mut mods = vec![1u32; len];

            // Initialize p = 2 contribution
            for i in 0..len {
                let n = start + i;
                let e = if n & 1 == 1 { 1 } else { n.trailing_zeros() + 2 };
                unsafe {
                    *logs.get_unchecked_mut(i) = e as f64 * log2v;
                    *mods.get_unchecked_mut(i) = 1u32 << e;
                }
            }

            // Odd primes
            for prime in &odd_primes {
                let d = prime.d as usize;
                if d >= end {
                    break;
                }
                let pu = prime.p as u64;
                let logp = prime.logp;

                let first = if start <= d {
                    d
                } else {
                    ((start + d - 1) / d) * d
                };

                let mut n = first;
                while n < end {
                    let idx = n - start;
                    unsafe {
                        *logs.get_unchecked_mut(idx) += logp;
                        let m = mods.get_unchecked_mut(idx);
                        *m = ((*m as u64 * pu) % MOD) as u32;
                    }
                    n += d;
                }

                // Prime powers: only primes with (p-1)*p < N can contribute
                if prime.p <= 4472 {
                    let mut pe_val = pu;
                    while (prime.d as u64) * pe_val < N as u64 {
                        let step = (prime.d as u64 * pe_val) as usize;
                        let first_pe = if start <= step {
                            step
                        } else {
                            ((start + step - 1) / step) * step
                        };
                        let mut n_pe = first_pe;
                        while n_pe < end {
                            let idx = n_pe - start;
                            unsafe {
                                *logs.get_unchecked_mut(idx) += logp;
                                let m = mods.get_unchecked_mut(idx);
                                *m = ((*m as u64 * pu) % MOD) as u32;
                            }
                            n_pe += step;
                        }
                        if pe_val > (N as u64 - 1) / pu {
                            break;
                        }
                        pe_val *= pu;
                    }
                }
            }

            // Find local maximum in chunk
            let mut chunk_best_log = -1.0f64;
            let mut chunk_best_mod = 0u32;
            for i in 0..len {
                let l = logs[i];
                if l > chunk_best_log {
                    chunk_best_log = l;
                    chunk_best_mod = mods[i];
                }
            }

            (chunk_best_log, chunk_best_mod)
        })
        .reduce(
            || (-1.0f64, 0u32),
            |(l1, m1), (l2, m2)| {
                if l2 > l1 {
                    (l2, m2)
                } else {
                    (l1, m1)
                }
            },
        );

    let _ = best_log;
    let answer = best_mod as u64 + 1;
    println!("{answer}");
}
