// Project Euler 470: Super Ramvok
//
// Game theory problem with subset enumeration and tridiagonal system solving.

use rayon::prelude::*;

const N: usize = 20;
const FULL: usize = 1 << N;

#[inline(always)]
fn r_func(subset: usize, c: f64) -> f64 {
    if c == 0.0 {
        return (usize::BITS - subset.leading_zeros()) as f64;
    }

    let mut vals = [0.0f64; N];
    let mut bits = subset;
    let mut cnt = 0usize;
    let mut sum = 0.0;
    while bits != 0 {
        let v = (bits.trailing_zeros() + 1) as f64;
        // SAFETY: cnt < popcount(subset) <= N
        unsafe {
            *vals.get_unchecked_mut(cnt) = v;
        }
        sum += v;
        cnt += 1;
        bits &= bits - 1;
    }

    let inv_cnt = 1.0 / cnt as f64;
    let mut suffix_sum = sum;
    let mut k = 0usize;
    let mut best = 0.0;
    let mut ct = c;
    loop {
        let mean = sum * inv_cnt;
        let expected = mean - ct;
        if expected < best {
            return best;
        }
        best = expected;
        ct += c;

        // vals stays the original sorted faces; k is how many have been
        // raised to the running mean. Equivalent to clamping a prefix.
        while k < cnt {
            // SAFETY: k < cnt <= N
            let v = unsafe { *vals.get_unchecked(k) };
            if v >= mean {
                break;
            }
            suffix_sum -= v;
            k += 1;
        }
        sum = mean.mul_add(k as f64, suffix_sum);
    }
}

fn tridiag_last(a: &[f64; N + 1], c: &[f64; N + 1], d: &[f64; N + 1], n: usize) -> f64 {
    let mut c_prime = [0.0f64; N + 1];
    let mut d_prime = [0.0f64; N + 1];
    // Diagonal b[i] = 1 for all i.
    c_prime[0] = c[0];
    d_prime[0] = d[0];
    for i in 1..n {
        // SAFETY: i < n <= N+1, i-1 >= 0
        let denom = unsafe { 1.0 - a.get_unchecked(i) * c_prime.get_unchecked(i - 1) };
        if i < n - 1 {
            unsafe {
                *c_prime.get_unchecked_mut(i) = c.get_unchecked(i) / denom;
            }
        }
        unsafe {
            *d_prime.get_unchecked_mut(i) =
                (d.get_unchecked(i) - a.get_unchecked(i) * d_prime.get_unchecked(i - 1)) / denom;
        }
    }
    // x[n-1] = d_prime[n-1]; back-substitution is not needed.
    d_prime[n - 1]
}

fn process_c(c_val: usize) -> f64 {
    let c = c_val as f64;
    let mut r_cache = vec![0.0f64; FULL];

    const CHUNK: usize = 4096;
    r_cache.par_chunks_mut(CHUNK).enumerate().for_each(|(ci, chunk)| {
        let base = ci * CHUNK;
        for (off, slot) in chunk.iter_mut().enumerate() {
            let subset = base + off;
            if subset != 0 {
                *slot = r_func(subset, c);
            }
        }
    });

    let mut local = 0.0;
    for d in 4..=N {
        let n = d + 1;
        let mut a = [0.0f64; N + 1];
        let mut c_arr = [0.0f64; N + 1];
        let mut d_arr = [0.0f64; N + 1];
        let inv_d = 1.0 / d as f64;
        for i in 1..=d {
            a[i] = -((d - i + 1) as f64) * inv_d;
        }
        for i in 1..d {
            c_arr[i] = -((i + 1) as f64) * inv_d;
        }
        for subset in 1..(1usize << d) {
            let pc = subset.count_ones() as usize;
            // SAFETY: 1 <= pc <= d <= N; subset < 1<<d <= FULL
            unsafe {
                *d_arr.get_unchecked_mut(pc) += *r_cache.get_unchecked(subset);
            }
        }
        local += tridiag_last(&a, &c_arr, &d_arr, n);
    }
    local
}

fn main() {
    let ans: f64 = (0..N + 1)
        .into_par_iter()
        .with_min_len(1)
        .map(process_c)
        .sum();
    println!("{}", ans.round() as i64);
}
