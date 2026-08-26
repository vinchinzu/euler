// Project Euler 677 - Coloured Graphs
// Flattened u32 DP, AVX2 convolution, rayon on the independent f3 tail.
// N=10000, K=4. Sequential in `size` (H depends on previous sizes).

#![allow(unsafe_op_in_unsafe_fn)]

use rayon::prelude::*;

const MOD: u64 = 1_000_000_007;
const MOD128: u128 = MOD as u128;
const N: usize = 10_000;
const K: usize = 4;
const N2: usize = N / 2;

fn mod_inv(mut a: u64) -> u64 {
    let mut result = 1u64;
    let mut exp = MOD - 2;
    a %= MOD;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * a % MOD;
        }
        a = a * a % MOD;
        exp >>= 1;
    }
    result
}

/// sum_{i=lo..=hi} a[i] * b[sum_idx - i]  (mod MOD)
///
/// Indices lo..=hi must be in-bounds for `a`; sum_idx-i in-bounds for `b`.
#[inline]
fn conv_range(a: &[u32], b: &[u32], lo: usize, hi: usize, sum_idx: usize) -> u32 {
    if lo > hi {
        return 0;
    }
    // SAFETY: caller guarantees lo..=hi and sum_idx-i are in range.
    unsafe { conv_range_dispatch(a, b, lo, hi, sum_idx) }
}

#[inline]
unsafe fn conv_range_dispatch(a: &[u32], b: &[u32], lo: usize, hi: usize, sum_idx: usize) -> u32 {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx2") {
            return conv_range_avx2(a, b, lo, hi, sum_idx);
        }
    }
    conv_range_scalar(a, b, lo, hi, sum_idx)
}

