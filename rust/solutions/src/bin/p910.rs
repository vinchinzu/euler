// Project Euler 910 - Power tower modular exponentiation
// Compute (2^^infinity + 90) mod 10^9

fn mod_pow(mut base: u64, mut exp: u64, m: u64) -> u64 {
    if m == 1 { return 0; }
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

fn euler_totient(mut n: u64) -> u64 {
    let mut res = n;
    let mut i = 2u64;
    while i * i <= n {
        if n % i == 0 {
            while n % i == 0 { n /= i; }
            res -= res / i;
        }
        i += 1;
    }
    if n > 1 { res -= res / n; }
    res
}

fn power_tower_stable(base: u64, m: u64) -> u64 {
    if m == 1 { return 0; }
    let phi = euler_totient(m);
    let exp = power_tower_stable(base, phi);
    mod_pow(base, exp + 100 * phi, m)
}

fn main() {
    let modulus = 1_000_000_000u64;
    let e_val = 90u64;
    let stable_val = power_tower_stable(2, modulus);
    let result = (stable_val + e_val) % modulus;
    println!("{}", result);
}
