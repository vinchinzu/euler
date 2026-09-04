// Project Euler 782 - Distinct Rows and Columns
// C(n) = 3n^2 - 1 - N2 + N4 via bitarray sieve for achievability.

use rayon::prelude::*;

fn main() {
    let n: i64 = 10000;
    let big_n = n * n;
    let big_n_u = big_n as usize;

    // Plain u64 bitset: 100M bits ≈ 12.5 MB
    let nwords = (big_n_u + 64) / 64;
    let mut achievable = vec![0u64; nwords];

    #[inline(always)]
    fn bit_set(ach: &mut [u64], k: usize) {
        let word = k >> 6;
        let bit = 1u64 << (k & 63);
        unsafe {
            *ach.get_unchecked_mut(word) |= bit;
        }
    }

    bit_set(&mut achievable, 0);
    bit_set(&mut achievable, big_n_u);

    // S2: comp=2 values from 2x2 block matrices
    let mut s2 = Vec::with_capacity(40000);
    for c in 1..n {
        let v = (c * c) as usize;
        s2.push(v);
        s2.push(big_n_u - v);
    }
    for x in 1..n {
        let y = n - x;
        let v1 = (x * x + y * y) as usize;
        let v2 = (2 * x * y) as usize;
        s2.push(v1);
        s2.push(v2);
    }
    s2.sort_unstable();
    s2.dedup();
    let n2 = s2.len() as i64;
    for &k in &s2 {
        bit_set(&mut achievable, k);
    }

    // Construction 1: Products d*m with 1 <= d,m <= n-1 (chunked parallel sieve)
    const CHUNK_WORDS: usize = 8192; // 64 KB cache-resident chunk
    let n_u = n as usize;
    achievable
        .par_chunks_mut(CHUNK_WORDS)
        .enumerate()
        .for_each(|(chunk_idx, chunk_slice)| {
            let word_start = chunk_idx * CHUNK_WORDS;
            let word_end = (word_start + chunk_slice.len()).min(nwords);
            let k_lo = word_start * 64;
            let k_hi = (word_end * 64).min(big_n_u);

            let d_min = if n_u > 1 { ((k_lo + n_u - 2) / (n_u - 1)).max(1) } else { 1 };
            let d_max = (n_u - 1).min(if k_hi > 0 { k_hi - 1 } else { 0 });

            for d in d_min..=d_max {
                let first_m = k_lo.div_ceil(d).max(1);
                let mut k = first_m * d;
                let limit = k_hi.min(d * n_u);
                while k < limit {
                    let rel_word = (k >> 6) - word_start;
                    unsafe {
                        *chunk_slice.get_unchecked_mut(rel_word) |= 1u64 << (k & 63);
                    }
                    k += d;
                }
            }
        });

    // Construction 3: Kernel 3x3 matrices (6 canonical non-redundant forms)
    let forms: [[i64; 6]; 6] = [
        [1, -1, 0, -2 * n, 0, big_n],
        [1, 2, 2, -2 * n, -2 * n, big_n],
        [-2, 1, -2, 2 * n, 0, 0],
        [-1, 2, 0, 0, -2 * n, big_n],
        [-1, -1, -1, n, n, 0],
        [-2, -2, -2, 2 * n, 2 * n, 0],
    ];

    let partials: Vec<Vec<u64>> = forms
        .into_par_iter()
        .map(|f| {
            let mut local = vec![0u64; nwords];
            let aa = f[0];
            let bb = f[1];
            let ab = f[2];
            let a1 = f[3];
            let b1 = f[4];
            let c0 = f[5];
            let two_bb = 2 * bb;
            let limit = (big_n as u64) - 1;

            for a in 0..=n {
                let b_max = n - a;
                let base = aa * a * a + a1 * a + c0;
                let lin = ab * a + b1;
                let mut k = base;
                let mut delta = bb + lin;

                if (k as u64).wrapping_sub(1) < limit {
                    bit_set(&mut local, k as usize);
                }

                let mut b = 1;
                while b + 3 <= b_max {
                    k += delta;
                    delta += two_bb;
                    if (k as u64).wrapping_sub(1) < limit {
                        bit_set(&mut local, k as usize);
                    }
                    k += delta;
                    delta += two_bb;
                    if (k as u64).wrapping_sub(1) < limit {
                        bit_set(&mut local, k as usize);
                    }
                    k += delta;
                    delta += two_bb;
                    if (k as u64).wrapping_sub(1) < limit {
                        bit_set(&mut local, k as usize);
                    }
                    k += delta;
                    delta += two_bb;
                    if (k as u64).wrapping_sub(1) < limit {
                        bit_set(&mut local, k as usize);
                    }
                    b += 4;
                }
                while b <= b_max {
                    k += delta;
                    delta += two_bb;
                    if (k as u64).wrapping_sub(1) < limit {
                        bit_set(&mut local, k as usize);
                    }
                    b += 1;
                }
            }
            local
        })
        .collect();

    achievable
        .par_iter_mut()
        .enumerate()
        .for_each(|(w, val)| {
            let mut bits = *val;
            for local in &partials {
                bits |= local[w];
            }
            *val = bits;
        });
    drop(partials);

    // Construction 2: Complement symmetry via word-level bit reversal
    let w_max = big_n_u / 64;
    let reversed: Vec<u64> = (0..nwords)
        .into_par_iter()
        .map(|w| {
            if w < w_max {
                let big_w = w_max - 1 - w;
                (achievable[big_w].reverse_bits() << 1) | (achievable[big_w + 1] & 1)
            } else {
                achievable[0] & 1
            }
        })
        .collect();

    achievable
        .par_iter_mut()
        .zip(reversed.into_par_iter())
        .for_each(|(a, r)| *a |= r);

    // Count N4: non-achievable values in [1, big_n-1]
    let n4 = (!achievable[0] & !1u64).count_ones() as i64
        + achievable[1..w_max]
            .par_iter()
            .map(|&w| (!w).count_ones() as i64)
            .sum::<i64>();

    println!("{}", 3 * big_n - 1 - n2 + n4);
}
