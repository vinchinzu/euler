// Project Euler 603 - Concatenation of Consecutive Primes
// Build digit string of first 10^6 primes, compute S(k) mod 10^9+7

const MOD: i64 = 1_000_000_007;
const N_PRIMES: usize = 1_000_000;
const K_COPIES: i64 = 1_000_000_000_000;
const SIEVE_LIMIT: usize = 16_000_000;

fn mod_inv(a: i64, m: i64) -> i64 {
    let (mut t, mut new_t) = (0i64, 1i64);
    let (mut r, mut new_r) = (m, a % m);
    while new_r != 0 {
        let q = r / new_r;
        let tmp = new_t;
        new_t = t - q * new_t;
        t = tmp;
        let tmp = new_r;
        new_r = r - q * new_r;
        r = tmp;
    }
    ((t % m) + m) % m
}

fn pow_mod(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut result = 1i64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result as i128 * base as i128 % m as i128) as i64;
        }
        base = (base as i128 * base as i128 % m as i128) as i64;
        exp >>= 1;
    }
    result
}

fn main() {
    // Sieve
    let mut is_prime = vec![true; SIEVE_LIMIT + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut i = 2;
    while i * i <= SIEVE_LIMIT {
        if is_prime[i] {
            let mut j = i * i;
            while j <= SIEVE_LIMIT {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }

    let mut primes = Vec::with_capacity(N_PRIMES);
    for i in 2..=SIEVE_LIMIT {
        if primes.len() >= N_PRIMES { break; }
        if is_prime[i] { primes.push(i); }
    }

    // Build digit string
    let mut digits = Vec::new();
    for &p in &primes {
        for ch in p.to_string().bytes() {
            digits.push((ch - b'0') as i64);
        }
    }

    let l = digits.len() as i64;
    let m = MOD;
    let b: i64 = 10;

    let mut layered_num: i64 = 0;
    let mut num: i64 = 0;
    let mut layered_sum: i64 = 0;
    let mut sum_val: i64 = 0;

    for (n, &d) in digits.iter().enumerate() {
        let n_mod = (n as i64) % m;
        layered_num = ((layered_num as i128 + n_mod as i128 * d as i128 % m as i128) as i64) % m;
        layered_sum = ((layered_sum as i128 + n_mod as i128 * d as i128 % m as i128) as i64) % m;
        num = (num + d) % m;
        sum_val = (sum_val + d) % m;
        layered_num = (layered_num as i128 * b as i128 % m as i128) as i64;
        num = (num as i128 * b as i128 % m as i128) as i64;
    }

    let piece = pow_mod(b, l, m);
    let all_pow = (pow_mod(piece, K_COPIES, m) - 1 + m) % m;
    let inv_den = mod_inv((piece - 1 + m) % m, m);

    let k_mod = K_COPIES % m;
    let km1_mod = (K_COPIES - 1) % m;
    let tr_km1 = (km1_mod as i128 * k_mod as i128 % m as i128) as i64;
    let tr_km1 = (tr_km1 as i128 * mod_inv(2, m) as i128 % m as i128) as i64;
    let l_mod = l % m;

    let term1 = (all_pow as i128 * inv_den as i128 % m as i128) as i64;
    let term1 = (term1 as i128 * layered_num as i128 % m as i128) as i64;

    let inner1 = (l_mod as i128 * all_pow as i128 % m as i128) as i64;
    let inner1 = (inner1 as i128 * inv_den as i128 % m as i128) as i64;
    let inner2 = (k_mod as i128 * l_mod as i128 % m as i128) as i64;
    let term2_inner = (all_pow + inner1 - inner2 % m + 2 * m) % m;
    let term2 = (term2_inner as i128 * inv_den as i128 % m as i128) as i64;
    let term2 = (term2 as i128 * num as i128 % m as i128) as i64;

    let term3 = (k_mod as i128 * (layered_sum % m) as i128 % m as i128) as i64;

    let mut term4 = (l_mod as i128 * tr_km1 as i128 % m as i128) as i64;
    term4 = (term4 + k_mod) % m;
    term4 = (term4 as i128 * sum_val as i128 % m as i128) as i64;

    let res = (term1 + term2 - term3 - term4 + 2 * m) % m;
    let res = (res as i128 * mod_inv(b - 1, m) as i128 % m as i128) as i64;

    println!("{}", res);
}
