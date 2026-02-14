// Project Euler Problem 179: Consecutive positive divisors
// Count n in [2, 10^7-1] where tau(n) == tau(n+1).
// Uses linear sieve to compute tau (number of divisors).

fn main() {
    const LIMIT: usize = 10_000_000;

    let mut spf = vec![0u32; LIMIT + 1]; // smallest prime factor
    let mut tau = vec![1u32; LIMIT + 1]; // number of divisors
    let mut exponent = vec![0u8; LIMIT + 1];
    let mut primes: Vec<u32> = Vec::with_capacity(700_000);

    for i in 2..=LIMIT {
        if spf[i] == 0 {
            spf[i] = i as u32;
            primes.push(i as u32);
            tau[i] = 2;
            exponent[i] = 1;
        }

        for &p in &primes {
            let pi = p as usize * i;
            if pi > LIMIT {
                break;
            }
            spf[pi] = p;

            if p == spf[i] as u32 {
                exponent[pi] = exponent[i] + 1;
                tau[pi] = tau[i] / (exponent[i] as u32 + 1) * (exponent[pi] as u32 + 1);
                break;
            } else {
                exponent[pi] = 1;
                tau[pi] = tau[i] * 2;
            }
        }
    }

    let count = (2..LIMIT).filter(|&n| tau[n] == tau[n + 1]).count();
    println!("{}", count);
}
