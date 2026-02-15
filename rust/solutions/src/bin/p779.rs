// Project Euler 779 - Prime Factor and Order
// Sum over primes p of 1/(p*(p-1)^2) * product_{q<p} (1-1/q).

fn main() {
    const LIMIT: usize = 100_000;

    let mut is_prime = vec![true; LIMIT + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=LIMIT {
        if is_prime[i] {
            let mut j = i * i;
            while j <= LIMIT {
                is_prime[j] = false;
                j += i;
            }
        }
    }

    let primes: Vec<f64> = (2..=LIMIT).filter(|&i| is_prime[i]).map(|i| i as f64).collect();

    let mut ans = 0.0f64;
    let mut prod = 1.0f64;

    for &p in &primes {
        let res = prod / (p * (p - 1.0) * (p - 1.0));
        ans += res;
        prod *= 1.0 - 1.0 / p;
    }

    println!("{:.12}", ans);
}
