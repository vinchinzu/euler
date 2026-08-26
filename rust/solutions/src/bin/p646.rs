// Project Euler 646 - Bounded Divisors
// Sum of lambda(d)*d over divisors d of N! where L <= d <= H.
// Meet-in-the-middle: sort both halves, parallel two-pointer over log ranges.

use rayon::prelude::*;

const MOD: u64 = 1_000_000_007;
const EPS: f64 = 1e-14;

fn num_factors_in_factorial(n: i32, p: i32) -> i32 {
    let mut count = 0;
    let mut power = p as i64;
    while power <= n as i64 {
        count += n as i64 / power;
        power *= p as i64;
    }
    count as i32
}

#[derive(Clone, Copy)]
struct Divisor {
    log_val: f64,
    lio: u64,
}

fn gen_divisors(primes: &[i32], exponents: &[i32]) -> Vec<Divisor> {
    let mut total: usize = 1;
    for &e in exponents {
        total *= e as usize + 1;
    }
    let mut divs = Vec::with_capacity(total);
    divs.push(Divisor {
        log_val: 0.0,
        lio: 1,
    });
    for (&p, &e_max) in primes.iter().zip(exponents.iter()) {
        let log_p = (p as f64).ln();
        let neg_p = MOD - p as u64;
        let old = divs.len();
        let mut cur_lio = 1u64;
        for e in 1..=e_max {
            // Match original log association: e * ln(p), not repeated += ln(p).
            let cur_log = e as f64 * log_p;
            cur_lio = cur_lio * neg_p % MOD;
            let dest = divs.len();
            // SAFETY: dest + old <= total; new slots are written before any read.
            unsafe {
                divs.set_len(dest + old);
            }
            let (src, rest) = divs.split_at_mut(dest);
            let src = &src[..old];
            let dst = &mut rest[..old];
            if old >= 65_536 {
                src.par_iter().zip(dst.par_iter_mut()).for_each(|(&s, d)| {
                    *d = Divisor {
                        log_val: s.log_val + cur_log,
                        lio: s.lio * cur_lio % MOD,
                    };
                });
            } else {
                // SAFETY: src and dst are disjoint length-old slices; j < old.
                unsafe {
                    let src_ptr = src.as_ptr();
                    let dst_ptr = dst.as_mut_ptr();
                    for j in 0..old {
                        let s = *src_ptr.add(j);
                        *dst_ptr.add(j) = Divisor {
                            log_val: s.log_val + cur_log,
                            lio: s.lio * cur_lio % MOD,
                        };
                    }
                }
            }
        }
    }
    divs
}

fn main() {
    let n = 70;
    let log_l = 20.0f64 * 10.0f64.ln();
    let log_h = 60.0f64 * 10.0f64.ln();

    let primes = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67,
    ];
    let num_primes = primes.len();
    let mut exponents = [0i32; 19];
    for i in 0..num_primes {
        exponents[i] = num_factors_in_factorial(n, primes[i]);
    }

    let mut total_factors: i64 = 1;
    for i in 0..num_primes {
        total_factors *= (exponents[i] + 1) as i64;
    }

    let mut half_index = 0;
    let mut nf: i64 = 1;
    while nf * nf < total_factors {
        nf *= (exponents[half_index] + 1) as i64;
        half_index += 1;
    }

    let (mut left, mut right) = rayon::join(
        || gen_divisors(&primes[..half_index], &exponents[..half_index]),
        || gen_divisors(&primes[half_index..], &exponents[half_index..]),
    );

    rayon::join(
        || left.par_sort_unstable_by(|a, b| a.log_val.total_cmp(&b.log_val)),
        || right.par_sort_unstable_by(|a, b| a.log_val.total_cmp(&b.log_val)),
    );

    let nleft = left.len();
    let mut prefix = vec![0u64; nleft + 1];
    for i in 0..nleft {
        let mut s = prefix[i] + left[i].lio;
        if s >= MOD {
            s -= MOD;
        }
        prefix[i + 1] = s;
    }

    let nthreads = rayon::current_num_threads().max(1);
    let chunk_size = (right.len() / nthreads).max(1);

    let ans = right
        .par_chunks(chunk_size)
        .map(|chunk| {
            if chunk.is_empty() {
                return 0u64;
            }
            // Chunk is sorted by increasing log; process large→small so [lo,hi] grows.
            let last = chunk.last().unwrap();
            let mut a = left.partition_point(|d| d.log_val <= log_l - last.log_val - EPS);
            let mut b = left.partition_point(|d| d.log_val <= log_h - last.log_val + EPS);
            let mut local = 0u64;
            for r in chunk.iter().rev() {
                let lo = log_l - r.log_val - EPS;
                let hi = log_h - r.log_val + EPS;
                // SAFETY: a,b start in 0..=nleft and only increase while a,b < nleft
                while a < nleft && unsafe { left.get_unchecked(a).log_val } <= lo {
                    a += 1;
                }
                while b < nleft && unsafe { left.get_unchecked(b).log_val } <= hi {
                    b += 1;
                }
                // SAFETY: 0 <= a,b <= nleft == prefix.len()-1
                let mut range_sum =
                    unsafe { *prefix.get_unchecked(b) + MOD - *prefix.get_unchecked(a) };
                if range_sum >= MOD {
                    range_sum -= MOD;
                }
                // r.lio and range_sum are both in 0..MOD, so the product fits in u64.
                local = (local + r.lio * range_sum) % MOD;
            }
            local
        })
        .reduce(
            || 0u64,
            |x, y| {
                let mut s = x + y;
                if s >= MOD {
                    s -= MOD;
                }
                s
            },
        );

    println!("{}", ans);
}
