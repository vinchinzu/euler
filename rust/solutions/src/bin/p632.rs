// Project Euler 632 - Square prime factors
// C_k(N) = number of integers 1..N divisible by p^2 for exactly k primes p.
// Product of all nonzero C_k(N) mod 10^9+7.
// N = 10^16, sieve Mobius/omega up to sqrt(N) = 10^8.
//
// Approach:
// 1. Eratosthenes-style SPF sieve using u16 (~200MB). Only sieve primes up to sqrt(L).
//    spf[n]=0 means n is prime, else spf[n] is the smallest prime factor.
// 2. Fused parallel factorization + accumulation: for each n, factorize via SPF,
//    compute omega & squarefree in-place, and accumulate c[k] directly.
//    This avoids a separate 100MB data array.

use rayon::prelude::*;

const BIG_N: u64 = 10_000_000_000_000_000;
const MOD: u64 = 1_000_000_007;
const L: usize = 100_000_000;

fn main() {
    // Phase 1: Eratosthenes-style SPF sieve (u16)
    let mut spf = vec![0u16; L + 1];
    let sqrt_l = 10_000usize; // sqrt(10^8)
    for i in 2..=sqrt_l {
        if spf[i] == 0 {
            let mut j = i * i;
            while j <= L {
                if spf[j] == 0 {
                    spf[j] = i as u16;
                }
                j += i;
            }
        }
    }

    // Phase 2: Fused parallel factorization + accumulation
    let max_k = 26usize; // floor(log2(10^8))
    let mk1 = max_k + 1;

    // Precompute flat nCr table
    let mut ncr = vec![0u64; mk1 * mk1];
    for i in 0..=max_k {
        ncr[i * mk1] = 1;
        for j in 1..=i {
            ncr[i * mk1 + j] = (ncr[(i - 1) * mk1 + j - 1] + ncr[(i - 1) * mk1 + j]) % MOD;
        }
    }

    let chunk_size: usize = 2_000_000;
    let n_chunks = (L - 1) / chunk_size + 1;

    let local_cs: Vec<Vec<i64>> = (0..n_chunks).into_par_iter().map(|ci| {
        let start = 2 + ci * chunk_size;
        let end = std::cmp::min(start + chunk_size, L + 1);
        let mut c = vec![0i64; mk1];

        for n in start..end {
            // Factorize n using SPF
            let mut tmp = n;
            let mut om = 0u32;
            let mut squarefree = true;

            while tmp > 1 {
                // SAFETY: tmp <= L; spf has size L+1
                let p = unsafe { *spf.get_unchecked(tmp) } as usize;
                if p == 0 {
                    // tmp is prime (last factor)
                    om += 1;
                    break;
                }
                tmp /= p;
                // Check if p still divides tmp (p^2 | n on this path)
                if tmp > 1 {
                    let next_p = unsafe { *spf.get_unchecked(tmp) } as usize;
                    if next_p == p || (next_p == 0 && tmp == p) {
                        squarefree = false;
                        break;
                    }
                }
                om += 1;
            }

            if !squarefree { continue; }
            let k = om as usize;
            let n2 = n as u64 * n as u64;
            let count = (BIG_N / n2) % MOD;

            let ncr_base = k * mk1;
            for i in 0..=k {
                let prod = unsafe { *ncr.get_unchecked(ncr_base + i) } * count % MOD;
                let idx = k - i;
                if i & 1 == 0 {
                    unsafe { *c.get_unchecked_mut(idx) += prod as i64; }
                } else {
                    unsafe { *c.get_unchecked_mut(idx) -= prod as i64; }
                }
            }
        }
        c
    }).collect();

    // Merge
    let mut c = vec![0i64; mk1];
    c[0] = (BIG_N % MOD) as i64;
    for local_c in &local_cs {
        for i in 0..=max_k {
            c[i] += local_c[i];
        }
    }

    // Final product
    let m = MOD as i64;
    let mut ans = 1i64;
    for i in 0..=max_k {
        let ci = ((c[i] % m) + m) % m;
        if ci != 0 {
            ans = (ans as i128 * ci as i128 % m as i128) as i64;
        }
    }

    println!("{}", ans);
}
