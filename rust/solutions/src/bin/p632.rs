// Project Euler 632 - Square prime factors
// C_k(N) = number of integers 1..N divisible by p^2 for exactly k primes p.
// Product of all nonzero C_k(N) mod 10^9+7.
// N = 10^16, sieve omega/squarefree up to sqrt(N) = 10^8.
//
// Segmented parallel sieve: primes to sqrt(L) mark omega + p^2 non-squarefree
// and a remaining cofactor (always 1 or a single large prime for n <= L).
// Fused with C_k accumulation so the 200MB SPF array is never stored.

use rayon::prelude::*;

const BIG_N: u64 = 10_000_000_000_000_000;
const MOD: u64 = 1_000_000_007;
const L: usize = 100_000_000;
// primorial(19) = 9_699_690 <= L < primorial(23), so ω(n) <= 8 for squarefree n <= L.
const MAX_K: usize = 8;
const MK1: usize = MAX_K + 1;

fn sieve_primes(limit: usize) -> Vec<u32> {
    let mut is_comp = vec![false; limit + 1];
    let mut primes = Vec::with_capacity(limit / 8);
    for i in 2..=limit {
        if !is_comp[i] {
            primes.push(i as u32);
            let mut j = i * i;
            while j <= limit {
                is_comp[j] = true;
                j += i;
            }
        }
    }
    primes
}

fn main() {
    let sqrt_l = (L as u64).isqrt() as usize;
    let primes = sieve_primes(sqrt_l);

    let mut ncr = [0u64; MK1 * MK1];
    for i in 0..=MAX_K {
        ncr[i * MK1] = 1;
        for j in 1..=i {
            ncr[i * MK1 + j] = (ncr[(i - 1) * MK1 + j - 1] + ncr[(i - 1) * MK1 + j]) % MOD;
        }
    }

    const SEG: usize = 1 << 20;
    let n_seg = (L + SEG - 1) / SEG;

    let local_cs: Vec<[i64; MK1]> = (0..n_seg)
        .into_par_iter()
        .map(|si| {
            let lo = if si == 0 { 0 } else { si * SEG };
            let hi = (lo + SEG).min(L + 1);
            let len = hi - lo;
            let mut rem = vec![0u32; len];
            let mut omega = vec![0u8; len];
            let mut sqfree = vec![true; len];
            for i in 0..len {
                rem[i] = (lo + i) as u32;
            }
            if lo == 0 {
                sqfree[0] = false;
                rem[0] = 1;
                if len > 1 {
                    sqfree[1] = false;
                    rem[1] = 1;
                }
            }

            for &p_u in &primes {
                let p = p_u as usize;
                let start = if lo <= p {
                    p
                } else {
                    let r = lo % p;
                    if r == 0 { lo } else { lo + (p - r) }
                };
                let mut j = start;
                while j < hi {
                    let idx = j - lo;
                    unsafe {
                        *rem.get_unchecked_mut(idx) /= p_u;
                        *omega.get_unchecked_mut(idx) += 1;
                    }
                    j += p;
                }
                let p2 = p.saturating_mul(p);
                if p2 != 0 && p2 < hi {
                    let start2 = if lo <= p2 {
                        p2
                    } else {
                        let r = lo % p2;
                        if r == 0 { lo } else { lo + (p2 - r) }
                    };
                    let mut j = start2;
                    while j < hi {
                        unsafe {
                            *sqfree.get_unchecked_mut(j - lo) = false;
                        }
                        j += p2;
                    }
                }
            }

            let mut c = [0i64; MK1];
            let start_n = if lo < 2 { 2 } else { lo };
            for n in start_n..hi {
                let idx = n - lo;
                if !unsafe { *sqfree.get_unchecked(idx) } {
                    continue;
                }
                let mut k = unsafe { *omega.get_unchecked(idx) } as usize;
                if unsafe { *rem.get_unchecked(idx) } > 1 {
                    k += 1;
                }
                if k > MAX_K {
                    continue;
                }
                let n2 = n as u64 * n as u64;
                let count = (BIG_N / n2) % MOD;
                let ncr_base = k * MK1;
                for i in 0..=k {
                    let prod = unsafe { *ncr.get_unchecked(ncr_base + i) } * count % MOD;
                    let idxk = k - i;
                    if i & 1 == 0 {
                        c[idxk] += prod as i64;
                    } else {
                        c[idxk] -= prod as i64;
                    }
                }
            }
            c
        })
        .collect();

    let mut c = [0i64; MK1];
    c[0] = (BIG_N % MOD) as i64;
    for local_c in &local_cs {
        for i in 0..=MAX_K {
            c[i] += local_c[i];
        }
    }

    let m = MOD;
    let mut ans = 1u64;
    for i in 0..=MAX_K {
        let ci = c[i] % (m as i64);
        let ci = if ci < 0 { (ci + m as i64) as u64 } else { ci as u64 };
        if ci != 0 {
            ans = ans * ci % m;
        }
    }

    println!("{}", ans);
}
