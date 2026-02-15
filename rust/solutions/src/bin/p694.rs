// Project Euler 694 - Cube-full Divisors
// Enumerate all cube-full numbers recursively, sum floor(N/k).

fn main() {
    let big_n: i64 = 1_000_000_000_000_000_000;
    let limit = (big_n as f64).cbrt() as usize + 2;

    // Sieve primes up to limit
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    if limit >= 1 { is_prime[1] = false; }
    let mut i = 2;
    while i * i <= limit {
        if is_prime[i] {
            let mut j = i * i;
            while j <= limit { is_prime[j] = false; j += i; }
        }
        i += 1;
    }
    let primes: Vec<i64> = (2..=limit).filter(|&i| is_prime[i]).map(|i| i as i64).collect();

    let mut total = 0i64;

    fn helper(min_idx: usize, k: i64, n: i64, primes: &[i64], total: &mut i64) {
        *total += n / k;
        for i in min_idx..primes.len() {
            let p = primes[i];
            let p3 = p * p * p;
            if k > n / p3 { break; }
            let mut nk = k * p3;
            loop {
                helper(i + 1, nk, n, primes, total);
                if nk > n / p { break; }
                nk *= p;
            }
        }
    }

    helper(0, 1, big_n, &primes, &mut total);
    println!("{}", total);
}
