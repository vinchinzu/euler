// Project Euler 688 - Piles of Plates
// Contribution at k is closed-form in n = N - k(k-1)/2; parallelize over k.

use rayon::prelude::*;

const MOD: u64 = 1_000_000_007;
const N: i64 = 10_000_000_000_000_000;

fn k_max(n: i64) -> i64 {
    // Largest k with k*(k-1)/2 < n.
    let mut k = ((1.0 + (1.0 + 8.0 * n as f64).sqrt()) * 0.5) as i64 + 2;
    while k > 1 && k.saturating_mul(k - 1) / 2 >= n {
        k -= 1;
    }
    while k.saturating_mul(k + 1) / 2 < n {
        k += 1;
    }
    k
}

fn chunk_sum(k0: i64, k1: i64, inv2: u64) -> u64 {
    let mut local = 0u64;
    for k in k0..=k1 {
        let tri = k * (k - 1) / 2;
        let n = N - tri;
        if n <= 0 {
            continue;
        }
        let q = n / k;
        let r = n - q * k;
        let limit = (q as u64) % MOD;
        let km = (k as u64) % MOD;
        let term1 = km * limit % MOD * ((limit + MOD - 1) % MOD) % MOD * inv2 % MOD;
        let term2 = ((r as u64 + 1) % MOD) * limit % MOD;
        local += term1 + term2;
    }
    local % MOD
}

fn main() {
    let inv2 = (MOD + 1) / 2;
    let k_hi = k_max(N);
    const CHUNK: i64 = 200_000;
    let n_chunks = (k_hi + CHUNK - 1) / CHUNK;

    let ans: u64 = (0..n_chunks)
        .into_par_iter()
        .map(|ci| {
            let k0 = ci * CHUNK + 1;
            let k1 = ((ci + 1) * CHUNK).min(k_hi);
            chunk_sum(k0, k1, inv2)
        })
        .sum();

    println!("{}", ans % MOD);
}
