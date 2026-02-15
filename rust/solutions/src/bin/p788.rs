// Project Euler 788 - Dominating Numbers
// Count numbers with up to N digits where more than half the digits are equal.

const MOD: i64 = 1_000_000_007;

fn powmod(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut result = 1i64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 { result = result * base % m; }
        base = base * base % m;
        exp >>= 1;
    }
    result
}

fn main() {
    let n = 2022;
    let b = 10i64;
    let max_n = 2023usize;

    let mut fact = vec![1i64; max_n];
    for i in 1..max_n { fact[i] = fact[i - 1] * i as i64 % MOD; }
    let mut inv_fact = vec![1i64; max_n];
    inv_fact[max_n - 1] = powmod(fact[max_n - 1], MOD - 2, MOD);
    for i in (0..max_n - 1).rev() { inv_fact[i] = inv_fact[i + 1] * (i as i64 + 1) % MOD; }

    let ncr = |n: usize, k: usize| -> i64 {
        if k > n { return 0; }
        fact[n] * inv_fact[k] % MOD * inv_fact[n - k] % MOD
    };

    let mut ans: i64 = 0;

    for l in 1..=n {
        for k in (l / 2 + 1)..=l {
            // Case 1: d != 0, first digit is d
            ans = (ans + (b - 1) * ncr(l - 1, k - 1) % MOD * powmod(b - 1, (l - k) as i64, MOD) % MOD) % MOD;

            // Case 2: d != 0, first digit is not d
            if l >= k + 1 {
                ans = (ans + (b - 1) * ncr(l - 1, k) % MOD * ((b - 2) % MOD) % MOD
                    * powmod(b - 1, (l - k - 1) as i64, MOD) % MOD) % MOD;
            }

            // Case 3: d = 0
            ans = (ans + ncr(l - 1, k) * powmod(b - 1, (l - k) as i64, MOD) % MOD) % MOD;
        }
    }

    println!("{}", ans % MOD);
}
