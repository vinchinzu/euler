// Project Euler 650 - Divisors of Binomial Product
// B(n) = product C(n,k). D(n) = sigma(B(n)). S(N) = sum D(n) mod 10^9+7.

use rayon::prelude::*;

const N_VAL: usize = 20000;
const MOD: u64 = 1_000_000_007;

#[inline(always)]
fn mul_mod(a: u32, b: u32) -> u32 {
    ((a as u64 * b as u64) % MOD) as u32
}

#[inline(always)]
fn power(mut base: u32, mut exp: u64) -> u32 {
    let mut r = 1u32;
    base = (base as u64 % MOD) as u32;
    while exp > 0 {
        if exp & 1 == 1 {
            r = mul_mod(r, base);
        }
        base = mul_mod(base, base);
        exp >>= 1;
    }
    r
}

#[inline(always)]
fn mod_inv(n: u32) -> u32 {
    power(n, MOD - 2)
}

fn main() {
    // Sieve primes up to N_VAL
    let mut is_prime = vec![true; N_VAL + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut primes = Vec::new();
    for i in 2..=N_VAL {
        if is_prime[i] {
            primes.push(i);
            for j in (i * 2..=N_VAL).step_by(i) {
                is_prime[j] = false;
            }
        }
    }

    // Prefix product of (p - 1)^(-1) mod MOD for all primes p <= n
    let mut inv_all_pm1 = vec![1u32; N_VAL + 1];
    let mut cur_inv = 1u32;
    for n in 2..=N_VAL {
        if is_prime[n] {
            cur_inv = mul_mod(cur_inv, mod_inv(n as u32 - 1));
        }
        inv_all_pm1[n] = cur_inv;
    }

    // Partition primes across threads to balance total steps: sum (N_VAL - p)
    let nthreads = rayon::current_num_threads().clamp(1, 16);
    let mut buckets: Vec<Vec<usize>> = vec![Vec::new(); nthreads];
    let mut bucket_weights = vec![0usize; nthreads];

    // Distribute primes greedily (largest work N_VAL - p first)
    for &p in &primes {
        let min_idx = bucket_weights
            .iter()
            .enumerate()
            .min_by_key(|&(_, &w)| w)
            .map(|(i, _)| i)
            .unwrap();
        bucket_weights[min_idx] += N_VAL - p;
        buckets[min_idx].push(p);
    }

    // Compute numerator contributions in parallel
    let final_num = buckets
        .into_par_iter()
        .map(|prime_list| {
            let mut num = vec![1u32; N_VAL + 1];
            for p in prime_list {
                let inv_p = mod_inv(p as u32);
                let mut e = 0i64;
                let mut f = 0i64;
                let mut p_pow;
                let mut step_mult;

                let mut n = p;
                while n <= N_VAL {
                    if n == p {
                        e = p as i64 - 1;
                        f = 1;
                        p_pow = power(p as u32, (e + 1) as u64);
                        step_mult = inv_p;
                    } else {
                        let mut m = n;
                        let mut v = 0i64;
                        while m % p == 0 {
                            m /= p;
                            v += 1;
                        }
                        e += (n as i64 - 1) * v - f;
                        f += v;
                        p_pow = power(p as u32, (e + 1) as u64);
                        step_mult = power(inv_p, f as u64);
                    }

                    let term_num = p_pow - 1;
                    num[n] = mul_mod(num[n], term_num);

                    let next_mult = std::cmp::min(n + p, N_VAL + 1);
                    let steps = (next_mult - 1 - n) as i64;
                    n += 1;
                    while n < next_mult {
                        p_pow = mul_mod(p_pow, step_mult);
                        let term_num = p_pow - 1;
                        num[n] = mul_mod(num[n], term_num);
                        n += 1;
                    }
                    e -= steps * f;
                }
            }
            num
        })
        .reduce(
            || vec![1u32; N_VAL + 1],
            |mut acc, item| {
                for i in 1..=N_VAL {
                    acc[i] = mul_mod(acc[i], item[i]);
                }
                acc
            },
        );

    // Sum D(n) = num[n] * inv_all_pm1[n] mod MOD for n = 1..=N_VAL
    let mut answer = 0u64;
    for n in 1..=N_VAL {
        let d = mul_mod(final_num[n], inv_all_pm1[n]) as u64;
        answer = (answer + d) % MOD;
    }

    println!("{}", answer);
}
