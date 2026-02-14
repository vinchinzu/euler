// Project Euler 72: Counting fractions
// Sum of phi(n) for n = 2..1,000,000 using sieve-based Euler totient.

fn main() {
    let limit = 1_000_000usize;
    let mut phi = vec![0u64; limit + 1];
    for i in 0..=limit {
        phi[i] = i as u64;
    }

    for i in 2..=limit {
        if phi[i] == i as u64 {
            // i is prime
            let mut j = i;
            while j <= limit {
                phi[j] -= phi[j] / i as u64;
                j += i;
            }
        }
    }

    let total: u64 = phi[2..=limit].iter().sum();
    println!("{total}");
}
