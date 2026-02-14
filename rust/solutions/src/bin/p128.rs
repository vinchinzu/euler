// Project Euler 128: Hexagonal tile differences
//
// Find the 2000th tile n for which PD(n) = 3.
// Only two positions per ring can have PD=3:
//   Type A: n = 3r^2 - 3r + 2, needs 6r-1, 6r+1, 12r+5 all prime
//   Type B: n = 3r^2 + 3r + 1, needs 6r-1, 6r+5, 12r-7 all prime (r >= 2)

use euler_utils::sieve;

fn main() {
    const MAX_RING: usize = 70_000;
    const MAX_PRIME: usize = 12 * MAX_RING + 5;
    const TARGET: usize = 2000;

    let is_prime = sieve(MAX_PRIME);

    let mut count = 2usize; // tiles 1 and 2 both have PD=3

    for r in 2..MAX_RING {
        // Type A: n = 3r^2 - 3r + 2
        if is_prime[6 * r - 1] && is_prime[6 * r + 1] && is_prime[12 * r + 5] {
            count += 1;
            if count == TARGET {
                let n: u64 = 3 * (r as u64) * (r as u64) - 3 * (r as u64) + 2;
                println!("{}", n);
                return;
            }
        }

        // Type B: n = 3r^2 + 3r + 1
        if is_prime[6 * r - 1] && is_prime[6 * r + 5] && is_prime[12 * r - 7] {
            count += 1;
            if count == TARGET {
                let n: u64 = 3 * (r as u64) * (r as u64) + 3 * (r as u64) + 1;
                println!("{}", n);
                return;
            }
        }
    }
}
