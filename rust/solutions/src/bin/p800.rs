// Project Euler 800 - Hybrid Integers
// Count pairs (p,q) with p<q both prime where p*log(q) + q*log(p) < N*log(N)
fn main() {
    let n: u64 = 800800;
    let max_prime = (n as usize) * 21;

    // Optimized sieve: only track odd numbers (wheel factorization with 2)
    let odd_size = (max_prime / 2) + 1;
    let mut is_prime_odd = vec![true; odd_size];
    is_prime_odd[0] = false; // 1 is not prime
    
    let mut i = 3;
    while i * i <= max_prime {
        let idx = i / 2;
        // SAFETY: idx = i/2 where i is odd and >= 3, so idx >= 1 < odd_size
        if unsafe { *is_prime_odd.get_unchecked(idx) } {
            let mut j = i * i;
            while j <= max_prime {
                let j_idx = j / 2;
                // SAFETY: j_idx = j/2 where j is odd and <= max_prime
                unsafe {
                    *is_prime_odd.get_unchecked_mut(j_idx) = false;
                }
                j += 2 * i; // Skip even multiples
            }
        }
        i += 2;
    }

    // Pre-allocate and collect primes with logs
    let approx_count = max_prime / (max_prime as f64).ln() as usize;
    let mut primes_f64 = Vec::with_capacity(approx_count);
    let mut log_primes = Vec::with_capacity(approx_count);
    
    // Add 2 first
    primes_f64.push(2.0);
    log_primes.push(2.0_f64.ln());
    
    // Add odd primes
    for i in (3..=max_prime).step_by(2) {
        let idx = i / 2;
        // SAFETY: idx = i/2 where i is odd <= max_prime
        if unsafe { *is_prime_odd.get_unchecked(idx) } {
            let p_f64 = i as f64;
            primes_f64.push(p_f64);
            log_primes.push(p_f64.ln());
        }
    }

    let limit = n as f64 * (n as f64).ln();
    let num_primes = primes_f64.len();

    let mut ans: i64 = 0;
    for i in 0..num_primes {
        // SAFETY: i < num_primes guaranteed by loop bounds
        let (p, lp) = unsafe {
            (*primes_f64.get_unchecked(i), *log_primes.get_unchecked(i))
        };

        // Binary search for largest j > i where p*log(q) + q*log(p) < limit
        let mut low = i;
        let mut high = num_primes;
        while low + 1 < high {
            let mid = (low + high) / 2;
            // SAFETY: mid is always in range [i, num_primes) by binary search invariant
            let val = unsafe {
                p * log_primes.get_unchecked(mid) + primes_f64.get_unchecked(mid) * lp
            };
            if val < limit {
                low = mid;
            } else {
                high = mid;
            }
        }
        ans += (low - i) as i64;
    }

    println!("{}", ans);
}
