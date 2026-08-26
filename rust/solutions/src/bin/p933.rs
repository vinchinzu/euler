// Project Euler 933 - Paper Cutting
//
// D(W,H) = sum_{w=2..W} sum_{h=2..H} C(w,h)
// C(w,h) = # winning first-player moves on w x h paper (Sprague-Grundy game).
// Find D(123, 1234567).
//
// Key insights:
// 1. G(w,h) = mex over all cuts (a,b) of G(a,b)^G(a,h-b)^G(w-a,b)^G(w-a,h-b)
// 2. C(w,h) = #{(a,b) : L_a(b) = L_a(h-b)} where L_a(b) = G(a,b)^G(w-a,b)
//    C(w,h) only depends on G values for widths < w.
// 3. G(w,h) stabilizes to G_inf(w) for h >= H_stab(w), so L_a also stabilizes.
// 4. Rewrite sum over h as: S(a,c,H) = |{(b1,b2): b1,b2>=1, b1+b2<=H, L(b1)=L(b2)}|
//    For large H, this decomposes into prefix (transient) and tail (stabilized) parts
//    with a closed-form formula, avoiding enumeration over all h.
//
// Phase-1 speedups:
// - XOR(a,b,w,h) is symmetric in b <-> h-b, so only b=1..=floor((h-1)/2).
// - V_a[b] = G(a,b)^G(w-a,b) is constant for b >= ta = max(H_stab(a), H_stab(w-a));
//   once h/2 >= ta the contribution of a is a precomputed bitset (includes 0).
// - G(w,h) is independent of h for h >= 2*max_{a<w} H_stab(a); cap height there
//   (still empirically capped at 3500, matching the original budget).

use rayon::prelude::*;

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

#[inline(always)]
fn mex_bytes(seen: &[u8; 256]) -> u8 {
    let mut m = 0u16;
    while m < 256 && seen[m as usize] != 0 {
        m += 1;
    }
    m as u8
}

