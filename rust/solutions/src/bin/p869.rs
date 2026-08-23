// Project Euler 869 - Prime Bit Guessing Game
// E(N) = expected score when optimally guessing binary of random prime <= N.
//
// Levels 0..15: primes with bitlen > 16 never drop out, so group majorities
// are LSB histograms plus the few short primes. Remaining bits are
// independent per 16-bit residue class and scored in L1.

use rayon::prelude::*;

const LIMIT: usize = 100_000_000;
const LOW_BITS: u32 = 16;
const LOW_MASK: usize = (1 << LOW_BITS) - 1;
const SEG_ODDS: usize = 1 << 18;
const STACK_CAP: usize = 256;

fn small_odd_primes(limit: usize) -> Vec<u32> {
    let half = limit / 2 + 1;
    let mut comp = vec![0u8; half];
    if !comp.is_empty() {
        comp[0] = 1;
    }
    let mut primes = Vec::new();
    let mut i = 1usize;
    while 2 * i + 1 <= limit {
        if comp[i] == 0 {
            let p = 2 * i + 1;
            primes.push(p as u32);
            let mut j = (p * p - 1) / 2;
            while j < half {
                comp[j] = 1;
                j += p;
            }
        }
        i += 1;
    }
    primes
}

fn primes_upto(limit: usize) -> Vec<u32> {
    let sqrt_lim = (limit as f64).sqrt() as usize;
    let small = small_odd_primes(sqrt_lim);
    let half = limit / 2;
    let nsegs = (half + SEG_ODDS - 1) / SEG_ODDS;

    let parts: Vec<Vec<u32>> = (0..nsegs)
        .into_par_iter()
        .map(|si| {
            let start = si * SEG_ODDS;
            let end = (start + SEG_ODDS).min(half);
            let len = end - start;
            let nwords = (len + 63) / 64;
            let mut bits = vec![0u64; nwords];
            let bits_ptr = bits.as_mut_ptr();

            for &sp in &small {
                let p = sp as usize;
                let mut idx = (p * p - 1) / 2;
                if idx < start {
                    let rem = (start - idx) % p;
                    idx = if rem == 0 { start } else { start + (p - rem) };
                }
                while idx < end {
                    let off = idx - start;
                    // SAFETY: off < len; nwords covers every bit of this segment
                    unsafe {
                        *bits_ptr.add(off >> 6) |= 1u64 << (off & 63);
                    }
                    idx += p;
                }
            }

            let mut out = Vec::with_capacity(len / 6 + 8);
            for w in 0..nwords {
                let base = w * 64;
                // SAFETY: w < nwords
                let mut word = unsafe { !*bits.get_unchecked(w) };
                if base + 64 > len {
                    word &= (1u64 << (len - base)) - 1;
                }
                while word != 0 {
                    let b = word.trailing_zeros() as usize;
                    let idx = start + base + b;
                    word &= word - 1;
                    if idx == 0 {
                        continue; // 1 is not prime
                    }
                    out.push((2 * idx + 1) as u32);
                }
            }
            out
        })
        .collect();

    let total: usize = parts.iter().map(|v| v.len()).sum();
    let mut primes = Vec::with_capacity(total + 1);
    primes.push(2);
    for part in parts {
        primes.extend_from_slice(&part);
    }
    primes
}

/// Stable-enough (order unused) partition of `src[s..e]` into `dst`.
/// Continuing primes (p >= thresh) are written 0-bit then 1-bit.
/// SAFETY: src[s..e) and dst[s..e) are valid, dst may alias src only if equal.
#[inline(always)]
unsafe fn partition_range(
    src: *const u32,
    dst: *mut u32,
    s: usize,
    e: usize,
    shift: u32,
    thresh: u32,
) -> (u64, usize, usize) {
    let n = e - s;
    let mut c0 = 0u32;
    let mut cont0 = 0usize;
    for i in 0..n {
        let p = unsafe { *src.add(s + i) };
        let bit = (p >> shift) & 1;
        c0 += bit ^ 1;
        cont0 += ((p >= thresh) as usize) & (bit ^ 1) as usize;
    }
    let mut p0 = s;
    let mut p1 = s + cont0;
    for i in 0..n {
        let p = unsafe { *src.add(s + i) };
        if p >= thresh {
            if (p >> shift) & 1 == 0 {
                unsafe {
                    *dst.add(p0) = p;
                }
                p0 += 1;
            } else {
                unsafe {
                    *dst.add(p1) = p;
                }
                p1 += 1;
            }
        }
    }
    let score = if c0 > n as u32 - c0 { c0 } else { n as u32 - c0 } as u64;
    (score, cont0, p1 - s - cont0)
}

