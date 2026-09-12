// Problem 977: Iterated Functions
// Ported from python/977.py.
// Incremental a^L across L, rayon over independent small-L (heavy q-loops).

use rayon::prelude::*;

const MOD: u64 = 1_000_000_007;
const INV3: u64 = 333_333_336; // 3^{-1} mod MOD
const PAR_L: usize = 256;

#[inline(always)]
fn mod_mul(a: u64, b: u64) -> u64 {
    (a * b) % MOD
}

fn mod_pow(mut base: u64, mut exp: usize) -> u64 {
    let mut res = 1u64;
    base %= MOD;
    while exp > 0 {
        if exp & 1 == 1 {
            res = mod_mul(res, base);
        }
        base = mod_mul(base, base);
        exp >>= 1;
    }
    res
}

/// Contribution of a single L, given pow_a[a] = a^L for a = 1..=max_a.
#[inline(always)]
fn contrib_from_pow(n: usize, l: usize, pow_a: &[u64]) -> u64 {
    let r = n - l;
    let mut total = 0u64;

    if r >= 1 {
        let q_full = (r - 1) / l;

        for q in 0..q_full {
            let a = q + 1;
            let b = q + 2;
            let a_l = unsafe { *pow_a.get_unchecked(a) };
            let b_l = unsafe { *pow_a.get_unchecked(b) };
            let a_l1 = mod_mul(a_l, a as u64);
            let t0 = mod_mul(q as u64, a_l);
            let t1 = mod_mul(mod_mul(a as u64, a as u64), b_l);
            let t2 = mod_mul(b as u64, a_l1);
            let term = (t0 + t1 + MOD - t2) % MOD;
            total += term;
            if total >= MOD {
                total -= MOD;
            }
        }

        let q = q_full;
        let m_last = (r - 1) - q_full * l;
        let a = q + 1;
        let b = q + 2;
        let a_l = unsafe { *pow_a.get_unchecked(a) };
        let mut term = mod_mul(q as u64, a_l);

        if m_last >= 1 {
            let a_l1 = mod_mul(a_l, a as u64);
            let exp = l + 1 - m_last;
            let a_l1_m = match exp {
                1 => a as u64,
                2 => mod_mul(a as u64, a as u64),
                3 => {
                    let aa = mod_mul(a as u64, a as u64);
                    mod_mul(aa, a as u64)
                }
                _ => mod_pow(a as u64, exp),
            };
            let b_m = match m_last {
                1 => b as u64,
                2 => mod_mul(b as u64, b as u64),
                3 => {
                    let bb = mod_mul(b as u64, b as u64);
                    mod_mul(bb, b as u64)
                }
                _ => mod_pow(b as u64, m_last),
            };
            let inner = (mod_mul(a_l1_m, b_m) + MOD - a_l1) % MOD;
            term = (term + mod_mul(b as u64, inner)) % MOD;
        }
        total += term;
        if total >= MOD {
            total -= MOD;
        }
    }

    let q = r / l;
    let rem = r - q * l;
    let a = q + 1;
    let b = q + 2;
    let base = if rem == 0 {
        if a < pow_a.len() {
            unsafe { *pow_a.get_unchecked(a) }
        } else {
            mod_pow(a as u64, l)
        }
    } else {
        mod_mul(mod_pow(a as u64, l - rem), mod_pow(b as u64, rem))
    };
    total += base;
    if total >= MOD {
        total -= MOD;
    }
    total
}

fn contrib_l_independent(n: usize, l: usize) -> u64 {
    let r = n - l;
    let max_a = if r >= 1 { (r - 1) / l + 2 } else { 2 };
    let mut pow_a = vec![0u64; max_a + 1];
    match l {
        2 => {
            for a in 1..=max_a {
                pow_a[a] = mod_mul(a as u64, a as u64);
            }
        }
        3 => {
            for a in 1..=max_a {
                let aa = mod_mul(a as u64, a as u64);
                pow_a[a] = mod_mul(aa, a as u64);
            }
        }
        _ => {
            for a in 1..=max_a {
                pow_a[a] = mod_pow(a as u64, l);
            }
        }
    }
    contrib_from_pow(n, l, &pow_a)
}

fn count_mod(n: usize) -> u64 {
    if n == 1 {
        return 1;
    }

    // L = 1: sum_{q=1}^{n-2} q(q+1) + n = m(m+1)(m+2)/3 + n
    let m = (n - 2) as u64;
    let mut total = (m % MOD) * ((m + 1) % MOD) % MOD * ((m + 2) % MOD) % MOD * INV3 % MOD;
    total += n as u64 % MOD;
    if total >= MOD {
        total -= MOD;
    }

    let par_hi = PAR_L.min(n);
    let par_sum: u64 = (2..par_hi + 1)
        .into_par_iter()
        .with_min_len(1)
        .map(|l| contrib_l_independent(n, l))
        .sum();
    total += par_sum % MOD;
    if total >= MOD {
        total -= MOD;
    }

    if par_hi < n {
        // Incremental a^L for the long tail. max_a shrinks with L.
        let l0 = par_hi + 1;
        let r0 = n - l0;
        let max_a0 = if r0 >= 1 { (r0 - 1) / l0 + 2 } else { 2 };
        let mut pow_a = vec![0u64; max_a0 + 1];
        for a in 1..=max_a0 {
            pow_a[a] = mod_pow(a as u64, l0);
        }
        total += contrib_from_pow(n, l0, &pow_a);
        if total >= MOD {
            total -= MOD;
        }

        for l in l0 + 1..=n {
            let r = n - l;
            let max_a = if r >= 1 { (r - 1) / l + 2 } else { 2 };
            let ua = max_a.min(pow_a.len() - 1);
            for a in 1..=ua {
                pow_a[a] = mod_mul(pow_a[a], a as u64);
            }
            total += contrib_from_pow(n, l, &pow_a);
            if total >= MOD {
                total -= MOD;
            }
        }
    }

    total % MOD
}

fn main() {
    debug_assert_eq!(count_mod(3), 8);
    debug_assert_eq!(count_mod(7), 174);
    println!("{}", count_mod(1_000_000));
}
