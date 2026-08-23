// Project Euler 614 - Special partitions II
// Compute sum of P(i) for i=1..10^7, where P(n) counts partitions of n into
// distinct parts not congruent to 2 mod 4.
//
// Uses the identity: F(x) * K(x) = B(x), where
//   K(x) = P(x)*P(x^4)/P(x^2) has sparse support on triangular numbers
//   B(x) = P(x^2)*P(x^8)/P(x^4) has sparse support on pronic numbers
//
// Single blocked recurrence: F(n) = B(n) + sum_{t>=1} s_t * F(n - T_t)
// where T_t = t(t+1)/2, s_t = (-1)^floor((t-1)/2)
//
// Phase 1 (large offsets) is dest-chunk parallel: each worker owns a slice of
// `temp` so there is no per-thread merge. Phase 2 stays sequential.

use rayon::prelude::*;

const N: usize = 10_000_000;
const MOD: u64 = 1_000_000_007;
const MODI: i64 = MOD as i64;
const BLOCK: usize = 1 << 14; // 16384

fn triangular_terms(max_val: usize) -> Vec<(usize, i64)> {
    let mut terms = Vec::new();
    let mut t: usize = 1;
    loop {
        let tri = t * (t + 1) / 2;
        if tri > max_val {
            break;
        }
        // s_t = (-1)^floor((t-1)/2): +,+,-,-,+,+,-,-,...
        let sign: i64 = if ((t - 1) / 2) % 2 == 0 { 1 } else { -1 };
        terms.push((tri, sign));
        t += 1;
    }
    terms
}

#[inline(always)]
fn reduce_mod(x: i64) -> u32 {
    let r = x % MODI;
    if r < 0 { (r + MODI) as u32 } else { r as u32 }
}

/// Accumulate large-offset contributions into a destination chunk of `temp`.
///
/// `tchunk` covers dest indices [d0, d0 + tchunk.len()) within the block.
/// Large triangular offsets are >= BLOCK, so every source lies in a previous
/// block and workers only read `f` (no races).
///
/// Signs on `large_tri` are grouped ++--; the 4-way inner loop uses that.
fn accum_chunk(tchunk: &mut [i64], d0: usize, f: &[u32], bstart: usize, large_tri: &[(usize, i64)]) {
    let len = tchunk.len();
    if len == 0 {
        return;
    }
    tchunk.fill(0);

    let n0 = bstart + d0;
    let n1 = n0 + len;

    // Terms with w >= n1 cannot contribute to this chunk.
    let end = large_tri.partition_point(|&(w, _)| w < n1);
    if end == 0 {
        return;
    }
    let terms = &large_tri[..end];
    // w <= n0: every dest in the chunk is valid (full-length vector add)
    let full_end = terms.partition_point(|&(w, _)| w <= n0);

    unsafe {
        let dst = tchunk.as_mut_ptr();
        let fp = f.as_ptr();
        let full = &terms[..full_end];

        let mut k = 0;
        while k + 4 <= full.len() {
            // SAFETY: k+3 < full.len(); w <= n0 so n0 - w >= 0; last source
            // index n1 - 1 - w < n1 <= N + 1, and f has length N + 1.
            let (w0, _) = *full.get_unchecked(k);
            let (w1, _) = *full.get_unchecked(k + 1);
            let (w2, _) = *full.get_unchecked(k + 2);
            let (w3, _) = *full.get_unchecked(k + 3);
            let p0 = fp.add(n0 - w0);
            let p1 = fp.add(n0 - w1);
            let p2 = fp.add(n0 - w2);
            let p3 = fp.add(n0 - w3);
            for j in 0..len {
                let a = *p0.add(j) as i64;
                let b = *p1.add(j) as i64;
                let c = *p2.add(j) as i64;
                let d = *p3.add(j) as i64;
                *dst.add(j) += a + b - c - d;
            }
            k += 4;
        }
        while k < full.len() {
            let (w, sign) = *full.get_unchecked(k);
            let src = fp.add(n0 - w);
            if sign > 0 {
                for j in 0..len {
                    *dst.add(j) += *src.add(j) as i64;
                }
            } else {
                for j in 0..len {
                    *dst.add(j) -= *src.add(j) as i64;
                }
            }
            k += 1;
        }

        // Partial terms: n0 < w < n1. Dest starts at w, source at 0.
        for idx in full_end..terms.len() {
            let (w, sign) = *terms.get_unchecked(idx);
            let dst_off = w - n0;
            let slen = len - dst_off;
            let dp = dst.add(dst_off);
            if sign > 0 {
                for j in 0..slen {
                    *dp.add(j) += *fp.add(j) as i64;
                }
            } else {
                for j in 0..slen {
                    *dp.add(j) -= *fp.add(j) as i64;
                }
            }
        }
    }
}

