// Project Euler 432 - Totient sum
// S(K, N) = sum of phi(K*i) for i=1..N, K=510510, N=10^11, mod 10^9.

use std::collections::HashMap;

const BIG_N: i64 = 100_000_000_000;
const K: i64 = 510510;
const P_MOD: i64 = 1_000_000_000;
const LIMIT: usize = 5_000_000;

static PRIMES_OF_K: [i64; 7] = [2, 3, 5, 7, 11, 13, 17];

fn main() {
    // Phi sieve
    let mut phi_arr = vec![0i32; LIMIT + 1];
    for i in 0..=LIMIT {
        phi_arr[i] = i as i32;
    }
    for i in 2..=LIMIT {
        if phi_arr[i] == i as i32 {
            // prime
            let mut j = i;
            while j <= LIMIT {
                phi_arr[j] -= phi_arr[j] / i as i32;
                j += i;
            }
        }
    }

    let mut sum_phi_small = vec![0i64; LIMIT + 1];
    for i in 1..=LIMIT {
        sum_phi_small[i] = sum_phi_small[i - 1] + phi_arr[i] as i64;
    }

    // Cache for totient_sum
    let mut ts_cache: HashMap<i64, i128> = HashMap::new();

    fn totient_sum(
        q: i64,
        sum_phi_small: &[i64],
        ts_cache: &mut HashMap<i64, i128>,
    ) -> i128 {
        if q <= LIMIT as i64 {
            return sum_phi_small[q as usize] as i128;
        }
        if let Some(&v) = ts_cache.get(&q) {
            return v;
        }

        let mut result: i128 = q as i128 * (q as i128 + 1) / 2;
        let sqrt_q = (q as f64).sqrt() as i64;

        let mut d = 2i64;
        while d <= sqrt_q {
            let qd = q / d;
            result -= totient_sum(qd, sum_phi_small, ts_cache);
            d += 1;
        }

        let mut m = 1i64;
        while m <= sqrt_q {
            if q / m > sqrt_q {
                let count = q / m - q / (m + 1);
                result -= count as i128 * totient_sum(m, sum_phi_small, ts_cache);
            }
            m += 1;
        }

        ts_cache.insert(q, result);
        result
    }

    // Precompute totient_sum for quotient values bottom-up
    let sqrt_n = (BIG_N as f64).sqrt() as i64;

    for q in 1..=sqrt_n + 1 {
        if q as usize > LIMIT {
            totient_sum(q, &sum_phi_small, &mut ts_cache);
        }
    }
    for k in (1..=sqrt_n + 1).rev() {
        let q = BIG_N / k;
        if q as usize > LIMIT {
            totient_sum(q, &sum_phi_small, &mut ts_cache);
        }
    }

    // phi(K) = product of (p-1) for primes p | K
    let mut phi_k: i64 = 1;
    for &p in &PRIMES_OF_K {
        phi_k *= p - 1;
    }

    // Enumerate all d that are products of prime factors of K
    let mut ans: i64 = 0;

    fn enumerate_d(
        idx: usize,
        d: i64,
        ans: &mut i64,
        sum_phi_small: &[i64],
        ts_cache: &mut HashMap<i64, i128>,
    ) {
        if idx == 7 {
            let ts = totient_sum(BIG_N / d, sum_phi_small, ts_cache);
            *ans = (*ans + (ts % P_MOD as i128) as i64) % P_MOD;
            return;
        }
        // Don't include this prime
        enumerate_d(idx + 1, d, ans, sum_phi_small, ts_cache);
        // Include repeatedly
        let mut d2 = d * PRIMES_OF_K[idx];
        while d2 <= BIG_N {
            enumerate_d(idx + 1, d2, ans, sum_phi_small, ts_cache);
            d2 *= PRIMES_OF_K[idx];
        }
    }

    enumerate_d(0, 1, &mut ans, &sum_phi_small, &mut ts_cache);

    ans = ((ans % P_MOD) * (phi_k % P_MOD)) % P_MOD;
    ans = (ans % P_MOD + P_MOD) % P_MOD;

    println!("{}", ans);
}
