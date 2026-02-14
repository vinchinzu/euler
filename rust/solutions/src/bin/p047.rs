// Project Euler 47: Distinct primes factors
// Find the first of four consecutive integers each having four distinct prime factors.

fn main() {
    let limit = 200_000usize;
    let mut num_factors = vec![0u8; limit + 1];

    // Sieve: for each prime p, increment count for all multiples
    for p in 2..=limit {
        if num_factors[p] == 0 {
            // p is prime
            let mut j = p;
            while j <= limit {
                num_factors[j] += 1;
                j += p;
            }
        }
    }

    for i in 2..=limit - 3 {
        if num_factors[i] == 4
            && num_factors[i + 1] == 4
            && num_factors[i + 2] == 4
            && num_factors[i + 3] == 4
        {
            println!("{i}");
            return;
        }
    }
}
