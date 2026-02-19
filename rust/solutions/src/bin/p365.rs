// Project Euler 365: A huge binomial coefficient
//
// Compute sum(C(10^18, 10^9) mod p*q*r) for all primes 1000 < p < q < r < 5000,
// using Lucas' theorem and CRT.

use euler_utils::{mod_inv, primes_up_to, BinomialMod};

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

fn crt_three(a1: i64, m1: i64, a2: i64, m2: i64, a3: i64, m3: i64) -> i64 {
    let m12 = m1 as i128 * m2 as i128;
    let m = m12 * m3 as i128;

    let inv1 = mod_inv(m1 as u64, m2 as u64).unwrap() as i128;
    let x12 = ((a1 as i128 + m1 as i128 * (((a2 as i128 - a1 as i128) * inv1 % m2 as i128 + m2 as i128) % m2 as i128)) % m12 + m12) % m12;

    let inv12 = mod_inv((m12 % m3 as i128) as u64, m3 as u64).unwrap() as i128;
    let x = ((x12 + m12 * (((a3 as i128 - x12 % m3 as i128) * inv12 % m3 as i128 + m3 as i128) % m3 as i128)) % m + m) % m;

    x as i64
}

fn main() {
    let all_primes = primes_up_to(5000);
    let primes: Vec<usize> = all_primes.into_iter().filter(|&p| p > 1000).collect();
    let np = primes.len();

    let n_val: u64 = 1_000_000_000_000_000_000; // 10^18
    let k_val: u64 = 1_000_000_000; // 10^9

    // Precompute BinomialMod and Lucas values for each prime
    let mut lucas_vals = vec![0u64; np];
    for i in 0..np {
        let binom = BinomialMod::new(primes[i] - 1, primes[i] as u64);
        lucas_vals[i] = lucas_mod(n_val, k_val, primes[i], &binom);
    }

    let mut total_sum: i64 = 0;
    for i in 0..np.saturating_sub(2) {
        let ap = lucas_vals[i] as i64;
        let p = primes[i] as i64;
        for j in i + 1..np.saturating_sub(1) {
            let aq = lucas_vals[j] as i64;
            let q = primes[j] as i64;
            for ki in j + 1..np {
                let ar = lucas_vals[ki] as i64;
                let r = primes[ki] as i64;
                total_sum += crt_three(ap, p, aq, q, ar, r);
            }
        }
    }

    println!("{}", total_sum);
}
