// Project Euler 662 - Fibonacci Paths
// 2D DP on lattice with Fibonacci-length jumps. N=10000.
//
// Optimizations:
// - Row-aligned stride (10016 elements) ensuring 64-byte cache line alignment for all rows
// - Parallel worker threads with spin-barrier synchronizing row updates without thread respawning
// - Disjoint column chunking across threads for conflict-free AVX2 4-way vectorized row accumulation
// - Statically unrolled horizontal DP recurrence with dedicated step functions per jump range

use std::sync::atomic::{AtomicUsize, Ordering};

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

#[inline(always)]
unsafe fn step_full(p: *mut u32, x: usize, prev: &mut u32) {
    unsafe {
        let sum = *p.add(x) as u64
            + *p.add(x - 2) as u64
            + *p.add(x - 3) as u64
            + *p.add(x - 5) as u64
            + *p.add(x - 8) as u64
            + *p.add(x - 13) as u64
            + *p.add(x - 21) as u64
            + *p.add(x - 34) as u64
            + *p.add(x - 55) as u64
            + *p.add(x - 89) as u64
            + *p.add(x - 144) as u64
            + *p.add(x - 233) as u64
            + *p.add(x - 377) as u64
            + *p.add(x - 610) as u64
            + *p.add(x - 987) as u64
            + *p.add(x - 1597) as u64
            + *p.add(x - 2584) as u64
            + *p.add(x - 4181) as u64
            + *p.add(x - 6765) as u64;
        let cur = sum + *prev as u64;
        let q = ((cur as u128 * 18446743944u128) >> 64) as u64;
        let mut rem = (cur - q * 1_000_000_007) as u32;
        if rem >= 1_000_000_007 {
            rem -= 1_000_000_007;
        }
        *p.add(x) = rem;
        *prev = rem;
    }
}

#[inline(always)]
unsafe fn horizontal_step(p: *mut u32, x: usize, jumps: &[usize], prev: &mut u32) {
    unsafe {
        let mut sum = *p.add(x) as u64;
        for &j in jumps {
            sum += *p.add(x - j) as u64;
        }
        let cur = sum + *prev as u64;
        let q = ((cur as u128 * 18446743944u128) >> 64) as u64;
        let mut rem = (cur - q * 1_000_000_007) as u32;
        if rem >= 1_000_000_007 {
            rem -= 1_000_000_007;
        }
        *p.add(x) = rem;
        *prev = rem;
    }
}

#[inline(always)]
unsafe fn run_horizontal_dp(dst: *mut u32, w: usize) {
    unsafe {
        let mut prev = *dst;
        for x in 1..2 { horizontal_step(dst, x, &[], &mut prev); }
        for x in 2..3 { horizontal_step(dst, x, &[2], &mut prev); }
        for x in 3..5 { horizontal_step(dst, x, &[2, 3], &mut prev); }
        for x in 5..8 { horizontal_step(dst, x, &[2, 3, 5], &mut prev); }
        for x in 8..13 { horizontal_step(dst, x, &[2, 3, 5, 8], &mut prev); }
        for x in 13..21 { horizontal_step(dst, x, &[2, 3, 5, 8, 13], &mut prev); }
        for x in 21..34 { horizontal_step(dst, x, &[2, 3, 5, 8, 13, 21], &mut prev); }
        for x in 34..55 { horizontal_step(dst, x, &[2, 3, 5, 8, 13, 21, 34], &mut prev); }
        for x in 55..89 { horizontal_step(dst, x, &[2, 3, 5, 8, 13, 21, 34, 55], &mut prev); }
        for x in 89..144 { horizontal_step(dst, x, &[2, 3, 5, 8, 13, 21, 34, 55, 89], &mut prev); }
        for x in 144..233 { horizontal_step(dst, x, &[2, 3, 5, 8, 13, 21, 34, 55, 89, 144], &mut prev); }
        for x in 233..377 { horizontal_step(dst, x, &[2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233], &mut prev); }
        for x in 377..610 { horizontal_step(dst, x, &[2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377], &mut prev); }
        for x in 610..987 { horizontal_step(dst, x, &[2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610], &mut prev); }
        for x in 987..1597 { horizontal_step(dst, x, &[2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987], &mut prev); }
        for x in 1597..2584 { horizontal_step(dst, x, &[2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597], &mut prev); }
        for x in 2584..4181 { horizontal_step(dst, x, &[2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584], &mut prev); }
        for x in 4181..6765 { horizontal_step(dst, x, &[2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181], &mut prev); }
        for x in 6765..w { step_full(dst, x, &mut prev); }
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
    let stride = ((w + 31) / 32) * 32;
    let mut dp = vec![0u32; stride * w];
    dp[0] = 1;

    let num_threads = 8usize;
    let raw_chunk = (w + num_threads - 1) / num_threads;
    let chunk_size = ((raw_chunk + 31) / 32) * 32;

    let mut ranges = Vec::new();
    for t in 0..num_threads {
        let x0 = (t * chunk_size).min(w);
        let x1 = ((t + 1) * chunk_size).min(w);
        if x0 < x1 {
            ranges.push((x0, x1));
        }
    }
    let actual_threads = ranges.len();

    let round = AtomicUsize::new(0);
    let done = AtomicUsize::new(0);

    let dp_ptr_usize = dp.as_mut_ptr() as usize;

    std::thread::scope(|s| {
        for &(x0, x1) in &ranges {
            let round = &round;
            let done = &done;
            let v_jumps = &v_jumps;
            let diag_jumps = &diag_jumps;

            s.spawn(move || {
                let dp_ptr = dp_ptr_usize as *mut u32;
                for y in 1..w {
                    while round.load(Ordering::Acquire) < y {
                        std::hint::spin_loop();
                    }

                    let dst_row = y * stride;
                    let dst = unsafe { dp_ptr.add(dst_row + x0) };
                    let len = x1 - x0;

                    for &vi in v_jumps.iter() {
                        if y < vi { continue; }
                        let src = unsafe { dp_ptr.add((y - vi) * stride + x0) };
                        unsafe { add_row_avx2(dst, src, len); }
                    }

                    for &(dx, dyv) in diag_jumps.iter() {
                        if y < dyv { continue; }
                        if x1 <= dx { continue; }
                        let start = x0.max(dx);
                        let sub_len = x1 - start;
                        let dst_diag = unsafe { dp_ptr.add(dst_row + start) };
                        let src_diag = unsafe { dp_ptr.add((y - dyv) * stride + (start - dx)) };
                        unsafe { add_row_avx2(dst_diag, src_diag, sub_len); }
                    }

                    done.fetch_add(1, Ordering::Release);
                }
            });
        }

        // Row 0: horizontal DP
        let dp_ptr = dp_ptr_usize as *mut u32;
        unsafe { run_horizontal_dp(dp_ptr, w); }

        for y in 1..w {
            round.store(y, Ordering::Release);

            while done.load(Ordering::Acquire) < y * actual_threads {
                std::hint::spin_loop();
            }

            let dst_row = y * stride;
            let dst = unsafe { dp_ptr.add(dst_row) };
            unsafe { run_horizontal_dp(dst, w); }
        }
    });

    println!("{}", dp[n * stride + n]);
}
