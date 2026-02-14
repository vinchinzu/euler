// Project Euler 533 - Carmichael Lambda Function
//
// Sieve-based computation of lambda values mod 10^9.
// For each prime p, update multiples of (p-1) and prime powers.

use euler_utils::sieve;

const N: usize = 20_000_000;
const MOD: u64 = 1_000_000_000;

fn main() {
    let is_prime = sieve(N + 1);

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

    // Odd primes
    for p in (3..=N).step_by(2) {
        if !is_prime[p] { continue; }
        let d = p - 1;
        let logp = (p as f64).ln();

        let mut n = d;
        while n < N {
            logs[n] += logp;
            mods[n] = ((mods[n] as u64 * p as u64) % MOD) as u32;
            n += d;
        }

        let mut pe_val = p as u64;
        while (d as u64) * pe_val < N as u64 {
            let step = d as u64 * pe_val;
            let mut n = step;
            while n < N as u64 {
                logs[n as usize] += logp;
                mods[n as usize] = ((mods[n as usize] as u64 * p as u64) % MOD) as u32;
                n += step;
            }
            if pe_val > (N as u64 - 1) / p as u64 { break; }
            pe_val *= p as u64;
        }
    }

    let mut best_log = -1.0f64;
    let mut best_mod = 0u32;
    for n in 1..N {
        if logs[n] > best_log {
            best_log = logs[n];
            best_mod = mods[n];
        }
    }

    let answer = best_mod as u64 + 1;
    println!("{answer}");
}
