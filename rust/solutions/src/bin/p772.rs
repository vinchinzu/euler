// Project Euler 772 - Balanceable Partitions
// Answer is 2 * LCM(1..N) mod (10^9+7).

fn main() {
    const N: usize = 100_000_000;
    const M: u64 = 1_000_000_007;

    fn pow_mod(mut base: u64, mut exp: u64, m: u64) -> u64 {
        let mut result = 1u64;
        base %= m;
        while exp > 0 {
            if exp & 1 == 1 {
                result = (result as u128 * base as u128 % m as u128) as u64;
            }
            base = (base as u128 * base as u128 % m as u128) as u64;
            exp >>= 1;
        }
        result
    }

    // Sieve
    let mut is_prime = vec![true; N + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let sq = (N as f64).sqrt() as usize;
    for i in 2..=sq {
        if is_prime[i] {
            let mut j = i * i;
            while j <= N {
                is_prime[j] = false;
                j += i;
            }
        }
    }

    let mut ans: u64 = 2;
    for p in 2..=N {
        if !is_prime[p] { continue; }
        let mut pe = p as u64;
        let mut exp = 1u64;
        while pe <= N as u64 / p as u64 {
            pe *= p as u64;
            exp += 1;
        }
        ans = (ans as u128 * pow_mod(p as u64, exp, M) as u128 % M as u128) as u64;
    }

    println!("{}", ans);
}
