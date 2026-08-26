// Project Euler 602 - Product of Head Counts
// Inclusion-exclusion. t^N is multiplicative: SPF sieve + parallel prime powers.

use rayon::prelude::*;

const MOD: u64 = 1_000_000_007;
const NN: u64 = 10_000_000;
const K: usize = 4_000_000;

#[inline(always)]
fn pow_mod(mut base: u64, mut exp: u64) -> u64 {
    let mut result = 1u64;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % MOD;
        }
        base = base * base % MOD;
        exp >>= 1;
    }
    result
}

fn ncr_table() -> Vec<u32> {
    let mut inv = vec![0u32; K + 1];
    inv[1] = 1;
    for i in 2..=K {
        inv[i] = (MOD - (MOD / i as u64) * inv[(MOD % i as u64) as usize] as u64 % MOD) as u32;
    }

    let mut ncr = vec![0u32; K + 1];
    ncr[0] = 1;
    ncr[1..=K]
        .par_iter_mut()
        .enumerate()
        .with_min_len(4096)
        .for_each(|(idx, slot)| {
            let i = idx + 1;
            *slot = ((NN + 2 - i as u64) * inv[i] as u64 % MOD) as u32;
        });

    // Parallel prefix product of terms[1..=K] → ncr[i] = C(N+1, i)
    let chunk_size = 1 << 16;
    let chunk_prods: Vec<u64> = ncr[1..]
        .par_chunks_mut(chunk_size)
        .map(|chunk| {
            let mut p = 1u64;
            for x in chunk.iter_mut() {
                p = p * *x as u64 % MOD;
                *x = p as u32;
            }
            p
        })
        .collect();

    let mut offset = 1u64;
    let mut offsets = Vec::with_capacity(chunk_prods.len());
    for &prod in &chunk_prods {
        offsets.push(offset);
        offset = offset * prod % MOD;
    }

    ncr[1..]
        .par_chunks_mut(chunk_size)
        .zip(offsets)
        .for_each(|(chunk, off)| {
            if off != 1 {
                for x in chunk.iter_mut() {
                    *x = (*x as u64 * off % MOD) as u32;
                }
            }
        });

    ncr
}

fn pows_table() -> Vec<u32> {
    let mut spf = vec![0u32; K + 1];
    for i in 0..=K {
        spf[i] = i as u32;
    }
    let mut i = 2usize;
    while i * i <= K {
        if spf[i] == i as u32 {
            let mut j = i * i;
            while j <= K {
                if spf[j] == j as u32 {
                    spf[j] = i as u32;
                }
                j += i;
            }
        }
        i += 1;
    }

    let primes: Vec<usize> = (2..=K).filter(|&n| spf[n] == n as u32).collect();
    let prime_pows: Vec<u32> = primes
        .par_iter()
        .map(|&p| pow_mod(p as u64, NN) as u32)
        .collect();

    let mut pows = vec![0u32; K + 1];
    pows[1] = 1;
    for (idx, &p) in primes.iter().enumerate() {
        pows[p] = prime_pows[idx];
    }
    for n in 2..=K {
        if spf[n] != n as u32 {
            // SAFETY: spf[n] < n and n/spf[n] < n, both already filled; arrays length K+1
            unsafe {
                let a = *pows.get_unchecked(spf[n] as usize) as u64;
                let b = *pows.get_unchecked(n / spf[n] as usize) as u64;
                *pows.get_unchecked_mut(n) = (a * b % MOD) as u32;
            }
        }
    }
    pows
}

fn main() {
    let (ncr, pows) = rayon::join(ncr_table, pows_table);

    let ans: u64 = ncr
        .par_iter()
        .zip(pows.par_iter().rev())
        .enumerate()
        .with_min_len(4096)
        .map(|(j, (&c, &p))| {
            let sign = if j & 1 == 1 { MOD - 1 } else { 1 };
            c as u64 * sign % MOD * p as u64 % MOD
        })
        .sum();

    println!("{}", ans % MOD);
}
