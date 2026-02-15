// Project Euler 800 - Hybrid Integers
// Count pairs (p,q) with p<q both prime where p*log(q) + q*log(p) < N*log(N)
fn main() {
    let n: u64 = 800800;
    let max_prime = (n as usize) * 21;

    // Sieve of Eratosthenes
    let mut is_prime = vec![true; max_prime + 1];
    is_prime[0] = false;
    is_prime[1] = false;
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

    let primes: Vec<usize> = (2..=max_prime).filter(|&i| is_prime[i]).collect();
    let log_primes: Vec<f64> = primes.iter().map(|&p| (p as f64).ln()).collect();

    let limit = n as f64 * (n as f64).ln();
    let num_primes = primes.len();

    let mut ans: i64 = 0;
    for i in 0..num_primes {
        let p = primes[i] as f64;
        let lp = log_primes[i];

        // Binary search for largest j > i where p*log(q) + q*log(p) < limit
        let mut low = i as i64;
        let mut high = num_primes as i64;
        while low + 1 < high {
            let mid = (low + high) / 2;
            let val = p * log_primes[mid as usize] + primes[mid as usize] as f64 * lp;
            if val < limit {
                low = mid;
            } else {
                high = mid;
            }
        }
        ans += low - i as i64;
    }

    println!("{}", ans);
}
