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
// `temp` so there is no per-thread merge. Phase 2 is a forward sweep; T_1=1
// and T_2=3 are loop-carried, other small offsets are independent across a
// group of 4 consecutive n.

use rayon::prelude::*;

const N: usize = 10_000_000;
const MOD: u64 = 1_000_000_007;
const MODI: i64 = MOD as i64;
const BLOCK: usize = 1 << 12; // 4096; more large terms, shorter serial small sweep

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

/// Align `k` so `full[k..]` starts on a ++-- sign group.
///
/// `large_tri` is a suffix of all triangular terms. The ++-- pattern is
/// relative to t=1, not to the suffix start; BLOCK=4096 splits at t=91
/// (--++), so a blind 4/8-way unroll from k=0 is wrong.
fn align_ppmm(full: &[(usize, i64)], mut k: usize) -> usize {
    while k + 4 <= full.len() {
        let s0 = unsafe { full.get_unchecked(k).1 };
        let s1 = unsafe { full.get_unchecked(k + 1).1 };
        let s2 = unsafe { full.get_unchecked(k + 2).1 };
        let s3 = unsafe { full.get_unchecked(k + 3).1 };
        if s0 > 0 && s1 > 0 && s2 < 0 && s3 < 0 {
            break;
        }
        k += 1;
    }
    k
}