/// Score remaining bits of one 16-bit residue class. `arr.len() <= STACK_CAP`.
fn score_group(arr: &mut [u32], level0: u32, max_bits: u32) -> u64 {
    let n = arr.len();
    if n == 0 {
        return 0;
    }
    debug_assert!(n <= STACK_CAP);
    let mut buf = [0u32; STACK_CAP];
    let mut gs_a = [0u16; STACK_CAP];
    let mut ge_a = [0u16; STACK_CAP];
    let mut gs_b = [0u16; STACK_CAP];
    let mut ge_b = [0u16; STACK_CAP];
    gs_a[0] = 0;
    ge_a[0] = n as u16;
    let mut ng = 1usize;
    let mut score = 0u64;
    let mut src = arr.as_mut_ptr();
    let mut dst = buf.as_mut_ptr();
    let mut gs = gs_a.as_mut_ptr();
    let mut ge = ge_a.as_mut_ptr();
    let mut ngs = gs_b.as_mut_ptr();
    let mut nge = ge_b.as_mut_ptr();

    for shift in level0..max_bits {
        if ng == 0 {
            break;
        }
        let thresh = 1u32 << (shift + 1);
        let mut nng = 0usize;
        for g in 0..ng {
            // SAFETY: g < ng, group lists hold valid subranges of 0..n
            let (s, e) = unsafe { (*gs.add(g) as usize, *ge.add(g) as usize) };
            let (sc, c0, c1) = unsafe { partition_range(src, dst, s, e, shift, thresh) };
            score += sc;
            if c0 > 0 {
                unsafe {
                    *ngs.add(nng) = s as u16;
                    *nge.add(nng) = (s + c0) as u16;
                }
                nng += 1;
            }
            if c1 > 0 {
                unsafe {
                    *ngs.add(nng) = (s + c0) as u16;
                    *nge.add(nng) = (s + c0 + c1) as u16;
                }
                nng += 1;
            }
        }
        ng = nng;
        std::mem::swap(&mut src, &mut dst);
        std::mem::swap(&mut gs, &mut ngs);
        std::mem::swap(&mut ge, &mut nge);
    }
    score
}

