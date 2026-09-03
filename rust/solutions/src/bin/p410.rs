// Project Euler 410: Circle and Tangent Line
//
// Optimized: Segmented sieve for prime omega using only primes <= sqrt(a) = 10_000.
// Any integer x <= 10^8 has at most one prime factor > 10^4.
// By sieving with the 1,229 primes <= 10^4, the remaining cofactor is either 1 or prime.
// Processing in 100K-element chunks keeps memory in L1 cache (400KB per thread)
// and computes the sum on the fly with Rayon parallelism.

use rayon::prelude::*;

fn sieve_primes(lim: usize) -> Vec<u32> {
    let mut is_p = vec![true; lim + 1];
    is_p[0] = false;
    is_p[1] = false;
    let mut p = 2;
    while p * p <= lim {
        if is_p[p] {
            let mut j = p * p;
            while j <= lim {
                is_p[j] = false;
                j += p;
            }
        }
        p += 1;
    }
    (2..=lim).filter(|&x| is_p[x]).map(|x| x as u32).collect()
}

fn main() {
    let a: usize = 100_000_000; // 10^8
    let b: i64 = 1_000_000_000; // 10^9

    let primes = sieve_primes(10_000);

    let chunk_size = 100_000usize;
    let num_chunks = (a - 1 + chunk_size - 1) / chunk_size;

    let sum: i64 = (0..num_chunks)
        .into_par_iter()
        .map(|ci| {
            let low = 2 + ci * chunk_size;
            let high = (low + chunk_size - 1).min(a);
            let len = high - low + 1;

            let mut omega = vec![0u8; len];
            let mut rem = vec![0u32; len];
            for i in 0..len {
                rem[i] = (low + i) as u32;
            }

            for &p in &primes {
                let p = p as usize;
                let start = if low % p == 0 {
                    0
                } else {
                    p - (low % p)
                };
                let mut j = start;
                while j < len {
                    omega[j] += 1;
                    let mut r = rem[j];
                    let p_u32 = p as u32;
                    while r % p_u32 == 0 {
                        r /= p_u32;
                    }
                    rem[j] = r;
                    j += p;
                }
            }

            let mut chunk_ans = 0i64;
            for i in 0..len {
                let mut om = omega[i];
                if rem[i] > 1 {
                    om += 1;
                }
                let j = low + i;
                let res: i64;
                if j % 2 == 0 {
                    let aj = (a / j) as i64;
                    let bj = b / j as i64;
                    let aj_even = aj / 2;
                    let bj_even = bj / 2;
                    let aj_odd = (aj + 1) / 2;
                    let bj_odd = (bj + 1) / 2;
                    res = aj_even * bj_even + aj_odd * bj_odd;
                } else {
                    res = (a / j) as i64 * (b / j as i64);
                }
                chunk_ans += (1i64 << om) * res;
            }
            chunk_ans
        })
        .sum();

    let mut ans = a as i64 * b + sum;
    ans *= 4;
    println!("{ans}");
}
