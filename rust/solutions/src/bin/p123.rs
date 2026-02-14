// Project Euler 123: Prime square remainders
//
// For odd n: remainder = 2*n*p_n mod p_n^2 = 2*n*p_n (when 2*n < p_n).
// Find smallest odd n such that 2*n*p_n > 10^10.

use euler_utils::primes_up_to;

fn main() {
    let primes = primes_up_to(300_000);
    let target: u64 = 10_000_000_000;

    for n in (1..=primes.len()).step_by(2) {
        let p = primes[n - 1] as u64;
        let remainder = 2 * n as u64 * p;
        if remainder > target {
            println!("{}", n);
            return;
        }
    }
}
