// Project Euler 195: Inscribed circles of 60-degree triangles
use rayon::prelude::*;

#[inline(always)]
fn gcd(mut u: u32, mut v: u32) -> u32 {
    if u == 0 {
        return v;
    }
    if v == 0 {
        return u;
    }
    let shift = (u | v).trailing_zeros();
    u >>= u.trailing_zeros();
    loop {
        v >>= v.trailing_zeros();
        if u > v {
            std::mem::swap(&mut u, &mut v);
        }
        v -= u;
        if v == 0 {
            break;
        }
    }
    u << shift
}

fn process_n(n: u32, a: u64, b: u64) -> i64 {
    let n64 = n as u64;
    let n3 = n64 * 3;
    // k(k + 3n) <= B  =>  k <= (-3n + sqrt(9n^2 + 4B)) / 2
    let disc = 9 * n64 * n64 + (b << 2);
    let k_max = (disc.isqrt() - n3) / 2;
    if k_max == 0 {
        return 0;
    }

    let mut local = 0i64;
    // gcd(k, n) = gcd(m, n); n even => only odd k can be coprime
    let step = if n % 2 == 0 { 2u64 } else { 1u64 };
    let mut k = 1u64;
    while k <= k_max {
        if gcd(k as u32, n) == 1 {
            let d = k * (k + n3);
            local += (if k % 3 == 0 { b / d } else { a / d }) as i64;
        }
        k += step;
    }
    local
}

fn main() {
    const N: u64 = 1_053_779;
    // A = floor(2 N √3), B = floor(6 N √3). floor(c √3 / d) = floor(c √3) / d.
    let a = (12 * N * N).isqrt();
    let b = (108 * N * N).isqrt();
    let n_max = ((b - 1) / 3) as u32;

    // Stripe n across threads: small n has ~3300 k's, large n has 1.
    let nthreads = rayon::current_num_threads().max(1) as u32;
    let ans: i64 = (0..nthreads)
        .into_par_iter()
        .map(|tid| {
            let mut local = 0i64;
            let mut n = tid + 1;
            while n <= n_max {
                local += process_n(n, a, b);
                n += nthreads;
            }
            local
        })
        .sum();

    println!("{}", ans);
}
