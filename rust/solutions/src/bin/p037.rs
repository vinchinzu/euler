// Project Euler 037: Truncatable Primes
// Sum of the eleven primes truncatable both left-to-right and right-to-left.

use euler_utils::is_prime;

fn main() {
    let mut count = 0;
    let mut sum: u64 = 0;
    let mut n = 11u64;

    while count < 11 {
        if is_prime(n) && is_left_truncatable(n) && is_right_truncatable(n) {
            sum += n;
            count += 1;
        }
        n += 2;
        if n % 5 == 0 {
            n += 2;
        }
    }

    println!("{sum}");
}

fn is_left_truncatable(n: u64) -> bool {
    let mut pow10 = 10u64;
    while pow10 < n {
        if !is_prime(n % pow10) {
            return false;
        }
        pow10 *= 10;
    }
    true
}

fn is_right_truncatable(n: u64) -> bool {
    let mut tmp = n / 10;
    while tmp > 0 {
        if !is_prime(tmp) {
            return false;
        }
        tmp /= 10;
    }
    true
}
