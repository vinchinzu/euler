// Project Euler 657 - Incomplete Words
// Inclusion-exclusion for words of length <= N using K letters, N=10^12, K=10^7.
// Optimization: rayon-parallel pow[] init; pure u64 modular arithmetic.

use rayon::prelude::*;

const MOD: u64 = 1_000_000_007;

#[inline(always)]
fn power_mod(mut base: u64, mut exp: u64) -> u64 {
    let mut r = 1u64;
    base %= MOD;
    while exp > 0 {
        if exp & 1 == 1 {
            r = r * base % MOD;
        }
        base = base * base % MOD;
        exp >>= 1;
    }
    r
}

fn main() {
    let n: u64 = 1_000_000_000_000;
    let k = 10_000_000usize;
    let exp = n + 1;

    // Parallel precompute t^(N+1) mod MOD for t = 1..K
    let mut pows = vec![0u64; k + 1];
    pows
        .par_iter_mut()
        .enumerate()
        .skip(1)
        .for_each(|(i, slot)| {
            *slot = power_mod(i as u64, exp);
        });

    // Linear modular inverses 1..K
    let mut invs = vec![0u64; k + 1];
    invs[1] = 1;
    for i in 2..=k {
        invs[i] = (MOD - (MOD / i as u64) * invs[(MOD % i as u64) as usize] % MOD) % MOD;
    }

    let mut ans = 0u64;
    let mut num_choices = 1u64;
    for t in 0..k {
        let num_words = if t == 0 {
            1u64
        } else if t == 1 {
            (n + 1) % MOD
        } else {
            (pows[t] + MOD - 1) % MOD * invs[t - 1] % MOD
        };
        let sign = if (k - t) % 2 == 0 { 1u64 } else { MOD - 1 };
        // ans -= sign * num_words * num_choices
        let term = sign * num_words % MOD * num_choices % MOD;
        ans = (ans + MOD - term % MOD) % MOD;
        if t < k - 1 {
            num_choices = num_choices * ((k - t) as u64 % MOD) % MOD * invs[t + 1] % MOD;
        }
    }
    println!("{}", ans % MOD);
}
