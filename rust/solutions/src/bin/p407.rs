// Project Euler 407 - Idempotents
// For each n, find M(n) = max a in [0,n) with a^2 = a (mod n).
// By CRT, idempotents correspond to choosing a=0 or a=1 mod each prime power factor.
// For n = p1^e1 * ... * pk^ek, compute basis idempotents e_i where
// e_i ≡ 1 (mod pi^ei), e_i ≡ 0 (mod pj^ej for j≠i).
// Then idempotent for subset S = sum of e_i for i in S, mod n.
// Max over all 2^k subsets.

use rayon::prelude::*;

const MAXN: usize = 10_000_001;

fn main() {
    let n_max: usize = 10_000_000;

    // SPF sieve
    let mut spf = vec![0u32; MAXN];
    for i in 2..MAXN {
        if spf[i] == 0 {
            let mut j = i;
            while j < MAXN {
                if spf[j] == 0 {
                    spf[j] = i as u32;
                }
                j += i;
            }
        }
    }

    // Process in parallel chunks
    let chunk_size = 50_000;
    let sum: i64 = (0..=(n_max / chunk_size)).into_par_iter().map(|chunk_idx| {
        let lo = chunk_idx * chunk_size;
        let hi = std::cmp::min(lo + chunk_size, n_max + 1);
        let mut local_sum: i64 = 0;

        for n in lo..hi {
            if n <= 1 {
                continue;
            }

            // Factorize n using SPF
            let mut pp: [u64; 8] = [0; 8]; // prime power p^e for each distinct prime
            let mut nf = 0usize;
            let mut tmp = n;
            while tmp > 1 {
                // SAFETY: tmp < MAXN guaranteed since n < MAXN and tmp decreases
                let p = unsafe { *spf.get_unchecked(tmp) } as usize;
                let mut pe = 1u64;
                loop {
                    tmp /= p;
                    pe *= p as u64;
                    if tmp <= 0 || unsafe { *spf.get_unchecked(tmp) } as usize != p {
                        break;
                    }
                }
                pp[nf] = pe;
                nf += 1;
            }

            if nf <= 1 {
                // n is a prime power: only idempotents are 0 and 1
                local_sum += 1;
                continue;
            }

            let nn = n as u64;

            // Compute basis idempotents: e_i = (n/pp[i]) * modinv(n/pp[i], pp[i]) mod n
            let mut basis: [u64; 8] = [0; 8];
            for i in 0..nf {
                let q = nn / pp[i]; // n / pp[i], coprime to pp[i]
                let inv = mod_inv_u64(q % pp[i], pp[i]);
                basis[i] = (q as u128 * inv as u128 % nn as u128) as u64;
            }

            // Enumerate all 2^nf subsets, compute sum of basis[i] mod n, take max
            let mut best = 1u64;
            let num_subsets = 1u32 << nf;
            // Use Gray code enumeration for efficiency: each step flips one bit
            // so we add or subtract one basis element
            let mut cur: u64 = 0; // subset 0 -> a=0
            for s in 1..num_subsets {
                let bit = (s ^ (s >> 1)) ^ ((s - 1) ^ ((s - 1) >> 1));
                // bit is the position that changed in Gray code
                let pos = bit.trailing_zeros() as usize;
                let gray = s ^ (s >> 1);
                if (gray >> pos) & 1 == 1 {
                    // bit was added
                    cur += basis[pos];
                    if cur >= nn { cur -= nn; }
                } else {
                    // bit was removed
                    if cur >= basis[pos] {
                        cur -= basis[pos];
                    } else {
                        cur = cur + nn - basis[pos];
                    }
                }
                if cur > best {
                    best = cur;
                }
            }

            local_sum += best as i64;
        }
        local_sum
    }).sum();

    println!("{}", sum);
}

/// Modular inverse of a mod m using extended GCD. Assumes gcd(a,m)=1.
#[inline(always)]
fn mod_inv_u64(a: u64, m: u64) -> u64 {
    if m == 1 { return 0; }
    let (mut old_r, mut r) = (a as i64, m as i64);
    let (mut old_s, mut s) = (1i64, 0i64);
    while r != 0 {
        let q = old_r / r;
        let tmp = r;
        r = old_r - q * r;
        old_r = tmp;
        let tmp = s;
        s = old_s - q * s;
        old_s = tmp;
    }
    ((old_s % m as i64 + m as i64) % m as i64) as u64
}
