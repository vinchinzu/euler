// Project Euler 927 - Prime Trees (R(N))
// S = intersection of S_p over all primes p.
// A number m is in S if for all prime factors p of phi(m),
// the map x -> 1 + x^p mod m reaches 0 from 1.
// R(N) = sum of elements of S not exceeding N.
//
// Two-phase: 1) find primes in S using reachability check
//            2) generate all composites from those primes, checking each

use std::collections::{BinaryHeap, HashSet};
use std::cmp::Reverse;

const N_LIMIT: usize = 10_000_000;

fn power_mod(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut res: u64 = 1;
    base %= modulus;
    while exp > 0 {
        if exp & 1 == 1 {
            res = (res as u128 * base as u128 % modulus as u128) as u64;
        }
        base = (base as u128 * base as u128 % modulus as u128) as u64;
        exp >>= 1;
    }
    res
}

fn check_reachability(m: usize, exponent: u64, visited: &mut [u32], generation: &mut u32) -> bool {
    *generation += 1;
    let g = *generation;
    let mut x: u64 = 1;
    loop {
        if (x as usize) < visited.len() && visited[x as usize] == g {
            return false;
        }
        if (x as usize) < visited.len() {
            visited[x as usize] = g;
        }
        if x == 0 {
            return true;
        }
        x = (1 + power_mod(x, exponent, m as u64)) % m as u64;
        if x == 0 {
            return true;
        }
    }
}

fn is_prime_in_s(q: usize, spf: &[usize], visited: &mut [u32], generation: &mut u32) -> bool {
    if q == 2 {
        return true;
    }
    let mut phi = q - 1;
    if phi % 2 == 0 {
        if !check_reachability(q, 2, visited, generation) {
            return false;
        }
        while phi % 2 == 0 {
            phi /= 2;
        }
    }
    while phi > 1 {
        let p = spf[phi];
        if !check_reachability(q, p as u64, visited, generation) {
            return false;
        }
        while phi % p == 0 {
            phi /= p;
        }
    }
    true
}

fn get_prime_factors(mut n: usize) -> Vec<usize> {
    let mut factors = Vec::new();
    let mut d = 2usize;
    while d * d <= n {
        if n % d == 0 {
            factors.push(d);
            while n % d == 0 {
                n /= d;
            }
        }
        d += 1;
    }
    if n > 1 {
        factors.push(n);
    }
    factors
}

fn is_composite_in_s(m: usize, visited: &mut [u32], generation: &mut u32) -> bool {
    // Get prime factors of m
    let factors_m = get_prime_factors(m);

    // Get prime factors of phi(m)
    let mut phi_factors: Vec<usize> = Vec::new();

    for &p in &factors_m {
        // Add prime factors of p-1
        let pf = get_prime_factors(p - 1);
        for &f in &pf {
            if !phi_factors.contains(&f) {
                phi_factors.push(f);
            }
        }
        // Check if p^2 divides m
        let mut temp = m;
        let mut count = 0;
        while temp % p == 0 {
            count += 1;
            temp /= p;
        }
        if count >= 2 && !phi_factors.contains(&p) {
            phi_factors.push(p);
        }
    }

    for &pf in &phi_factors {
        if !check_reachability(m, pf as u64, visited, generation) {
            return false;
        }
    }
    true
}

fn main() {
    // Sieve of smallest prime factor
    let mut spf = vec![0usize; N_LIMIT + 1];
    for i in 0..=N_LIMIT {
        spf[i] = i;
    }
    let mut i = 2;
    while i * i <= N_LIMIT {
        if spf[i] == i {
            let mut j = i * i;
            while j <= N_LIMIT {
                if spf[j] == j {
                    spf[j] = i;
                }
                j += i;
            }
        }
        i += 1;
    }

    let mut visited = vec![0u32; N_LIMIT + 1];
    let mut generation: u32 = 0;

    // Phase 1: Find primes in S
    let mut s_primes: Vec<usize> = Vec::new();
    for q in 2..=N_LIMIT {
        if spf[q] == q {
            // q is prime
            if is_prime_in_s(q, &spf, &mut visited, &mut generation) {
                s_primes.push(q);
            }
        }
    }

    // Phase 2: Generate all numbers in S up to N_LIMIT using min-heap
    let mut seen: HashSet<u64> = HashSet::new();
    let mut heap: BinaryHeap<Reverse<u64>> = BinaryHeap::new();

    heap.push(Reverse(1));
    seen.insert(1);

    let mut total_sum: i64 = 0;

    while let Some(Reverse(curr)) = heap.pop() {
        // Check if curr is in S
        let in_s = if curr == 1 {
            true
        } else if curr <= N_LIMIT as u64 && spf[curr as usize] == curr as usize {
            // curr is prime, already verified in phase 1
            true
        } else if curr <= N_LIMIT as u64 {
            is_composite_in_s(curr as usize, &mut visited, &mut generation)
        } else {
            false
        };

        if in_s {
            total_sum += curr as i64;
        }

        for &p in &s_primes {
            let nxt = curr * p as u64;
            if nxt > N_LIMIT as u64 {
                break;
            }
            if !seen.contains(&nxt) {
                seen.insert(nxt);
                heap.push(Reverse(nxt));
            }
        }
    }

    println!("{}", total_sum);
}
