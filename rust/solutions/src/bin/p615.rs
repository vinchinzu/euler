// Project Euler 615 - The millionth number with at least one million prime factors
// Priority queue approach: enumerate numbers with Omega(n) >= 1,000,000 in ascending order.
//
// Key insight: each such number = 2^(1000000 - k) * m, where m has k prime factors (>= 2).
// We track a "variable part" with ~27 factors in a min-heap, ordered by integer value.
// Since all candidates get multiplied by the same power of 2 (when factor count matches),
// ordering by the variable-part value correctly orders the full numbers.
// At the end, multiply by 2^(1000000 - 27) mod 123454321.

use std::collections::BinaryHeap;
use std::cmp::Reverse;

const NUM_CANDIDATES: usize = 1_000_000;
const MODULO: u64 = 123_454_321;

fn main() {
    // Sieve primes up to empirical bound (15770th prime = 173207 suffices)
    let max_prime: usize = 173_207;
    let mut is_prime = vec![true; max_prime + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    {
        let mut i = 2;
        while i * i <= max_prime {
            if is_prime[i] {
                let mut j = i * i;
                while j <= max_prime {
                    is_prime[j] = false;
                    j += i;
                }
            }
            i += 1;
        }
    }
    let primes: Vec<u64> = (2..=max_prime as u64).filter(|&x| is_prime[x as usize]).collect();

    let num_var_factors: u32 = 27;
    let seed: u64 = 1u64 << num_var_factors; // 2^27

    // Min-heap of (value, largest_prime_factor)
    let mut heap: BinaryHeap<Reverse<(u64, u64)>> = BinaryHeap::new();
    heap.push(Reverse((seed, 2)));

    // Upper bound on variable-part values we need to consider
    let too_large: u64 = (seed / 2) * (*primes.last().unwrap());

    let mut previous: u64 = 0;

    for _ in 0..NUM_CANDIDATES {
        // Pop next candidate, skipping duplicates
        let Reverse((mut current, mut max_factor)) = heap.pop().expect("heap empty");
        while current == previous {
            let Reverse((v, mf)) = heap.pop().expect("heap empty");
            current = v;
            max_factor = mf;
        }
        previous = current;

        // Generate successors:
        // 1. "Append" a prime p >= max_factor: variable_part * p (increases factor count by 1)
        for &p in &primes {
            if p < max_factor { continue; }
            match current.checked_mul(p) {
                Some(nv) if nv < too_large => heap.push(Reverse((nv, p))),
                _ => break,
            }
        }

        // 2. "Replace a 2" with prime p > 2, p >= max_factor: (variable_part / 2) * p
        //    C++ code does integer division current/2 even for odd numbers
        {
            let half = current / 2;
            for &p in &primes {
                let nv = half.checked_mul(p);
                match nv {
                    Some(nv) if nv >= too_large => break,
                    None => break,
                    _ => {}
                }
                if p >= max_factor && p > 2 {
                    heap.push(Reverse((nv.unwrap(), p)));
                }
            }
        }
    }

    // The millionth number: previous * 2^(1000000 - 27) mod MODULO
    let mut result = previous % MODULO;
    for _ in num_var_factors as usize..NUM_CANDIDATES {
        result = (result * 2) % MODULO;
    }

    println!("{}", result);
}