#[inline(always)]
unsafe fn mark_pairs_scalar(
    seen: &mut [u8; 256],
    vp: *const u8,
    h: usize,
    mut b: usize,
    maxb: usize,
) {
    // SAFETY: caller guarantees vp[1..=maxb] and vp[h-maxb..=h-1] are in-bounds;
    // xor results are u8 so seen[v] is in 0..256.
    unsafe {
        while b + 3 <= maxb {
            let v0 = *vp.add(b) ^ *vp.add(h - b);
            let v1 = *vp.add(b + 1) ^ *vp.add(h - b - 1);
            let v2 = *vp.add(b + 2) ^ *vp.add(h - b - 2);
            let v3 = *vp.add(b + 3) ^ *vp.add(h - b - 3);
            *seen.get_unchecked_mut(v0 as usize) = 1;
            *seen.get_unchecked_mut(v1 as usize) = 1;
            *seen.get_unchecked_mut(v2 as usize) = 1;
            *seen.get_unchecked_mut(v3 as usize) = 1;
            b += 4;
        }
        while b <= maxb {
            let v = *vp.add(b) ^ *vp.add(h - b);
            *seen.get_unchecked_mut(v as usize) = 1;
            b += 1;
        }
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn mark_pairs_avx2(seen: &mut [u8; 256], vp: *const u8, h: usize, maxb: usize) {
    // SAFETY: b+31 <= maxb so both 32-byte windows lie in vp[1..=h-1].
    unsafe {
        let shuf = _mm256_setr_epi8(
            15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 15, 14, 13, 12, 11, 10, 9, 8, 7,
            6, 5, 4, 3, 2, 1, 0,
        );
        let mut b = 1usize;
        while b + 31 <= maxb {
            let lo = _mm256_loadu_si256(vp.add(b) as *const __m256i);
            let hi = _mm256_loadu_si256(vp.add(h - b - 31) as *const __m256i);
            let swapped = _mm256_permute2x128_si256(hi, hi, 0x01);
            let hi_rev = _mm256_shuffle_epi8(swapped, shuf);
            let x = _mm256_xor_si256(lo, hi_rev);
            let mut buf = [0u8; 32];
            _mm256_storeu_si256(buf.as_mut_ptr() as *mut __m256i, x);
            for i in 0..32 {
                *seen.get_unchecked_mut(*buf.get_unchecked(i) as usize) = 1;
            }
            b += 32;
        }
        mark_pairs_scalar(seen, vp, h, b, maxb);
    }
}

struct Prep {
    v_stride: usize,
    v: Vec<u8>,
    tas: Vec<usize>,
    order: Vec<usize>,
    prefix_seen: Vec<u8>,
    w_even: bool,
}

#[cfg_attr(target_arch = "x86_64", target_feature(enable = "avx2"))]
unsafe fn grundy_h(h: usize, prep: &Prep) -> u8 {
    // SAFETY: k in 0..=n_a, prefix_seen has (n_a+1)*256 bytes, V rows are v_stride
    // with indices 1..=h-1 valid for this width's computed limit.
    unsafe {
        let half = h >> 1;
        let k = prep
            .order
            .partition_point(|&i| *prep.tas.get_unchecked(i) <= half);
        let mut seen = [0u8; 256];
        std::ptr::copy_nonoverlapping(
            prep.prefix_seen.as_ptr().add(k * 256),
            seen.as_mut_ptr(),
            256,
        );
        if prep.w_even || (h & 1) == 0 {
            seen[0] = 1;
        }
        let maxb = if (h & 1) == 0 {
            half.saturating_sub(1)
        } else {
            half
        };
        if maxb >= 1 {
            let vp0 = prep.v.as_ptr();
            let v_stride = prep.v_stride;
            for &ai in prep.order.get_unchecked(k..) {
                let vp = vp0.add(ai * v_stride);
                #[cfg(target_arch = "x86_64")]
                mark_pairs_avx2(&mut seen, vp, h, maxb);
                #[cfg(not(target_arch = "x86_64"))]
                mark_pairs_scalar(&mut seen, vp, h, 1, maxb);
            }
        }
        mex_bytes(&seen)
    }
}

fn main() {
    let w_max: usize = 123;
    let h_target: usize = 1_234_567;

    // Empirically, max stabilization point for w<=123 is 3320.
    let h_budget: usize = 3500;
    let stride = h_budget + 1;
    let v_stride = (h_budget + 64) & !63;

    let mut g = vec![0u8; (w_max + 1) * stride];
    let mut h_stab = vec![0usize; w_max + 1];
    let mut g_inf = vec![0u8; w_max + 1];

    h_stab[0] = 1;
    h_stab[1] = 1;
    let mut max_prev_stab = 1usize;

    for w in 2..=w_max {
        let half_w = (w - 1) / 2;
        let w_even = w % 2 == 0;
        let limit = (2 * max_prev_stab).min(h_budget).max(2);

        let mut v = vec![0u8; half_w.saturating_mul(v_stride)];
        let mut tas = vec![0usize; half_w];
        let mut large = vec![0u8; half_w.saturating_mul(256)];

        for a_idx in 0..half_w {
            let a = a_idx + 1;
            let c = w - a;
            let ta = h_stab[a].max(h_stab[c]).max(1);
            tas[a_idx] = ta;
            let v_inf = g_inf[a] ^ g_inf[c];
            let dest_off = a_idx * v_stride;
            let ga = a * stride;
            let gc = c * stride;
            for b in 1..=limit {
                v[dest_off + b] = g[ga + b] ^ g[gc + b];
            }
            let ls = a_idx * 256;
            large[ls] = 1;
            let tmax = ta.min(limit + 1);
            for b in 1..tmax {
                large[ls + (v[dest_off + b] ^ v_inf) as usize] = 1;
            }
        }

        let mut order: Vec<usize> = (0..half_w).collect();
        order.sort_unstable_by_key(|&i| tas[i]);
        let mut prefix_seen = vec![0u8; (half_w + 1) * 256];
        for i in 0..half_w {
            let prev = i * 256;
            let next = (i + 1) * 256;
            let ai = order[i];
            let ls = ai * 256;
            for j in 0..256 {
                prefix_seen[next + j] = prefix_seen[prev + j] | large[ls + j];
            }
        }

        let prep = Prep {
            v_stride,
            v,
            tas,
            order,
            prefix_seen,
            w_even,
        };

        let work = half_w.saturating_mul(limit);
        let row: Vec<u8> = if work < 8192 || half_w == 0 {
            (2..limit + 1)
                .map(|h| unsafe { grundy_h(h, &prep) })
                .collect()
        } else {
            (2..limit + 1)
                .into_par_iter()
                .with_min_len(8)
                .map(|h| unsafe { grundy_h(h, &prep) })
                .collect()
        };

        let row_start = w * stride;
        g[row_start + 2..row_start + 2 + row.len()].copy_from_slice(&row);

        let val = g[row_start + limit];
        let mut first = limit;
        for h in (2..=limit).rev() {
            if g[row_start + h] == val {
                first = h;
            } else {
                break;
            }
        }
        h_stab[w] = first;
        g_inf[w] = val;
        if first > max_prev_stab {
            max_prev_stab = first;
        }
        // Later widths read G(w, b) past H_stab(w); fill the stable tail.
        for h in first..=h_budget {
            g[row_start + h] = val;
        }
    }

    // Phase 2: Compute D(W,H) using the analytical pair-counting formula.
    //
    // D(W,H) = sum_{w=2}^W sum_{a=1}^{w-1} S(a, w-a, H)
    //
    // S(a, c, H) = |{(b1,b2): 1<=b1, 1<=b2, b1+b2<=H, L(b1)=L(b2)}|
    // where L(b) = G(a,b) ^ G(c,b), stabilizing to l_inf for b >= b_stab.
    //
    // Decompose b-values into:
    //   Prefix: b in [1, b_stab-1]  (L may vary)
    //   Tail:   b in [b_stab, H-1]  (L = l_inf)
    //
    // For H >> b_stab (which holds since H=1234567 >> max b_stab ~ 3320):
    //   S = sum_{v != l_inf} count_v^2          (prefix-prefix, same non-stable value)
    //     + p^2                                   (prefix-prefix, stable value)
    //     + 2*(p*(H - bs + 1) - sum_P)           (prefix-tail cross terms)
    //     + M*(M+1)/2                             (tail-tail, M = H - 2*bs + 1)
    // where p = #{b in prefix : L(b) = l_inf}, sum_P = sum of those b values,
    //       count_v = #{b in prefix : L(b) = v}, bs = b_stab.

    let mut total_d: i64 = 0;

    for w in 2..=w_max {
        for a in 1..w {
            let c = w - a;
            let b_stab = std::cmp::max(std::cmp::max(h_stab[a], h_stab[c]), 1);
            let l_inf = g_inf[a] ^ g_inf[c];

            let mut count_map = [0i64; 256];
            let mut prefix_b_sum_linf: i64 = 0;
            for b in 1..b_stab {
                let lb = unsafe {
                    *g.get_unchecked(a * stride + b) ^ *g.get_unchecked(c * stride + b)
                };
                count_map[lb as usize] += 1;
                if lb == l_inf {
                    prefix_b_sum_linf += b as i64;
                }
            }

            let p = count_map[l_inf as usize];
            let sum_p = prefix_b_sum_linf;
            let h = h_target as i64;
            let bs = b_stab as i64;

            let mut s: i64 = 0;
            for v in 0..256u16 {
                if v as u8 != l_inf {
                    let cnt = count_map[v as usize];
                    s += cnt * cnt;
                }
            }

            s += p * p;
            s += 2 * (p * (h - bs + 1) - sum_p);
            let m = h - 2 * bs + 1;
            if m > 0 {
                s += m * (m + 1) / 2;
            }

            total_d += s;
        }
    }

    println!("{}", total_d);
}
