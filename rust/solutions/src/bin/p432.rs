// Project Euler 432: Totient sum
// S(K, N) = Σ_{i=1..N} φ(K i), K=510510=2·3·5·7·11·13·17, N=10^11, mod 10^9.
//
// For square-free K, φ(K n) = φ(K) φ(n) · Π_{p|gcd(K,n)} p/(p-1), which
// unfolds to S(K,N) = φ(K) · Σ_{d 17-smooth} Φ(⌊N/d⌋), Φ(n)=Σ_{k≤n} φ(k).
//
// Φ via linearized Du Jiao: sieve to ~N^{2/3}; remaining Φ(⌊N/i⌋) filled
// bottom-up in small/large arrays (no HashMap). Arithmetic mod 10^9.
// large[] is filled in doubling waves (i > max_i/2, then > max_i/4, …)
// so each wave only reads already-written slots and can run in parallel.

use rayon::prelude::*;

const N: u64 = 100_000_000_000;
const MOD: u64 = 1_000_000_000;
const PRIMES_OF_K: [u64; 7] = [2, 3, 5, 7, 11, 13, 17];

#[inline(always)]
fn add_mod(a: u64, b: u64) -> u64 {
    let s = a + b;
    if s >= MOD { s - MOD } else { s }
}

#[inline(always)]
fn sub_mod(a: u64, b: u64) -> u64 {
    if a >= b { a - b } else { a + MOD - b }
}

#[inline(always)]
fn mul_mod(a: u64, b: u64) -> u64 {
    a * b % MOD
}

/// n(n+1)/2 mod MOD. n(n+1) is even; reduce n mod 2·MOD first.
#[inline(always)]
fn triangle_mod(n: u64) -> u64 {
    let n = n % (2 * MOD);
    (n * (n + 1) / 2) % MOD
}

#[inline(always)]
fn isqrt_u64(n: u64) -> u64 {
    let mut s = (n as f64).sqrt() as u64;
    while s * s > n {
        s -= 1;
    }
    while s < n / (s + 1) {
        s += 1;
    }
    s
}

#[inline(always)]
fn isqrt_u32(n: u32) -> u32 {
    let mut s = (n as f64).sqrt() as u32;
    let n64 = n as u64;
    while (s as u64) * (s as u64) > n64 {
        s -= 1;
    }
    while (s as u64 + 1) * (s as u64 + 1) <= n64 {
        s += 1;
    }
    s
}

/// Φ(q): `small[q]` if q ≤ limit, else `large_tail[parent/q - offset]`.
#[inline(always)]
fn lookup_phi(
    q: u64,
    limit: u64,
    small: &[u32],
    large_tail: &[u32],
    parent: u64,
    offset: usize,
) -> u64 {
    if q <= limit {
        // SAFETY: q <= limit and small.len() == limit+1.
        unsafe { *small.get_unchecked(q as usize) as u64 }
    } else {
        // SAFETY: q > limit ⇒ parent/q >= offset, index in large_tail.
        unsafe { *large_tail.get_unchecked((parent / q) as usize - offset) as u64 }
    }
}

/// Φ(x) from Σ_{d≤x} Φ(⌊x/d⌋) = x(x+1)/2, split at √x (one div per term).
#[inline(always)]
fn compute_phi(
    x: u64,
    limit: u64,
    small: &[u32],
    large_tail: &[u32],
    parent: u64,
    offset: usize,
) -> u32 {
    if x <= u32::MAX as u64 {
        compute_phi_u32(x as u32, limit, small, large_tail, parent, offset)
    } else {
        compute_phi_u64(x, limit, small, large_tail, parent, offset)
    }
}

#[inline(always)]
fn compute_phi_u64(
    x: u64,
    limit: u64,
    small: &[u32],
    large_tail: &[u32],
    parent: u64,
    offset: usize,
) -> u32 {
    let mut f = triangle_mod(x);
    let s = isqrt_u64(x);

    let mut d = 2u64;
    while d <= s {
        let q = x / d;
        f = sub_mod(f, lookup_phi(q, limit, small, large_tail, parent, offset));
        d += 1;
    }

    let mut prev = x;
    let mut m = 1u64;
    while m <= s {
        let nxt = x / (m + 1);
        if prev > s {
            let sm = unsafe { *small.get_unchecked(m as usize) as u64 };
            f = sub_mod(f, mul_mod((prev - nxt) % MOD, sm));
        }
        prev = nxt;
        m += 1;
    }
    f as u32
}

