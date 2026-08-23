// Project Euler 522 - Hilbert's Hotel
//
// Combinatorial counting with modular arithmetic.
// Compute factorials and inverse factorials mod MOD, then sum terms.
// The per-l term is an independent ~24-mul mod_pow; parallelize over l.

use rayon::prelude::*;

const NN: u64 = 12_344_321;
const MOD: u64 = 135_707_531;
const CHUNK: usize = 4096;

#[inline(always)]
fn mod_pow_local(mut base: u64, mut exp: u64) -> u64 {
    // NN-l-1 < MOD, so no initial reduction.
    let mut result = 1u64;
    loop {
        if exp & 1 == 1 {
            result = result * base % MOD;
        }
        exp >>= 1;
        if exp == 0 {
            return result;
        }
        base = base * base % MOD;
    }
}

fn main() {
    let nn = NN as usize;
    let mut fact = vec![0u64; nn + 1];
    let mut inv_fact = vec![0u64; nn + 1];

    fact[0] = 1;
    for i in 1..=nn {
        fact[i] = fact[i - 1] * (i as u64) % MOD;
    }

    inv_fact[nn] = mod_pow_local(fact[nn], MOD - 2);
    for i in (1..=nn).rev() {
        inv_fact[i - 1] = inv_fact[i] * (i as u64) % MOD;
    }

    let mut ans = NN * (NN - 1) % MOD;
    ans = ans * mod_pow_local(NN - 2, NN - 1) % MOD;

    let fact = fact.as_slice();
    let inv_fact = inv_fact.as_slice();
    let fnn = fact[nn];
    let n_chunks = (nn - 2).div_ceil(CHUNK);

    // Each term < MOD; ~12.3M terms fit in u64 before a final reduction.
    let loop_sum: u64 = (0..n_chunks)
        .into_par_iter()
        .map(|ci| {
            let l0 = 2 + ci * CHUNK;
            let l1 = (l0 + CHUNK).min(nn);
            let mut local = 0u64;
            for lu in l0..l1 {
                let l = lu as u64;
                // SAFETY: lu in 2..nn, arrays have length nn+1
                unsafe {
                    let ncr = fnn * *inv_fact.get_unchecked(lu) % MOD
                        * *inv_fact.get_unchecked(nn - lu) % MOD;
                    local += ncr * *fact.get_unchecked(lu - 1) % MOD
                        * mod_pow_local(NN - l - 1, NN - l)
                        % MOD;
                }
            }
            local
        })
        .sum();

    println!("{}", (ans + loop_sum) % MOD);
}