fn worker_count(work: u64) -> usize {
    if work < 2_000_000 {
        return 1;
    }
    let cap = rayon::current_num_threads().clamp(1, 16);
    let want = (work / 2_000_000) as usize;
    want.clamp(2, cap)
}

fn main() {
    let tri_terms = triangular_terms(N);
    let split = tri_terms
        .iter()
        .position(|&(w, _)| w >= BLOCK)
        .unwrap_or(tri_terms.len());
    let small_tri = &tri_terms[..split];
    let large_tri = &tri_terms[split..];

    // Sparse B[n]: nonzero only at pronic numbers m(m+1).
    // Sign: (-1)^floor((m+1)/2)
    let mut pronics: Vec<(usize, i64)> = Vec::new();
    {
        let mut m: usize = 0;
        loop {
            let p = m * (m + 1);
            if p > N {
                break;
            }
            let sign: i64 = if ((m + 1) / 2) % 2 == 0 { 1 } else { -1 };
            pronics.push((p, sign));
            m += 1;
        }
    }

    // u32: values fit in 0..MOD and the 40MB array fits in L3.
    let mut f = vec![0u32; N + 1];
    let mut temp = vec![0i64; BLOCK];
    let num_blocks = (N + BLOCK) / BLOCK;
    let mut pr_idx = 0usize;

    for b in 0..num_blocks {
        let bstart = b * BLOCK;
        let bend = std::cmp::min(bstart + BLOCK, N + 1);
        let blen = bend - bstart;

        // Phase 1: dest-chunk parallel accumulation of large-offset contribs
        let n_terms = large_tri.partition_point(|&(w, _)| w < bend);
        let work = n_terms as u64 * blen as u64;
        let n_workers = worker_count(work);
        if n_workers == 1 {
            accum_chunk(&mut temp[..blen], 0, &f, bstart, large_tri);
        } else {
            let mut chunk_len = (blen + n_workers - 1) / n_workers;
            chunk_len = (chunk_len + 15) & !15;
            if chunk_len == 0 {
                chunk_len = 1;
            }
            let f_ref: &[u32] = &f;
            temp[..blen]
                .par_chunks_mut(chunk_len)
                .enumerate()
                .for_each(|(cid, tchunk)| {
                    accum_chunk(tchunk, cid * chunk_len, f_ref, bstart, large_tri);
                });
        }

        // Phase 2: forward sweep within the block (loop-carried via T_1 = 1)
        for i in 0..blen {
            let n = bstart + i;
            let mut acc = unsafe { *temp.get_unchecked(i) };
            if pr_idx < pronics.len() {
                let (p, s) = unsafe { *pronics.get_unchecked(pr_idx) };
                if p == n {
                    acc += s;
                    pr_idx += 1;
                }
            }

            let mut k = 0;
            while k + 4 <= small_tri.len() {
                let (w3, _) = unsafe { *small_tri.get_unchecked(k + 3) };
                if w3 > n {
                    break;
                }
                // SAFETY: w3 <= n and the four ++-- weights are increasing,
                // so n - w >= 0; f has length N + 1 and n <= N.
                unsafe {
                    let (w0, _) = *small_tri.get_unchecked(k);
                    let (w1, _) = *small_tri.get_unchecked(k + 1);
                    let (w2, _) = *small_tri.get_unchecked(k + 2);
                    acc += *f.get_unchecked(n - w0) as i64
                        + *f.get_unchecked(n - w1) as i64
                        - *f.get_unchecked(n - w2) as i64
                        - *f.get_unchecked(n - w3) as i64;
                }
                k += 4;
            }
            while k < small_tri.len() {
                let (w, sign) = unsafe { *small_tri.get_unchecked(k) };
                if w > n {
                    break;
                }
                acc += sign * unsafe { *f.get_unchecked(n - w) } as i64;
                k += 1;
            }

            unsafe {
                *f.get_unchecked_mut(n) = reduce_mod(acc);
            }
        }
    }

    let mut ans = 0u64;
    for i in 1..=N {
        ans += unsafe { *f.get_unchecked(i) } as u64;
        if ans >= MOD {
            ans -= MOD;
        }
    }

    println!("{}", ans);
}
