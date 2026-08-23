// Project Euler 526 - Largest Prime Factor Sum
//
// N=10^16, K=9. Best 9-tuples live in two residue families mod
// 2^4*3^3*5^2*7, then lift through remaining pe<=30 while keeping
// the window free of those pe. Search near N with a small-prime
// window filter + deterministic SPRP (no sieve to 1e8).

use rayon::prelude::*;

const K: u64 = 9;
const N: u64 = 10_000_000_000_000_000;
const SQRT_N: u64 = 100_000_000;
const MOD0: u64 = 2520;
const SEED_A: u64 = 2201;
const SEED_B: u64 = 311;

const STEPS: [(u64, u64); 9] = [
    (11, 11),
    (13, 13),
    (16, 2),
    (17, 17),
    (19, 19),
    (23, 23),
    (25, 5),
    (27, 3),
    (29, 29),
];

const SMALL_P: [u64; 44] = [
    31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107,
    109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191,
    193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251,
];

#[inline(always)]
fn mul_mod(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}

#[inline(always)]
fn pow_mod(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut r = 1u64;
    while exp > 0 {
        if exp & 1 == 1 {
            r = mul_mod(r, base, m);
        }
        base = mul_mod(base, base, m);
        exp >>= 1;
    }
    r
}

#[inline(always)]
fn sprp(n: u64, a: u64) -> bool {
    let mut d = n - 1;
    let s = d.trailing_zeros();
    d >>= s;
    let mut x = pow_mod(a, d, n);
    if x == 1 || x == n - 1 {
        return true;
    }
    for _ in 1..s {
        x = mul_mod(x, x, n);
        if x == n - 1 {
            return true;
        }
    }
    false
}

/// Deterministic for n < 3_825_123_056_546_413_051 (covers all cofactors <= N).
#[inline(always)]
fn is_prime_sprp(n: u64) -> bool {
    sprp(n, 2)
        && sprp(n, 3)
        && sprp(n, 5)
        && sprp(n, 7)
        && sprp(n, 11)
        && sprp(n, 13)
        && sprp(n, 23)
}

/// True iff none of n..n+8 is 0 mod any p in SMALL_P.
#[inline(always)]
fn window_ok(n: u64) -> bool {
    for &p in &SMALL_P {
        let r = n % p;
        if r == 0 || r > p - K {
            return false;
        }
    }
    true
}

/// After dividing out the residue denom, leftover 2/3/5/7-powers may remain
/// (32, 49, 81, 125, ...). Strip those, then require a prime > sqrt(N).
#[inline(always)]
fn cofactor_ok(mut c: u64) -> bool {
    while c & 1 == 0 {
        c >>= 1;
    }
    while c % 3 == 0 {
        c /= 3;
    }
    while c % 5 == 0 {
        c /= 5;
    }
    while c % 7 == 0 {
        c /= 7;
    }
    c == 1 || (c > SQRT_N && is_prime_sprp(c))
}

#[inline(always)]
fn eval_a(n: u64) -> Option<u64> {
    if !window_ok(n) {
        return None;
    }
    if !is_prime_sprp(n)
        || !is_prime_sprp(n + 8)
        || !is_prime_sprp(n + 2)
        || !is_prime_sprp(n + 6)
    {
        return None;
    }
    let c1 = (n + 1) / 6;
    let c3 = (n + 3) / 4;
    let c4 = (n + 4) / 315;
    let c5 = (n + 5) / 2;
    let c7 = (n + 7) / 24;
    if !cofactor_ok(c1) || !cofactor_ok(c3) || !cofactor_ok(c4) || !cofactor_ok(c5) || !cofactor_ok(c7)
    {
        return None;
    }
    Some(4 * n + 16 + c1 + c3 + c4 + c5 + c7)
}

#[inline(always)]
fn eval_b(n: u64) -> Option<u64> {
    if !window_ok(n) {
        return None;
    }
    if !is_prime_sprp(n)
        || !is_prime_sprp(n + 8)
        || !is_prime_sprp(n + 2)
        || !is_prime_sprp(n + 6)
    {
        return None;
    }
    let c1 = (n + 1) / 24;
    let c3 = (n + 3) / 2;
    let c4 = (n + 4) / 315;
    let c5 = (n + 5) / 4;
    let c7 = (n + 7) / 6;
    if !cofactor_ok(c1) || !cofactor_ok(c3) || !cofactor_ok(c4) || !cofactor_ok(c5) || !cofactor_ok(c7)
    {
        return None;
    }
    Some(4 * n + 16 + c1 + c3 + c4 + c5 + c7)
}

fn lift(seed: u64) -> Vec<u64> {
    let mut offs = vec![seed];
    let mut modulus = MOD0;
    for &(pe, p) in &STEPS {
        let mut new = Vec::with_capacity(offs.len() * p as usize);
        for &a in &offs {
            let mut start = a;
            for _ in 0..p {
                let r = start % pe;
                if r > 0 && r + 8 < pe {
                    new.push(start);
                }
                start += modulus;
            }
        }
        offs = new;
        modulus *= p;
    }
    offs
}

fn max_h(base: u64, nlim: u64, offs: &[u64], pat_a: bool) -> u64 {
    offs.par_chunks(4096)
        .map(|chunk| {
            let mut best = 0u64;
            if pat_a {
                for &a in chunk {
                    let n = base + a;
                    if n > nlim {
                        continue;
                    }
                    if let Some(h) = eval_a(n) {
                        if h > best {
                            best = h;
                        }
                    }
                }
            } else {
                for &a in chunk {
                    let n = base + a;
                    if n > nlim {
                        continue;
                    }
                    if let Some(h) = eval_b(n) {
                        if h > best {
                            best = h;
                        }
                    }
                }
            }
            best
        })
        .max()
        .unwrap_or(0)
}

fn main() {
    let (offs_a, offs_b) = rayon::join(|| lift(SEED_A), || lift(SEED_B));
    let mut modulus = MOD0;
    for &(_, p) in &STEPS {
        modulus *= p;
    }

    let mut base = (N / modulus) * modulus;
    loop {
        let nlim = N;
        let (ha, hb) = rayon::join(
            || max_h(base, nlim, &offs_a, true),
            || max_h(base, nlim, &offs_b, false),
        );
        let ans = ha.max(hb);
        if ans != 0 {
            println!("{ans}");
            return;
        }
        if base < modulus {
            break;
        }
        base -= modulus;
    }
    println!("0");
}
