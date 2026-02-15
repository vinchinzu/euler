// Project Euler 960 - Stone Pile Game
// F(n) = (n-1)!/2 * sum_{k=1}^{n-1} [C(n,k) * k^(k-1) * (n-k)^(n-k-1) * min(k, n-k)]
// Find F(100) mod 10^9+7

const MOD: u64 = 1_000_000_007;

fn power_mod(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut result = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result as u128 * base as u128 % m as u128) as u64;
        }
        base = (base as u128 * base as u128 % m as u128) as u64;
        exp >>= 1;
    }
    result
}

fn main() {
    let n = 100usize;
    let mut fact = [0u64; 101];
    let mut inv_fact = [0u64; 101];
    fact[0] = 1;
    for i in 1..=n {
        fact[i] = fact[i - 1] * i as u64 % MOD;
    }
    inv_fact[n] = power_mod(fact[n], MOD - 2, MOD);
    for i in (0..n).rev() {
        inv_fact[i] = inv_fact[i + 1] * (i as u64 + 1) % MOD;
    }

    let mut total_sum = 0u64;
    for k in 1..n {
        let mut term = fact[n];
        term = term * inv_fact[k] % MOD;
        term = term * inv_fact[n - k] % MOD;
        term = term * power_mod(k as u64, (k as u64).wrapping_sub(1), MOD) % MOD;
        term = term * power_mod((n - k) as u64, (n - k) as u64 - 1, MOD) % MOD;
        let mk = k.min(n - k) as u64;
        term = term * mk % MOD;
        total_sum = (total_sum + term) % MOD;
    }

    let mut result = fact[n - 1];
    result = result * power_mod(2, MOD - 2, MOD) % MOD;
    result = result * total_sum % MOD;

    println!("{}", result);
}
