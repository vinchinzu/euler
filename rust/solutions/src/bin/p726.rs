// Project Euler 726 - Falling Bottles
//
// Recurrence for a(n), b(n) and Young tableau formula f(n) = a(n) * tr(n)! / b(n).
// All mod M = 10^9 + 33.
//
// M^2 < 2^64 so every mulmod is a u64 multiply. Factorials of T_n = n(n+1)/2
// (~50e6) are a parallel prefix product with checkpoints at triangular indices.

use rayon::prelude::*;

const NMAX: usize = 10000;
const MOD: u64 = 1_000_000_033;

#[inline(always)]
fn mul(a: u64, b: u64) -> u64 {
    a * b % MOD
}

fn pow_mod(mut base: u64, mut exp: u64) -> u64 {
    let mut result = 1u64;
    base %= MOD;
    while exp > 0 {
        if exp & 1 == 1 {
            result = mul(result, base);
        }
        base = mul(base, base);
        exp >>= 1;
    }
    result
}

#[inline]
fn mod_inv(a: u64) -> u64 {
    pow_mod(a, MOD - 2)
}

#[inline]
fn tr(n: usize) -> usize {
    n * (n + 1) / 2
}

#[inline]
fn isqrt_u64(n: u64) -> u64 {
    let mut x = (n as f64).sqrt() as u64;
    while x > 0 && x.saturating_mul(x) > n {
        x -= 1;
    }
    while x < u64::MAX && (x + 1).saturating_mul(x + 1) <= n {
        x += 1;
    }
    x
}

/// Smallest i with i*(i+1)/2 >= t.
fn tri_ge(t: usize) -> usize {
    if t == 0 {
        return 0;
    }
    let disc = 1 + 8 * (t as u64);
    let s = isqrt_u64(disc);
    let mut i = ((s - 1) / 2) as usize;
    while i * (i + 1) / 2 < t {
        i += 1;
    }
    while i > 0 && (i - 1) * i / 2 >= t {
        i -= 1;
    }
    i
}

/// Largest i with i*(i+1)/2 <= t.
fn tri_le(t: usize) -> usize {
    let disc = 1 + 8 * (t as u64);
    let s = isqrt_u64(disc);
    let mut i = ((s - 1) / 2) as usize;
    while i > 0 && i * (i + 1) / 2 > t {
        i -= 1;
    }
    while (i + 1) * (i + 2) / 2 <= t {
        i += 1;
    }
    i
}

struct ChunkOut {
    product: u64,
    /// (triangle index i, product of [lo..=T_i] within this chunk)
    parts: Vec<(usize, u64)>,
}

fn main() {
    let n = NMAX;
    let trn = tr(n);

    // Parallel factorial at triangular checkpoints only.
    let chunk_size = 262_144usize;
    let n_chunks = (trn + chunk_size - 1) / chunk_size;

    let chunk_outs: Vec<ChunkOut> = (0..n_chunks)
        .into_par_iter()
        .map(|ci| {
            let lo = ci * chunk_size + 1;
            let hi = (lo + chunk_size - 1).min(trn);

            let i_lo = tri_ge(lo).max(1);
            let i_hi = tri_le(hi).min(n);

            let mut product = 1u64;
            let mut parts = Vec::new();
            let mut next_i = i_lo;
            let mut next_t = if i_lo <= i_hi {
                next_i * (next_i + 1) / 2
            } else {
                usize::MAX
            };

            let mut k = lo;
            while k <= hi {
                let run_end = if next_t <= hi { next_t } else { hi + 1 };
                // Product of [k, run_end) — no triangular checkpoint in this span.
                while k + 3 < run_end {
                    let a = (k as u64) * ((k + 1) as u64);
                    let b = ((k + 2) as u64) * ((k + 3) as u64);
                    product = mul(product, a % MOD);
                    product = mul(product, b % MOD);
                    k += 4;
                }
                while k < run_end {
                    product = mul(product, k as u64);
                    k += 1;
                }
                if k == next_t && next_t <= hi {
                    product = mul(product, k as u64);
                    parts.push((next_i, product));
                    next_i += 1;
                    next_t = if next_i <= i_hi {
                        next_i * (next_i + 1) / 2
                    } else {
                        usize::MAX
                    };
                    k += 1;
                }
            }

            ChunkOut { product, parts }
        })
        .collect();

    let mut fact_tri = vec![0u64; n + 1];
    fact_tri[0] = 1;
    let mut scale = 1u64;
    for co in &chunk_outs {
        for &(i, p) in &co.parts {
            fact_tri[i] = mul(scale, p);
        }
        scale = mul(scale, co.product);
    }

    let mut a = vec![0u64; n + 1];
    a[0] = 1;
    a[1] = 1;
    let mut two_i = 2u64; // 2^1
    for i in 2..=n {
        two_i = mul(two_i, 2); // 2^i
        let p2 = two_i - 1; // 2^i - 1  (2^i never 0 mod odd prime)
        let sq = mul(a[i - 1], a[i - 1]);
        a[i] = mul(mul(sq, p2), mod_inv(a[i - 2]));
    }

    let mut b = vec![0u64; n + 1];
    b[0] = 1;
    b[1] = 1;
    for i in 2..=n {
        let sq = mul(b[i - 1], b[i - 1]);
        let odd = (2 * i as u64 - 1) % MOD;
        b[i] = mul(mul(sq, odd), mod_inv(b[i - 2]));
    }

    let mut ans = 0u64;
    for i in 1..=n {
        let term = mul(mul(a[i], fact_tri[i]), mod_inv(b[i]));
        ans += term;
        if ans >= MOD {
            ans -= MOD;
        }
    }

    println!("{}", ans);
}
