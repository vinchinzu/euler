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

fn cnt_lookup(x: i64, l: i64, small_cnt: &[i64], big_cnt: &[i64]) -> i64 {
    if x == 0 { return 0; }
    let q = N / x;
    if q < N / l { small_cnt[q as usize] } else { big_cnt[(N / q) as usize] }
}

fn sum_lookup(x: i64, l: i64, small_sum: &[i64], big_sum: &[i64]) -> i64 {
    if x == 0 { return 0; }
    let q = N / x;
    if q < N / l { small_sum[q as usize] } else { big_sum[(N / q) as usize] }
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

        let removed = cnt_lookup(p, l, &small_cnt, &big_cnt) - small_cnt[(p - 1) as usize];
        ans = (ans + (p % M) * (removed % M)) % M;

        // Update big arrays
        let mut i = 1i64;
        while i <= l && N / i >= p2 {
            let cnt_remove = cnt_lookup(i * p, l, &small_cnt, &big_cnt) - small_cnt[(p - 1) as usize];
            big_cnt[i as usize] -= cnt_remove;
            let sum_remove = (sum_lookup(i * p, l, &small_sum, &big_sum) - small_sum[(p - 1) as usize] + M) % M;
            big_sum[i as usize] = (big_sum[i as usize] - (p % M) * sum_remove % M + M) % M;
            i += 1;
        }

        // Update small arrays
        let mut i = N / l - 1;
        while i >= p2 {
            let cnt_remove = small_cnt[(i / p) as usize] - small_cnt[(p - 1) as usize];
            small_cnt[i as usize] -= cnt_remove;
            let sum_remove = (small_sum[(i / p) as usize] - small_sum[(p - 1) as usize] + M) % M;
            small_sum[i as usize] = (small_sum[i as usize] - (p % M) * sum_remove % M + M) % M;
            i -= 1;
        }
    }

    // sum_div(1) = big_sum[1] since N/1 = N, and N/N = 1 which is <= l
    let final_sum = big_sum[1];
    ans = (ans + final_sum) % M;

    println!("{ans}");
}
