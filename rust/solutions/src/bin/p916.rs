// Project Euler 916 - Permutation Subsequences
// P(n) = C_n^2 * (1 + (3n/(n+2))^2) mod 10^9+7
// where C_n is the nth Catalan number.

const MOD: u64 = 1_000_000_007;

fn power(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut res = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            res = res * base % m;
        }
        base = base * base % m;
        exp >>= 1;
    }
    res
}

fn inverse(a: u64, m: u64) -> u64 {
    power(a, m - 2, m)
}

fn main() {
    let n: u64 = 100_000_000; // 10^8

    // Compute n! and (2n)! mod MOD
    let mut fact_n = 1u64;
    let mut fact_2n = 1u64;
    for i in 1..=2 * n {
        fact_2n = fact_2n * (i % MOD) % MOD;
        if i == n {
            fact_n = fact_2n;
        }
    }

    // C_n = (2n)! / ((n+1)! * n!) = (2n)! * inv(n+1) * inv(n!)^2
    let inv_fact_n = inverse(fact_n, MOD);
    let inv_n_plus_1 = inverse((n + 1) % MOD, MOD);

    let mut cn = fact_2n * inv_n_plus_1 % MOD;
    cn = cn * inv_fact_n % MOD;
    cn = cn * inv_fact_n % MOD;

    // term2_val = 3n / (n+2) mod p
    let term2_num = 3 * (n % MOD) % MOD;
    let term2_den = inverse((n + 2) % MOD, MOD);
    let term2_val = term2_num * term2_den % MOD;

    // P(n) = Cn^2 * (1 + term2_val^2)
    let cn_sq = cn * cn % MOD;
    let term2_sq = term2_val * term2_val % MOD;
    let bracket = (1 + term2_sq) % MOD;

    let ans = cn_sq * bracket % MOD;
    println!("{}", ans);
}
