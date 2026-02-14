// Project Euler 153 - Investigating Gaussian Integers
// Sum of all divisors (including Gaussian) of all n from 1 to 10^8.

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

/// G(n) = sum_{k=1}^{n} sigma_1(k) = sum_{d=1}^{n} d * floor(n/d)
/// Computed via hyperbola method (grouping by quotient).
fn g_function(n: i64) -> i64 {
    if n <= 0 { return 0; }
    let mut result: i64 = 0;
    let mut k: i64 = 1;
    while k <= n {
        let q = n / k;
        let next_k = n / q + 1;
        let last_k = std::cmp::min(next_k - 1, n);
        let count = last_k - k + 1;
        let sum_k = count * (k + last_k) / 2;
        result += sum_k * q;
        k = next_k;
    }
    result
}

fn main() {
    let n: i64 = 100_000_000;
    let sqrt_limit = (n as f64).sqrt() as i64;

    // S1 = G(N) - sum of real divisor sums
    let total_sum = g_function(n);

    let mut s2_prime: i64 = 0;

    // Case u = v = 1: d = 2, contribution = G(N/2)
    s2_prime += g_function(n / 2);

    // Case u > v >= 1, gcd(u,v) = 1
    for u in 2..=sqrt_limit {
        for v in 1..u {
            if gcd(u, v) != 1 { continue; }
            let d = u * u + v * v;
            if d > n { break; }
            s2_prime += (u + v) * g_function(n / d);
        }
    }

    println!("{}", total_sum + 2 * s2_prime);
}
