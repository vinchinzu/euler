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

    for p in 2..=sqrt_n {
        if s_small[p] == s_small[p - 1] {
            continue;
        }
        let p64 = p as i64;
        let p2 = p64 * p64;
        let sp = s_small[p - 1] as i64;
        let max_k = (n / p2).min(sqrt_n as i64) as usize;
        // k*p <= sqrt_n ⇔ k <= sqrt_n/p; then π((n/k)/p) = S_large[k*p]
        let split = (sqrt_n / p).min(max_k);
        for k in 1..=split {
            // SAFETY: k*p <= sqrt_n, k <= max_k <= sqrt_n
            unsafe {
                let sub = *s_large.get_unchecked(k * p);
                *s_large.get_unchecked_mut(k) -= sub - sp;
            }
        }
        for k in split + 1..=max_k {
            // SAFETY: k*p > sqrt_n ⇒ n/(k*p) < sqrt_n; k <= sqrt_n
            unsafe {
                let d = k as i64 * p64;
                let sub = *s_small.get_unchecked((n / d) as usize) as i64;
                *s_large.get_unchecked_mut(k) -= sub - sp;
            }
        }
        if p2 <= sqrt_n as i64 {
            let sp32 = s_small[p - 1];
            for v in (p2 as usize..=sqrt_n).rev() {
                // SAFETY: v <= sqrt_n, v/p < v
                unsafe {
                    *s_small.get_unchecked_mut(v) -= *s_small.get_unchecked(v / p) - sp32;
                }
            }
        }
    }

    // Primes <= sqrt_n from finished π table. q <= isqrt(n/2) < sqrt_n.
    let mut primes: Vec<u32> = Vec::with_capacity(80_000);
    for i in 2..=sqrt_n {
        if s_small[i] != s_small[i - 1] {
            primes.push(i as u32);
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