/// Accumulate large-offset contributions into a destination chunk of `temp`.
///
/// `tchunk` covers dest indices [d0, d0 + tchunk.len()) within the block.
/// Large triangular offsets are >= BLOCK, so every source lies in a previous
/// block and workers only read `f` (no races).
fn accum_chunk(tchunk: &mut [i64], d0: usize, f: &[u32], bstart: usize, large_tri: &[(usize, i64)]) {
    let len = tchunk.len();
    if len == 0 {
        return;
    }
    tchunk.fill(0);

    let n0 = bstart + d0;
    let n1 = n0 + len;

    let end = large_tri.partition_point(|&(w, _)| w < n1);
    if end == 0 {
        return;
    }
    let terms = &large_tri[..end];
    let full_end = terms.partition_point(|&(w, _)| w <= n0);

    unsafe {
        let dst = tchunk.as_mut_ptr();
        let fp = f.as_ptr();
        let full = &terms[..full_end];

        let mut k = 0;
        // Scalar until ++-- aligned (or leftover < 4).
        let aligned = align_ppmm(full, 0);
        while k < aligned {
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

        while k + 8 <= full.len() {
            let (w0, _) = *full.get_unchecked(k);
            let (w1, _) = *full.get_unchecked(k + 1);
            let (w2, _) = *full.get_unchecked(k + 2);
            let (w3, _) = *full.get_unchecked(k + 3);
            let (w4, _) = *full.get_unchecked(k + 4);
            let (w5, _) = *full.get_unchecked(k + 5);
            let (w6, _) = *full.get_unchecked(k + 6);
            let (w7, _) = *full.get_unchecked(k + 7);
            let p0 = fp.add(n0 - w0);
            let p1 = fp.add(n0 - w1);
            let p2 = fp.add(n0 - w2);
            let p3 = fp.add(n0 - w3);
            let p4 = fp.add(n0 - w4);
            let p5 = fp.add(n0 - w5);
            let p6 = fp.add(n0 - w6);
            let p7 = fp.add(n0 - w7);
            for j in 0..len {
                let s = *p0.add(j) as i64 + *p1.add(j) as i64
                    - *p2.add(j) as i64 - *p3.add(j) as i64
                    + *p4.add(j) as i64 + *p5.add(j) as i64
                    - *p6.add(j) as i64 - *p7.add(j) as i64;
                *dst.add(j) += s;
            }
            k += 8;
        }
        while k + 4 <= full.len() {
            let (w0, _) = *full.get_unchecked(k);
            let (w1, _) = *full.get_unchecked(k + 1);
            let (w2, _) = *full.get_unchecked(k + 2);
            let (w3, _) = *full.get_unchecked(k + 3);
            let p0 = fp.add(n0 - w0);
            let p1 = fp.add(n0 - w1);
            let p2 = fp.add(n0 - w2);
            let p3 = fp.add(n0 - w3);
            for j in 0..len {
                *dst.add(j) += *p0.add(j) as i64 + *p1.add(j) as i64
                    - *p2.add(j) as i64 - *p3.add(j) as i64;
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

#[inline]
fn worker_count(work: u64) -> usize {
    if work < 1_000_000 {
        return 1;
    }
    let cap = rayon::current_num_threads().clamp(1, 32);
    if cap < 2 {
        return 1;
    }
    let want = (work / 1_000_000) as usize;
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

    let small_w: Vec<usize> = small_tri.iter().map(|&(w, _)| w).collect();
    let small_s: Vec<i64> = small_tri.iter().map(|&(_, s)| s).collect();

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

    let mut f = vec![0u32; N + 1];
    let mut temp = vec![0i64; BLOCK];
    let num_blocks = (N + BLOCK) / BLOCK;
    let mut pr_idx = 0usize;
    let sn = small_w.len();

    for b in 0..num_blocks {
        let bstart = b * BLOCK;
        let bend = std::cmp::min(bstart + BLOCK, N + 1);
        let blen = bend - bstart;

        let n_terms = large_tri.partition_point(|&(w, _)| w < bend);
        let work = n_terms as u64 * blen as u64;
        let n_workers = worker_count(work);
        if n_workers == 1 {
            accum_chunk(&mut temp[..blen], 0, &f, bstart, large_tri);
        } else {
            let mut chunk_len = (blen + n_workers - 1) / n_workers;
            chunk_len = (chunk_len + 31) & !31;
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

        if b == 0 {
            // First block: n may be < some small offsets.
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
                while k + 4 <= sn {
                    let w3 = unsafe { *small_w.get_unchecked(k + 3) };
                    if w3 > n {
                        break;
                    }
                    unsafe {
                        let w0 = *small_w.get_unchecked(k);
                        let w1 = *small_w.get_unchecked(k + 1);
                        let w2 = *small_w.get_unchecked(k + 2);
                        acc += *f.get_unchecked(n - w0) as i64
                            + *f.get_unchecked(n - w1) as i64
                            - *f.get_unchecked(n - w2) as i64
                            - *f.get_unchecked(n - w3) as i64;
                    }
                    k += 4;
                }
                while k < sn {
                    let w = unsafe { *small_w.get_unchecked(k) };
                    if w > n {
                        break;
                    }
                    let s = unsafe { *small_s.get_unchecked(k) };
                    acc += s * unsafe { *f.get_unchecked(n - w) } as i64;
                    k += 1;
                }
                unsafe {
                    *f.get_unchecked_mut(n) = reduce_mod(acc);
                }
            }
        } else {
            // n >= BLOCK > every small w. T_1=1 and T_2=3 are intra-group;
            // remaining small offsets read f < n0.
            let fp = f.as_ptr();
            let fmp = f.as_mut_ptr();
            let tp = temp.as_ptr();
            let sw = small_w.as_ptr();
            let ss = small_s.as_ptr();
            let mut i = 0;
            while i + 4 <= blen {
                let n0 = bstart + i;
                let mut a0 = unsafe { *tp.add(i) };
                let mut a1 = unsafe { *tp.add(i + 1) };
                let mut a2 = unsafe { *tp.add(i + 2) };
                let mut a3 = unsafe { *tp.add(i + 3) };
                while pr_idx < pronics.len() && unsafe { pronics.get_unchecked(pr_idx).0 } < n0 {
                    pr_idx += 1;
                }
                if pr_idx < pronics.len() && unsafe { pronics.get_unchecked(pr_idx).0 } == n0 {
                    a0 += unsafe { pronics.get_unchecked(pr_idx).1 };
                    pr_idx += 1;
                }
                if pr_idx < pronics.len() && unsafe { pronics.get_unchecked(pr_idx).0 } == n0 + 1 {
                    a1 += unsafe { pronics.get_unchecked(pr_idx).1 };
                    pr_idx += 1;
                }
                if pr_idx < pronics.len() && unsafe { pronics.get_unchecked(pr_idx).0 } == n0 + 2 {
                    a2 += unsafe { pronics.get_unchecked(pr_idx).1 };
                    pr_idx += 1;
                }
                if pr_idx < pronics.len() && unsafe { pronics.get_unchecked(pr_idx).0 } == n0 + 3 {
                    a3 += unsafe { pronics.get_unchecked(pr_idx).1 };
                    pr_idx += 1;
                }
                unsafe {
                    // k=2,3 (w=6,10): outside the 4-group, not ++-- fused.
                    let mut k = 2;
                    while k < 4 && k < sn {
                        let w = *sw.add(k);
                        let s = *ss.add(k);
                        if s > 0 {
                            a0 += *fp.add(n0 - w) as i64;
                            a1 += *fp.add(n0 + 1 - w) as i64;
                            a2 += *fp.add(n0 + 2 - w) as i64;
                            a3 += *fp.add(n0 + 3 - w) as i64;
                        } else {
                            a0 -= *fp.add(n0 - w) as i64;
                            a1 -= *fp.add(n0 + 1 - w) as i64;
                            a2 -= *fp.add(n0 + 2 - w) as i64;
                            a3 -= *fp.add(n0 + 3 - w) as i64;
                        }
                        k += 1;
                    }
                    while k + 4 <= sn {
                        let w0 = *sw.add(k);
                        let w1 = *sw.add(k + 1);
                        let w2 = *sw.add(k + 2);
                        let w3 = *sw.add(k + 3);
                        let s0 = fp.add(n0 - w0);
                        let s1 = fp.add(n0 - w1);
                        let s2 = fp.add(n0 - w2);
                        let s3 = fp.add(n0 - w3);
                        a0 += *s0 as i64 + *s1 as i64 - *s2 as i64 - *s3 as i64;
                        a1 += *s0.add(1) as i64 + *s1.add(1) as i64
                            - *s2.add(1) as i64 - *s3.add(1) as i64;
                        a2 += *s0.add(2) as i64 + *s1.add(2) as i64
                            - *s2.add(2) as i64 - *s3.add(2) as i64;
                        a3 += *s0.add(3) as i64 + *s1.add(3) as i64
                            - *s2.add(3) as i64 - *s3.add(3) as i64;
                        k += 4;
                    }
                    while k < sn {
                        let w = *sw.add(k);
                        let s = *ss.add(k);
                        if s > 0 {
                            a0 += *fp.add(n0 - w) as i64;
                            a1 += *fp.add(n0 + 1 - w) as i64;
                            a2 += *fp.add(n0 + 2 - w) as i64;
                            a3 += *fp.add(n0 + 3 - w) as i64;
                        } else {
                            a0 -= *fp.add(n0 - w) as i64;
                            a1 -= *fp.add(n0 + 1 - w) as i64;
                            a2 -= *fp.add(n0 + 2 - w) as i64;
                            a3 -= *fp.add(n0 + 3 - w) as i64;
                        }
                        k += 1;
                    }
                    // T_1=1 (+), T_2=3 (+) resolved in order inside the group.
                    let f_m1 = *fp.add(n0 - 1) as i64;
                    let f_m2 = *fp.add(n0 - 2) as i64;
                    let f_m3 = *fp.add(n0 - 3) as i64;
                    let v0 = reduce_mod(a0 + f_m1 + f_m3);
                    *fmp.add(n0) = v0;
                    let v0i = v0 as i64;
                    let v1 = reduce_mod(a1 + v0i + f_m2);
                    *fmp.add(n0 + 1) = v1;
                    let v1i = v1 as i64;
                    let v2 = reduce_mod(a2 + v1i + f_m1);
                    *fmp.add(n0 + 2) = v2;
                    let v2i = v2 as i64;
                    let v3 = reduce_mod(a3 + v2i + v0i);
                    *fmp.add(n0 + 3) = v3;
                }
                i += 4;
            }
            for j in i..blen {
                let n = bstart + j;
                let mut acc = unsafe { *tp.add(j) };
                while pr_idx < pronics.len() {
                    let (p, s) = unsafe { *pronics.get_unchecked(pr_idx) };
                    if p < n {
                        pr_idx += 1;
                        continue;
                    }
                    if p == n {
                        acc += s;
                        pr_idx += 1;
                    }
                    break;
                }
                let mut k = 0;
                unsafe {
                    while k + 4 <= sn {
                        let w0 = *sw.add(k);
                        let w1 = *sw.add(k + 1);
                        let w2 = *sw.add(k + 2);
                        let w3 = *sw.add(k + 3);
                        acc += *fp.add(n - w0) as i64 + *fp.add(n - w1) as i64
                            - *fp.add(n - w2) as i64 - *fp.add(n - w3) as i64;
                        k += 4;
                    }
                    while k < sn {
                        let w = *sw.add(k);
                        let s = *ss.add(k);
                        if s > 0 {
                            acc += *fp.add(n - w) as i64;
                        } else {
                            acc -= *fp.add(n - w) as i64;
                        }
                        k += 1;
                    }
                    *fmp.add(n) = reduce_mod(acc);
                }
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
