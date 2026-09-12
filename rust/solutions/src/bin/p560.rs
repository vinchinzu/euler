// Project Euler 560 - Coprime Nim
//
// Nimber of pile s: 0 if even, 1 if s=1, pi(p)+1 for smallest prime p|s otherwise.
// Use Walsh-Hadamard XOR convolution for K-fold XOR convolution of counts.

use rayon::prelude::*;

const N_VAL: usize = 10_000_000;
const K_VAL: u64 = 10_000_000;
const MOD: u64 = 1_000_000_007;

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

fn fwht(arr: &mut [u64], sz: usize) {
    let mut len = 1usize;
    while len < sz {
        let step = len << 1;
        if len >= 256 {
            arr.par_chunks_mut(step).for_each(|chunk| {
                for j in 0..len {
                    unsafe {
                        let u = *chunk.get_unchecked(j);
                        let v = *chunk.get_unchecked(j + len);
                        let s = u + v;
                        *chunk.get_unchecked_mut(j) = if s >= MOD { s - MOD } else { s };
                        *chunk.get_unchecked_mut(j + len) =
                            if u >= v { u - v } else { u + MOD - v };
                    }
                }
            });
        } else {
            let mut i = 0;
            while i < sz {
                for j in 0..len {
                    unsafe {
                        let u = *arr.get_unchecked(i + j);
                        let v = *arr.get_unchecked(i + j + len);
                        let s = u + v;
                        *arr.get_unchecked_mut(i + j) = if s >= MOD { s - MOD } else { s };
                        *arr.get_unchecked_mut(i + j + len) =
                            if u >= v { u - v } else { u + MOD - v };
                    }
                }
                i += step;
            }
        }
        len = step;
    }
}

fn main() {
    // Sieve smallest prime factor
    let mut spf = vec![0u32; N_VAL + 1];
    for i in 2..=N_VAL {
        if spf[i] == 0 {
            let mut j = i;
            while j <= N_VAL {
                if spf[j] == 0 { spf[j] = i as u32; }
                j += i;
            }
        }
    }

    // Collect primes and their indices
    let mut primes: Vec<usize> = Vec::new();
    let mut prime_idx = vec![0usize; N_VAL + 1];
    for i in 2..=N_VAL {
        if spf[i] == i as u32 {
            prime_idx[i] = primes.len();
            primes.push(i);
        }
    }

    let nprimes = primes.len();
    let max_nimber = nprimes;
    let mut counts = vec![0u64; max_nimber + 2];

    for s in 1..N_VAL {
        let nim = if s % 2 == 0 {
            0
        } else if s == 1 {
            1
        } else {
            prime_idx[spf[s] as usize] + 1
        };
        counts[nim] += 1;
    }

    // Find next power of 2
    let mut sz = 1;
    while sz <= max_nimber { sz <<= 1; }

    let mut arr = vec![0u64; sz];
    for i in 0..=max_nimber {
        arr[i] = counts[i] % MOD;
    }

    fwht(&mut arr, sz);

    arr.par_iter_mut().with_min_len(256).for_each(|x| {
        *x = pow_mod(*x, K_VAL);
    });

    fwht(&mut arr, sz);

    let inv_sz = pow_mod(sz as u64, MOD - 2);
    arr.par_iter_mut().with_min_len(1024).for_each(|x| {
        *x = *x * inv_sz % MOD;
    });

    println!("{}", arr[0]);
}
