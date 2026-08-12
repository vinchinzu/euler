// Project Euler 772 - Balanceable Partitions
// Answer is 2 * LCM(1..N) mod (10^9+7).

fn main() {
    const N: usize = 100_000_000;
    const M: u64 = 1_000_000_007;

    #[inline]
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

    // Odd-only sieve
    let n_odds = (N + 1) / 2;
    let mut is_odd_prime = vec![true; n_odds];
    is_odd_prime[0] = false;
    let mut i = 1usize;
    while {
        let p = 2 * i + 1;
        p * p <= N
    } {
        if is_odd_prime[i] {
            let p = 2 * i + 1;
            let mut j = (p * p) / 2;
            while j < n_odds {
                is_odd_prime[j] = false;
                j += p;
            }
        }
        i += 1;
    }

    let mut ans: u64 = 2;
    // p=2
    {
        let mut pe = 2u64;
        let mut exp = 1u64;
        while pe <= N as u64 / 2 {
            pe *= 2;
            exp += 1;
        }
        ans = (ans as u128 * pow_mod(2, exp, M) as u128 % M as u128) as u64;
    }
    for i in 1..n_odds {
        if !is_odd_prime[i] {
            continue;
        }
        let p = 2 * i + 1;
        if p > N {
            break;
        }
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
