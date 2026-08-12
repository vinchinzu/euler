// Problem 995: Particular Pair of Polynomials
// Product of S(p) for primes p < 20000, scientific notation with 6 sig digits.
// Optimize: accumulate log10(S(p)) instead of a huge BigUint product; parallelize S(p).

use num::BigUint;
use num_traits::{One, Zero};
use rayon::prelude::*;
use std::collections::HashMap;

const LIMIT: u64 = 20_000;
const PRIME_SEARCH_LIMIT: usize = 2_000_000;

fn sieve(n: usize) -> Vec<u64> {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    if n >= 1 {
        is_prime[1] = false;
    }
    let r = (n as f64).sqrt() as usize;
    for i in 2..=r {
        if is_prime[i] {
            let mut j = i * i;
            while j <= n {
                is_prime[j] = false;
                j += i;
            }
        }
    }
    (2..=n).filter(|&i| is_prime[i]).map(|i| i as u64).collect()
}

fn factor(n: u64, primes: &[u64]) -> Vec<(u64, u32)> {
    let mut out = Vec::new();
    let mut t = n;
    for &p in primes {
        if p * p > t {
            break;
        }
        if t % p == 0 {
            let mut e = 0u32;
            while t % p == 0 {
                t /= p;
                e += 1;
            }
            out.push((p, e));
        }
    }
    if t > 1 {
        out.push((t, 1));
    }
    out
}

fn divisors_from_factorization(factors: &[(u64, u32)]) -> Vec<u64> {
    let mut divs = vec![1u64];
    for &(p, e) in factors {
        let old = divs.clone();
        let mut next = Vec::new();
        let mut power = 1u64;
        for _ in 0..=e {
            for &d in &old {
                next.push(d * power);
            }
            power *= p;
        }
        divs = next;
    }
    divs.sort_unstable();
    divs
}

fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result = 1u64;
    base %= modulus;
    while exp > 0 {
        if exp & 1 == 1 {
            result = ((result as u128 * base as u128) % modulus as u128) as u64;
        }
        base = ((base as u128 * base as u128) % modulus as u128) as u64;
        exp >>= 1;
    }
    result
}

fn gcd_u64(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn primitive_root(p: u64, prime_factors_of_p_minus_1: &[u64]) -> u64 {
    if p == 2 {
        return 1;
    }
    let m = p - 1;
    for g in 2..p {
        let mut ok = true;
        for &q in prime_factors_of_p_minus_1 {
            if mod_pow(g, m / q, p) == 1 {
                ok = false;
                break;
            }
        }
        if ok {
            return g;
        }
    }
    panic!("primitive root not found");
}

fn discrete_log_table(p: u64, root: u64) -> Vec<i32> {
    let mut table = vec![-1i32; p as usize];
    let mut x = 1u64;
    for k in 0..(p - 1) {
        table[x as usize] = k as i32;
        x = ((x as u128 * root as u128) % p as u128) as u64;
    }
    table
}

fn s_for_prime(p: u64, primes: &[u64]) -> BigUint {
    if p == 2 {
        return BigUint::one();
    }

    let m = p - 1;
    let factors = factor(m, primes);
    let divs = divisors_from_factorization(&factors);
    let pf: Vec<u64> = factors.iter().map(|&(q, _)| q).collect();
    let root = primitive_root(p, &pf);
    let dlog = discrete_log_table(p, root);

    let needed_c_count = divs.len() - 1;
    let mut least_prime_for_c: HashMap<u64, u64> = HashMap::new();
    for &q in primes {
        if q == p {
            continue;
        }
        let c = gcd_u64(dlog[(q % p) as usize] as u64, m);
        if c < m && !least_prime_for_c.contains_key(&c) {
            least_prime_for_c.insert(c, q);
            if least_prime_for_c.len() == needed_c_count {
                break;
            }
        }
    }
    if least_prime_for_c.len() != needed_c_count {
        panic!("increase PRIME_SEARCH_LIMIT");
    }

    let c_items: Vec<(u64, u64)> = least_prime_for_c.into_iter().collect();
    let mut best_by_m: HashMap<u64, HashMap<u64, u64>> = HashMap::new();
    for &mm in &divs {
        if mm == 1 {
            continue;
        }
        let mut best: HashMap<u64, u64> = HashMap::new();
        for &(c, q) in &c_items {
            let d = gcd_u64(c, mm);
            if d < mm {
                best.entry(d).and_modify(|e| *e = (*e).min(q)).or_insert(q);
            }
        }
        best_by_m.insert(mm, best);
    }

    let mut dp_value: HashMap<u64, BigUint> = HashMap::new();
    dp_value.insert(1, BigUint::one());

    for &h in &divs {
        if !dp_value.contains_key(&h) {
            continue;
        }
        let mm = m / h;
        if mm == 1 {
            continue;
        }
        let best = &best_by_m[&mm];
        let base_value = dp_value[&h].clone();
        for &l in &divs {
            if l > 1 && mm % l == 0 {
                let next_h = h * l;
                let q = best[&(mm / l)];
                let candidate = &base_value * BigUint::from(q).pow((l - 1) as u32);
                match dp_value.get(&next_h) {
                    Some(cur) if candidate >= *cur => {}
                    _ => {
                        dp_value.insert(next_h, candidate);
                    }
                }
            }
        }
    }

    dp_value.remove(&m).expect("S(p) missing")
}

/// log10 of a positive BigUint via leading digits (f64-safe, ~15 digits).
fn log10_big(n: &BigUint) -> f64 {
    debug_assert!(!n.is_zero());
    let s = n.to_str_radix(10);
    let exp = (s.len() - 1) as f64;
    // Use up to 17 leading digits for a solid f64 mantissa.
    let take = s.len().min(17);
    let mut mant_str = String::with_capacity(18);
    mant_str.push(s.as_bytes()[0] as char);
    if take > 1 {
        mant_str.push('.');
        mant_str.push_str(&s[1..take]);
    }
    let mant: f64 = mant_str.parse().unwrap();
    mant.log10() + exp
}

/// Scientific notation from sum of log10 values, 5 digits after decimal.
fn scientific_from_log10(sum_log10: f64) -> String {
    // sum_log10 = log10(N) = exponent + log10(mantissa), mantissa in [1,10)
    let mut exponent = sum_log10.floor() as i64;
    let mut frac = sum_log10 - exponent as f64;
    // Guard against floor edge cases (frac ≈ 1 due to rounding)
    if frac >= 1.0 {
        exponent += 1;
        frac -= 1.0;
    }
    if frac < 0.0 {
        exponent -= 1;
        frac += 1.0;
    }

    let mut mantissa = 10f64.powf(frac);
    // Round to 5 decimal places (6 significant digits total).
    let rounded = (mantissa * 1e5).round() / 1e5;
    if rounded >= 10.0 {
        mantissa = rounded / 10.0;
        exponent += 1;
    } else {
        mantissa = rounded;
    }

    // Format with exactly 5 digits after decimal.
    format!("{:.5}e{}", mantissa, exponent)
}

fn main() {
    let primes = sieve(PRIME_SEARCH_LIMIT);
    let primes_ref = &primes;

    let sum_log10: f64 = primes
        .par_iter()
        .filter(|&&p| p < LIMIT)
        .map(|&p| {
            let s = s_for_prime(p, primes_ref);
            log10_big(&s)
        })
        .sum();

    println!("{}", scientific_from_log10(sum_log10));
}
