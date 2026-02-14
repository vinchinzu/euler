// Project Euler 515 - Dissonant Numbers
// Sum d(p, p-1, K) for all primes A <= p < A+B, where d(p, p-1, K) = mod_inv(K-1, p).

use euler_utils::{is_prime, mod_inv};

fn main() {
    let a: u64 = 1_000_000_000;
    let b: u64 = 100_000;
    let k: u64 = 100_000;

    let mut ans: u64 = 0;
    for p in a..a + b {
        if is_prime(p) {
            if let Some(inv) = mod_inv(k - 1, p) {
                ans += inv;
            }
        }
    }

    println!("{}", ans);
}
