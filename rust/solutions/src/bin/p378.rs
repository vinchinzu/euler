// Project Euler 378 - Triangle Triples

use rayon::prelude::*;

const N: usize = 60_000_000;
const M: usize = N + 1; // d[1..=N+1] needed for T(n) = n(n+1)/2
const MOD: i64 = 1_000_000_000_000_000_000;
const CHUNK: usize = 1 << 18; // 512 KiB of u16, one L2 per core

#[inline(always)]
fn bit_add(bit: &mut [i32], mut pos: usize, n: usize) {
    pos += 1;
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
    pos += 1;
    let mut s = 0i32;
    while pos > 0 {
        // SAFETY: pos starts at the 1-based index and strictly decreases
        unsafe {
            s += *bit.get_unchecked(pos);
        }
        pos -= pos & pos.wrapping_neg();
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

fn fill_right(dt: &[u16], right: &mut [i32], nbit: usize) {
    let mut bit = vec![0i32; nbit + 1];
    for j in (1..=N).rev() {
        // SAFETY: j in 1..=N; dt and right have length N+1
        let v = unsafe { *dt.get_unchecked(j) } as usize;
        unsafe {
            *right.get_unchecked_mut(j) = bit_query(&bit, v - 1);
        }
        bit_add(&mut bit, v, nbit);
    }
}

fn fill_left(dt: &[u16], left: &mut [i32], nbit: usize) {
    let mut bit = vec![0i32; nbit + 1];
    for j in 1..=N {
        // SAFETY: j in 1..=N; dt and left have length N+1
        let v = unsafe { *dt.get_unchecked(j) } as usize;
        unsafe {
            *left.get_unchecked_mut(j) = j as i32 - 1 - bit_query(&bit, v);
        }
        bit_add(&mut bit, v, nbit);
    }
}

fn main() {
    let sq = isqrt(M);
    let mut d = vec![0u16; M + 1];

    // Segmented pair-divisor sieve: d[x] += 1 for squares, += 2 for i*j (i < j).
    d.par_chunks_mut(CHUNK).enumerate().for_each(|(ci, chunk)| {
        let lo = ci * CHUNK;
        let hi = lo + chunk.len();
        let start = lo.max(1);
        for i in 1..=sq {
            let sqi = i * i;
            if sqi >= start && sqi < hi {
                // SAFETY: start <= sqi < hi maps into this chunk
                unsafe {
                    *chunk.get_unchecked_mut(sqi - lo) += 1;
                }
            }
            let j_lo = (i + 1).max(start.div_ceil(i));
            let j_hi = (hi - 1) / i;
            if j_lo <= j_hi {
                let mut v = i * j_lo;
                let last = i * j_hi;
                while v <= last {
                    // SAFETY: start <= v < hi
                    unsafe {
                        *chunk.get_unchecked_mut(v - lo) += 2;
                    }
                    v += i;
                }
            }
        }
    });

    let mut dt = vec![0u16; N + 1];
    let max_dt = dt[1..]
        .par_chunks_mut(CHUNK)
        .enumerate()
        .map(|(ci, chunk)| {
            let base = 1 + ci * CHUNK;
            let mut local_max = 0u16;
            for (o, slot) in chunk.iter_mut().enumerate() {
                let i = base + o;
                let (a, b) = if i & 1 == 0 {
                    (i >> 1, i + 1)
                } else {
                    (i, (i + 1) >> 1)
                };
                // SAFETY: 1 <= a,b <= N+1; d.len() = N+2
                let val = unsafe {
                    *d.get_unchecked(a) as u32 * *d.get_unchecked(b) as u32
                } as u16;
                *slot = val;
                if val > local_max {
                    local_max = val;
                }
            }
            local_max
        })
        .max()
        .unwrap_or(0) as usize;
    drop(d);

    let nbit = max_dt + 1;
    let mut right_arr = vec![0i32; N + 1];
    let mut left_arr = vec![0i32; N + 1];
    {
        let dt_s = dt.as_slice();
        let right_s = right_arr.as_mut_slice();
        let left_s = left_arr.as_mut_slice();
        // Two independent sequential Fenwick passes (loop-carried within each).
        rayon::join(
            || fill_right(dt_s, right_s, nbit),
            || fill_left(dt_s, left_s, nbit),
        );
    }
    drop(dt);

    let mut answer = 0i64;
    for j in 1..=N {
        // SAFETY: left_arr and right_arr have length N+1; counts fit in i32
        let prod = unsafe {
            *left_arr.get_unchecked(j) as i64 * *right_arr.get_unchecked(j) as i64
        };
        answer += prod;
        if answer >= MOD {
            answer -= MOD;
        }
    }

    println!("{}", answer);
}
