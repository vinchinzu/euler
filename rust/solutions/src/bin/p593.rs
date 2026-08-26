// Project Euler 593 - Fleeting Medians
//
// Odd-only segmented sieve for the first N primes, discrete log in Z/10007Z,
// and a sequential frequency table for the sliding-window median.

use rayon::prelude::*;

const N: usize = 10_000_000;
const K: usize = 100_000;
const M: usize = 10_007;
const DD: usize = 10_000;
const PHI: u32 = (M - 1) as u32;
const MAX_VAL: usize = 2 * M + 1;
const LIMIT: usize = 180_000_000;
const SEG_ODDS: usize = 1 << 19;

fn isqrt(n: usize) -> usize {
    (n as f64).sqrt() as usize
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

fn generator() -> u64 {
    let phi = M - 1;
    let mut factors = Vec::new();
    let mut temp = phi;
    let mut p = 2;
    while p * p <= temp {
        if temp % p == 0 {
            factors.push(p);
            while temp % p == 0 {
                temp /= p;
            }
        }
        p += 1;
    }
    if temp > 1 {
        factors.push(temp);
    }

    let mut candidate = 2u64;
    loop {
        let mut is_gen = true;
        for &f in &factors {
            let mut x = 1u64;
            let exp = phi / f;
            let mut b = candidate;
            let mut e = exp;
            while e > 0 {
                if e & 1 == 1 {
                    x = x * b % M as u64;
                }
                b = b * b % M as u64;
                e >>= 1;
            }
            if x == 1 {
                is_gen = false;
                break;
            }
        }
        if is_gen {
            return candidate;
        }
        candidate += 1;
    }
}

/// Bit i of the segment is number `2*(start_odd + i) + 1`. 1-bits are prime.
fn sieve_segment(start_odd: usize, len: usize, small: &[u32]) -> Vec<u64> {
    let nwords = (len + 63) >> 6;
    let mut bits = vec![u64::MAX; nwords];
    if start_odd == 0 {
        bits[0] &= !1;
    }
    let rem = len & 63;
    if rem != 0 {
        bits[nwords - 1] &= (1u64 << rem) - 1;
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
            while j < len {
                *ptr.add(j >> 6) &= !(1u64 << (j & 63));
                j += p;
            }
        }
    }
    bits
}

fn extract_residues(start_odd: usize, len: usize, bits: &[u64]) -> Vec<u16> {
    let mut out = Vec::with_capacity(len / 12 + 8);
    let nwords = (len + 63) >> 6;
    for w in 0..nwords {
        let mut word = unsafe { *bits.get_unchecked(w) };
        let base = w << 6;
        while word != 0 {
            let b = word.trailing_zeros() as usize;
            let idx = base + b;
            let n = 2 * (start_odd + idx) + 1;
            out.push((n % M) as u16);
            word &= word - 1;
        }
    }
    out
}

fn main() {
    let g = generator();
    let mut pows = [0u16; M];
    let mut logs = [0u16; M];
    pows[0] = 1;
    let mut gp = 1u64;
    for i in 1..M {
        gp = gp * g % M as u64;
        pows[i] = gp as u16;
    }
    gp = 1;
    for i in 0..PHI as usize {
        logs[gp as usize] = i as u16;
        gp = gp * g % M as u64;
    }

    let small = odd_primes_upto(isqrt(LIMIT) + 2);
    let n_odd = (LIMIT - 1) / 2 + 1;
    let nsegs = (n_odd + SEG_ODDS - 1) / SEG_ODDS;

    let segs: Vec<Vec<u16>> = (0..nsegs)
        .into_par_iter()
        .map(|si| {
            let start = si * SEG_ODDS;
            let len = (n_odd - start).min(SEG_ODDS);
            let bits = sieve_segment(start, len, &small);
            extract_residues(start, len, &bits)
        })
        .collect();

    let mut s = vec![0u16; N + 1];
    let mut k = 0usize;
    let mut km = 0u32;
    let mut push = |r: usize| -> bool {
        if k >= N {
            return true;
        }
        k += 1;
        km += 1;
        if km == PHI {
            km = 0;
        }
        s[k] = if r == 0 {
            0
        } else {
            let idx = (km * logs[r] as u32) % PHI;
            pows[idx as usize]
        };
        k >= N
    };
    push(2);
    'outer: for seg in &segs {
        for &r in seg {
            if push(r as usize) {
                break 'outer;
            }
        }
    }
    drop(segs);

    for i in (1..=N).rev() {
        s[i] = s[i].wrapping_add(s[i / DD + 1]);
    }

    let mut freq = [0u32; MAX_VAL];
    for i in 1..=K {
        freq[s[i] as usize] += 1;
    }

    let target = (K / 2) as u32;
    let target_hi = target + 1;
    let mut below = 0u32;
    let mut med = 0usize;
    while below + freq[med] < target {
        below += freq[med];
        med += 1;
    }

    let mut twice = 0i64;
    let ptr = s.as_ptr();
    for next in K + 1..=N + 1 {
        let at = unsafe { *freq.get_unchecked(med) };
        let mut hi = med;
        if below + at < target_hi {
            hi += 1;
            while unsafe { *freq.get_unchecked(hi) } == 0 {
                hi += 1;
            }
        }
        twice += med as i64 + hi as i64;

        if next > N {
            break;
        }
        let old = unsafe { *ptr.add(next - K) } as usize;
        let new = unsafe { *ptr.add(next) } as usize;
        unsafe {
            *freq.get_unchecked_mut(old) -= 1;
            if old < med {
                below -= 1;
            }
            *freq.get_unchecked_mut(new) += 1;
            if new < med {
                below += 1;
            }
        }
        loop {
            if below >= target {
                med -= 1;
                below -= unsafe { *freq.get_unchecked(med) };
            } else if unsafe { *freq.get_unchecked(med) } == 0
                || below + unsafe { *freq.get_unchecked(med) } < target
            {
                below += unsafe { *freq.get_unchecked(med) };
                med += 1;
            } else {
                break;
            }
        }
    }

    println!("{:.1}", twice as f64 / 2.0);
}
