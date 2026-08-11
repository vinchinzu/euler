// Project Euler 540 - Counting Primitive Pythagorean Triples
//
// Count primitive Pythagorean triples with hypotenuse <= N = pi * 10^15.
// Uses Euler's totient sieve for small m, inclusion-exclusion for large m.
// Optimized: stack prime-factor list; rayon over m.

use rayon::prelude::*;

const N: i64 = 3_141_592_653_589_793;

#[inline(always)]
fn isqrt(n: i64) -> i64 {
    if n <= 0 {
        return 0;
    }
    let mut x = (n as f64).sqrt() as i64;
    while x > 0 && x * x > n {
        x -= 1;
    }
    while (x + 1) * (x + 1) <= n {
        x += 1;
    }
    x
}

#[inline(always)]
fn sq(x: i64) -> i64 {
    x * x
}

#[inline(always)]
fn fill_prime_factors(mut m: usize, ff: &[u32], out: &mut [i64; 16]) -> usize {
    let mut n = 0usize;
    while m > 1 {
        let p = ff[m] as i64;
        if n == 0 || out[n - 1] != p {
            out[n] = p;
            n += 1;
        }
        m /= p as usize;
    }
    n
}

#[inline(always)]
fn num_relatively_prime(m: usize, limit: i64, ff: &[u32]) -> i64 {
    if limit <= 0 {
        return 0;
    }
    let mut factors = [0i64; 16];
    let nf = fill_prime_factors(m, ff, &mut factors);
    let mut count = 0i64;
    for mask in 0..(1u32 << nf) {
        let mut prod = 1i64;
        let mut bits = 0;
        for i in 0..nf {
            if mask & (1 << i) != 0 {
                prod *= factors[i];
                bits += 1;
            }
        }
        if bits % 2 == 0 {
            count += limit / prod;
        } else {
            count -= limit / prod;
        }
    }
    count
}

fn main() {
    let l = isqrt(N / 2);
    let sqrt_n = isqrt(N) as usize;
    let m_max = isqrt(N);

    let mut ff = vec![0u32; sqrt_n + 1];
    for i in 2..=sqrt_n {
        if ff[i] == 0 {
            for j in (i..=sqrt_n).step_by(i) {
                if ff[j] == 0 {
                    ff[j] = i as u32;
                }
            }
        }
    }

    let l_usize = l as usize;
    let mut phi = vec![0u32; l_usize + 1];
    for i in 0..=l_usize {
        phi[i] = i as u32;
    }
    for i in 2..=l_usize {
        if phi[i] == i as u32 {
            for j in (i..=l_usize).step_by(i) {
                phi[j] -= phi[j] / i as u32;
            }
        }
    }

    let ans: i64 = (2..=m_max)
        .into_par_iter()
        .map(|m| {
            let mult = if m % 2 == 0 { 1 } else { 2 };
            if m <= l {
                phi[m as usize] as i64 / mult
            } else {
                let limit = isqrt(N - sq(m)) / mult;
                num_relatively_prime(m as usize, limit, &ff)
            }
        })
        .sum();

    println!("{ans}");
}
