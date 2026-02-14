// Project Euler 70: Totient permutation
// Find n < 10^7 where phi(n) is a digit permutation of n, minimizing n/phi(n).
// Best candidates are products of two primes near sqrt(10^7).

use euler_utils::primes_up_to;

fn are_permutations(mut a: u64, mut b: u64) -> bool {
    let mut counts = [0i32; 10];
    while a > 0 {
        counts[(a % 10) as usize] += 1;
        a /= 10;
    }
    while b > 0 {
        counts[(b % 10) as usize] -= 1;
        b /= 10;
    }
    counts.iter().all(|&c| c == 0)
}

fn main() {
    let limit = 10_000_000u64;
    let primes = primes_up_to(5000);

    let mut min_ratio = f64::MAX;
    let mut result_n = 0u64;

    for (i, &p1) in primes.iter().enumerate() {
        if (p1 as u64) * (p1 as u64) >= limit { break; }
        for &p2 in &primes[i..] {
            let n = p1 as u64 * p2 as u64;
            if n >= limit { break; }
            if p1 == p2 { continue; }

            let phi_n = (p1 as u64 - 1) * (p2 as u64 - 1);
            if are_permutations(n, phi_n) {
                let ratio = n as f64 / phi_n as f64;
                if ratio < min_ratio {
                    min_ratio = ratio;
                    result_n = n;
                }
            }
        }
    }

    println!("{result_n}");
}