#[inline(always)]
fn compute_phi_u32(
    x: u32,
    limit: u64,
    small: &[u32],
    large_tail: &[u32],
    parent: u64,
    offset: usize,
) -> u32 {
    let mut f = triangle_mod(x as u64);
    let s = isqrt_u32(x);

    let mut d = 2u32;
    while d <= s {
        let q = x / d;
        f = sub_mod(
            f,
            lookup_phi(q as u64, limit, small, large_tail, parent, offset),
        );
        d += 1;
    }

    let mut prev = x as u64;
    let mut m = 1u32;
    while m <= s {
        let nxt = (x / (m + 1)) as u64;
        if prev > s as u64 {
            let sm = unsafe { *small.get_unchecked(m as usize) as u64 };
            f = sub_mod(f, mul_mod((prev - nxt) % MOD, sm));
        }
        prev = nxt;
        m += 1;
    }
    f as u32
}

fn main() {
    // Sieve ~ N^{2/3} so Du Jiao work is O(N^{2/3}) rather than O(N^{3/4}).
    let cbrt_n = {
        let mut x = (N as f64).cbrt() as u64;
        while x.saturating_mul(x).saturating_mul(x) > N {
            x -= 1;
        }
        while (x + 1).saturating_mul(x + 1).saturating_mul(x + 1) <= N {
            x += 1;
        }
        x
    };
    let limit = (cbrt_n * cbrt_n) as usize;
    let limit_u = limit as u64;

    // Linear sieve for φ, then overwrite with prefix Φ(k) mod MOD.
    let mut phi = vec![0u32; limit + 1];
    let mut lp = vec![0u32; limit + 1];
    let mut primes: Vec<u32> = Vec::with_capacity(limit / 16);
    phi[1] = 1;
    for i in 2..=limit {
        // SAFETY: i <= limit; lp/phi have length limit+1.
        let lpi = unsafe { *lp.get_unchecked(i) };
        if lpi == 0 {
            unsafe {
                *lp.get_unchecked_mut(i) = i as u32;
                *phi.get_unchecked_mut(i) = (i - 1) as u32;
            }
            primes.push(i as u32);
        }
        let lpi = unsafe { *lp.get_unchecked(i) };
        let phi_i = unsafe { *phi.get_unchecked(i) };
        for &p in &primes {
            let ip = i as u64 * p as u64;
            if ip > limit_u || p > lpi {
                break;
            }
            let j = ip as usize;
            // SAFETY: j = i*p <= limit.
            unsafe {
                *lp.get_unchecked_mut(j) = p;
                *phi.get_unchecked_mut(j) = if p == lpi {
                    phi_i.wrapping_mul(p)
                } else {
                    phi_i.wrapping_mul(p - 1)
                };
            }
            if p == lpi {
                break;
            }
        }
    }
    drop(lp);
    drop(primes);

    let mut acc = 0u64;
    for k in 1..=limit {
        acc += unsafe { *phi.get_unchecked(k) } as u64;
        if acc >= MOD {
            acc -= MOD;
        }
        unsafe {
            *phi.get_unchecked_mut(k) = acc as u32;
        }
    }
    let small = phi;

    let max_i = (N / (limit_u + 1)) as usize;
    let mut large = vec![0u32; max_i + 1];
    // Waves i ∈ (hi/2, hi]: Φ(⌊N/i⌋/d) only hits small[] or large[j] for j > hi.
    let mut hi = max_i;
    while hi > 0 {
        let lo = hi / 2;
        let (low_part, high_part) = large.split_at_mut(hi + 1);
        let high_part: &[u32] = high_part;
        let dest = &mut low_part[lo + 1..=hi];
        let offset = hi + 1;
        dest.par_iter_mut().enumerate().for_each(|(k, slot)| {
            let i = lo + 1 + k;
            *slot = compute_phi(N / i as u64, limit_u, &small, high_part, N, offset);
        });
        hi = lo;
    }

    // All 17-smooth d ≤ N (products of the primes of K, with repetition).
    let mut smooth = vec![1u64];
    for &p in &PRIMES_OF_K {
        let n0 = smooth.len();
        for i in 0..n0 {
            let mut v = smooth[i];
            while v <= N / p {
                v *= p;
                smooth.push(v);
            }
        }
    }

    let mut ans = 0u64;
    for &d in &smooth {
        let q = N / d;
        let ts = if q <= limit_u {
            // SAFETY: q <= limit.
            unsafe { *small.get_unchecked(q as usize) as u64 }
        } else {
            // SAFETY: q > limit ⇒ d <= N/(limit+1) = max_i.
            unsafe { *large.get_unchecked(d as usize) as u64 }
        };
        ans = add_mod(ans, ts);
    }

    let mut phi_k = 1u64;
    for &p in &PRIMES_OF_K {
        phi_k *= p - 1;
    }
    ans = mul_mod(ans, phi_k);
    println!("{ans}");
}
