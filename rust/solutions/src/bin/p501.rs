// Project Euler 501 - Eight Divisors
// Count integers <= N with exactly 8 divisors.
// Forms: p*q*r (3 distinct primes), p^3*q (p!=q), p^7.
// Lucy_Hedgehog prime counting (sequential); p*q*r / p^3*q over p in rayon.

use rayon::prelude::*;

const N: i64 = 1_000_000_000_000;

#[inline(always)]
fn isqrt(n: i64) -> i64 {
    let mut s = (n as f64).sqrt() as i64;
    while s * s > n {
        s -= 1;
    }
    while {
        let t = s + 1;
        t <= 3_037_000_499 && t * t <= n
    } {
        s += 1;
    }
    s
}

#[inline(always)]
unsafe fn sub_slice_i64(ptr: *mut i64, len: usize, delta: i64) {
    if delta == 0 {
        return;
    }
    unsafe {
        for i in 0..len {
            *ptr.add(i) -= delta;
        }
    }
}

#[inline(always)]
unsafe fn sub_slice_i32(ptr: *mut i32, len: usize, delta: i32) {
    if delta == 0 {
        return;
    }
    unsafe {
        for i in 0..len {
            *ptr.add(i) -= delta;
        }
    }
}

/// π(N/d) from completed Lucy tables.
#[inline(always)]
fn pi_div(n: i64, sqrt_n: usize, s_small: &[i32], s_large: &[i64], d: i64) -> i64 {
    if d <= sqrt_n as i64 {
        // SAFETY: 1 <= d <= sqrt_n
        unsafe { *s_large.get_unchecked(d as usize) }
    } else {
        // SAFETY: d > sqrt_n ⇒ N/d < sqrt_n
        unsafe { *s_small.get_unchecked((n / d) as usize) as i64 }
    }
}

fn main() {
    let n = N;
    let sqrt_n = isqrt(n) as usize;

    // Sieve primes up to sqrt_n
    let mut is_prime = vec![true; sqrt_n + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut i = 2;
    while i * i <= sqrt_n {
        if is_prime[i] {
            let mut j = i * i;
            while j <= sqrt_n {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    let primes: Vec<u32> = (2..=sqrt_n).filter(|&x| is_prime[x]).map(|x| x as u32).collect();

    // Lucy DP: S_small[v] = π(v) for v <= sqrt_n, S_large[k] = π(n/k).
    // Loop-carried in p — do not rayon.
    let mut s_small = vec![0i32; sqrt_n + 2];
    let mut s_large = vec![0i64; sqrt_n + 2];
    for i in 0..=sqrt_n {
        s_small[i] = i as i32 - 1;
    }
    for k in 1..=sqrt_n {
        s_large[k] = n / (k as i64) - 1;
    }

    let s_small_ptr = s_small.as_mut_ptr();
    let s_large_ptr = s_large.as_mut_ptr();

    for &p_u32 in &primes {
        let p = p_u32 as usize;
        let p64 = p_u32 as i64;
        let p2 = p64 * p64;
        let sp = unsafe { *s_small_ptr.add(p - 1) } as i64;
        let max_k = (n / p2).min(sqrt_n as i64) as usize;
        let lim_div_p = n / p64;
        let split = (sqrt_n / p).min(max_k);

        let mut kp = p;
        for k in 1..=split {
            unsafe {
                let sub = *s_large_ptr.add(kp);
                *s_large_ptr.add(k) -= sub - sp;
            }
            kp += p;
        }

        if lim_div_p <= u32::MAX as i64 {
            let m_u32 = lim_div_p as u32;
            let isqrt_m = (m_u32 as f64).sqrt() as usize;
            let mid_k = isqrt_m.min(max_k);
            let mut k = split + 1;

            while k <= mid_k {
                let v = (m_u32 / k as u32) as usize;
                let sub = unsafe { *s_small_ptr.add(v) } as i64;
                unsafe {
                    *s_large_ptr.add(k) -= sub - sp;
                }
                k += 1;
            }

            if k <= max_k {
                let v_start = (m_u32 / k as u32) as usize;
                let v_end = ((m_u32 / max_k as u32) as usize).max(1);
                for v in (v_end..=v_start).rev() {
                    let k_end = ((m_u32 / v as u32) as usize).min(max_k);
                    if k > k_end {
                        continue;
                    }
                    let sub = unsafe { *s_small_ptr.add(v) } as i64;
                    let delta = sub - sp;
                    let len = k_end - k + 1;
                    unsafe {
                        sub_slice_i64(s_large_ptr.add(k), len, delta);
                    }
                    k = k_end + 1;
                }
            }
        } else {
            let isqrt_m = (lim_div_p as f64).sqrt() as usize;
            let mid_k = isqrt_m.min(max_k);
            let mut k = split + 1;

            while k <= mid_k {
                let v = (lim_div_p / k as i64) as usize;
                let sub = unsafe { *s_small_ptr.add(v) } as i64;
                unsafe {
                    *s_large_ptr.add(k) -= sub - sp;
                }
                k += 1;
            }

            if k <= max_k {
                let v_start = (lim_div_p / k as i64) as usize;
                let v_end = ((lim_div_p / max_k as i64) as usize).max(1);
                for v in (v_end..=v_start).rev() {
                    let k_end = ((lim_div_p / v as i64) as usize).min(max_k);
                    if k > k_end {
                        continue;
                    }
                    let sub = unsafe { *s_small_ptr.add(v) } as i64;
                    let delta = sub - sp;
                    let len = k_end - k + 1;
                    unsafe {
                        sub_slice_i64(s_large_ptr.add(k), len, delta);
                    }
                    k = k_end + 1;
                }
            }
        }

        if p2 <= sqrt_n as i64 {
            let sp32 = sp as i32;
            let max_qp = sqrt_n / p;
            let min_qp = p;
            for qp in (min_qp..=max_qp).rev() {
                let delta = unsafe { *s_small_ptr.add(qp) } - sp32;
                let start = qp * p;
                let end = (start + p - 1).min(sqrt_n);
                let len = end - start + 1;
                unsafe {
                    sub_slice_i32(s_small_ptr.add(start), len, delta);
                }
            }
        }
    }

    // p^3 <= n; i128 so a rayon pass over all primes cannot wrap i64 cubes.
    let n128 = n as i128;
    let p_end = primes.partition_point(|&p| {
        let p = p as i128;
        p * p * p <= n128
    });

    let ans_main: i64 = (0..p_end)
        .into_par_iter()
        .with_min_len(1)
        .map(|pi_idx| {
            let p = unsafe { *primes.get_unchecked(pi_idx) } as i64;
            let mut local = 0i64;

            // p^3 * q, q != p
            let p3 = (p as i128) * (p as i128) * (p as i128);
            local += pi_div(n, sqrt_n, &s_small, &s_large, p3 as i64);
            if p3 * (p as i128) <= n128 {
                local -= 1;
            }

            // p < q < r, p*q*r <= n
            let q_lim = isqrt(n / p);
            let q_end = primes.partition_point(|&q| q as i64 <= q_lim);
            for qi_idx in pi_idx + 1..q_end {
                let q = unsafe { *primes.get_unchecked(qi_idx) } as i64;
                let pq = p * q;
                local += pi_div(n, sqrt_n, &s_small, &s_large, pq) - (qi_idx as i64 + 1);
            }
            local
        })
        .sum();

    // p^7 <= n
    let mut ans = ans_main;
    for &p in &primes {
        let p = p as i128;
        if p.pow(7) > n128 {
            break;
        }
        ans += 1;
    }

    println!("{}", ans);
}
