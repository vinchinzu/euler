// Project Euler 245: Coresilience
// Find sum of composite n <= N where (n - phi(n)) / (n - 1) is a unit fraction.

use euler_utils::miller_rabin;

const N: i64 = 200_000_000_000;
const L: usize = 447_213;
const LIMIT: usize = 34_199_519;

fn power_mod(base: i64, exp: i64, modulus: i64) -> i64 {
    let mut result: u128 = 1;
    let mut b = (base % modulus) as u128;
    let m = modulus as u128;
    let mut e = exp;
    while e > 0 {
        if e & 1 == 1 {
            result = result * b % m;
        }
        b = b * b % m;
        e >>= 1;
    }
    result as i64
}

fn is_sq(n: i64, p: i64) -> bool {
    power_mod(n, (p - 1) / 2, p) <= 1
}

fn sqrt_mod(n: i64, p: i64) -> i64 {
    let n = ((n % p) + p) % p;
    if n == 0 { return 0; }
    if p % 4 == 3 {
        return power_mod(n, (p + 1) / 4, p);
    }
    // Tonelli-Shanks
    let mut q = p - 1;
    let mut s = 0;
    while q % 2 == 0 { q /= 2; s += 1; }
    let mut z = 2i64;
    while power_mod(z, (p - 1) / 2, p) != p - 1 { z += 1; }
    let mut m = s;
    let mut c = power_mod(z, q, p);
    let mut t = power_mod(n, q, p);
    let mut r = power_mod(n, (q + 1) / 2, p);
    loop {
        if t == 1 { return r; }
        let mut i = 1;
        let mut tmp = (t as u128 * t as u128 % p as u128) as i64;
        while tmp != 1 {
            tmp = (tmp as u128 * tmp as u128 % p as u128) as i64;
            i += 1;
        }
        let mut b = c;
        for _ in 0..m - i - 1 {
            b = (b as u128 * b as u128 % p as u128) as i64;
        }
        m = i;
        c = (b as u128 * b as u128 % p as u128) as i64;
        t = (t as u128 * c as u128 % p as u128) as i64;
        r = (r as u128 * b as u128 % p as u128) as i64;
    }
}

fn main() {
    // Build sieves
    let mut is_prime_sieve = vec![true; LIMIT + 1];
    is_prime_sieve[0] = false;
    is_prime_sieve[1] = false;
    {
        let mut i = 2;
        while i * i <= LIMIT {
            if is_prime_sieve[i] {
                let mut j = i * i;
                while j <= LIMIT {
                    is_prime_sieve[j] = false;
                    j += i;
                }
            }
            i += 1;
        }
    }

    let mut is_prime_small = vec![true; L + 1];
    is_prime_small[0] = false;
    is_prime_small[1] = false;
    {
        let mut i = 2;
        while i * i <= L {
            if is_prime_small[i] {
                let mut j = i * i;
                while j <= L {
                    is_prime_small[j] = false;
                    j += i;
                }
            }
            i += 1;
        }
    }

    let mut primes: Vec<i64> = Vec::new();
    for i in 3..=L {
        if is_prime_small[i] {
            primes.push(i as i64);
        }
    }

    let check_prime = |n: i64| -> bool {
        if n < 2 { return false; }
        if (n as usize) <= L { return is_prime_small[n as usize]; }
        if (n as usize) <= LIMIT { return is_prime_sieve[n as usize]; }
        miller_rabin(n as u64)
    };

    // Precompute prime factor data
    let mut pf_data: Vec<Vec<i64>> = vec![Vec::new(); L + 1];

    for qi in 0..primes.len() {
        let q = primes[qi];
        if q >= 3 && is_sq(q - 3, q) {
            let r1 = sqrt_mod(q - 3, q);
            let inv2 = (q + 1) / 2;
            let s1_raw = ((1 + r1) as u128 * inv2 as u128 % q as u128) as i64;
            let s1 = if s1_raw == 0 { q } else { s1_raw };
            let mut p = s1;
            while p <= L as i64 {
                pf_data[p as usize].push(q);
                p += q;
            }
            let s2_raw = (((1 - r1 + q) as u128 * inv2 as u128) % q as u128) as i64;
            let s2 = if s2_raw == 0 { q } else { s2_raw };
            let mut p = s2;
            while p <= L as i64 {
                pf_data[p as usize].push(q);
                p += q;
            }
        }
    }

    let mut ans: i64 = 0;

    // Two primes case
    for i in 0..primes.len() {
        let p = primes[i];
        let val = p * (p - 1) + 1;
        let divs = all_divisors(val, &pf_data[p as usize]);
        for d in divs {
            if d >= p {
                let q = d - (p - 1);
                if p < q && p * q <= N && check_prime(q) {
                    ans += p * q;
                }
            }
        }
    }

    // More than two primes case
    let mut factors = Vec::new();
    helper(0, 1, 1, &mut factors, &primes, &check_prime, &mut ans);

    println!("{}", ans);
}

fn all_divisors(n: i64, prime_factors: &[i64]) -> Vec<i64> {
    let mut divs = vec![1i64];
    let mut temp = n;
    for &p in prime_factors {
        if temp % p == 0 {
            let size = divs.len();
            let mut power = 1i64;
            while temp % p == 0 {
                temp /= p;
                power *= p;
                for i in 0..size {
                    divs.push(divs[i] * power);
                }
            }
        }
    }
    if temp > 1 {
        let size = divs.len();
        for i in 0..size {
            divs.push(divs[i] * temp);
        }
    }
    divs
}

fn helper(
    index: usize,
    big_p: i64,
    phi: i64,
    factors: &mut Vec<i64>,
    primes: &[i64],
    check_prime: &dyn Fn(i64) -> bool,
    ans: &mut i64,
) {
    if factors.len() >= 2 {
        let smallest = factors[0];
        let mut k = 2;
        while k < smallest {
            let num = phi * k + 1;
            let den = big_p - (big_p - phi) * k;
            if den > 0 && num % den == 0 {
                let q = num / den;
                if *factors.last().unwrap() < q && big_p * q <= N && check_prime(q) {
                    *ans += big_p * q;
                }
            }
            k += 2;
        }
    }
    let mut idx = index;
    while idx < primes.len() {
        let q = primes[idx];
        if big_p as f64 * q as f64 * q as f64 > N as f64 { break; }
        factors.push(q);
        helper(idx + 1, big_p * q, phi * (q - 1), factors, primes, check_prime, ans);
        factors.pop();
        idx += 1;
    }
}
