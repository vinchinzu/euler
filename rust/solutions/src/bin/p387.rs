// Project Euler 387 - Strong right truncatable Harshad primes
// Find the sum of strong, right truncatable Harshad primes below 10^14.

use euler_utils::is_prime;
use std::collections::VecDeque;

const LIMIT: u64 = 100_000_000_000_000; // 10^14
const MAX_M: u64 = 10_000_000_000_000;  // 10^13

fn main() {
    let mut total_sum: u64 = 0;

    // BFS queue: (number, digit_sum)
    let mut queue: VecDeque<(u64, u32)> = VecDeque::new();

    // Initialize with single-digit Harshad numbers (1-9)
    for d in 1..=9u64 {
        queue.push_back((d, d as u32));
    }

    while let Some((num, digit_sum)) = queue.pop_front() {
        // Check if num is a strong Harshad number
        if digit_sum > 0 && num % digit_sum as u64 == 0 && is_prime(num / digit_sum as u64) {
            // Try appending digits 0-9 to form a prime
            for d in 0..=9u64 {
                let candidate = num * 10 + d;
                if candidate < LIMIT && is_prime(candidate) {
                    total_sum += candidate;
                }
            }
        }

        // Extend right-truncatable Harshad numbers
        if num < MAX_M {
            for d in 0..=9u64 {
                let new_num = num * 10 + d;
                let new_ds = digit_sum + d as u32;
                if new_ds > 0 && new_num % new_ds as u64 == 0 {
                    queue.push_back((new_num, new_ds));
                }
            }
        }
    }

    println!("{}", total_sum);
}
