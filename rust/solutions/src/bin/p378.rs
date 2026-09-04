// Project Euler 378 - Triangle Triples

use rayon::prelude::*;

const N: usize = 60_000_000;
const M: usize = N + 1; // d[1..=N+1] needed for T(n) = n(n+1)/2
const MOD: u128 = 1_000_000_000_000_000_000;
const CHUNK: usize = 1 << 16; // 128 KiB of u16
const NUM_CHUNKS: usize = 128;
const MAX_DT_CAP: usize = 24_576; // max_dt = 23,040 for N = 60,000,000

#[inline(always)]
fn bit_add(bit: &mut [i32], mut pos: usize, n: usize) {
    while pos <= n {
        // SAFETY: pos in 1..=n and bit.len() >= n + 1
        unsafe {
            *bit.get_unchecked_mut(pos) += 1;
        }
        pos += pos & pos.wrapping_neg();
    }
}

#[inline(always)]
fn bit_query(bit: &[i32], mut pos: usize) -> i32 {
    let mut s = 0i32;
    while pos > 0 {
        // SAFETY: pos starts <= n and strictly decreases to 0
        unsafe {
            s += *bit.get_unchecked(pos);
        }
        pos &= pos - 1;
    }
    s
}

fn isqrt(n: usize) -> usize {
    let mut s = (n as f64).sqrt() as usize;
    while s * s > n {
        s -= 1;
    }
    while s + 1 <= n / (s + 1) {
        s += 1;
    }
    s
}

