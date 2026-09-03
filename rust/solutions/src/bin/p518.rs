// Project Euler 518 - Prime Triples and Geometric Sequences
// Find all triples (a,b,c) of primes < 10^8 forming a geometric sequence.
// a+1 = k*p^2, b+1 = k*p*q, c+1 = k*q^2 with gcd(p,q)=1, 1 <= p < q.

use rayon::prelude::*;

const N: usize = 100_000_000;

#[inline(always)]
fn is_coprime(mut u: u32, mut v: u32) -> bool {
    u >>= u.trailing_zeros();
    while v != 0 {
        v >>= v.trailing_zeros();
        if u > v {
            std::mem::swap(&mut u, &mut v);
        }
        v -= u;
    }
    u == 1
}

#[inline(always)]
unsafe fn is_prime_odd(sieve: &[u8], n: usize) -> bool {
    let m = n >> 1;
    unsafe {
        (*sieve.get_unchecked(m >> 3) & (1 << (m & 7))) == 0
    }
}

#[inline(always)]
fn solve_m(m: usize, sieve: &[u8]) -> i64 {
    let k = 2 * m;
    if k % 3 == 1 {
        return 0;
    }
    let k_mod3_is_2 = k % 3 == 2;
    let q_max = ((N / k) as f64).sqrt() as usize;
    let mut sum: i64 = 0;

    for q in 2..=q_max {
        let c = k * q * q - 1;
        if unsafe { !is_prime_odd(sieve, c) } {
            continue;
        }

        for p in 1..q {
            if (p | q) & 1 == 0 {
                continue;
            }
            if k_mod3_is_2 && (p * q) % 3 == 2 {
                continue;
            }
            let a = k * p * p - 1;
            if unsafe { !is_prime_odd(sieve, a) } {
                continue;
            }
            if p != 1 && !is_coprime(q as u32, p as u32) {
                continue;
            }
            let b = k * p * q - 1;
            if unsafe { !is_prime_odd(sieve, b) } {
                continue;
            }
            sum += (a + b + c) as i64;
        }
    }
    sum
}

fn main() {
    let num_odds = N / 2;
    let bytes_len = num_odds.div_ceil(8);

    // Collect base primes up to sqrt(N) = 10,000
    let mut base_primes = Vec::new();
    let mut is_p = [true; 10001];
    for p in 2..=10000 {
        if is_p[p] {
            if p > 2 {
                base_primes.push(p);
            }
            let mut j = p * p;
            while j <= 10000 {
                is_p[j] = false;
                j += p;
            }
        }
    }

    // Parallel segmented odd-only bit sieve
    let mut sieve = vec![0u8; bytes_len];
    sieve[0] |= 1; // 1 is not prime

    let chunk_size = 32768;
    sieve.par_chunks_mut(chunk_size).enumerate().for_each(|(chunk_idx, chunk)| {
        let chunk_start_m = chunk_idx * chunk_size * 8;
        let chunk_end_m = (chunk_start_m + chunk.len() * 8).min(num_odds);
        let chunk_start_val = 2 * chunk_start_m + 1;
        let chunk_end_val = 2 * chunk_end_m;

        for &p in &base_primes {
            let p_sq = p * p;
            if p_sq > chunk_end_val {
                break;
            }
            let mut val = if p_sq >= chunk_start_val {
                p_sq
            } else {
                let mut v = chunk_start_val.div_ceil(p) * p;
                if v % 2 == 0 {
                    v += p;
                }
                v
            };
            while val < chunk_end_val {
                let m = val >> 1;
                let local_m = m - chunk_start_m;
                chunk[local_m >> 3] |= 1 << (local_m & 7);
                val += 2 * p;
            }
        }
    });

    let mut ans: i64 = 0;

    // Special case k = 3, p = 1: a = 3(1)^2 - 1 = 2 (prime)
    // q must be even, q >= 2, 3*q^2 < N
    let q_max_3 = ((N as f64 / 3.0).sqrt()) as usize;
    for q in (2..=q_max_3).step_by(2) {
        let c = 3 * q * q - 1;
        if unsafe { is_prime_odd(&sieve, c) } {
            let b = 3 * q - 1;
            if unsafe { is_prime_odd(&sieve, b) } {
                ans += 2 + b as i64 + c as i64;
            }
        }
    }

    // Range 1: m in 1..=10 (k <= 20) -> flat parallel list of (k, q)
    let mut r1_pairs = Vec::new();
    for m in 1..=10 {
        let k = 2 * m;
        if k % 3 == 1 {
            continue;
        }
        let q_max = ((N / k) as f64).sqrt() as usize;
        for q in 2..=q_max {
            r1_pairs.push((k, q));
        }
    }

    let ans1: i64 = r1_pairs.into_par_iter().with_min_len(64).map(|(k, q)| {
        let c = k * q * q - 1;
        if unsafe { !is_prime_odd(&sieve, c) } {
            return 0;
        }
        let k_mod3_is_2 = k % 3 == 2;
        let mut local = 0i64;

        for p in 1..q {
            if (p | q) & 1 == 0 {
                continue;
            }
            if k_mod3_is_2 && (p * q) % 3 == 2 {
                continue;
            }
            let a = k * p * p - 1;
            if unsafe { !is_prime_odd(&sieve, a) } {
                continue;
            }
            if p != 1 && !is_coprime(q as u32, p as u32) {
                continue;
            }
            let b = k * p * q - 1;
            if unsafe { !is_prime_odd(&sieve, b) } {
                continue;
            }
            local += (a + b + c) as i64;
        }
        local
    }).sum();
    ans += ans1;

    // Range 2A: m in 11..=1,000 (1 m per parallel task)
    let ans2a: i64 = (11..=1000).into_par_iter().map(|m| solve_m(m, &sieve)).sum();
    ans += ans2a;

    // Range 2B: m in 1,001..=50,000 (chunks of 100 m)
    let m2_start = 1001;
    let m2_end = 50_000;
    let c2_size = 100;
    let num_c2 = (m2_end - m2_start + c2_size) / c2_size;
    let ans2b: i64 = (0..num_c2).into_par_iter().map(|ci| {
        let start = m2_start + ci * c2_size;
        let end = (start + c2_size - 1).min(m2_end);
        let mut s = 0i64;
        for m in start..=end {
            s += solve_m(m, &sieve);
        }
        s
    }).sum();
    ans += ans2b;

    // Range 3: k in 100,002..=N/4 (m in 50,001..=m_max) (chunks of 50,000 m)
    let k_max = N / 4;
    let m_max = k_max / 2;
    let m3_start = 50_001;
    let c3_size = 50_000;
    let num_c3 = (m_max - m3_start + c3_size) / c3_size;
    let ans3: i64 = (0..num_c3).into_par_iter().map(|ci| {
        let start = m3_start + ci * c3_size;
        let end = (start + c3_size - 1).min(m_max);
        let mut s = 0i64;
        for m in start..=end {
            s += solve_m(m, &sieve);
        }
        s
    }).sum();
    ans += ans3;

    println!("{}", ans);
}
