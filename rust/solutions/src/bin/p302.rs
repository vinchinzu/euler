// Project Euler 302: Strong Achilles Numbers
// Count Strong Achilles numbers up to 10^18.
// Recursive generation of candidates with factorizations of n and phi(n).

use std::collections::HashSet;
use euler_utils::{gcd_i32, is_prime, sieve, sieve_smallest_factor};

const N: i64 = 1_000_000_000_000_000_000; // 10^18
const MAX_SPF: usize = 1_000_100;

#[derive(Clone)]
struct Dict {
    p: Vec<i32>,
    e: Vec<i32>,
}

impl Dict {
    fn new() -> Self { Dict { p: Vec::new(), e: Vec::new() } }

    fn get(&self, key: i32) -> i32 {
        for i in 0..self.p.len() { if self.p[i] == key { return self.e[i]; } }
        0
    }

    fn has(&self, key: i32) -> bool {
        self.p.contains(&key)
    }

    fn set(&mut self, key: i32, val: i32) {
        for i in 0..self.p.len() {
            if self.p[i] == key { self.e[i] = val; return; }
        }
        self.p.push(key);
        self.e.push(val);
    }

    fn inc(&mut self, key: i32, val: i32) {
        for i in 0..self.p.len() {
            if self.p[i] == key { self.e[i] += val; return; }
        }
        self.p.push(key);
        self.e.push(val);
    }

    fn del(&mut self, key: i32) {
        for i in 0..self.p.len() {
            if self.p[i] == key {
                let last = self.p.len() - 1;
                self.p[i] = self.p[last];
                self.e[i] = self.e[last];
                self.p.pop();
                self.e.pop();
                return;
            }
        }
    }

    fn gcd_of_vals(&self) -> i32 {
        if self.p.is_empty() { return 0; }
        let mut g = 0i32;
        for &e in &self.e { g = gcd_i32(g, e); }
        g
    }
}

fn main() {
    let spf = sieve_smallest_factor(MAX_SPF);

    // Primes up to N^(1/3)
    let limit = (N as f64).cbrt() as usize + 2;
    let is_p = sieve(limit);
    let primes: Vec<i32> = (2..=limit).filter(|&i| is_p[i]).map(|i| i as i32).collect();

    let mut achilles_set: HashSet<i64> = HashSet::new();

    let mut factors = Dict::new();
    let mut phi = Dict::new();

    helper(1, &mut factors, &mut phi, 1_000_000_000, &primes, &spf, &mut achilles_set);

    println!("{}", achilles_set.len());
}

fn helper(
    n: i64,
    factors: &mut Dict,
    phi: &mut Dict,
    max_p: i32,
    primes: &[i32],
    spf: &[u32],
    achilles_set: &mut HashSet<i64>,
) {
    // Find largest prime in phi with exponent 1
    let mut bad_p = 0i32;
    for i in 0..phi.p.len() {
        if phi.e[i] == 1 && phi.p[i] > bad_p {
            bad_p = phi.p[i];
        }
    }

    if bad_p == 0 {
        if !factors.p.is_empty() {
            let ge = factors.gcd_of_vals();
            let gp = phi.gcd_of_vals();
            if ge == 1 && gp == 1 {
                achilles_set.insert(n);
            }
        }

        let phi_keys: Vec<i32> = phi.p.clone();

        for &p in &phi_keys {
            if p < max_p {
                add_prime_fn(n, p, 2, factors, phi, p, primes, spf, achilles_set);
            }
        }

        for &p in primes {
            if p >= max_p { break; }
            if (n as f64) * (p as f64) * (p as f64) * (p as f64) >= N as f64 { break; }
            if !phi.has(p) {
                add_prime_fn(n, p, 3, factors, phi, p, primes, spf, achilles_set);
            }
        }
    } else if (n as f64) * (bad_p as f64) * (bad_p as f64) < N as f64 {
        add_prime_fn(n, bad_p, 2, factors, phi, max_p, primes, spf, achilles_set);

        let mut p = bad_p + 1;
        while p < max_p && (n as f64) * (p as f64) * (p as f64) < N as f64 {
            if p % bad_p == 1 && is_prime(p as u64) {
                add_prime_fn(n, p, 2, factors, phi, max_p, primes, spf, achilles_set);
            }
            p += bad_p;
        }
    }
}

fn add_prime_fn(
    n: i64,
    p: i32,
    min_e: i32,
    factors: &mut Dict,
    phi: &mut Dict,
    max_p: i32,
    primes: &[i32],
    spf: &[u32],
    achilles_set: &mut HashSet<i64>,
) {
    if factors.has(p) { return; }

    let prev_e = phi.get(p);
    let had_prev = phi.has(p);

    phi.inc(p, min_e - 1);

    // Factor p-1 and add to phi
    let mut phi_p_factors = Vec::new();
    let mut temp = p - 1;
    while temp > 1 {
        let pf = if (temp as usize) < MAX_SPF {
            spf[temp as usize] as i32
        } else {
            let mut pf = temp;
            for &pr in primes {
                if (pr as i64) * (pr as i64) > temp as i64 { break; }
                if temp % pr == 0 { pf = pr; break; }
            }
            pf
        };
        phi_p_factors.push(pf);
        phi.inc(pf, 1);
        temp /= pf;
    }

    let mut e = min_e;
    let mut power_p = 1i64;
    for _ in 0..min_e {
        if power_p > N / p as i64 { power_p = N + 1; break; }
        power_p *= p as i64;
    }

    while power_p <= N && n <= N / power_p {
        factors.set(p, e);
        helper(n * power_p, factors, phi, max_p, primes, spf, achilles_set);

        phi.inc(p, 1);
        e += 1;
        if power_p > N / p as i64 { break; }
        power_p *= p as i64;
    }

    // Restore
    factors.del(p);

    if had_prev {
        phi.set(p, prev_e);
    } else {
        phi.del(p);
    }

    for &pf in &phi_p_factors {
        let cur = phi.get(pf);
        if cur <= 1 { phi.del(pf); } else { phi.set(pf, cur - 1); }
    }
}
