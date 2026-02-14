// Project Euler 578 - Integers with Decreasing Prime Powers
//
// Count integers n = prod(p_i^e_i) up to N=10^13 such that e_i >= e_j if p_i <= p_j.
// Uses recursive inclusion-exclusion over powerful numbers.

fn main() {
    let nn: i64 = 10_000_000_000_000; // 10^13
    let limit = (nn as f64).sqrt() as usize + 1;

    // Sieve primes
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    if limit >= 1 {
        is_prime[1] = false;
    }
    let mut i = 2usize;
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
    let prime_list: Vec<i64> = (2..=limit).filter(|&i| is_prime[i]).map(|i| i as i64).collect();

    let mut ans: i64 = 0;

    fn parity(n: i32) -> i64 {
        if n % 2 == 0 { 1 } else { -1 }
    }

    fn find_remaining(
        min_index: usize,
        n: i64,
        count: i32,
        threshold: usize,
        prime_list: &[i64],
        nn: i64,
        ans: &mut i64,
    ) {
        *ans += (nn / n) * parity(count);
        for index in min_index..prime_list.len() {
            let p = prime_list[index];
            if index >= threshold {
                if n > nn / (p * p) {
                    break;
                }
                find_remaining(index + 1, n * p * p, count + 1, threshold, prime_list, nn, ans);
            } else {
                if n > nn / p {
                    break;
                }
                find_remaining(index + 1, n * p, count + 1, threshold, prime_list, nn, ans);
            }
        }
    }

    fn find_powerfuls(
        min_index: usize,
        n: i64,
        prev_e: i32,
        prime_list: &[i64],
        nn: i64,
        ans: &mut i64,
    ) {
        find_remaining(0, n, 0, min_index, prime_list, nn, ans);
        for index in min_index..prime_list.len() {
            let p = prime_list[index];
            if n > nn / (p * p) {
                break;
            }
            let mut nn_local = n * p;
            for e in 2..=prev_e {
                nn_local *= p;
                if nn_local > nn / p {
                    break;
                }
                find_powerfuls(index + 1, nn_local * p, e, prime_list, nn, ans);
            }
        }
    }

    find_powerfuls(0, 1, 999, &prime_list, nn, &mut ans);

    println!("{}", ans);
}
