// Project Euler 268: Counting numbers with at least four distinct
// prime factors less than 100.
//
// Inclusion-exclusion over subsets of primes < 100.
// For each subset of size >= 4, weight = (-1)^(s-4) * C(s-1, 3).

const PRIMES: [u64; 25] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47,
    53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
];

fn ncr(n: i64, r: i64) -> i64 {
    if r < 0 || r > n {
        return 0;
    }
    let r = r.min(n - r);
    let mut result: i64 = 1;
    for i in 0..r {
        result = result * (n - i) / (i + 1);
    }
    result
}

fn helper(index: usize, s: i64, prod: u64, n: u64, ans: &mut i64) {
    if index == PRIMES.len() {
        if s >= 4 {
            let parity = if (s - 4) % 2 == 0 { 1i64 } else { -1i64 };
            let weight = parity * ncr(s - 1, 3);
            *ans += weight * (n / prod) as i64;
        }
        return;
    }

    // Don't include this prime
    helper(index + 1, s, prod, n, ans);

    // Include this prime (if product doesn't exceed n)
    if prod <= n / PRIMES[index] {
        helper(index + 1, s + 1, prod * PRIMES[index], n, ans);
    }
}

fn main() {
    let n: u64 = 10_000_000_000_000_000; // 10^16
    let mut ans: i64 = 0;
    helper(0, 0, 1, n, &mut ans);
    println!("{}", ans);
}
