// Project Euler 802 - Iterated Composition
// S = sum_{d=1}^N mu(d) * 2^floor(N/d) mod M

fn mod_pow(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
    let mut result: i64 = 1;
    base %= modulus;
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result as i128 * base as i128 % modulus as i128) as i64;
        }
        base = (base as i128 * base as i128 % modulus as i128) as i64;
        exp >>= 1;
    }
    result
}

fn main() {
    let n: usize = 10_000_000;
    let m: i64 = 1_020_340_567;

    // Linear sieve for Mobius function
    let mut mu = vec![0i8; n + 1];
    mu[1] = 1;
    let mut smallest_prime = vec![0u32; n + 1];
    let mut primes = Vec::with_capacity(700_000);

    for i in 2..=n {
        if smallest_prime[i] == 0 {
            smallest_prime[i] = i as u32;
            primes.push(i);
            mu[i] = -1;
        }
        for &p in &primes {
            if p > smallest_prime[i] as usize || i * p > n {
                break;
            }
            smallest_prime[i * p] = p as u32;
            if i % p == 0 {
                mu[i * p] = 0;
            } else {
                mu[i * p] = -mu[i];
            }
        }
    }

    let mut ans: i64 = 0;
    for d in 1..=n {
        if mu[d] == 0 { continue; }
        let p = mod_pow(2, (n / d) as i64, m);
        if mu[d] == 1 {
            ans = (ans + p) % m;
        } else {
            ans = (ans - p + m) % m;
        }
    }

    println!("{}", ans);
}
