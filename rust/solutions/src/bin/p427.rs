// Project Euler 427: n-sequences
// Sum of L(S) over all sequences of length N with values 1..N.

use rayon::prelude::*;

const N: usize = 7_500_000;
const MOD: u64 = 1_000_000_009;

#[inline(always)]
fn mul_mod(a: u64, b: u64) -> u64 {
    a * b % MOD
}

#[inline(always)]
fn mul32(a: u32, b: u32) -> u32 {
    ((a as u64) * (b as u64) % MOD) as u32
}

fn pow_mod(mut base: u64, mut exp: u64) -> u64 {
    let mut r = 1u64;
    while exp > 0 {
        if exp & 1 == 1 {
            r = mul_mod(r, base);
        }
        base = mul_mod(base, base);
        exp >>= 1;
    }
    r
}

#[inline(always)]
fn ncr(n: usize, r: usize, fact: &[u32], inv_fact: &[u32]) -> u64 {
    if r > n {
        return 0;
    }
    // SAFETY: r <= n <= N-1 and tables have length N+1.
    unsafe {
        mul_mod(
            mul_mod(
                *fact.get_unchecked(n) as u64,
                *inv_fact.get_unchecked(r) as u64,
            ),
            *inv_fact.get_unchecked(n - r) as u64,
        )
    }
}

fn compute_fk(
    k: usize,
    fact: &[u32],
    inv_fact: &[u32],
    pow_n: &[u32],
    pow_nm1: &[u32],
) -> u64 {
    let imax = N / (k + 1);
    let mut fk = 0u64;
    for i in 0..=imax {
        let a = N - i * k - 1;
        let exp1 = a + 1 - i;
        // SAFETY: i <= N/(k+1), exp1 = N - i*(k+1) <= N, tables sized N+1 / N+2.
        let t1 = unsafe {
            mul_mod(
                mul_mod(ncr(a, i, fact, inv_fact), *pow_nm1.get_unchecked(i) as u64),
                *pow_n.get_unchecked(exp1) as u64,
            )
        };
        let mut term = t1;
        if i != 0 {
            let t2 = unsafe {
                mul_mod(
                    mul_mod(
                        ncr(a, i - 1, fact, inv_fact),
                        *pow_nm1.get_unchecked(i - 1) as u64,
                    ),
                    *pow_n.get_unchecked(exp1 + 1) as u64,
                )
            };
            term += t2;
        }
        if i & 1 == 0 {
            fk += term;
        } else {
            fk += (MOD << 1) - term;
        }
    }
    fk % MOD
}

fn main() {
    let n32 = N as u32;
    let (fact, (pow_n, pow_nm1)) = rayon::join(
        || {
            let mut fact = vec![1u32; N + 1];
            for i in 1..=N {
                fact[i] = mul32(fact[i - 1], i as u32);
            }
            fact
        },
        || {
            rayon::join(
                || {
                    let mut pow_n = vec![1u32; N + 2];
                    for i in 1..=N + 1 {
                        pow_n[i] = mul32(pow_n[i - 1], n32);
                    }
                    pow_n
                },
                || {
                    let mut pow_nm1 = vec![1u32; N + 2];
                    let nm1 = n32 - 1;
                    for i in 1..=N + 1 {
                        pow_nm1[i] = mul32(pow_nm1[i - 1], nm1);
                    }
                    pow_nm1
                },
            )
        },
    );

    let mut inv_fact = vec![1u32; N + 1];
    inv_fact[N] = pow_mod(fact[N] as u64, MOD - 2) as u32;
    for i in (0..N).rev() {
        inv_fact[i] = mul32(inv_fact[i + 1], (i + 1) as u32);
    }

    // Equal-work k-ranges: small k is a singleton (many i), large k is batched.
    let mut ranges = Vec::with_capacity(1024);
    let mut k = 1usize;
    while k <= N {
        let per = (N / (k + 1)).max(1);
        let width = (250_000 / per).max(1);
        let end = (k + width).min(N + 1);
        ranges.push(k..end);
        k = end;
    }

    let fact_r = &fact;
    let inv_fact_r = &inv_fact;
    let pow_n_r = &pow_n;
    let pow_nm1_r = &pow_nm1;
    let parts: Vec<Vec<u64>> = ranges
        .into_par_iter()
        .map(|r| {
            r.map(|kk| compute_fk(kk, fact_r, inv_fact_r, pow_n_r, pow_nm1_r))
                .collect()
        })
        .collect();

    let mut ans = 0u64;
    let mut prev_f = 0u64;
    let mut k = 1usize;
    for part in parts {
        for fk in part {
            let delta = (fk + MOD - prev_f) % MOD;
            ans = (ans + delta * k as u64) % MOD;
            prev_f = fk;
            k += 1;
        }
    }

    println!("{ans}");
}
