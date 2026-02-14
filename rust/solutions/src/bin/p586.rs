// Project Euler 586 - Binary Quadratic Forms
//
// Count numbers expressible as a^2 + 3ab + b^2 up to N = 10^15.
// Uses multiplicative structure and DFS over prime factorizations.

const N: i64 = 1_000_000_000_000_000;
const K: usize = 40;

fn main() {
    // Compute min_power using smallest prime factor sieve
    let limit = 2 * K + 2;
    let mut ff = vec![0u32; limit + 1];
    for i in 2..=limit {
        if ff[i] == 0 {
            ff[i] = i as u32;
            let mut j = i * i;
            while j <= limit {
                if ff[j] == 0 { ff[j] = i as u32; }
                j += i;
            }
        }
    }

    let mut min_power = vec![0usize; limit + 1];
    for k in 2..=limit {
        let mut n = k;
        let mut mp = 0;
        while n > 1 {
            let p = ff[n] as usize;
            let mut e = 0;
            while n % p == 0 {
                n /= p;
                e += 1;
            }
            mp += e * (p - 1);
        }
        min_power[k] = mp;
    }
    min_power[1] = 2;

    // Calculate max prime needed
    let min_mp = min_power[2 * K].min(min_power[2 * K + 1]);
    let mut max_prime: i64 = N;
    // Divide by 11^(min_mp - 1)
    for _ in 0..min_mp - 1 {
        max_prime /= 11;
        if max_prime == 0 { break; }
    }

    // Sieve primes up to max_prime
    let limit_p = max_prime as usize;
    let mut is_prime = vec![true; limit_p + 1];
    if limit_p >= 1 { is_prime[0] = false; }
    if limit_p >= 1 { is_prime[1] = false; }
    let mut i = 2usize;
    while i * i <= limit_p {
        if is_prime[i] {
            let mut j = i * i;
            while j <= limit_p {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    let primes: Vec<usize> = (2..=limit_p).filter(|&x| is_prime[x]).collect();

    let mut ans: i64 = 0;

    fn exceeds(mut prod: i128, p: i128, exp: usize, n: i64) -> bool {
        for _ in 0..exp {
            prod *= p;
            if prod > n as i128 { return true; }
        }
        false
    }

    fn helper(
        k: usize, last_index: i32, prod: i128,
        primes: &[usize], min_power: &[usize], n: i64, ans: &mut i64,
    ) {
        if k == 1 {
            *ans += 1;
        }

        for index in (last_index + 1) as usize..primes.len() {
            let p = primes[index];
            if k >= min_power.len() || exceeds(prod, p as i128, min_power[k], n) {
                break;
            }

            let mut new_prod = prod;
            let mut e = 1usize;
            while new_prod * (p as i128) <= n as i128 {
                new_prod *= p as i128;
                if p % 5 == 1 || p % 5 == 4 {
                    if k % (e + 1) == 0 {
                        helper(k / (e + 1), index as i32, new_prod, primes, min_power, n, ans);
                    }
                } else if p == 5 || e % 2 == 0 {
                    helper(k, index as i32, new_prod, primes, min_power, n, ans);
                }
                e += 1;
            }
        }
    }

    helper(2 * K, -1, 1, &primes, &min_power, N, &mut ans);
    helper(2 * K + 1, -1, 1, &primes, &min_power, N, &mut ans);

    println!("{}", ans);
}
