// Project Euler Problem 826: Birds on a Wire
// Computes sum of F(p) for primes p <= n, where F(n) = (7n+15)/(18(n+1))
// Returns the average rounded to 10 decimal places

fn prime_sieve(limit: usize) -> Vec<usize> {
    if limit < 2 {
        return Vec::new();
    }
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=((limit as f64).sqrt() as usize) {
        if is_prime[i] {
            for j in ((i * i)..=limit).step_by(i) {
                is_prime[j] = false;
            }
        }
    }
    is_prime
        .iter()
        .enumerate()
        .filter(|&(_, &b)| b)
        .map(|(i, _)| i)
        .collect()
}

fn f(n: usize) -> f64 {
    let num = 7.0 * n as f64 + 15.0;
    let den = 18.0 * (n as f64 + 1.0);
    num / den
}

fn compute(n: usize) -> f64 {
    let primes = prime_sieve(n);
    if primes.len() <= 1 {
        return 0.0;
    }
    // Skip first prime (2) as per original
    let total: f64 = primes.iter().skip(1).map(|&p| f(p)).sum();
    total / (primes.len() - 1) as f64
}

fn main() {
    // Test: F(3) should be 0.5
    assert!((f(3) - 0.5).abs() < 1e-10);

    let result = compute(1_000_000);
    println!("{:.10}", result);
}
