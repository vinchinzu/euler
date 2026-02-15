// Project Euler 758 - Buckets of Water
// For primes p < q < 1000, compute P(2^{p^5}-1, 2^{q^5}-1) using extended GCD on exponents.

fn main() {
    const MOD: i64 = 1_000_000_007;
    const MAXN: usize = 1001;

    // Sieve primes up to 1000
    let mut is_prime = vec![true; MAXN];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..MAXN {
        if is_prime[i] {
            let mut j = i * i;
            while j < MAXN {
                is_prime[j] = false;
                j += i;
            }
        }
    }
    let primes: Vec<i64> = (2..MAXN).filter(|&i| is_prime[i]).map(|i| i as i64).collect();

    fn pow_mod(mut base: i64, mut exp: i64, m: i64) -> i64 {
        let mut result = 1i64;
        base = ((base % m) + m) % m;
        while exp > 0 {
            if exp & 1 == 1 {
                result = (result as i128 * base as i128 % m as i128) as i64;
            }
            exp >>= 1;
            base = (base as i128 * base as i128 % m as i128) as i64;
        }
        result
    }

    fn mod_inv(a: i64, m: i64) -> i64 {
        pow_mod(a, m - 2, m)
    }

    fn lin_comb(e: i64, f: i64, m: i64) -> (i64, i64, i32) {
        if f == 0 {
            return (1, 0, 1);
        }

        let (prev_x, prev_y, prev_sign) = lin_comb(f, e % f, m);

        let num = (pow_mod(2, e, m) - pow_mod(2, e % f, m) + m) % m;
        let den = (pow_mod(2, f, m) - 1 + m) % m;
        let quotient = (num as i128 * mod_inv(den, m) as i128 % m as i128) as i64;

        let new_y = (prev_x as i128 - quotient as i128 * prev_y as i128 % m as i128 + m as i128 * 2) as i64 % m;
        (prev_y, new_y, -prev_sign)
    }

    let mut ans: i64 = 0;

    for i in 0..primes.len() {
        let p = primes[i];
        let e = p * p * p * p * p;
        for j in (i + 1)..primes.len() {
            let q = primes[j];
            let f = q * q * q * q * q;
            let (x, y, sign) = lin_comb(e, f, MOD);
            let k = ((sign as i64 * ((x - y + MOD) % MOD) % MOD + MOD) % MOD - 1 + MOD) % MOD;
            ans = (ans + 2 * k) % MOD;
        }
    }

    println!("{}", ans);
}
