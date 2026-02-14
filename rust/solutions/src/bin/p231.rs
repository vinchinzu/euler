// Project Euler 231: Prime Factorisation of Binomial Coefficients
//
// Sum of prime factors (with multiplicity) of C(20000000, 15000000).
// Uses smallest-prime-factor sieve to decompose numerator and denominator terms.

use euler_utils::sieve_smallest_factor;

fn main() {
    const N: usize = 20_000_000;
    const K: usize = 15_000_000;

    let spf = sieve_smallest_factor(N);

    let mut ans: i64 = 0;

    for i in 0..(N - K) {
        // Numerator factor: N - i
        let mut n = (N - i) as u32;
        while n > 1 {
            ans += spf[n as usize] as i64;
            n /= spf[n as usize];
        }

        // Denominator factor: i + 1
        let mut n = (i + 1) as u32;
        while n > 1 {
            ans -= spf[n as usize] as i64;
            n /= spf[n as usize];
        }
    }

    println!("{}", ans);
}
