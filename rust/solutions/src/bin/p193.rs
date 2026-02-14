// Project Euler 193: Squarefree Numbers
//
// Count squarefree numbers below 2^50 using Mobius function inclusion-exclusion.

fn main() {
    let limit: u64 = 1u64 << 50;
    let sqrt_limit = {
        let mut s = (limit as f64).sqrt() as u64;
        while (s + 1) * (s + 1) <= limit { s += 1; }
        while s * s > limit { s -= 1; }
        s as usize
    };

    // Linear sieve for Mobius function
    let mut mu = vec![0i8; sqrt_limit + 1];
    let mut composite = vec![false; sqrt_limit + 1];
    let mut primes: Vec<usize> = Vec::with_capacity(sqrt_limit / 2);

    mu[1] = 1;
    for i in 2..=sqrt_limit {
        if !composite[i] {
            primes.push(i);
            mu[i] = -1;
        }
        for &p in &primes {
            let ip = i * p;
            if ip > sqrt_limit { break; }
            composite[ip] = true;
            if i % p == 0 {
                mu[ip] = 0;
                break;
            } else {
                mu[ip] = -mu[i];
            }
        }
    }

    let mut total: i64 = 0;
    for d in 1..=sqrt_limit {
        if mu[d] == 0 { continue; }
        total += mu[d] as i64 * (limit / (d as u64 * d as u64)) as i64;
    }

    println!("{}", total);
}
