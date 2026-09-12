// Project Euler 302: Strong Achilles Numbers
// Count Strong Achilles numbers up to 10^18.
// Recursive generation of candidates with factorizations of n and phi(n).

use std::collections::HashSet;
use euler_utils::{gcd_i32, is_prime, sieve, sieve_smallest_factor};
use rayon::prelude::*;

const N: i64 = 1_000_000_000_000_000_000; // 10^18
const MAX_SPF: usize = 1_000_100;
const MAX_DICT: usize = 32;

#[derive(Clone, Copy)]
struct Dict {
    p: [i32; MAX_DICT],
    e: [i32; MAX_DICT],
    n: usize,
}

impl Dict {
    fn new() -> Self {
        Dict { p: [0; MAX_DICT], e: [0; MAX_DICT], n: 0 }
    }

    #[inline]
    fn get(&self, key: i32) -> i32 {
        for i in 0..self.n {
            if self.p[i] == key {
                return self.e[i];
            }
        }
        0
    }

    #[inline]
    fn has(&self, key: i32) -> bool {
        for i in 0..self.n {
            if self.p[i] == key {
                return true;
            }
        }
        false
    }

    #[inline]
    fn set(&mut self, key: i32, val: i32) {
        for i in 0..self.n {
            if self.p[i] == key {
                self.e[i] = val;
                return;
            }
        }
        self.p[self.n] = key;
        self.e[self.n] = val;
        self.n += 1;
    }

    #[inline]
    fn inc(&mut self, key: i32, val: i32) {
        for i in 0..self.n {
            if self.p[i] == key {
                self.e[i] += val;
                return;
            }
        }
        self.p[self.n] = key;
        self.e[self.n] = val;
        self.n += 1;
    }

    #[inline]
    fn del(&mut self, key: i32) {
        for i in 0..self.n {
            if self.p[i] == key {
                let last = self.n - 1;
                self.p[i] = self.p[last];
                self.e[i] = self.e[last];
                self.n = last;
                return;
            }
        }
    }

    #[inline]
    fn gcd_of_vals(&self) -> i32 {
        if self.n == 0 {
            return 0;
        }
        let mut g = 0i32;
        for i in 0..self.n {
            g = gcd_i32(g, self.e[i]);
        }
        g
    }
}

fn main() {
    let spf = sieve_smallest_factor(MAX_SPF);

    let limit = (N as f64).cbrt() as usize + 2;
    let is_p = sieve(limit);
    let primes: Vec<i32> = (2..=limit).filter(|&i| is_p[i]).map(|i| i as i32).collect();

    let nthreads = rayon::current_num_threads().max(1);
    let mut sets: Vec<HashSet<i64>> = (0..nthreads)
        .into_par_iter()
        .map(|t| {
            let mut set = HashSet::with_capacity(1_200_000 / nthreads + 1024);
            let mut factors = Dict::new();
            let mut phi = Dict::new();
            let mut i = t;
            while i < primes.len() {
                let p = primes[i];
                if (p as f64) * (p as f64) * (p as f64) < N as f64 {
                    add_prime_fn(1, p, 3, &mut factors, &mut phi, p, &primes, &spf, &mut set);
                }
                i += nthreads;
            }
            set
        })
        .collect();
    let mut best = 0usize;
    for i in 1..sets.len() {
        if sets[i].len() > sets[best].len() {
            best = i;
        }
    }
    let mut achilles_set = sets.swap_remove(best);
    for s in sets {
        achilles_set.extend(s);
    }
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
    let mut bad_p = 0i32;
    for i in 0..phi.n {
        if phi.e[i] == 1 && phi.p[i] > bad_p {
            bad_p = phi.p[i];
        }
    }

    if bad_p == 0 {
        if factors.n > 0 {
            let ge = factors.gcd_of_vals();
            let gp = phi.gcd_of_vals();
            if ge == 1 && gp == 1 {
                achilles_set.insert(n);
            }
        }

        let mut phi_keys = [0i32; MAX_DICT];
        let phi_n = phi.n;
        phi_keys[..phi_n].copy_from_slice(&phi.p[..phi_n]);

        for i in 0..phi_n {
            let p = phi_keys[i];
            if p < max_p {
                add_prime_fn(n, p, 2, factors, phi, p, primes, spf, achilles_set);
            }
        }

        for &p in primes {
            if p >= max_p {
                break;
            }
            if (n as f64) * (p as f64) * (p as f64) * (p as f64) >= N as f64 {
                break;
            }
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
    if factors.has(p) {
        return;
    }

    let prev_e = phi.get(p);
    let had_prev = phi.has(p);

    phi.inc(p, min_e - 1);

    let mut phi_p_factors = [0i32; 40];
    let mut npf = 0usize;
    let mut temp = p - 1;
    while temp > 1 {
        let pf = if (temp as usize) < MAX_SPF {
            spf[temp as usize] as i32
        } else {
            let mut pf = temp;
            for &pr in primes {
                if (pr as i64) * (pr as i64) > temp as i64 {
                    break;
                }
                if temp % pr == 0 {
                    pf = pr;
                    break;
                }
            }
            pf
        };
        phi_p_factors[npf] = pf;
        npf += 1;
        phi.inc(pf, 1);
        temp /= pf;
    }

    let mut e = min_e;
    let mut power_p = 1i64;
    for _ in 0..min_e {
        if power_p > N / p as i64 {
            power_p = N + 1;
            break;
        }
        power_p *= p as i64;
    }

    while power_p <= N && n <= N / power_p {
        factors.set(p, e);
        helper(n * power_p, factors, phi, max_p, primes, spf, achilles_set);

        phi.inc(p, 1);
        e += 1;
        if power_p > N / p as i64 {
            break;
        }
        power_p *= p as i64;
    }

    factors.del(p);

    if had_prev {
        phi.set(p, prev_e);
    } else {
        phi.del(p);
    }

    for i in 0..npf {
        let pf = phi_p_factors[i];
        let cur = phi.get(pf);
        if cur <= 1 {
            phi.del(pf);
        } else {
            phi.set(pf, cur - 1);
        }
    }
}
