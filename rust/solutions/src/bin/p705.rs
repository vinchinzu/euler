// Project Euler 705 - Total Inversion Count of Divided Sequences
//
// Odd-only segmented bit sieve (index i ↔ 2i+1). Reverse-scan the bitset
// so primes are never stored. Each prime is two precomputed 4-digit chunk
// transforms of the 10-bucket inversion state (S = 1, multiply at the end).

use rayon::prelude::*;

const N: usize = 100_000_000;
const M: u64 = 1_000_000_007;
const SEG_ODDS: usize = 1 << 19;
const CHUNK: usize = 10_000;

struct Chunk {
    c: [u64; 10],
    d: [u64; 10],
    extra: u64,
    p2: u32,
    p3: u32,
}

fn mod_inv(a: u64) -> u64 {
    let mut result = 1u64;
    let mut exp = M - 2;
    let mut base = a;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % M;
        }
        base = base * base % M;
        exp >>= 1;
    }
    result
}

fn mod_pow(mut base: u64, mut exp: u64) -> u64 {
    let mut result = 1u64;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % M;
        }
        base = base * base % M;
        exp >>= 1;
    }
    result
}

fn odd_primes_upto(limit: usize) -> Vec<u32> {
    let n_odd = (limit + 1) / 2;
    let mut comp = vec![0u8; n_odd];
    if n_odd > 0 {
        comp[0] = 1;
    }
    let mut p = 3usize;
    while p * p <= limit {
        if comp[p >> 1] == 0 {
            let mut j = (p * p) >> 1;
            while j < n_odd {
                comp[j] = 1;
                j += p;
            }
        }
        p += 2;
    }
    let mut primes = Vec::new();
    p = 3;
    while p <= limit {
        if comp[p >> 1] == 0 {
            primes.push(p as u32);
        }
        p += 2;
    }
    primes
}

/// Bit i of `bits` is number `2*(start_odd + i) + 1`. 1-bits are prime.
fn sieve_segment(bits: &mut [u64], start_odd: usize, small: &[u32]) {
    let len = bits.len() << 6;
    if start_odd == 0 && !bits.is_empty() {
        bits[0] &= !1;
    }
    let n0 = 2 * start_odd + 1;
    let end_n = n0 + 2 * len;
    let ptr = bits.as_mut_ptr();
    for &p32 in small {
        let p = p32 as usize;
        let pp = p * p;
        let mut n = if n0 > pp { n0 } else { pp };
        let r = n % p;
        if r != 0 {
            n += p - r;
        }
        if n & 1 == 0 {
            n += p;
        }
        if n >= end_n {
            continue;
        }
        let mut j = (n - n0) >> 1;
        unsafe {
            while j + 3 * p < len {
                *ptr.add(j >> 6) &= !(1u64 << (j & 63));
                let j1 = j + p;
                *ptr.add(j1 >> 6) &= !(1u64 << (j1 & 63));
                let j2 = j1 + p;
                *ptr.add(j2 >> 6) &= !(1u64 << (j2 & 63));
                let j3 = j2 + p;
                *ptr.add(j3 >> 6) &= !(1u64 << (j3 & 63));
                j = j3 + p;
            }
            while j < len {
                *ptr.add(j >> 6) &= !(1u64 << (j & 63));
                j += p;
            }
        }
    }
}

fn sieve_odd_bits() -> Vec<u64> {
    let sqrt = (N as f64).sqrt() as usize;
    let small = odd_primes_upto(sqrt);
    let n_odd = N / 2;
    let nwords = n_odd / 64;
    let mut bits = vec![u64::MAX; nwords];
    let chunk_words = SEG_ODDS / 64;
    bits.par_chunks_mut(chunk_words).enumerate().for_each(|(si, chunk)| {
        sieve_segment(chunk, si * SEG_ODDS, &small);
    });
    bits
}

