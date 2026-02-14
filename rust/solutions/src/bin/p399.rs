// Project Euler 399: Square-free Fibonacci
// Find the N-th square-free Fibonacci index, then compute F(index) mod 10^16.

fn main() {
    let n: usize = 100_000_000;
    let l: usize = 2 * n;
    let m: i64 = 10_000_000_000_000_000; // 10^16

    // Bitpacked sieve for square-free Fibonacci indices
    let mut sqf = vec![true; l];

    // Bitpacked prime sieve
    let bytes = (l >> 3) + 1;
    let mut prime_sieve = vec![0xFFu8; bytes];
    prime_sieve[0] &= !(1 << 0); // 0 not prime
    prime_sieve[0] &= !(1 << 1); // 1 not prime

    let mut i = 2usize;
    while i * i < l {
        if (prime_sieve[i >> 3] >> (i & 7)) & 1 == 1 {
            let mut j = i * i;
            while j < l {
                prime_sieve[j >> 3] &= !(1 << (j & 7));
                j += i;
            }
        }
        i += 1;
    }

    let is_prime = |n: usize| -> bool {
        if n < 2 { return false; }
        (prime_sieve[n >> 3] >> (n & 7)) & 1 == 1
    };

    // For each prime p, find the first Fibonacci index divisible by p^2
    for p in 2..l {
        if !is_prime(p) { continue; }

        let mut first_index: usize = 1;
        let pm = p as i64;
        let mut a: i64 = 1 % pm;
        let mut b: i64 = 1 % pm;

        while p * first_index < l {
            if a == 0 {
                let step = p * first_index;
                let mut idx = step;
                while idx < l {
                    sqf[idx] = false;
                    idx += step;
                }
                break;
            }
            let new_b = (a + b) % pm;
            a = b;
            b = new_b;
            first_index += 1;
        }
    }

    // Find the N-th square-free index
    let mut index: i64 = -1;
    let mut count: usize = 0;
    let mut idx = 0usize;
    while count < n {
        if sqf[idx] {
            count += 1;
            if count == n {
                index = idx as i64;
            }
        }
        idx += 1;
    }

    // Compute Fibonacci(index) mod M
    let mut fa: i64 = 1;
    let mut fb: i64 = 1;
    for _ in 0..index {
        let new_fb = (fa + fb) % m;
        fa = fb;
        fb = new_fb;
    }

    // Compute scientific notation
    let phi: f64 = (1.0 + 5.0_f64.sqrt()) / 2.0;
    let log_value = (index + 1) as f64 * phi.log10() - 5.0_f64.sqrt().log10();
    let exponent = log_value as i64;
    let mantissa = 10.0_f64.powf(log_value - exponent as f64);

    println!("{},{:.1}e{}", fa, mantissa, exponent);
}
