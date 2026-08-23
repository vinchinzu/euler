// Project Euler 921 - Fibonacci-based sum with Pisano period
// M = 398874989 (prime), M^2 < 2^64, so all mulmods are u64.

use rayon::prelude::*;

const M: u64 = 398_874_989;
const M_LIMIT: u64 = 1_618_034;
const INV2: u64 = (M + 1) / 2;
const TAB_N: usize = 32;
const CHUNK: u64 = 4096;

fn mod_pow(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut result = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % m;
        }
        base = base * base % m;
        exp >>= 1;
    }
    result
}

/// (F_n, F_{n+1}) mod m. Requires m^2 < 2^64.
fn fib_pair(n: u64, m: u64) -> (u64, u64) {
    if n == 0 {
        return (0, 1);
    }
    let mut a = 0u64;
    let mut b = 1u64;
    let mut mask = 1u64 << (63 - n.leading_zeros());
    while mask != 0 {
        let two_b = (b << 1) % m;
        let c = a * ((two_b + m - a) % m) % m;
        let d = (a * a % m + b * b % m) % m;
        if n & mask != 0 {
            a = d;
            b = (c + d) % m;
        } else {
            a = c;
            b = d;
        }
        mask >>= 1;
    }
    (a, b)
}

/// tab[k] = (F_{2^k}, F_{2^k+1}) mod M.
fn make_tab() -> [(u64, u64); TAB_N] {
    let mut tab = [(0u64, 0u64); TAB_N];
    tab[0] = (1, 1);
    for k in 1..TAB_N {
        let (a, b) = tab[k - 1];
        let two_b = (b << 1) % M;
        // 2 M^2 < 2^64: no intermediate reduction on the squares
        let t0 = a * ((two_b + M - a) % M) % M;
        let t1 = (a * a + b * b) % M;
        tab[k] = (t0, t1);
    }
    tab
}

#[inline(always)]
fn fib_pair_m(n: u64, tab: &[(u64, u64); TAB_N]) -> (u64, u64) {
    if n == 0 {
        return (0, 1);
    }
    let mut rest = n;
    let k0 = 63 - rest.leading_zeros();
    // SAFETY: n < 2^32 (K < pi(M) < 2^28) so k0 < TAB_N
    let (mut fx, mut fx1) = unsafe { *tab.get_unchecked(k0 as usize) };
    rest ^= 1u64 << k0;
    while rest != 0 {
        let k = 63 - rest.leading_zeros();
        let (fy, fy1) = unsafe { *tab.get_unchecked(k as usize) };
        let fxm1 = (fx1 + M - fx) % M;
        // 2 M^2 < 2^64
        let fxy = (fx * fy1 + fxm1 * fy) % M;
        let fxy1 = (fx1 * fy1 + fx * fy) % M;
        fx = fxy;
        fx1 = fxy1;
        rest ^= 1u64 << k;
    }
    (fx, fx1)
}

fn unique_prime_factors(mut n: u64) -> Vec<u64> {
    let mut v = Vec::new();
    if n % 2 == 0 {
        v.push(2);
        while n % 2 == 0 {
            n /= 2;
        }
    }
    let mut d = 3u64;
    while d * d <= n {
        if n % d == 0 {
            v.push(d);
            while n % d == 0 {
                n /= d;
            }
        }
        d += 2;
    }
    if n > 1 {
        v.push(n);
    }
    v
}

fn euler_phi(mut n: u64) -> u64 {
    let mut r = n;
    if n % 2 == 0 {
        r -= r / 2;
        while n % 2 == 0 {
            n /= 2;
        }
    }
    let mut d = 3u64;
    while d * d <= n {
        if n % d == 0 {
            r -= r / d;
            while n % d == 0 {
                n /= d;
            }
        }
        d += 2;
    }
    if n > 1 {
        r -= r / n;
    }
    r
}

fn find_pisano_period(p: u64) -> u64 {
    let mut n = if p % 5 == 1 || p % 5 == 4 {
        p - 1
    } else {
        2 * (p + 1)
    };
    let pfactors = unique_prime_factors(n);
    for &pf in &pfactors {
        while n % pf == 0 {
            let t = n / pf;
            let (f0, f1) = fib_pair(t, p);
            if f0 == 0 && f1 == 1 {
                n = t;
            } else {
                break;
            }
        }
    }
    n
}

fn find_order(a: u64, m: u64, phi_m: u64) -> u64 {
    let pfactors = unique_prime_factors(phi_m);
    let mut order = phi_m;
    for &pf in &pfactors {
        while order % pf == 0 && mod_pow(a, order / pf, m) == 1 {
            order /= pf;
        }
    }
    order
}

#[inline(always)]
fn term(k: u64, tab: &[(u64, u64); TAB_N]) -> u64 {
    let (fk, fk1) = fib_pair_m(k, tab);
    let lk = (2 * fk1 % M + M - fk) % M;
    let p = fk * INV2 % M;
    let q = lk * INV2 % M;
    let p2 = p * p % M;
    let q2 = q * q % M;
    (p2 * p2 % M * p % M + q2 * q2 % M * q % M) % M
}

fn main() {
    let pi_m = find_pisano_period(M);
    let phi_pi = euler_phi(pi_m);
    let l = find_order(5, pi_m, phi_pi);
    let tab = make_tab();

    let n_terms = M_LIMIT - 1;
    let n_chunks = ((n_terms + CHUNK - 1) / CHUNK) as usize;

    let total: u64 = (0..n_chunks)
        .into_par_iter()
        .map(|ci| {
            let i0 = 2 + ci as u64 * CHUNK;
            let i1 = (i0 + CHUNK).min(M_LIMIT + 1);
            let (fa, fb) = fib_pair(i0 - 1, l);
            let mut p5a = mod_pow(5, fa, pi_m);
            let mut p5b = mod_pow(5, fb, pi_m);
            let mut s = 0u64;
            for _ in i0..i1 {
                let k = 3 * p5b % pi_m;
                s += term(k, &tab);
                let nxt = p5a * p5b % pi_m;
                p5a = p5b;
                p5b = nxt;
            }
            s
        })
        .sum();

    println!("{}", total % M);
}
