// Project Euler 302: Strong Achilles Numbers
// Count Strong Achilles numbers up to 10^18.
// Recursive generation of candidates with factorizations of n and phi(n).

use std::collections::HashSet;

const N: i64 = 1_000_000_000_000_000_000; // 10^18
const MAX_SPF: usize = 1_000_100;
const MAX_DICT: usize = 64;

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

fn gcd_i32(a: i32, b: i32) -> i32 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 { let t = b; b = a % b; a = t; }
    a
}

fn is_prime_check(n: i32) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }
    let mut i = 3;
    while (i as i64) * (i as i64) <= n as i64 {
        if n % i == 0 { return false; }
        i += 2;
    }
    true
}

fn main() {
    // Sieve SPF
    let mut spf = vec![0u32; MAX_SPF];
    for i in 0..MAX_SPF { spf[i] = i as u32; }
    {
        let mut i = 2;
        while i * i < MAX_SPF {
            if spf[i] == i as u32 {
                let mut j = i * i;
                while j < MAX_SPF {
                    if spf[j] == j as u32 { spf[j] = i as u32; }
                    j += i;
                }
            }
            i += 1;
        }
    }

    // Sieve primes up to N^(1/3)
    let limit = (N as f64).cbrt() as usize + 2;
    let mut is_p = vec![true; limit + 1];
    is_p[0] = false;
    if limit >= 1 { is_p[1] = false; }
    {
        let mut i = 2;
        while i * i <= limit {
            if is_p[i] {
                let mut j = i * i;
                while j <= limit { is_p[j] = false; j += i; }
            }
            i += 1;
        }
    }
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
            if p % bad_p == 1 && is_prime_check(p) {
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
