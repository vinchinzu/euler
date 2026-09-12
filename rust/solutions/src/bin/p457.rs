// Project Euler 457 - A square on the hypotenuse
// f(n) = n^2 - 3n - 1, R(p) = smallest positive n with f(n) ≡ 0 (mod p^2).
// Sum R(p) for all primes p <= 10^7.

use rayon::prelude::*;

const MAXN: usize = 10_000_001;

// p <= 1e7 so p*p fits in u64; used for Tonelli / pow_mod modulo p.
fn pow_mod(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result = 1u64;
    base %= modulus;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % modulus;
        }
        base = base * base % modulus;
        exp >>= 1;
    }
    result
}

fn sqrt_mod(a: u64, p: u64) -> Option<u64> {
    let a = a % p;
    if a == 0 {
        return Some(0);
    }
    if p == 2 {
        return Some(a % 2);
    }
    if pow_mod(a, (p - 1) / 2, p) != 1 {
        return None;
    }

    let mut q = p - 1;
    let mut s = 0u32;
    while q % 2 == 0 {
        q /= 2;
        s += 1;
    }

    let mut z = 2u64;
    while pow_mod(z, (p - 1) / 2, p) != p - 1 {
        z += 1;
    }

    let mut m_val = s;
    let mut c = pow_mod(z, q, p);
    let mut t = pow_mod(a, q, p);
    let mut r = pow_mod(a, (q + 1) / 2, p);

    while t != 1 {
        let mut tt = t;
        let mut i = 0u32;
        while i < m_val && tt != 1 {
            tt = tt * tt % p;
            i += 1;
        }
        let b = pow_mod(c, 1u64 << (m_val - i - 1), p);
        m_val = i;
        c = b * b % p;
        t = t * c % p;
        r = r * b % p;
    }

    Some(r)
}

fn mod_inv_fn(a: u64, m: u64) -> u64 {
    let a = a % m;
    let (mut t, mut new_t) = (0i64, 1i64);
    let (mut r, mut new_r) = (m as i64, a as i64);
    while new_r != 0 {
        let q = r / new_r;
        let tmp = new_t;
        new_t = t - q * new_t;
        t = tmp;
        let tmp = new_r;
        new_r = r - q * new_r;
        r = tmp;
    }
    if t < 0 { (t + m as i64) as u64 } else { t as u64 }
}

fn compute_r(p: u64) -> i64 {
    if p <= 13 {
        let p2 = (p * p) as i64;
        for n in 1..=p2 {
            let fn_val = ((n * n - 3 * n - 1) % p2 + p2) % p2;
            if fn_val == 0 {
                return n;
            }
        }
        return 0;
    }

    let sv = match sqrt_mod(13, p) {
        Some(v) => v,
        None => return 0,
    };

    let inv2 = (p + 1) / 2;
    let mut r_min = u64::MAX;

    for &root in &[sv, p - sv] {
        let n_val = (3 + root) % p * inv2 % p;
        let fn_full = n_val as i128 * n_val as i128 - 3 * n_val as i128 - 1;
        let k_num = fn_full / p as i128;
        let deriv = (2 * n_val + p - 3) % p;
        let inv_deriv = mod_inv_fn(deriv, p);
        let kn = k_num.rem_euclid(p as i128) as u64;
        let k = (p - kn) % p * inv_deriv % p;
        let candidate = k * p + n_val;
        if candidate > 0 && candidate < r_min {
            r_min = candidate;
        }
    }

    if r_min < u64::MAX { r_min as i64 } else { 0 }
}

fn main() {
    let n = MAXN - 1;

    let mut is_prime = vec![true; MAXN];
    is_prime[0] = false;
    is_prime[1] = false;
    {
        let mut i = 2;
        while i * i < MAXN {
            if is_prime[i] {
                let mut j = i * i;
                while j < MAXN {
                    is_prime[j] = false;
                    j += i;
                }
            }
            i += 1;
        }
    }

    let mut primes = Vec::with_capacity(n / 10);
    for p in 2..=n {
        if is_prime[p] {
            primes.push(p as u64);
        }
    }

    let ans: i64 = primes.par_iter().map(|&p| compute_r(p)).sum();
    println!("{}", ans);
}
