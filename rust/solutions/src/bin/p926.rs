// Project Euler 926 - Total Roundness
// Compute total roundness of n! for n=10^7

const MOD: u64 = 1_000_000_007;

fn main() {
    let n = 10_000_000usize;

    // Sieve of Eratosthenes
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    if n >= 1 { is_prime[1] = false; }
    let mut i = 2;
    while i * i <= n {
        if is_prime[i] {
            let mut j = i * i;
            while j <= n {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }

    // Collect primes and compute exponents in n! using Legendre's formula
    let mut exponents: Vec<u64> = Vec::new();
    for i in 2..=n {
        if is_prime[i] {
            let mut count = 0u64;
            let mut power = i as u64;
            while power <= n as u64 {
                count += n as u64 / power;
                power *= i as u64;
            }
            exponents.push(count);
        }
    }

    // Sort exponents descending for early termination
    exponents.sort_unstable_by(|a, b| b.cmp(a));

    let max_v = exponents[0];
    let mut total = 0u64;

    for j in 1..=max_v {
        let mut product = 1u64;
        let mut all_one = true;

        for &vp in &exponents {
            if vp < j { break; }
            let factor = 1 + vp / j;
            product = product * (factor % MOD) % MOD;
            if factor > 1 { all_one = false; }
        }

        if !all_one {
            let contribution = (product + MOD - 1) % MOD;
            total = (total + contribution) % MOD;
        }
    }

    println!("{}", total);
}
