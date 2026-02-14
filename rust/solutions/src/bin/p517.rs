// Project Euler 517 - A Real Recursion
// g_a(x) = 1 for x<a, g_a(x) = g_a(x-1) + g_a(x-a) for x>=a.
// Sum g_{sqrt(p)}(p) over all primes A <= p < B.

use euler_utils::mod_pow;

const A_VAL: usize = 10_000_000;
const B_VAL: usize = 10_010_000;
const MOD: u64 = 1_000_000_007;

fn main() {
    // Precompute factorials
    let mut fact = vec![0u64; B_VAL + 1];
    let mut inv_fact = vec![0u64; B_VAL + 1];
    fact[0] = 1;
    for i in 1..=B_VAL {
        fact[i] = fact[i - 1] * i as u64 % MOD;
    }
    inv_fact[B_VAL] = mod_pow(fact[B_VAL], MOD - 2, MOD);
    for i in (1..=B_VAL).rev() {
        inv_fact[i - 1] = inv_fact[i] * i as u64 % MOD;
    }

    let ncr = |n: usize, k: usize| -> u64 {
        if k > n { return 0; }
        fact[n] % MOD * inv_fact[k] % MOD * inv_fact[n - k] % MOD
    };

    // Sieve primes in [0, B_VAL]
    let sieve = euler_utils::sieve(B_VAL);

    let mut ans: u64 = 0;
    for p in A_VAL..B_VAL {
        if !sieve[p] { continue; }

        let a = (p as f64).sqrt();
        let mut num_a = 0usize;
        while (num_a as f64) * a < p as f64 {
            let floor_val = (p as f64 - num_a as f64 * a).floor() as usize;
            let n = num_a + floor_val;
            ans = (ans + ncr(n, num_a)) % MOD;
            num_a += 1;
        }
    }

    println!("{}", ans);
}
