// Project Euler 154: Exploring Pascal's pyramid
use rayon::prelude::*;

const N_VAL: usize = 200000;
const K: i32 = 12;

fn main() {
    let mut f = vec![0i32; N_VAL + 1]; // v5(i!) cumulative
    let mut t = vec![0i32; N_VAL + 1]; // v2(i!) cumulative
    for i in 1..=N_VAL {
        let mut v5 = 0;
        let mut v2 = 0;
        let mut n = i;
        while n % 5 == 0 {
            v5 += 1;
            n /= 5;
        }
        n = i;
        while n % 2 == 0 {
            v2 += 1;
            n /= 2;
        }
        f[i] = f[i - 1] + v5;
        t[i] = t[i - 1] + v2;
    }

    // fr[k] = f[N-k] so f[N-a-b] = fr[a+b] (sequential with b)
    let mut fr = vec![0i32; N_VAL + 1];
    let mut tr = vec![0i32; N_VAL + 1];
    for i in 0..=N_VAL {
        fr[i] = f[N_VAL - i];
        tr[i] = t[N_VAL - i];
    }

    let fn_val = f[N_VAL];
    let tn_val = t[N_VAL];
    let a_max = N_VAL / 3;
    let nthreads = rayon::current_num_threads().max(1);
    let avx2 = cfg!(target_arch = "x86_64") && is_x86_feature_detected!("avx2");

    // Stripe `a` across threads: work per a is O(N-3a), so a contiguous
    // par_iter split is ~1000× imbalanced. Stripes mix heavy and light a.
    let ans: i64 = (0..nthreads)
        .into_par_iter()
        .map(|tid| {
            let mut local: i64 = 0;
            let mut a = tid;
            while a <= a_max {
                local += process_a(a, &f, &t, &fr, &tr, fn_val, tn_val, avx2);
                a += nthreads;
            }
            local
        })
        .sum();

    println!("{ans}");
}

fn process_a(
    a: usize,
    f: &[i32],
    t: &[i32],
    fr: &[i32],
    tr: &[i32],
    fn_val: i32,
    tn_val: i32,
    avx2: bool,
) -> i64 {
    // SAFETY: a <= N/3 < N, arrays have length N+1
    let temp_f = unsafe { *f.get_unchecked(a) } + K - fn_val;
    let temp_t = unsafe { *t.get_unchecked(a) } + K - tn_val;

    let mut local: i64 = 0;

    let b_lo = a + 1;
    let b_hi = (N_VAL - a - 1) / 2;
    if b_lo <= b_hi {
        let length = b_hi - b_lo + 1;
        let ab_lo = a + b_lo;
        // SAFETY: b in [b_lo, b_hi] ⊆ [0, N/2], a+b in [2a+1, (N+a-1)/2] ⊆ [1, N]
        unsafe {
            let fb = f.as_ptr().add(b_lo);
            let tb = t.as_ptr().add(b_lo);
            let fc = fr.as_ptr().add(ab_lo);
            let tc = tr.as_ptr().add(ab_lo);
            let hits = if avx2 {
                count_hits_avx2(fb, fc, tb, tc, length, temp_f, temp_t)
            } else {
                count_hits_scalar(fb, fc, tb, tc, length, temp_f, temp_t)
            };
            local += hits * 6;
        }
    }

    // Case a == b: c = N - 2a
    {
        let c = N_VAL - 2 * a;
        if c > a {
            // SAFETY: a <= N/3, c = N-2a in (a, N]
            let d5 = unsafe { *f.get_unchecked(a) + *f.get_unchecked(c) } + temp_f;
            let d2 = unsafe { *t.get_unchecked(a) + *t.get_unchecked(c) } + temp_t;
            if d5 <= 0 && d2 <= 0 {
                local += 3;
            }
        }
    }

    // Case b == c: b = c = (N-a)/2
    if (N_VAL - a) % 2 == 0 {
        let half = (N_VAL - a) / 2;
        if half > a {
            // SAFETY: half = (N-a)/2 <= N/2
            let d5 = 2 * unsafe { *f.get_unchecked(half) } + temp_f;
            let d2 = 2 * unsafe { *t.get_unchecked(half) } + temp_t;
            if d5 <= 0 && d2 <= 0 {
                local += 3;
            }
        }
    }

    local
}

