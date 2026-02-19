// Project Euler 365: A huge binomial coefficient
//
// Compute sum(C(10^18, 10^9) mod p*q*r) for all primes 1000 < p < q < r < 5000,
// using Lucas' theorem and CRT.

use euler_utils::{mod_inv, primes_up_to, BinomialMod};
use rayon::prelude::*;

fn lucas_mod(mut n: u64, mut k: u64, p: usize, binom: &BinomialMod) -> u64 {
    if k > n { return 0; }
    if k == 0 { return 1; }
    let mut result = 1u64;
    let pu = p as u64;
    while n > 0 || k > 0 {
        let nd = (n % pu) as usize;
        let kd = (k % pu) as usize;
        if kd > nd { return 0; }
        result = result * binom.choose(nd, kd) % pu;
        n /= pu;
        k /= pu;
    }
    result
}

fn main() {
    let all_primes = primes_up_to(5000);
    let primes: Vec<usize> = all_primes.into_iter().filter(|&p| p > 1000).collect();
    let np = primes.len();

    let n_val: u64 = 1_000_000_000_000_000_000;
    let k_val: u64 = 1_000_000_000;

    let mut lucas_vals = vec![0u64; np];
    for i in 0..np {
        let binom = BinomialMod::new(primes[i] - 1, primes[i] as u64);
        lucas_vals[i] = lucas_mod(n_val, k_val, primes[i], &binom);
    }

    // Parallelize over outer index; hoist pair CRT out of innermost loop
    let total_sum: i64 = (0..np.saturating_sub(2))
        .into_par_iter()
        .map(|i| {
            let ap = lucas_vals[i] as i128;
            let p = primes[i] as i128;
            let mut subtotal: i64 = 0;
            for j in i + 1..np.saturating_sub(1) {
                let aq = lucas_vals[j] as i128;
                let q = primes[j] as i128;
                // Precompute CRT for pair (p, q) — shared across all r
                let m12 = p * q;
                let inv1 = mod_inv(primes[i] as u64, primes[j] as u64).unwrap() as i128;
                let x12 = ((ap + p * (((aq - ap) * inv1 % q + q) % q)) % m12 + m12) % m12;

                for ki in j + 1..np {
                    let ar = lucas_vals[ki] as i128;
                    let r = primes[ki] as i128;
                    let inv12 = mod_inv((m12 % r) as u64, primes[ki] as u64).unwrap() as i128;
                    let m = m12 * r;
                    let x = ((x12 + m12 * (((ar - x12 % r) * inv12 % r + r) % r)) % m + m) % m;
                    subtotal += x as i64;
                }
            }
            subtotal
        })
        .sum();

    println!("{}", total_sum);
}
