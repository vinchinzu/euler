// Project Euler 484: Arithmetic derivative
// Uses multiplicative function properties to compute sum.

const N: i64 = 5_000_000_000_000_000; // 5 * 10^15

fn ipow(mut base: i64, mut exp: i64) -> i64 {
    let mut result: i64 = 1;
    while exp > 0 {
        if exp & 1 == 1 { result *= base; }
        base *= base;
        exp >>= 1;
    }
    result
}

fn main() {
    let limit = (N as f64).sqrt() as usize + 1;

    // Sieve primes up to limit
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    if limit >= 1 { is_prime[1] = false; }
    let mut i = 2;
    while i * i <= limit {
        if is_prime[i] {
            let mut j = i * i;
            while j <= limit {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    let primes: Vec<i64> = (2..=limit).filter(|&i| is_prime[i]).map(|i| i as i64).collect();

    let mut ans: i128 = 0;

    fn helper(min_index: usize, mult: i64, n: i64, primes: &[i64], ans: &mut i128) {
        *ans += mult as i128 * n as i128;

        for index in min_index..primes.len() {
            let p = primes[index];
            if p as i128 * p as i128 > n as i128 { break; }

            let mut new_n = n / (p * p);
            let mut e: i64 = 2;
            while new_n > 0 {
                let mut new_mult = mult * (p - 1) * ipow(p, e - 2);
                if e % p == 0 { new_mult *= p + 1; }
                if e % p != 1 {
                    helper(index + 1, new_mult, new_n, primes, ans);
                }
                new_n /= p;
                e += 1;
            }
        }
    }

    helper(0, 1, N, &primes, &mut ans);
    ans -= 1; // Subtract k=1 term

    println!("{}", ans);
}