#[inline]
unsafe fn conv_range_scalar(a: &[u32], b: &[u32], lo: usize, hi: usize, sum_idx: usize) -> u32 {
    let ap = a.as_ptr();
    let bp = b.as_ptr();
    let mut acc = 0u128;
    let mut i = lo;
    while i + 7 <= hi {
        acc += *ap.add(i) as u128 * *bp.add(sum_idx - i) as u128;
        acc += *ap.add(i + 1) as u128 * *bp.add(sum_idx - i - 1) as u128;
        acc += *ap.add(i + 2) as u128 * *bp.add(sum_idx - i - 2) as u128;
        acc += *ap.add(i + 3) as u128 * *bp.add(sum_idx - i - 3) as u128;
        acc += *ap.add(i + 4) as u128 * *bp.add(sum_idx - i - 4) as u128;
        acc += *ap.add(i + 5) as u128 * *bp.add(sum_idx - i - 5) as u128;
        acc += *ap.add(i + 6) as u128 * *bp.add(sum_idx - i - 6) as u128;
        acc += *ap.add(i + 7) as u128 * *bp.add(sum_idx - i - 7) as u128;
        i += 8;
    }
    while i <= hi {
        acc += *ap.add(i) as u128 * *bp.add(sum_idx - i) as u128;
        i += 1;
    }
    (acc % MOD128) as u32
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn conv_range_avx2(a: &[u32], b: &[u32], lo: usize, hi: usize, sum_idx: usize) -> u32 {
    use std::arch::x86_64::*;
    let ap = a.as_ptr();
    let bp = b.as_ptr();
    let mut acc = 0u128;
    let mut i = lo;
    let rev = _mm256_setr_epi32(7, 6, 5, 4, 3, 2, 1, 0);
    while i + 7 <= hi {
        let va = _mm256_loadu_si256(ap.add(i) as *const __m256i);
        let vb_fwd = _mm256_loadu_si256(bp.add(sum_idx - i - 7) as *const __m256i);
        let vb = _mm256_permutevar8x32_epi32(vb_fwd, rev);
        let pe = _mm256_mul_epu32(va, vb);
        let po = _mm256_mul_epu32(_mm256_srli_epi64(va, 32), _mm256_srli_epi64(vb, 32));
        let p = _mm256_add_epi64(pe, po);
        let hi128 = _mm256_extracti128_si256(p, 1);
        let lo128 = _mm256_castsi256_si128(p);
        let s = _mm_add_epi64(lo128, hi128);
        let s2 = _mm_add_epi64(s, _mm_unpackhi_epi64(s, s));
        acc += _mm_cvtsi128_si64(s2) as u64 as u128;
        i += 8;
    }
    while i <= hi {
        acc += *ap.add(i) as u128 * *bp.add(sum_idx - i) as u128;
        i += 1;
    }
    (acc % MOD128) as u32
}

/// sum_{cs=1..=cs_max} h[cs] * f1[size - 2*cs]
#[inline]
unsafe fn strided2(h: &[u32], f1: &[u32], size: usize, cs_max: usize) -> u64 {
    if cs_max == 0 {
        return 0;
    }
    let hp = h.as_ptr();
    let fp = f1.as_ptr();
    let mut acc = 0u128;
    let mut cs = 1usize;
    while cs + 3 <= cs_max {
        acc += *hp.add(cs) as u128 * *fp.add(size - 2 * cs) as u128;
        acc += *hp.add(cs + 1) as u128 * *fp.add(size - 2 * cs - 2) as u128;
        acc += *hp.add(cs + 2) as u128 * *fp.add(size - 2 * cs - 4) as u128;
        acc += *hp.add(cs + 3) as u128 * *fp.add(size - 2 * cs - 6) as u128;
        cs += 4;
    }
    while cs <= cs_max {
        acc += *hp.add(cs) as u128 * *fp.add(size - 2 * cs) as u128;
        cs += 1;
    }
    (acc % MOD128) as u64
}

fn fill_f3_tail(h: &[u32], f2: &[u32], f3: &mut [u32]) {
    f3[N2 + 1..=N]
        .par_iter_mut()
        .with_min_len(16)
        .enumerate()
        .for_each(|(i, slot)| {
            *slot = conv_range(h, f2, 1, N2, N2 + 1 + i);
        });
}

fn main() {
    let mut fact = [0u64; K + 1];
    fact[0] = 1;
    for i in 1..=K {
        fact[i] = fact[i - 1] * i as u64 % MOD;
    }
    let mut inv_fact = [0u64; K + 1];
    for i in 0..=K {
        inv_fact[i] = mod_inv(fact[i]);
    }
    let mut ncr = [[0u64; K + 1]; K + 1];
    for nn in 0..=K {
        ncr[nn][0] = 1;
        for rr in 1..=nn {
            ncr[nn][rr] = (ncr[nn - 1][rr - 1] + ncr[nn - 1][rr]) % MOD;
        }
    }

    // Contiguous 1D buffers (not Vec<Vec<Vec<_>>>): f{nc}_{yr}[size], h_{yr}[size]
    let mut h0 = vec![0u32; N + 1];
    let mut h1 = vec![0u32; N + 1];
    let mut f1_0 = vec![0u32; N + 1];
    let mut f1_1 = vec![0u32; N + 1];
    let mut f2_0 = vec![0u32; N + 1];
    let mut f2_1 = vec![0u32; N + 1];
    let mut f3_0 = vec![0u32; N + 1];
    let mut f3_1 = vec![0u32; N + 1];

    let inv2 = inv_fact[2];
    let inv6 = inv_fact[3];

    let mut hr_mid = 0u64;
    let mut hb_mid = 0u64;
    let mut hy_mid = 0u64;
    let mut h1_mid = 0u64;

    // Phase 1: sizes 1..=N/2. H is revealed online; f/g at this size use H[1..size-1].
    for size in 1..=N2 {
        // ---- yr = 0 ----
        // f[0] = [size==1]; f[1][size] = H[size-1] (cs = size-1 <= N2)
        let v1_0 = if size >= 2 {
            // SAFETY: size-1 < N+1
            unsafe { *h0.get_unchecked(size - 1) }
        } else {
            0
        };
        unsafe {
            *f1_0.get_unchecked_mut(size) = v1_0;
        }
        let hi = size - 1; // size >= 1
        let v2_0 = conv_range(&h0, &f1_0, 1, hi, size);
        unsafe {
            *f2_0.get_unchecked_mut(size) = v2_0;
        }
        let v3_0 = conv_range(&h0, &f2_0, 1, hi, size);
        unsafe {
            *f3_0.get_unchecked_mut(size) = v3_0;
        }

        let g0_0 = if size == 1 { 1u64 } else { 0 };
        let g1_0 = v1_0 as u64;
        let mut c2_0 = v2_0 as u64;
        if size >= 3 && (size & 1) == 1 {
            let cs = (size - 1) / 2;
            // SAFETY: cs = (size-1)/2 <= N2/2 < N+1
            c2_0 += unsafe { *h0.get_unchecked(cs) } as u64;
        }
        let g2_0 = c2_0 % MOD * inv2 % MOD;
        let mut c3_0 = v3_0 as u64;
        let cs2 = (size - 1) / 2;
        if cs2 >= 1 {
            // SAFETY: cs <= (size-1)/2, size-2*cs >= 1, both in 0..=N
            c3_0 += 3 * unsafe { strided2(&h0, &f1_0, size, cs2) };
        }
        if size > 1 && (size - 1) % 3 == 0 {
            let cs = (size - 1) / 3;
            if cs >= 1 && 3 * cs < size {
                c3_0 += 2 * unsafe { *h0.get_unchecked(cs) } as u64;
            }
        }
        let g3_0 = c3_0 % MOD * inv6 % MOD;

        // ---- yr = 1 ----
        let v1_1 = if size >= 2 {
            unsafe { *h1.get_unchecked(size - 1) }
        } else {
            0
        };
        unsafe {
            *f1_1.get_unchecked_mut(size) = v1_1;
        }
        let v2_1 = conv_range(&h1, &f1_1, 1, hi, size);
        unsafe {
            *f2_1.get_unchecked_mut(size) = v2_1;
        }
        let v3_1 = conv_range(&h1, &f2_1, 1, hi, size);
        unsafe {
            *f3_1.get_unchecked_mut(size) = v3_1;
        }

        let g0_1 = if size == 1 { 1u64 } else { 0 };
        let g1_1 = v1_1 as u64;
        let mut c2_1 = v2_1 as u64;
        if size >= 3 && (size & 1) == 1 {
            let cs = (size - 1) / 2;
            c2_1 += unsafe { *h1.get_unchecked(cs) } as u64;
        }
        let g2_1 = c2_1 % MOD * inv2 % MOD;
        // g[3][1] is unused for H (h_y only sums nc < K-1)

        let hr = (g0_0 + g1_0 + g2_0 + g3_0) % MOD;
        let hb = (g0_0 + g1_0 + g2_0) % MOD;
        let hy = (g0_1 + g1_1 + g2_1) % MOD;
        unsafe {
            *h0.get_unchecked_mut(size) = ((hr + hb + hy) % MOD) as u32;
            *h1.get_unchecked_mut(size) = ((hr + hb) % MOD) as u32;
        }
        if size == N2 {
            hr_mid = hr;
            hb_mid = hb;
            hy_mid = hy;
            h1_mid = (hr + hb) % MOD;
        }
    }

    // f[1] is H shifted: only nonzero on 2..=N2+1
    unsafe {
        *f1_0.get_unchecked_mut(N2 + 1) = *h0.get_unchecked(N2);
        *f1_1.get_unchecked_mut(N2 + 1) = *h1.get_unchecked(N2);
    }

    // f[2] tail: f1 has small support, so tighten the cs range.
    for size in N2 + 1..=N {
        let t_lo = 2.max(size - N2);
        let t_hi = (size - 1).min(N2 + 1);
        if t_lo <= t_hi {
            let cs_lo = size - t_hi;
            let cs_hi = size - t_lo;
            unsafe {
                *f2_0.get_unchecked_mut(size) = conv_range(&h0, &f1_0, cs_lo, cs_hi, size);
                *f2_1.get_unchecked_mut(size) = conv_range(&h1, &f1_1, cs_lo, cs_hi, size);
            }
        }
    }

    // f[3] tail: H and f[2] are fully known — independent across size.
    fill_f3_tail(&h0, &f2_0, &mut f3_0);
    fill_f3_tail(&h1, &f2_1, &mut f3_1);

    // size == N: f[4] and all g[nc][yr][N]
    let mut g_n = [[0u64; 2]; K + 1];
    for yr in 0..2 {
        let h: &[u32] = if yr == 0 { &h0 } else { &h1 };
        let f1: &[u32] = if yr == 0 { &f1_0 } else { &f1_1 };
        let f2: &[u32] = if yr == 0 { &f2_0 } else { &f2_1 };
        let f3: &[u32] = if yr == 0 { &f3_0 } else { &f3_1 };
        let f4 = conv_range(h, f3, 1, N2, N);

        for nc in 0..=K {
            let fcur = match nc {
                0 | 1 => 0u64,
                2 => f2[N] as u64,
                3 => f3[N] as u64,
                4 => f4 as u64,
                _ => 0,
            };
            let mut count = fcur;
            for kk in 2..=nc {
                let multiplier = fact[kk - 1] * ncr[nc][kk] % MOD;
                let cs_max = ((N - 1) / kk).min(N2);
                if cs_max == 0 {
                    continue;
                }
                let mut acc = 0u128;
                let fprev_nc = nc - kk;
                unsafe {
                    let hp = h.as_ptr();
                    for cs in 1..=cs_max {
                        let t = N - kk * cs;
                        let fv = match fprev_nc {
                            0 => {
                                if t == 1 {
                                    1u32
                                } else {
                                    0
                                }
                            }
                            1 => *f1.get_unchecked(t),
                            2 => *f2.get_unchecked(t),
                            3 => *f3.get_unchecked(t),
                            _ => 0,
                        };
                        acc += *hp.add(cs) as u128 * fv as u128;
                    }
                }
                count += multiplier * (acc % MOD128) as u64;
            }
            g_n[nc][yr] = count % MOD * inv_fact[nc] % MOD;
        }
    }

    let mut ans = 0u64;
    for nc in 0..=K {
        ans += g_n[nc][0];
    }
    for nc in 0..K {
        ans += g_n[nc][0];
    }
    for nc in 0..K {
        ans += g_n[nc][1];
    }
    ans %= MOD;

    ans = ans * 2 % MOD;
    let h_arr = [hr_mid, hb_mid, hy_mid];
    for i in 0..3 {
        for j in 0..3 {
            if i != 2 || j != 2 {
                ans = (ans + MOD - h_arr[i] * h_arr[j] % MOD) % MOD;
            }
        }
    }
    ans = (ans + h1_mid) % MOD;
    ans = ans * mod_inv(2) % MOD;
    println!("{}", ans);
}
