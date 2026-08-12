// Project Euler 533 - Carmichael Lambda Function
//
// Sieve-based computation of lambda values mod 10^9.
// For each prime p, update multiples of (p-1) and prime powers.

use rayon::prelude::*;

const N: usize = 20_000_000;
const MOD: u64 = 1_000_000_000;

fn main() {
    // Odd-only sieve
    let mut is_prime = vec![false; N + 1];
    if N >= 2 {
        is_prime[2] = true;
    }
    let n_odds = (N + 1) / 2;
    let mut odd = vec![true; n_odds];
    odd[0] = false;
    let mut i = 1usize;
    while {
        let p = 2 * i + 1;
        p * p <= N
    } {
        if odd[i] {
            let p = 2 * i + 1;
            let mut j = (p * p) / 2;
            while j < n_odds {
                odd[j] = false;
                j += p;
            }
        }
        i += 1;
    }
    for i in 1..n_odds {
        if odd[i] {
            let p = 2 * i + 1;
            if p <= N {
                is_prime[p] = true;
            }
        }
    }
    drop(odd);

    let mut logs = vec![0.0f64; N];
    let mut mods = vec![1u32; N];

    // Handle p = 2 separately
    let log2v = 2.0f64.ln();
    for n in 1..N {
        logs[n] += log2v;
        mods[n] = ((mods[n] as u64 * 2) % MOD) as u32;
    }
    let mut n = 2;
    while n < N {
        logs[n] += log2v;
        mods[n] = ((mods[n] as u64 * 2) % MOD) as u32;
        n += 2;
    }
    let mut pe = 2usize;
    while pe < N {
        let mut n = pe;
        while n < N {
            logs[n] += log2v;
            mods[n] = ((mods[n] as u64 * 2) % MOD) as u32;
            n += pe;
        }
        pe <<= 1;
    }

    // Odd primes: collect then process in parallel chunks into local buffers, merge
    let odd_primes: Vec<usize> = (3..=N).step_by(2).filter(|&p| is_prime[p]).collect();

    // Chunk primes; each chunk builds full local logs/mods updates is too much memory
    // Instead: sequential but with unsafe and tighter loops
    for &p in &odd_primes {
        let d = p - 1;
        let logp = (p as f64).ln();
        let pu = p as u64;

        let mut n = d;
        while n < N {
            unsafe {
                *logs.get_unchecked_mut(n) += logp;
                let m = mods.get_unchecked_mut(n);
                *m = ((*m as u64 * pu) % MOD) as u32;
            }
            n += d;
        }

        let mut pe_val = p as u64;
        while (d as u64) * pe_val < N as u64 {
            let step = d as u64 * pe_val;
            let mut n = step;
            while n < N as u64 {
                let nu = n as usize;
                unsafe {
                    *logs.get_unchecked_mut(nu) += logp;
                    let m = mods.get_unchecked_mut(nu);
                    *m = ((*m as u64 * pu) % MOD) as u32;
                }
                n += step;
            }
            if pe_val > (N as u64 - 1) / pu {
                break;
            }
            pe_val *= pu;
        }
    }

    // Parallel find max
    let (best_log, best_mod) = (1..N)
        .into_par_iter()
        .map(|n| (logs[n], mods[n]))
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
