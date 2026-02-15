// Project Euler 734 - A Bit of Prime
//
// OR-convolution via Mobius/Zeta transform on bitwise OR.

const MOD: i64 = 1_000_000_007;

fn pow_mod(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut result: i64 = 1;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result as i128 * base as i128 % m as i128) as i64;
        }
        base = (base as i128 * base as i128 % m as i128) as i64;
        exp >>= 1;
    }
    result
}

fn main() {
    let n = 1_000_000usize;
    let k = 999_983i64;

    // Find L = smallest power of 2 >= N
    let mut l = 1;
    while l < n {
        l <<= 1;
    }

    // Sieve of Eratosthenes
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut i = 2;
    while i * i <= n {
        if is_prime[i] {
            let mut j = i * i;
            while j <= n {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }

    let primes: Vec<usize> = (2..=n).filter(|&p| is_prime[p]).collect();

    let mut a = vec![0i64; l];
    for &p in &primes {
        a[p] = 1;
    }

    // Forward Zeta transform for bitwise OR
    let mut u = 1;
    while u < l {
        for x in 0..l {
            if (x & u) == 0 {
                a[x | u] = (a[x | u] + a[x]) % MOD;
            }
        }
        u <<= 1;
    }

    // Raise to K-th power
    for i in 0..l {
        a[i] = pow_mod(a[i], k, MOD);
    }

    // Inverse Mobius transform
    let mut u = 1;
    while u < l {
        for x in 0..l {
            if (x & u) == 0 {
                a[x | u] = (a[x | u] - a[x] % MOD + MOD) % MOD;
            }
        }
        u <<= 1;
    }

    let mut ans: i64 = 0;
    for &p in &primes {
        if p < l {
            ans = (ans + a[p]) % MOD;
        }
    }

    println!("{}", ans);
}
