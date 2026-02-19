// Project Euler 927 - Prime Trees (R(N))
// S = intersection of S_p over all primes p.
// A number m is in S if for all prime factors p of phi(m),
// the map x -> 1 + x^p mod m reaches 0 from 1.
// R(N) = sum of elements of S not exceeding N.
//
// Optimizations over naive visited-array orbit tracing:
// 1. Brent's cycle detection for large m (cache-friendly, O(1) memory)
// 2. Barrett reduction for fast modular arithmetic (avoid slow div)
// 3. Specialised step functions for small exponents (2, 3, 5, 7)
// 4. Stack-allocated phi_factors instead of Vec heap allocation
// 5. Collect all exponents first, amortize Barrett precomputation

use rayon::prelude::*;
use std::cell::RefCell;

const N_LIMIT: usize = 10_000_000;
const VIS_LIMIT: usize = 200_000;

thread_local! {
    static TL: RefCell<(Vec<u32>, u32)> =
        RefCell::new((vec![0u32; VIS_LIMIT + 1], 0));
}

/// Barrett reduction for fast modular arithmetic.
/// For m <= 10^7, products a*b < 10^14 < 2^47.
/// Precompute inv = floor(2^47 / m), then reduce via multiply+shift.
#[derive(Copy, Clone)]
struct Barrett {
    m: u64,
    inv: u64,
}

impl Barrett {
    #[inline(always)]
    fn new(m: u64) -> Self {
        Barrett {
            m,
            inv: (1u64 << 47) / m,
        }
    }

    /// Reduce x mod m for x < m^2 < 2^48.
    /// Uses u128 multiplication for Barrett quotient estimation.
    #[inline(always)]
    fn reduce(&self, x: u64) -> u64 {
        // q = floor(x * inv >> 47) approximates floor(x / m)
        // x < 2^48, inv < 2^24 => x*inv < 2^72, needs u128
        let q = ((x as u128 * self.inv as u128) >> 47) as u64;
        let r = x - q * self.m;
        // r < 2*m guaranteed by Barrett theory
        if r >= self.m { r - self.m } else { r }
    }

    /// Compute (a * b) % m where a, b < m.
    #[inline(always)]
    fn mul_mod(&self, a: u64, b: u64) -> u64 {
        self.reduce(a * b)
    }
}

#[inline(always)]
fn pow_mod_barrett(mut b: u64, mut e: u64, bar: &Barrett) -> u64 {
    let mut r = 1u64;
    b = bar.reduce(b);
    while e > 0 {
        if e & 1 == 1 {
            r = bar.mul_mod(r, b);
        }
        b = bar.mul_mod(b, b);
        e >>= 1;
    }
    r
}

/// f(x) = (x^exp + 1) % m using Barrett reduction, small-exp specialised.
#[inline(always)]
fn step_bar(x: u64, exp: u64, bar: &Barrett) -> u64 {
    let m = bar.m;
    match exp {
        2 => {
            let v = bar.mul_mod(x, x) + 1;
            if v >= m { v - m } else { v }
        }
        3 => {
            let x2 = bar.mul_mod(x, x);
            let v = bar.mul_mod(x2, x) + 1;
            if v >= m { v - m } else { v }
        }
        5 => {
            let x2 = bar.mul_mod(x, x);
            let x4 = bar.mul_mod(x2, x2);
            let v = bar.mul_mod(x4, x) + 1;
            if v >= m { v - m } else { v }
        }
        7 => {
            let x2 = bar.mul_mod(x, x);
            let x4 = bar.mul_mod(x2, x2);
            let v = bar.mul_mod(bar.mul_mod(x4, x2), x) + 1;
            if v >= m { v - m } else { v }
        }
        _ => {
            let v = pow_mod_barrett(x, exp, bar) + 1;
            if v >= m { v - m } else { v }
        }
    }
}

/// Brent's cycle detection for orbit x -> f(x) starting at x=1.
/// Returns true if 0 is reached before entering a cycle.
#[inline(never)]
fn check_reach_brent(bar: &Barrett, exp: u64) -> bool {
    let mut power = 1u64;
    let mut lam = 0u64;
    let mut tortoise = 1u64;
    let mut hare = step_bar(1, exp, bar);
    if hare == 0 {
        return true;
    }
    while tortoise != hare {
        if lam == power {
            tortoise = hare;
            power <<= 1;
            lam = 0;
        }
        hare = step_bar(hare, exp, bar);
        if hare == 0 {
            return true;
        }
        lam += 1;
    }
    false
}

/// Visited-array orbit detection for small m (fits in cache).
fn check_reach_visited(bar: &Barrett, exp: u64) -> bool {
    TL.with(|cell| {
        let (vis, gn) = &mut *cell.borrow_mut();
        *gn = gn.wrapping_add(1);
        if *gn == 0 {
            vis.fill(0);
            *gn = 1;
        }
        let g = *gn;
        let mut x = 1u64;
        loop {
            let i = x as usize;
            // SAFETY: x < m <= VIS_LIMIT, vis has VIS_LIMIT+1 entries
            unsafe {
                if *vis.get_unchecked(i) == g {
                    return false;
                }
                *vis.get_unchecked_mut(i) = g;
            }
            if x == 0 {
                return true;
            }
            x = step_bar(x, exp, bar);
            if x == 0 {
                return true;
            }
        }
    })
}

/// Check reachability for all exponents, amortizing Barrett precomputation.
fn check_reach_all(m: u64, exps: &[u64]) -> bool {
    let bar = Barrett::new(m);
    if m <= VIS_LIMIT as u64 {
        for &exp in exps {
            if !check_reach_visited(&bar, exp) {
                return false;
            }
        }
    } else {
        for &exp in exps {
            if !check_reach_brent(&bar, exp) {
                return false;
            }
        }
    }
    true
}

fn is_prime_in_s(q: usize, spf: &[usize]) -> bool {
    if q == 2 {
        return true;
    }
    // Collect distinct prime factors of phi(q) = q-1
    let mut exps = [0u64; 16];
    let mut ne = 0usize;
    let mut phi = q - 1;
    if phi & 1 == 0 {
        exps[ne] = 2;
        ne += 1;
        while phi & 1 == 0 {
            phi >>= 1;
        }
    }
    while phi > 1 {
        let p = spf[phi];
        exps[ne] = p as u64;
        ne += 1;
        while phi % p == 0 {
            phi /= p;
        }
    }
    check_reach_all(q as u64, &exps[..ne])
}

fn is_composite_in_s(m: usize, spf: &[usize]) -> bool {
    let mut phi_factors = [0u64; 32];
    let mut nf = 0usize;
    let mut temp = m;
    while temp > 1 {
        let p = spf[temp];
        let mut t = p - 1;
        while t > 1 {
            let f = spf[t];
            let fv = f as u64;
            let mut found = false;
            for i in 0..nf {
                if phi_factors[i] == fv {
                    found = true;
                    break;
                }
            }
            if !found {
                phi_factors[nf] = fv;
                nf += 1;
            }
            while t % f == 0 {
                t /= f;
            }
        }
        let mut cnt = 0;
        let mut tmp = m;
        while tmp % p == 0 {
            cnt += 1;
            tmp /= p;
        }
        if cnt >= 2 {
            let pv = p as u64;
            let mut found = false;
            for i in 0..nf {
                if phi_factors[i] == pv {
                    found = true;
                    break;
                }
            }
            if !found {
                phi_factors[nf] = pv;
                nf += 1;
            }
        }
        while temp % p == 0 {
            temp /= p;
        }
    }
    check_reach_all(m as u64, &phi_factors[..nf])
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
