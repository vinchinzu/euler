// Project Euler 662 - Fibonacci Paths
// 2D DP on lattice with Fibonacci-length jumps. N=10000.
//
// Optimizations:
// - AVX2 4-way unrolled vectorized row accumulation for vertical and diagonal jumps
// - Monotonic jump pointer and Barrett reduction in the horizontal DP loop
// - Elimination of redundant bounds checks via unsafe raw pointers

#[target_feature(enable = "avx2")]
unsafe fn add_row_avx2(dst: *mut u32, src: *const u32, len: usize) {
    use std::arch::x86_64::*;
    unsafe {
        let v_mod = _mm256_set1_epi32(1_000_000_007);
        let v_mod_minus_1 = _mm256_set1_epi32(1_000_000_006);
        let mut i = 0;
        while i + 32 <= len {
            let a0 = _mm256_loadu_si256(dst.add(i) as *const __m256i);
            let a1 = _mm256_loadu_si256(dst.add(i + 8) as *const __m256i);
            let a2 = _mm256_loadu_si256(dst.add(i + 16) as *const __m256i);
            let a3 = _mm256_loadu_si256(dst.add(i + 24) as *const __m256i);

            let b0 = _mm256_loadu_si256(src.add(i) as *const __m256i);
            let b1 = _mm256_loadu_si256(src.add(i + 8) as *const __m256i);
            let b2 = _mm256_loadu_si256(src.add(i + 16) as *const __m256i);
            let b3 = _mm256_loadu_si256(src.add(i + 24) as *const __m256i);

            let s0 = _mm256_add_epi32(a0, b0);
            let s1 = _mm256_add_epi32(a1, b1);
            let s2 = _mm256_add_epi32(a2, b2);
            let s3 = _mm256_add_epi32(a3, b3);

            let sub0 = _mm256_sub_epi32(s0, v_mod);
            let sub1 = _mm256_sub_epi32(s1, v_mod);
            let sub2 = _mm256_sub_epi32(s2, v_mod);
            let sub3 = _mm256_sub_epi32(s3, v_mod);

            let mask0 = _mm256_cmpgt_epi32(s0, v_mod_minus_1);
            let mask1 = _mm256_cmpgt_epi32(s1, v_mod_minus_1);
            let mask2 = _mm256_cmpgt_epi32(s2, v_mod_minus_1);
            let mask3 = _mm256_cmpgt_epi32(s3, v_mod_minus_1);

            let res0 = _mm256_blendv_epi8(s0, sub0, mask0);
            let res1 = _mm256_blendv_epi8(s1, sub1, mask1);
            let res2 = _mm256_blendv_epi8(s2, sub2, mask2);
            let res3 = _mm256_blendv_epi8(s3, sub3, mask3);

            _mm256_storeu_si256(dst.add(i) as *mut __m256i, res0);
            _mm256_storeu_si256(dst.add(i + 8) as *mut __m256i, res1);
            _mm256_storeu_si256(dst.add(i + 16) as *mut __m256i, res2);
            _mm256_storeu_si256(dst.add(i + 24) as *mut __m256i, res3);

            i += 32;
        }
        while i < len {
            let v = *dst.add(i) + *src.add(i);
            *dst.add(i) = if v >= 1_000_000_007 { v - 1_000_000_007 } else { v };
            i += 1;
        }
    }
}

fn main() {
    let n = 10000usize;
    let max_fib_limit = ((2.0 * (n as f64) * (n as f64)).sqrt() as usize) + 2;
    let mut fibs = Vec::new();
    let (mut fa, mut fb) = (1usize, 1usize);
    while fa <= max_fib_limit {
        fibs.push(fa);
        let tmp = fa + fb; fa = fb; fb = tmp;
    }
    let mut h_jumps = Vec::new();
    let mut v_jumps = Vec::new();
    let mut diag_jumps: Vec<(usize, usize)> = Vec::new();
    let mut h_seen = vec![false; max_fib_limit + 2];
    let mut v_seen = vec![false; max_fib_limit + 2];
    for &f in &fibs {
        for dx in 0..=f.min(n) {
            let dy2 = f * f - dx * dx;
            let dy = (dy2 as f64).sqrt() as usize;
            for ddy_try in dy.saturating_sub(1)..=dy+1 {
                if ddy_try * ddy_try == dy2 && ddy_try <= n {
                    if dx > 0 && ddy_try == 0 {
                        if !h_seen[dx] { h_seen[dx] = true; h_jumps.push(dx); }
                    } else if dx == 0 && ddy_try > 0 {
                        if !v_seen[ddy_try] { v_seen[ddy_try] = true; v_jumps.push(ddy_try); }
                    } else if dx > 0 && ddy_try > 0 {
                        diag_jumps.push((dx, ddy_try));
                    }
                }
            }
        }
    }
    let w = n + 1;
    let mut dp = vec![0u32; w * w];
    dp[0] = 1;

    let dp_ptr = dp.as_mut_ptr();
    const H_JUMPS: [usize; 19] = [1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765];

    for y in 0..w {
        let dst_row = y * w;
        let dst = unsafe { dp_ptr.add(dst_row) };

        for &vi in &v_jumps {
            if y < vi { continue; }
            let src_row = (y - vi) * w;
            unsafe {
                let src = dp_ptr.add(src_row);
                add_row_avx2(dst, src, w);
            }
        }

        for &(dx, dyv) in &diag_jumps {
            if y < dyv { continue; }
            let src_row = (y - dyv) * w;
            unsafe {
                let src = dp_ptr.add(src_row);
                add_row_avx2(dst.add(dx), src, w - dx);
            }
        }

        let mut k = 0;
        for x in 1..w {
            while k < 19 && H_JUMPS[k] <= x {
                k += 1;
            }
            let mut sum = 0u64;
            for i in 0..k {
                sum += unsafe { *dst.add(x - H_JUMPS[i]) } as u64;
            }
            let cur = unsafe { *dst.add(x) } as u64 + sum;
            // Barrett reduction: m = 1_000_000_007
            let q = ((cur as u128 * 18446743944u128) >> 64) as u64;
            let mut rem = cur - q * 1_000_000_007;
            if rem >= 1_000_000_007 {
                rem -= 1_000_000_007;
            }
            unsafe { *dst.add(x) = rem as u32; }
        }
    }
    println!("{}", dp[n * w + n]);
}
