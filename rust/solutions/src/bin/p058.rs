// Project Euler 58: Spiral primes
// Find the side length at which the ratio of primes on the diagonals of an Ulam spiral
// first falls below 10%.
use euler_utils::is_prime;

fn main() {
    let mut prime_count = 0u64;
    let mut total_diagonals = 1u64;
    let mut side: u64 = 1;

    loop {
        side += 2;
        let br = side * side;
        let bl = br - (side - 1);
        let tl = bl - (side - 1);
        let tr = tl - (side - 1);

        if is_prime(bl) { prime_count += 1; }
        if is_prime(tl) { prime_count += 1; }
        if is_prime(tr) { prime_count += 1; }

        total_diagonals += 4;

        if prime_count * 10 < total_diagonals {
            println!("{side}");
            return;
        }
    }
}
