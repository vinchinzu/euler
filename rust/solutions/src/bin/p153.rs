// Project Euler 153 - Investigating Gaussian Integers
// Sum of all divisors (including Gaussian) of all n from 1 to 10^8.

use rayon::prelude::*;

const N: i64 = 100_000_000;

#[inline(always)]
fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

/// G(n) = sum_{k=1}^{n} sigma_1(k) = sum_{d=1}^{n} d * floor(n/d)
fn g_function(n: i64) -> i64 {
    if n <= 0 {
        return 0;
    }
    let mut result: i64 = 0;
    let mut k: i64 = 1;
    while k <= n {
        let q = n / k;
        let next_k = n / q + 1;
        let last_k = if next_k - 1 < n { next_k - 1 } else { n };
        let count = last_k - k + 1;
        let sum_k = count * (k + last_k) / 2;
        result += sum_k * q;
        k = next_k;
    }
    result
}

#[inline(always)]
fn g_n_div_d(d: i64, n: i64, l: i64, g_small: &[i64], g_large: &[i64]) -> i64 {
    if d <= l {
        // SAFETY: 1 <= d <= l, g_large.len() == l+1
        unsafe { *g_large.get_unchecked(d as usize) }
    } else {
        // SAFETY: d > l => n/d <= n/(l+1) < l for N = l^2
        unsafe { *g_small.get_unchecked((n / d) as usize) }
    }
}

fn process_u(u: i64, n: i64, l: i64, u_diag: i64, g_small: &[i64], g_large: &[i64], spf: &[u32]) -> i64 {
    let uu = u * u;
    if uu >= n {
        return 0;
    }
    let v_lim = if u <= u_diag {
        u - 1
    } else {
        ((n - uu) as u64).isqrt() as i64
    };
    if v_lim < 1 {
        return 0;
    }

    let mut local = 0i64;
    let u32u = u as u32;
    // SAFETY: 2 <= u <= l, spf.len() == l+1
    let prime_u = unsafe { *spf.get_unchecked(u as usize) } == u32u;
    let step: i64 = if u & 1 == 0 { 2 } else { 1 };
    let mut v = 1i64;
    if prime_u {
        while v <= v_lim {
            let d = uu + v * v;
            local += (u + v) * g_n_div_d(d, n, l, g_small, g_large);
            v += step;
        }
    } else {
        while v <= v_lim {
            if gcd(u32u, v as u32) == 1 {
                let d = uu + v * v;
                local += (u + v) * g_n_div_d(d, n, l, g_small, g_large);
            }
            v += step;
        }
    }
    local
}

fn main() {
    let n: i64 = N;
    let l = (n as u64).isqrt() as i64;
    let lu = l as usize;

    // G(q) for q <= l via sigma prefix
    let mut sigma = vec![0i64; lu + 1];
    for i in 1..=lu {
        let mut j = i;
        while j <= lu {
            sigma[j] += i as i64;
            j += i;
        }
    }
    let mut g_small = vec![0i64; lu + 1];
    for i in 1..=lu {
        g_small[i] = g_small[i - 1] + sigma[i];
    }
    drop(sigma);

    // G(N/k) for k = 1..=l (hyperbola; work-steal individual k)
    let mut g_large = vec![0i64; lu + 1];
    g_large[1..]
        .par_iter_mut()
        .enumerate()
        .with_min_len(1)
        .for_each(|(i, slot)| {
            *slot = g_function(n / (i + 1) as i64);
        });

    let mut spf = vec![0u32; lu + 1];
    for i in 2..=lu {
        if spf[i] == 0 {
            spf[i] = i as u32;
            let mut j = i * i;
            while j <= lu {
                if spf[j] == 0 {
                    spf[j] = i as u32;
                }
                j += i;
            }
        }
    }

    // Largest u with u^2 + (u-1)^2 <= n, so v_lim = u-1
    let mut u_diag = ((n / 2) as u64).isqrt() as i64 + 1;
    while u_diag > 1 && u_diag * u_diag + (u_diag - 1) * (u_diag - 1) > n {
        u_diag -= 1;
    }

    let nthreads = rayon::current_num_threads().max(1);
    let s2_uv: i64 = (0..nthreads)
        .into_par_iter()
        .map(|tid| {
            let mut local = 0i64;
            let mut u = 2 + tid as i64;
            while u <= l {
                local += process_u(u, n, l, u_diag, &g_small, &g_large, &spf);
                u += nthreads as i64;
            }
            local
        })
        .sum();

    let total_sum = g_large[1];
    let s2_prime = g_large[2] + s2_uv;
    println!("{}", total_sum + 2 * s2_prime);
}