/// SAFETY: fb, fc, tb, tc each valid for `len` i32s.
unsafe fn count_hits_scalar(
    fb: *const i32,
    fc: *const i32,
    tb: *const i32,
    tc: *const i32,
    len: usize,
    temp_f: i32,
    temp_t: i32,
) -> i64 {
    let mut hits = 0i64;
    for i in 0..len {
        let d5 = unsafe { *fb.add(i) + *fc.add(i) } + temp_f;
        let d2 = unsafe { *tb.add(i) + *tc.add(i) } + temp_t;
        hits += (d5 <= 0 && d2 <= 0) as i64;
    }
    hits
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn count_hits_avx2(
    fb: *const i32,
    fc: *const i32,
    tb: *const i32,
    tc: *const i32,
    len: usize,
    temp_f: i32,
    temp_t: i32,
) -> i64 {
    use std::arch::x86_64::*;
    // SAFETY: i+16 <= len (or i+8 <= len) so 8-lane unaligned loads stay in-bounds.
    unsafe {
        let vf = _mm256_set1_epi32(temp_f);
        let vt = _mm256_set1_epi32(temp_t);
        let one = _mm256_set1_epi32(1);
        let mut counts = _mm256_setzero_si256();
        let mut i = 0usize;

        while i + 16 <= len {
            let d5_0 = _mm256_add_epi32(
                _mm256_add_epi32(
                    _mm256_loadu_si256(fb.add(i) as *const __m256i),
                    _mm256_loadu_si256(fc.add(i) as *const __m256i),
                ),
                vf,
            );
            let d2_0 = _mm256_add_epi32(
                _mm256_add_epi32(
                    _mm256_loadu_si256(tb.add(i) as *const __m256i),
                    _mm256_loadu_si256(tc.add(i) as *const __m256i),
                ),
                vt,
            );
            counts = _mm256_sub_epi32(
                counts,
                _mm256_and_si256(_mm256_cmpgt_epi32(one, d5_0), _mm256_cmpgt_epi32(one, d2_0)),
            );

            let d5_1 = _mm256_add_epi32(
                _mm256_add_epi32(
                    _mm256_loadu_si256(fb.add(i + 8) as *const __m256i),
                    _mm256_loadu_si256(fc.add(i + 8) as *const __m256i),
                ),
                vf,
            );
            let d2_1 = _mm256_add_epi32(
                _mm256_add_epi32(
                    _mm256_loadu_si256(tb.add(i + 8) as *const __m256i),
                    _mm256_loadu_si256(tc.add(i + 8) as *const __m256i),
                ),
                vt,
            );
            counts = _mm256_sub_epi32(
                counts,
                _mm256_and_si256(_mm256_cmpgt_epi32(one, d5_1), _mm256_cmpgt_epi32(one, d2_1)),
            );
            i += 16;
        }
        while i + 8 <= len {
            let d5 = _mm256_add_epi32(
                _mm256_add_epi32(
                    _mm256_loadu_si256(fb.add(i) as *const __m256i),
                    _mm256_loadu_si256(fc.add(i) as *const __m256i),
                ),
                vf,
            );
            let d2 = _mm256_add_epi32(
                _mm256_add_epi32(
                    _mm256_loadu_si256(tb.add(i) as *const __m256i),
                    _mm256_loadu_si256(tc.add(i) as *const __m256i),
                ),
                vt,
            );
            counts = _mm256_sub_epi32(
                counts,
                _mm256_and_si256(_mm256_cmpgt_epi32(one, d5), _mm256_cmpgt_epi32(one, d2)),
            );
            i += 8;
        }

        let mut buf = [0i32; 8];
        _mm256_storeu_si256(buf.as_mut_ptr() as *mut __m256i, counts);
        let mut hits =
            i64::from(buf[0] + buf[1] + buf[2] + buf[3] + buf[4] + buf[5] + buf[6] + buf[7]);
        while i < len {
            let d5 = *fb.add(i) + *fc.add(i) + temp_f;
            let d2 = *tb.add(i) + *tc.add(i) + temp_t;
            hits += (d5 <= 0 && d2 <= 0) as i64;
            i += 1;
        }
        hits
    }
}

#[cfg(not(target_arch = "x86_64"))]
unsafe fn count_hits_avx2(
    fb: *const i32,
    fc: *const i32,
    tb: *const i32,
    tc: *const i32,
    len: usize,
    temp_f: i32,
    temp_t: i32,
) -> i64 {
    count_hits_scalar(fb, fc, tb, tc, len, temp_f, temp_t)
}
