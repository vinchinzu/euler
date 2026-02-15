// Project Euler 903 - Harmonic-like sum with factorials
// Compute a complex expression involving factorials, harmonic numbers, and modular inverses

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
    let p = MOD;
    let n = 1_000_000usize;

    let inv2 = power_mod(2, p - 2, p);

    // Precompute factorials
    let mut f = vec![0u64; n + 1];
    f[0] = 1;
    for k in 1..=n {
        f[k] = f[k - 1] * k as u64 % p;
    }

    let fact = f[n];
    let total = fact * fact % p;

    // Compute harmonic sum mod p
    let mut h = 0u64;
    for k in 1..=n as u64 {
        h = (h + power_mod(k, p - 2, p)) % p;
    }

    // Compute coefficients
    let f_n2 = if n >= 2 { f[n - 2] } else { 1 };

    let mut c1 = fact * f_n2 % p;
    c1 = c1 * n as u64 % p;
    c1 = c1 * ((h + p - 1) % p) % p;

    let inner2 = (n as u64 + 1) % p * inv2 % p * ((n as u64 + p - h) % p) % p;
    let mut c2 = fact * f_n2 % p;
    c2 = c2 * n as u64 % p;
    c2 = c2 * inner2 % p;

    // Compute sums
    let mut sum_f = 0u64;
    for k in 0..n {
        sum_f = (sum_f + f[k]) % p;
    }
    let sum_kf = (f[n] + p - 1) % p;

    let sum1 = sum_f;
    let sum2 = (n as u64 % p * sum_f % p + p - sum_kf) % p;
    let sum3 = ((n as u64 + p - 1) % p * sum_f % p + p - sum_kf) % p;

    // Final computation
    let mut q = total;
    q = (q + c1 * sum2 % p) % p;
    q = (q + c2 * sum1 % p) % p;
    q = (q + p - total * sum1 % p) % p;
    q = (q + p - inv2 * total % p * sum3 % p) % p;

    println!("{}", q);
}
