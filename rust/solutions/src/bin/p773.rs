// Project Euler 773 - Numbers Relatively Prime to Primes Ending in 7
// Inclusion-exclusion with 97 primes ending in 7.

fn main() {
    const N_PRIMES: usize = 97;
    const MOD: i64 = 1_000_000_007;
    const B: i64 = 10;
    const DIGIT: i64 = 7;

    fn pow_mod(mut base: i64, mut exp: i64, m: i64) -> i64 {
        let mut result = 1i64;
        base = ((base % m) + m) % m;
        while exp > 0 {
            if exp & 1 == 1 {
                result = (result as i128 * base as i128 % m as i128) as i64;
            }
            base = (base as i128 * base as i128 % m as i128) as i64;
            exp >>= 1;
        }
        result
    }

    fn mod_inv_prime(a: i64, m: i64) -> i64 {
        pow_mod(a, m - 2, m)
    }

    fn mod_inv_general(a: i64, m: i64) -> i64 {
        let (mut g, mut x, mut y) = (m, 0i64, 1i64);
        let mut a0 = a;
        while a0 != 0 {
            let q = g / a0;
            let t = g - q * a0;
            g = a0;
            a0 = t;
            let t = x - q * y;
            x = y;
            y = t;
        }
        ((x % m) + m) % m
    }

    fn ncr(n: usize, k: usize) -> i64 {
        if k > n { return 0; }
        let kk = k.min(n - k);
        let mut result = 1i64;
        for i in 0..kk {
            result = (result as i128 * ((n - i) as i64 % MOD) as i128 % MOD as i128) as i64;
            result = (result as i128 * mod_inv_prime(i as i64 + 1, MOD) as i128 % MOD as i128) as i64;
        }
        result
    }

    fn is_prime(n: i64) -> bool {
        if n < 2 { return false; }
        if n < 4 { return true; }
        if n % 2 == 0 || n % 3 == 0 { return false; }
        let mut i = 5i64;
        while i * i <= n {
            if n % i == 0 || n % (i + 2) == 0 { return false; }
            i += 6;
        }
        true
    }

    // Find first 97 primes ending in 7
    let mut primes = Vec::new();
    let mut n = 2i64;
    while primes.len() < N_PRIMES {
        if is_prime(n) && n % B == DIGIT {
            primes.push(n);
        }
        n += 1;
    }

    // ans = B/2 * prod(p_i - 1) mod MOD
    let mut ans: i64 = B / 2;
    for &p in &primes {
        ans = (ans as i128 * ((p - 1) % MOD) as i128 % MOD as i128) as i64;
    }

    // Inclusion-exclusion correction
    for i in 0..=N_PRIMES {
        let ki = pow_mod(DIGIT, i as i64, B);
        let ki_inv = mod_inv_general(ki, B);
        let k_val = (DIGIT * ki_inv) % B;

        let sign: i64 = if i % 2 == 0 { 1 } else { MOD - 1 };
        let term = (sign as i128 * k_val as i128 % MOD as i128 * ncr(N_PRIMES, i) as i128 % MOD as i128) as i64;
        ans = (ans + term) % MOD;
    }

    // Multiply by prod(primes)
    for &p in &primes {
        ans = (ans as i128 * (p % MOD) as i128 % MOD as i128) as i64;
    }

    println!("{}", ans);
}