fn main() {
    let sq = isqrt(M);
    let mut d = vec![0u16; M + 1];

    // Segmented pair-divisor sieve: d[x] += 1 for squares, += 2 for i*j (i < j).
    d.par_chunks_mut(CHUNK).enumerate().for_each(|(ci, chunk)| {
        let lo = ci * CHUNK;
        let hi = lo + chunk.len();
        let start = lo.max(1);
        let chunk_len = chunk.len();

        // 1. Squares in this chunk
        let mut i_sq = isqrt(start).max(1);
        if i_sq * i_sq < start {
            i_sq += 1;
        }
        let i_sq_end = isqrt(hi.saturating_sub(1));
        while i_sq <= i_sq_end {
            unsafe {
                *chunk.get_unchecked_mut(i_sq * i_sq - lo) += 1;
            }
            i_sq += 1;
        }

        // 2. Pair divisors: i * j with j >= i + 1
        // i = 1: touches every element, easily vectorized by LLVM
        if 2 < hi {
            let j_lo = if 2 >= start { 2 } else { start };
            let idx = j_lo - lo;
            if idx < chunk_len {
                for slot in &mut chunk[idx..] {
                    *slot += 2;
                }
            }
        }

        for i in 2..=sq {
            if i * (i + 1) >= hi {
                break;
            }
            let j_lo = if i * (i + 1) >= start {
                i + 1
            } else {
                (i + 1).max(start.div_ceil(i))
            };
            let mut idx = i * j_lo - lo;
            while idx < chunk_len {
                unsafe {
                    *chunk.get_unchecked_mut(idx) += 2;
                }
                idx += i;
            }
        }
    });

    let mut dt = vec![0u16; N + 1];
    let chunk_size = N.div_ceil(NUM_CHUNKS);
    let stride = MAX_DT_CAP;

    // Flat prefix table: pref has (NUM_CHUNKS + 1) rows of length `stride`.
    // We write chunk histograms directly into pref[(c + 1) * stride..] during dt pass.
    let mut pref = vec![0i32; (NUM_CHUNKS + 1) * stride];
    let d_slice = d.as_slice();

    let global_max = dt[1..]
        .par_chunks_mut(chunk_size)
        .zip(pref[stride..].par_chunks_exact_mut(stride))
        .enumerate()
        .map(|(c, (dt_chunk, hist))| {
            let base = 1 + c * chunk_size;
            let mut local_max = 0u16;

            for (o, slot) in dt_chunk.iter_mut().enumerate() {
                let i = base + o;
                let (a, b) = if i & 1 == 0 {
                    (i >> 1, i + 1)
                } else {
                    (i, (i + 1) >> 1)
                };
                let val =
                    unsafe { *d_slice.get_unchecked(a) as u32 * *d_slice.get_unchecked(b) as u32 }
                        as u16;
                *slot = val;
                if val > local_max {
                    local_max = val;
                }
                unsafe {
                    *hist.get_unchecked_mut(val as usize) += 1;
                }
            }
            local_max
        })
        .max()
        .unwrap_or(0) as usize;
    drop(d);

    let max_dt = global_max;
    let nbit = max_dt;

    // 1. Parallel prefix sum over values within each chunk's histogram
    pref[stride..].par_chunks_exact_mut(stride).for_each(|row| {
        for v in 1..=max_dt {
            unsafe {
                *row.get_unchecked_mut(v) += *row.get_unchecked(v - 1);
            }
        }
    });

    // 2. Sequential prefix sum across chunks: row[c + 1] += row[c]
    for c in 1..NUM_CHUNKS {
        let (prev_part, curr_part) = pref[c * stride..].split_at_mut(stride);
        let prev_row = &prev_part[..stride];
        let curr_row = &mut curr_part[..stride];
        for v in 0..=max_dt {
            unsafe {
                *curr_row.get_unchecked_mut(v) += *prev_row.get_unchecked(v);
            }
        }
    }

    // Process each chunk in parallel
    let pref_slice = pref.as_slice();
    let answer = ((0..NUM_CHUNKS)
        .into_par_iter()
        .map(|c| {
            let start = c * chunk_size + 1;
            let end = ((c + 1) * chunk_size).min(N);
            if start > end {
                return 0u128;
            }
            let n_chunk = end - start + 1;

            // Unzeroed allocation for right buffer: entirely overwritten in backward pass
            let mut right: Vec<i32> = Vec::with_capacity(n_chunk);
            unsafe {
                right.set_len(n_chunk);
            }

            // Fenwick tree of size nbit + 1 (1-based indexing)
            let mut bit = vec![0i32; nbit + 1];

            let pref_all =
                unsafe { pref_slice.get_unchecked(NUM_CHUNKS * stride..(NUM_CHUNKS + 1) * stride) };
            let pref_c1 = unsafe { pref_slice.get_unchecked((c + 1) * stride..(c + 2) * stride) };

            let dt_chunk = &dt[start..=end];

            // Backward pass: compute right[j]
            for (idx, &v_u16) in dt_chunk.iter().enumerate().rev() {
                let v = v_u16 as usize;
                let s_val =
                    unsafe { *pref_all.get_unchecked(v - 1) - *pref_c1.get_unchecked(v - 1) };
                let local = bit_query(&bit, v - 1);
                unsafe {
                    *right.get_unchecked_mut(idx) = s_val + local;
                }
                bit_add(&mut bit, v, nbit);
            }

            // Clear bit for forward pass
            bit.fill(0);

            // Forward pass: compute left[j] and accumulate answer
            let pref_c = unsafe { pref_slice.get_unchecked(c * stride..(c + 1) * stride) };
            let mut chunk_ans = 0u128;
            let mut j_minus_1 = start - 1;
            for (&v_u16, &r_val) in dt_chunk.iter().zip(right.iter()) {
                let v = v_u16 as usize;
                let p_val = unsafe { *pref_c.get_unchecked(v) };
                let local_le = bit_query(&bit, v);
                let count_le = (p_val + local_le) as usize;
                let left = j_minus_1 - count_le;
                chunk_ans += (left as u64 * r_val as u64) as u128;
                bit_add(&mut bit, v, nbit);
                j_minus_1 += 1;
            }

            chunk_ans % MOD
        })
        .sum::<u128>()
        % MOD) as i64;

    println!("{}", answer);
}
