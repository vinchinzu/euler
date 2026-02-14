// Project Euler 211 - Divisor Square Sum
//
// Find the sum of all k < 64,000,000 such that sigma_2(k) is a perfect square.
// sigma_2(k) = sum of d^2 for all divisors d of k.
// Use multiplicative sieve via smallest prime factor decomposition.

use euler_utils::sieve_smallest_factor;

const LIMIT: usize = 64_000_000;

fn main() {
    let spf = sieve_smallest_factor(LIMIT);

    // Compute sigma_2 using SPF
    let mut sig2 = vec![0u64; LIMIT + 1];
    sig2[1] = 1;

    for i in 2..=LIMIT {
        let p = spf[i] as u64;
        let mut n = i;
        let mut mult: u64 = 1;
        while n > 0 && spf[n] as u64 == p {
            n /= p as usize;
            mult = mult * p * p + 1;
        }
        sig2[i] = sig2[n] * mult;
    }

    drop(spf);

    let mut ans: u64 = 0;
    for k in 1..LIMIT {
        let s = sig2[k];
        let r = (s as f64).sqrt() as u64;
        // Check r-1, r, r+1 to handle floating point imprecision
        if r * r == s || (r + 1) * (r + 1) == s || (r > 0 && (r - 1) * (r - 1) == s) {
            ans += k as u64;
        }
    }

    println!("{}", ans);
}
