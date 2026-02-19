// Project Euler 927 - Prime Trees (R(N))
// S = intersection of S_p over all primes p.
// A number m is in S if for all prime factors p of phi(m),
// the map x -> 1 + x^p mod m reaches 0 from 1.
// R(N) = sum of elements of S not exceeding N.

use rayon::prelude::*;
use std::cell::RefCell;

const N_LIMIT: usize = 10_000_000;

thread_local! {
    static TL: RefCell<(Vec<u32>, u32)> =
        RefCell::new((vec![0u32; N_LIMIT + 1], 0));
}

#[inline(always)]
fn pow_mod(mut b: u64, mut e: u64, m: u64) -> u64 {
    // m <= 10^7, so b*b <= (10^7)^2 = 10^14 < 2^47 — fits in u64
    let mut r = 1u64;
    b %= m;
    while e > 0 {
        if e & 1 == 1 {
            r = r * b % m;
        }
        b = b * b % m;
        e >>= 1;
    }
    r
}

fn check_reach(m: u64, exp: u64) -> bool {
    TL.with(|cell| {
        let (vis, gn) = &mut *cell.borrow_mut();
        *gn = gn.wrapping_add(1);
        if *gn == 0 {
            vis.fill(0);
            *gn = 1;
        }
        let g = *gn;
        let mut x = 1u64;
        if exp == 2 {
            loop {
                let i = x as usize;
                // SAFETY: x < m <= N_LIMIT, vis has N_LIMIT+1 entries
                unsafe {
                    if *vis.get_unchecked(i) == g {
                        return false;
                    }
                    *vis.get_unchecked_mut(i) = g;
                }
                if x == 0 {
                    return true;
                }
                x = (x * x % m + 1) % m;
                if x == 0 {
                    return true;
                }
            }
        } else {
            loop {
                let i = x as usize;
                unsafe {
                    if *vis.get_unchecked(i) == g {
                        return false;
                    }
                    *vis.get_unchecked_mut(i) = g;
                }
                if x == 0 {
                    return true;
                }
                x = (pow_mod(x, exp, m) + 1) % m;
                if x == 0 {
                    return true;
                }
            }
        }
    })
}

fn is_prime_in_s(q: usize, spf: &[usize]) -> bool {
    if q == 2 {
        return true;
    }
    let mut phi = q - 1;
    if phi & 1 == 0 {
        if !check_reach(q as u64, 2) {
            return false;
        }
        while phi & 1 == 0 {
            phi >>= 1;
        }
    }
    while phi > 1 {
        let p = spf[phi];
        if !check_reach(q as u64, p as u64) {
            return false;
        }
        while phi % p == 0 {
            phi /= p;
        }
    }
    true
}

fn is_composite_in_s(m: usize, spf: &[usize]) -> bool {
    let mut phi_factors = Vec::with_capacity(16);
    let mut temp = m;
    while temp > 1 {
        let p = spf[temp];
        // Add prime factors of p-1
        let mut t = p - 1;
        while t > 1 {
            let f = spf[t];
            if !phi_factors.contains(&f) {
                phi_factors.push(f);
            }
            while t % f == 0 {
                t /= f;
            }
        }
        // If p^2 | m, add p as a factor of phi(m)
        let mut cnt = 0;
        let mut tmp = m;
        while tmp % p == 0 {
            cnt += 1;
            tmp /= p;
        }
        if cnt >= 2 && !phi_factors.contains(&p) {
            phi_factors.push(p);
        }
        while temp % p == 0 {
            temp /= p;
        }
    }
    for &f in &phi_factors {
        if !check_reach(m as u64, f as u64) {
            return false;
        }
    }
    true
}

fn main() {
    // Smallest prime factor sieve
    let mut spf = vec![0usize; N_LIMIT + 1];
    for i in 0..=N_LIMIT {
        spf[i] = i;
    }
    {
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
    }

    // Phase 1: Find S-primes (parallel)
    let primes: Vec<usize> = (2..=N_LIMIT).filter(|&q| spf[q] == q).collect();
    let flags: Vec<bool> = primes.par_iter().map(|&q| is_prime_in_s(q, &spf)).collect();

    let mut is_sp = vec![false; N_LIMIT + 1];
    let mut total = 1i64; // 1 is always in S
    for (&q, &f) in primes.iter().zip(flags.iter()) {
        if f {
            is_sp[q] = true;
            total += q as i64;
        }
    }

    // Mark numbers with a non-S prime factor (smooth sieve)
    let mut smooth = vec![true; N_LIMIT + 1];
    for &q in &primes {
        if !is_sp[q] {
            let mut j = q;
            while j <= N_LIMIT {
                smooth[j] = false;
                j += q;
            }
        }
    }

    // Phase 2: Check smooth composites (parallel)
    let sum2: i64 = (4..=N_LIMIT)
        .into_par_iter()
        .map(|m| -> i64 {
            if !smooth[m] || spf[m] == m {
                return 0;
            }
            if is_composite_in_s(m, &spf) {
                m as i64
            } else {
                0
            }
        })
        .sum();

    total += sum2;
    println!("{}", total);
}
