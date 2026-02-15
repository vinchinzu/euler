// Project Euler 578 - Integers with Decreasing Prime Powers
//
// Count integers n = prod(p_i^e_i) up to N=10^13 such that e_i >= e_j if p_i <= p_j.
// Uses recursive inclusion-exclusion over powerful numbers.
//
// Ported directly from the working C solution.

const NN: i64 = 10_000_000_000_000; // 10^13

#[inline]
fn parity(n: i32) -> i64 {
    if n % 2 == 0 { 1 } else { -1 }
}

fn find_remaining(min_index: usize, n: i64, count: i32, threshold: usize, primes: &[i32], ans: &mut i64) {
    *ans += (NN / n) * parity(count);
    let mut index = min_index;
    while index < primes.len() {
        let p = primes[index] as i64;
        if index >= threshold {
            if n > NN / (p * p) {
                break;
            }
            find_remaining(index + 1, n * p * p, count + 1, threshold, primes, ans);
        } else {
            if n > NN / p {
                break;
            }
            find_remaining(index + 1, n * p, count + 1, threshold, primes, ans);
        }
        index += 1;
    }
}

fn find_powerfuls(min_index: usize, n: i64, prev_e: i32, primes: &[i32], ans: &mut i64) {
    find_remaining(0, n, 0, min_index, primes, ans);
    let mut index = min_index;
    while index < primes.len() {
        let p = primes[index] as i64;
        if n > NN / (p * p) {
            break;
        }
        let mut nn = n * p; // nn starts at n*p
        for e in 2..=prev_e {
            nn *= p; // nn = n*p^2, n*p^3, n*p^4, ...
            if nn > NN / p {
                break;
            }
            find_powerfuls(index + 1, nn * p, e, primes, ans); // pass nn*p
        }
        index += 1;
    }
}

fn main() {
    let limit = (NN as f64).sqrt() as usize + 1;

    // Sieve primes up to sqrt(NN)
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
    let primes: Vec<i32> = (2..=limit).filter(|&i| is_prime[i]).map(|i| i as i32).collect();

    let mut ans: i64 = 0;
    find_powerfuls(0, 1, 999, &primes, &mut ans);

    println!("{}", ans);
}
