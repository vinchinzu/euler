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

// Packed odd-n slot: rem in bits 0..26, ω in 27..30, bit 31 = not squarefree.
const REM_MASK: u32 = (1 << 27) - 1;
const OMEGA_SHIFT: u32 = 27;
const NSQ_BIT: u32 = 1 << 31;

#[inline(always)]
fn accum(c: &mut [i64; MK1], ncr: &[u64; MK1 * MK1], k: usize, n: u64) {
    if k > MAX_K {
        return;
    }
    let count = (BIG_N / (n * n)) % MOD;
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

    // Skip 2: odd-only segments. n=2 is special-cased below.
    let odd_primes: Vec<u32> = primes.iter().copied().skip_while(|&p| p == 2).collect();

    const SEG: usize = 1 << 20;
    let n_seg = (L + SEG - 1) / SEG;
    let half_l = L / 2;

    let local_cs: Vec<[i64; MK1]> = (0..n_seg)
        .into_par_iter()
        .map(|si| {
            let lo = si * SEG;
            let hi = (lo + SEG).min(L + 1);
            // Even lo: odds lo+1, lo+3, ... < hi.
            if lo + 1 >= hi {
                return [0i64; MK1];
            }
            let first_odd = lo + 1;
            let last_odd = if (hi - 1) & 1 == 1 { hi - 1 } else { hi - 2 };
            if last_odd < first_odd {
                return [0i64; MK1];
            }
            let len = ((last_odd - first_odd) >> 1) + 1;
            let mut data = vec![0u32; len];
            for i in 0..len {
                data[i] = (first_odd + 2 * i) as u32;
            }
            if lo == 0 {
                data[0] = NSQ_BIT | 1; // n=1: not squarefree
            }

            for &p_u in &odd_primes {
                let p = p_u as usize;
                let start = if first_odd <= p {
                    p
                } else {
                    let r = first_odd % p;
                    let mut s = if r == 0 { first_odd } else { first_odd + (p - r) };
                    if s & 1 == 0 {
                        s += p;
                    }
                    s
                };
                if start < hi {
                    let mut idx = (start - first_odd) >> 1;
                    let mut j = start;
                    while j < hi {
                        unsafe {
                            let slot = data.get_unchecked_mut(idx);
                            let d = *slot;
                            let rem = (d & REM_MASK) / p_u;
                            let omega = ((d >> OMEGA_SHIFT) & 15) + 1;
                            *slot = (d & NSQ_BIT) | (omega << OMEGA_SHIFT) | rem;
                        }
                        j += p << 1;
                        idx += p;
                    }
                }
                let p2 = p.saturating_mul(p);
                if p2 != 0 && p2 < hi {
                    let start2 = if first_odd <= p2 {
                        p2
                    } else {
                        let r = first_odd % p2;
                        let mut s = if r == 0 { first_odd } else { first_odd + (p2 - r) };
                        if s & 1 == 0 {
                            s += p2;
                        }
                        s
                    };
                    if start2 < hi {
                        let mut idx = (start2 - first_odd) >> 1;
                        let mut j = start2;
                        while j < hi {
                            unsafe {
                                *data.get_unchecked_mut(idx) |= NSQ_BIT;
                            }
                            j += p2 << 1;
                            idx += p2;
                        }
                    }
                }
            }

            let mut c = [0i64; MK1];
            let start_i = if lo == 0 { 1 } else { 0 }; // skip n=1
            for i in start_i..len {
                let d = unsafe { *data.get_unchecked(i) };
                if d & NSQ_BIT != 0 {
                    continue;
                }
                let mut k = ((d >> OMEGA_SHIFT) & 15) as usize;
                if (d & REM_MASK) > 1 {
                    k += 1;
                }
                let n = first_odd + 2 * i;
                accum(&mut c, &ncr, k, n as u64);
                if n <= half_l {
                    accum(&mut c, &ncr, k + 1, (n as u64) << 1);
                }
            }
            c
        })
        .collect();

    let mut c = [0i64; MK1];
    c[0] = (BIG_N % MOD) as i64;
    // n=2 is the only even prime; odd-only segments emit 2m for odd m>=3.
    accum(&mut c, &ncr, 1, 2);
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