fn main() {
    let primes = primes_upto(LIMIT);
    let nprimes = primes.len();
    let low_cut = 1u32 << LOW_BITS;
    let nshort = primes.partition_point(|&p| p < low_cut);
    let shorts = &primes[..nshort];
    let longs = &primes[nshort..];

    let hist = longs
        .par_chunks(1 << 16)
        .map(|chunk| {
            let mut h = vec![0u32; LOW_MASK + 1];
            for &p in chunk {
                h[(p as usize) & LOW_MASK] += 1;
            }
            h
        })
        .reduce(
            || vec![0u32; LOW_MASK + 1],
            |mut a, b| {
                for i in 0..=LOW_MASK {
                    a[i] += b[i];
                }
                a
            },
        );

    let mut total_score = 0u64;
    let mut folded = hist.clone();
    let mut extra0 = vec![0u32; LOW_MASK + 1];
    let mut extra1 = vec![0u32; LOW_MASK + 1];
    for k in (0..LOW_BITS).rev() {
        let ngroups = 1usize << k;
        extra0[..ngroups].fill(0);
        extra1[..ngroups].fill(0);
        let live = 1u32 << k;
        for &p in shorts {
            if p >= live {
                let r = (p as usize) & (ngroups - 1);
                if (p >> k) & 1 == 0 {
                    extra0[r] += 1;
                } else {
                    extra1[r] += 1;
                }
            }
        }
        for r in 0..ngroups {
            let c0 = folded[r] + extra0[r];
            let c1 = folded[r + ngroups] + extra1[r];
            total_score += if c0 > c1 { c0 } else { c1 } as u64;
        }
        for r in 0..ngroups {
            folded[r] += folded[r + ngroups];
        }
    }

    let mut starts = vec![0usize; LOW_MASK + 2];
    for i in 0..=LOW_MASK {
        starts[i + 1] = starts[i] + hist[i] as usize;
    }
    let n_long = starts[LOW_MASK + 1];
    let mut packed = vec![0u32; n_long];

    // Two-pass radix on the low 16 bits (8+8). First pass: per-thread 256-way
    // scatter then merge. Second: 256 independent 8-bit counting sorts.
    {
        let mut st256 = [0usize; 257];
        for i in 0..=LOW_MASK {
            st256[(i >> 8) + 1] += hist[i] as usize;
        }
        for i in 0..256 {
            st256[i + 1] += st256[i];
        }
        let nt = rayon::current_num_threads().max(1);
        let chunk = (longs.len() + nt - 1) / nt;
        let locals: Vec<(Vec<usize>, Vec<u32>)> = (0..nt)
            .into_par_iter()
            .map(|t| {
                let lo = t * chunk;
                let hi = (lo + chunk).min(longs.len());
                let slice = &longs[lo..hi];
                let mut h = vec![0usize; 256];
                for &p in slice {
                    h[((p as usize) >> 8) & 0xff] += 1;
                }
                let mut st = vec![0usize; 257];
                for i in 0..256 {
                    st[i + 1] = st[i] + h[i];
                }
                let mut local = vec![0u32; slice.len()];
                let mut pos = st.clone();
                for &p in slice {
                    let b = ((p as usize) >> 8) & 0xff;
                    local[pos[b]] = p;
                    pos[b] += 1;
                }
                (st, local)
            })
            .collect();

        let mut mid = vec![0u32; n_long];
        let mid_addr = mid.as_mut_ptr() as usize;
        (0..256usize).into_par_iter().for_each(|hi| {
            let mut dest = st256[hi];
            let mid = mid_addr as *mut u32;
            for t in 0..nt {
                let st = &locals[t].0;
                let local = &locals[t].1;
                let ls = st[hi];
                let ln = st[hi + 1] - st[hi];
                if ln > 0 {
                    // SAFETY: disjoint dest ranges per hi; ln bytes live in local
                    unsafe {
                        std::ptr::copy_nonoverlapping(local.as_ptr().add(ls), mid.add(dest), ln);
                    }
                    dest += ln;
                }
            }
        });

        let mid_addr = mid.as_ptr() as usize;
        let packed_addr = packed.as_mut_ptr() as usize;
        let starts_addr = starts.as_ptr() as usize;
        let st256_addr = st256.as_ptr() as usize;
        (0..256usize).into_par_iter().for_each(|hi| {
            let st256 = st256_addr as *const usize;
            // SAFETY: hi < 256, st256 has 257 entries
            let s = unsafe { *st256.add(hi) };
            let e = unsafe { *st256.add(hi + 1) };
            if s == e {
                return;
            }
            let mid = mid_addr as *const u32;
            let packed = packed_addr as *mut u32;
            let starts = starts_addr as *const usize;
            let mut pos = [0usize; 256];
            for lo in 0..256 {
                // SAFETY: (hi<<8)|lo is a 16-bit residue; starts has 65537 entries
                pos[lo] = unsafe { *starts.add((hi << 8) | lo) };
            }
            for i in s..e {
                let p = unsafe { *mid.add(i) };
                let lo = p as usize & 0xff;
                unsafe {
                    *packed.add(pos[lo]) = p;
                }
                pos[lo] += 1;
            }
        });
    }

    let mut ranges: Vec<(usize, usize)> = Vec::with_capacity(1 << (LOW_BITS - 1));
    for r in 0..=LOW_MASK {
        if starts[r + 1] > starts[r] {
            ranges.push((starts[r], starts[r + 1]));
        }
    }

    let max_bits = 32 - (LIMIT as u32).leading_zeros();
    let packed_addr = packed.as_mut_ptr() as usize;
    let high: u64 = ranges
        .par_iter()
        .with_min_len(32)
        .map(|&(s, e)| {
            // SAFETY: ranges are disjoint sub-slices of packed
            let slice = unsafe {
                std::slice::from_raw_parts_mut((packed_addr as *mut u32).add(s), e - s)
            };
            score_group(slice, LOW_BITS, max_bits)
        })
        .sum();
    total_score += high;
    drop(packed);

    println!("{:.8}", total_score as f64 / nprimes as f64);
}