fn build_chunks() -> Vec<Chunk> {
    let divisors: [&[usize]; 10] = [
        &[],
        &[1],
        &[1, 2],
        &[1, 3],
        &[1, 2, 4],
        &[1, 5],
        &[1, 2, 3, 6],
        &[1, 7],
        &[1, 2, 4, 8],
        &[1, 3, 9],
    ];
    let num_divisors: [usize; 10] = [0, 1, 2, 2, 3, 2, 4, 2, 4, 3];

    let mut invs = [0u64; 5];
    for i in 1..5 {
        invs[i] = mod_inv(i as u64);
    }

    let mut inv_coeff = [[0u64; 10]; 10];
    let mut delta = [[0u64; 10]; 10];
    let mut p2_d = [0u32; 10];
    let mut p3_d = [0u32; 10];
    for k in 1..10 {
        let inv = invs[num_divisors[k]];
        for &div in divisors[k] {
            for i in 1..div {
                inv_coeff[k][i] += inv;
                if inv_coeff[k][i] >= M {
                    inv_coeff[k][i] -= M;
                }
            }
            delta[k][div] = inv;
        }
        match k {
            2 | 3 | 5 | 7 => p2_d[k] = 1,
            6 | 8 => p2_d[k] = 2,
            4 | 9 => p3_d[k] = 1,
            _ => {}
        }
    }

    let mut chunks = Vec::with_capacity(CHUNK);
    for n in 0..CHUNK {
        let mut c = [0u64; 10];
        let mut extra = 0u64;
        let mut d = [0u64; 10];
        let mut p2 = 0u32;
        let mut p3 = 0u32;
        let mut x = n;
        while x > 0 {
            let k = x % 10;
            x /= 10;
            if k == 0 {
                continue;
            }
            p2 += p2_d[k];
            p3 += p3_d[k];
            for i in 1..10 {
                let coef = inv_coeff[k][i];
                extra += d[i] * coef;
                extra %= M;
                c[i] += coef;
                if c[i] >= M {
                    c[i] -= M;
                }
            }
            for i in 1..10 {
                d[i] += delta[k][i];
                if d[i] >= M {
                    d[i] -= M;
                }
            }
        }
        chunks.push(Chunk { c, d, extra, p2, p3 });
    }
    chunks
}

#[inline(always)]
fn apply(ch: &Chunk, cnt: &mut [u64; 10], ans: &mut u128, p2: &mut u64, p3: &mut u64) {
    *p2 += ch.p2 as u64;
    *p3 += ch.p3 as u64;
    let add = ch.extra
        + ch.c[1] * cnt[1]
        + ch.c[2] * cnt[2]
        + ch.c[3] * cnt[3]
        + ch.c[4] * cnt[4]
        + ch.c[5] * cnt[5]
        + ch.c[6] * cnt[6]
        + ch.c[7] * cnt[7]
        + ch.c[8] * cnt[8]
        + ch.c[9] * cnt[9];
    *ans += add as u128;
    cnt[1] += ch.d[1];
    if cnt[1] >= M {
        cnt[1] -= M;
    }
    cnt[2] += ch.d[2];
    if cnt[2] >= M {
        cnt[2] -= M;
    }
    cnt[3] += ch.d[3];
    if cnt[3] >= M {
        cnt[3] -= M;
    }
    cnt[4] += ch.d[4];
    if cnt[4] >= M {
        cnt[4] -= M;
    }
    cnt[5] += ch.d[5];
    if cnt[5] >= M {
        cnt[5] -= M;
    }
    cnt[6] += ch.d[6];
    if cnt[6] >= M {
        cnt[6] -= M;
    }
    cnt[7] += ch.d[7];
    if cnt[7] >= M {
        cnt[7] -= M;
    }
    cnt[8] += ch.d[8];
    if cnt[8] >= M {
        cnt[8] -= M;
    }
    cnt[9] += ch.d[9];
    if cnt[9] >= M {
        cnt[9] -= M;
    }
}

fn main() {
    let chunks = build_chunks();
    let bits = sieve_odd_bits();

    let mut cnt = [0u64; 10];
    let mut ans = 0u128;
    let mut p2 = 0u64;
    let mut p3 = 0u64;

    // Reverse scan: largest odd prime first. LSD-first digits = lo chunk then hi.
    for wi in (0..bits.len()).rev() {
        let mut word = unsafe { *bits.get_unchecked(wi) };
        while word != 0 {
            let b = 63 - word.leading_zeros() as usize;
            word ^= 1u64 << b;
            let p = (wi << 7) + (b << 1) + 1;
            let hi = p / CHUNK;
            let lo = p - hi * CHUNK;
            unsafe {
                apply(chunks.get_unchecked(lo), &mut cnt, &mut ans, &mut p2, &mut p3);
                if hi != 0 {
                    apply(chunks.get_unchecked(hi), &mut cnt, &mut ans, &mut p2, &mut p3);
                }
            }
        }
    }
    apply(&chunks[2], &mut cnt, &mut ans, &mut p2, &mut p3);

    let s = mod_pow(2, p2) * mod_pow(3, p3) % M;
    let ans = ((ans % M as u128) as u64) * s % M;
    println!("{}", ans);
}
