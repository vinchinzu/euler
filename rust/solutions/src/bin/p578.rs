// Project Euler 578 - Integers with Decreasing Prime Powers
//
// Count n <= N where the exponents in prime factorization are non-increasing.
// Uses inclusion-exclusion: find_powerfuls enumerates the "powerful" prefix
// (primes with exponent >= 2), find_remaining counts squarefree/inclusion-exclusion
// for the remaining primes.

const NN: i64 = 10_000_000_000_000; // 10^13

fn sieve_primes(limit: usize) -> Vec<i64> {
    let mut is_p = vec![true; limit + 1];
    is_p[0] = false;
    if limit >= 1 { is_p[1] = false; }
    let mut i = 2;
    while i * i <= limit {
        if is_p[i] {
            let mut j = i * i;
            while j <= limit { is_p[j] = false; j += i; }
        }
        i += 1;
    }
    (2..=limit).filter(|&i| is_p[i]).map(|i| i as i64).collect()
}

fn parity(n: i32) -> i64 {
    if n % 2 == 0 { 1 } else { -1 }
}

fn find_remaining(primes: &[i64], min_index: usize, n: i64, count: i32, threshold: usize, ans: &mut i64) {
    *ans += (NN / n) * parity(count);
    for index in min_index..primes.len() {
        let p = primes[index];
        if index >= threshold {
            if n > NN / (p * p) { break; }
            find_remaining(primes, index + 1, n * p * p, count + 1, threshold, ans);
        } else {
            if n > NN / p { break; }
            find_remaining(primes, index + 1, n * p, count + 1, threshold, ans);
        }
    }
}

fn find_powerfuls(primes: &[i64], min_index: usize, n: i64, prev_e: i32, ans: &mut i64) {
    find_remaining(primes, 0, n, 0, min_index, ans);
    for index in min_index..primes.len() {
        let p = primes[index];
        if n > NN / (p * p) { break; }
        let mut nn = n * p;
        for e in 2..=prev_e {
            nn *= p;
            if nn > NN { break; }
            find_powerfuls(primes, index + 1, nn, e, ans);
        }
    }
}

fn main() {
    // Spawn with large stack to handle deep recursion at N=10^13
    let builder = std::thread::Builder::new().stack_size(64 * 1024 * 1024);
    let handler = builder.spawn(|| {
        let limit = (NN as f64).sqrt() as usize + 1;
        let primes = sieve_primes(limit);
        let mut ans: i64 = 0;
        find_powerfuls(&primes, 0, 1, 999, &mut ans);
        println!("{}", ans);
    }).unwrap();
    handler.join().unwrap();
}
