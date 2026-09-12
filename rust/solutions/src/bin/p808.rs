// Project Euler 808 - Reversible Prime Squares
// Find sum of first 50 prime squares whose reversal is also a different prime square
//
// Odd-only bit sieve: bit i ↔ 2i+1. Parallel segmented fill; scan stays sequential.

use rayon::prelude::*;

const LIMIT: usize = 100_000_001;
const N_ODD: usize = (LIMIT + 1) / 2; // bit i ↔ number 2i+1
const CHUNK_ODDS: usize = 1 << 19;

fn odd_primes_upto(limit: usize) -> Vec<u32> {
    let n_odd = (limit + 1) / 2;
    let mut comp = vec![0u8; n_odd];
    if n_odd > 0 {
        comp[0] = 1;
    }
    let mut p = 3usize;
    while p * p <= limit {
        if comp[p >> 1] == 0 {
            let mut j = (p * p) >> 1;
            while j < n_odd {
                comp[j] = 1;
                j += p;
            }
        }
        p += 2;
    }
    let mut primes = Vec::new();
    p = 3;
    while p <= limit {
        if comp[p >> 1] == 0 {
            primes.push(p as u32);
        }
        p += 2;
    }
    primes
}

/// Mark odd composites in one segment. `start_odd` is the odd-index of bit 0.
fn sieve_segment(comp: &mut [u64], start_odd: usize, nbits: usize, small: &[u32]) {
    let n0 = 2 * start_odd + 1;
    let ptr = comp.as_mut_ptr();
    for &p32 in small {
        let p = p32 as usize;
        let pp = p * p;
        let mut n = if n0 > pp { n0 } else { pp };
        let r = n % p;
        if r != 0 {
            n += p - r;
        }
        if n & 1 == 0 {
            n += p;
        }
        if n < n0 {
            continue;
        }
        let mut j = (n - n0) >> 1;
        // SAFETY: j < nbits and nbits <= comp.len()*64, so j>>6 is in-bounds.
        unsafe {
            while j + 3 * p < nbits {
                *ptr.add(j >> 6) |= 1u64 << (j & 63);
                let j1 = j + p;
                *ptr.add(j1 >> 6) |= 1u64 << (j1 & 63);
                let j2 = j1 + p;
                *ptr.add(j2 >> 6) |= 1u64 << (j2 & 63);
                let j3 = j2 + p;
                *ptr.add(j3 >> 6) |= 1u64 << (j3 & 63);
                j = j3 + p;
            }
            while j < nbits {
                *ptr.add(j >> 6) |= 1u64 << (j & 63);
                j += p;
            }
        }
    }
}

#[inline(always)]
fn is_composite_odd(comp: &[u64], n: usize) -> bool {
    let i = n >> 1;
    (comp[i >> 6] >> (i & 63)) & 1 != 0
}

fn reverse_num(mut n: u64) -> u64 {
    let mut rev = 0u64;
    while n > 0 {
        rev = rev * 10 + n % 10;
        n /= 10;
    }
    rev
}

fn isqrt(n: u64) -> u64 {
    let mut x = (n as f64).sqrt() as u64;
    while x > 0 && x * x > n {
        x -= 1;
    }
    while (x + 1) * (x + 1) <= n {
        x += 1;
    }
    x
}

fn main() {
    let sqrt = (LIMIT as f64).sqrt() as usize + 1;
    let small = odd_primes_upto(sqrt);

    let nwords = (N_ODD + 63) / 64;
    let mut comp = vec![0u64; nwords];
    comp[0] |= 1; // 1 is not prime

    let chunk_words = CHUNK_ODDS / 64;
    comp.par_chunks_mut(chunk_words).enumerate().for_each(|(si, chunk)| {
        let start_odd = si * CHUNK_ODDS;
        if start_odd >= N_ODD {
            return;
        }
        let remaining = N_ODD - start_odd;
        let nbits = remaining.min(chunk.len() * 64);
        sieve_segment(chunk, start_odd, nbits, &small);
    });

    let mut count = 0;
    let mut sum: u64 = 0;

    // p=2 → 4 is a palindromic square, skip.
    let mut p = 3usize;
    while p < LIMIT {
        if count >= 50 {
            break;
        }
        if !is_composite_odd(&comp, p) {
            let sq = p as u64 * p as u64;
            let rev = reverse_num(sq);
            if rev != sq {
                let sr = isqrt(rev);
                if sr * sr == rev {
                    let sr_us = sr as usize;
                    if sr_us < LIMIT {
                        let prime_sr = sr_us == 2
                            || (sr_us >= 3
                                && (sr_us & 1) == 1
                                && !is_composite_odd(&comp, sr_us));
                        if prime_sr {
                            sum += sq;
                            count += 1;
                        }
                    }
                }
            }
        }
        p += 2;
    }

    println!("{}", sum);
}
