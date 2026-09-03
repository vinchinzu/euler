// Project Euler 521 - Smallest Prime Factor Sum
//
// Lucy DP sieve: compute sum of smallest prime factors for 2..N mod M.

use euler_utils::sieve;

const N: i64 = 1_000_000_000_000;
const M: i64 = 1_000_000_000;

fn isqrt(n: i64) -> i64 {
    let mut x = (n as f64).sqrt() as i64;
    while x > 0 && x * x > n { x -= 1; }
    while (x + 1) * (x + 1) <= n { x += 1; }
    x
}

fn sum_2_to_n(n: i64) -> i64 {
    if n < 2 { return 0; }
    let s = n as i128 * (n as i128 + 1) / 2 - 1;
    (s % M as i128) as i64
}

fn main() {
    let l = isqrt(N);

    let is_prime = sieve(l as usize);
    let primes: Vec<i64> = (2..=l).filter(|&i| is_prime[i as usize]).collect();

    let small_len = (N / l + 2) as usize;
    let big_len = (l + 2) as usize;

    let mut big_cnt = vec![0i64; big_len];
    let mut small_cnt = vec![0i64; small_len];
    let mut big_sum = vec![0i64; big_len];
    let mut small_sum = vec![0i64; small_len];

    for i in 1..=l {
        big_cnt[i as usize] = N / i - 1;
        big_sum[i as usize] = sum_2_to_n(N / i);
    }
    for i in 1..(N / l) {
        small_cnt[i as usize] = i - 1;
        small_sum[i as usize] = sum_2_to_n(i);
    }

    let mut ans: i64 = 0;

    for &p in &primes {
        let p2 = p * p;
        let p_cnt = small_cnt[(p - 1) as usize];
        let p_sum = small_sum[(p - 1) as usize];
        let p_mod = p % M;

        let removed = big_cnt[p as usize] - p_cnt;
        ans = (ans + p_mod * (removed % M)) % M;

        // Update big arrays
        let max_i = (N / p2).min(l);
        let i_split = (l / p).min(max_i);

        let mut idx = p as usize;
        let p_stride = p as usize;
        for i in 1..=i_split {
            let cnt_remove = big_cnt[idx] - p_cnt;
            big_cnt[i as usize] -= cnt_remove;
            let mut sum_remove = big_sum[idx] - p_sum;
            if sum_remove < 0 {
                sum_remove += M;
            }
            let term = (p_mod * sum_remove) % M;
            let mut s = big_sum[i as usize] - term;
            if s < 0 {
                s += M;
            }
            big_sum[i as usize] = s;
            idx += p_stride;
        }

        let n_div_p = N / p;
        for i in (i_split + 1)..=max_i {
            let q = (n_div_p / i) as usize;
            let cnt_remove = small_cnt[q] - p_cnt;
            big_cnt[i as usize] -= cnt_remove;
            let mut sum_remove = small_sum[q] - p_sum;
            if sum_remove < 0 {
                sum_remove += M;
            }
            let term = (p_mod * sum_remove) % M;
            let mut s = big_sum[i as usize] - term;
            if s < 0 {
                s += M;
            }
            big_sum[i as usize] = s;
        }

        // Update small arrays
        let mut i = N / l - 1;
        while i >= p2 {
            let q = (i / p) as usize;
            let cnt_remove = small_cnt[q] - p_cnt;
            let mut sum_remove = small_sum[q] - p_sum;
            if sum_remove < 0 {
                sum_remove += M;
            }
            let term = (p_mod * sum_remove) % M;
            let start = (q as i64 * p).max(p2) as usize;
            let end = i as usize;
            let c_slice = &mut small_cnt[start..=end];
            let s_slice = &mut small_sum[start..=end];
            for (c, s) in c_slice.iter_mut().zip(s_slice.iter_mut()) {
                *c -= cnt_remove;
                let mut val = *s - term;
                if val < 0 {
                    val += M;
                }
                *s = val;
            }
            i = start as i64 - 1;
        }
    }

    // sum_div(1) = big_sum[1] since N/1 = N, and N/N = 1 which is <= l
    let final_sum = big_sum[1];
    ans = (ans + final_sum) % M;

    println!("{ans}");
}
