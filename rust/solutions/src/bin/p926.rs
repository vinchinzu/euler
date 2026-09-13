// Project Euler 926 - Total Roundness
// Compute total roundness of n! for n=10^7

use euler_utils::primes::primes_up_to;

const MOD: u64 = 1_000_000_007;

fn main() {
    let n = 10_000_000usize;

    // Get all primes up to n using optimized library function
    let primes = primes_up_to(n);

    // Compute exponents in n! using Legendre's formula
    let mut exponents: Vec<usize> = Vec::with_capacity(primes.len());
    for &p in &primes {
        let mut count = 0usize;
        let mut power = p;
        while power <= n {
            count += n / power;
            power *= p;
        }
        exponents.push(count);
    }

    // Sort exponents descending for early termination
    exponents.sort_unstable_by(|a, b| b.cmp(a));

    let max_v = exponents[0];
    let exp_len = exponents.len();
    let mut total = 0u64;

    for j in 1..=max_v {
        let mut product = 1u64;
        let mut all_one = true;
        let mut idx = 0;

        // Hot loop: use unsafe get_unchecked for proven safe bounds
        // SAFETY: idx starts at 0 and only increments while idx < exp_len,
        // so all accesses are within bounds of exponents vector
        while idx < exp_len {
            let vp = unsafe { *exponents.get_unchecked(idx) };
            if vp < j { break; }
            let factor = (1 + vp / j) as u64;
            product = (product * factor) % MOD;
            if factor > 1 { all_one = false; }
            idx += 1;
        }

        if !all_one {
            let contribution = (product + MOD - 1) % MOD;
            total = (total + contribution) % MOD;
        }
    }

    println!("{}", total);
}
