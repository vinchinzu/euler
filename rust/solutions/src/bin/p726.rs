// Project Euler 726 - Falling Bottles
//
// Recurrence for a(n), b(n) and Young tableau formula f(n) = a(n) * tr(n)! / b(n).
// All mod M = 10^9 + 33.

const NMAX: usize = 10000;
const MOD: i64 = 1_000_000_033;

fn tr(n: usize) -> i64 {
    n as i64 * (n as i64 + 1) / 2
}

fn pow_mod(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut result: i64 = 1;
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

fn mod_inv(a: i64, m: i64) -> i64 {
    pow_mod(a, m - 2, m)
}

fn main() {
    let n = NMAX;
    let trn = tr(n) as usize;

    // Precompute factorials mod m
    let mut fact = vec![0i64; trn + 1];
    fact[0] = 1;
    for i in 1..=trn {
        fact[i] = (fact[i - 1] as i128 * (i as i64 % MOD) as i128 % MOD as i128) as i64;
    }

    // Compute a[i]
    let mut a = vec![0i64; n + 1];
    a[0] = 1;
    a[1] = 1;
    for i in 2..=n {
        let sq = (a[i - 1] as i128 * a[i - 1] as i128 % MOD as i128) as i64;
        let p2 = (pow_mod(2, i as i64, MOD) - 1 + MOD) % MOD;
        let inv_prev = mod_inv(a[i - 2], MOD);
        a[i] = (sq as i128 * p2 as i128 % MOD as i128 * inv_prev as i128 % MOD as i128) as i64;
    }

    // Compute b[i]
    let mut b = vec![0i64; n + 1];
    b[0] = 1;
    b[1] = 1;
    for i in 2..=n {
        let sq = (b[i - 1] as i128 * b[i - 1] as i128 % MOD as i128) as i64;
        let odd = (2 * i as i64 - 1) % MOD;
        let inv_prev = mod_inv(b[i - 2], MOD);
        b[i] = (sq as i128 * odd as i128 % MOD as i128 * inv_prev as i128 % MOD as i128) as i64;
    }

    // Sum f(i) for i = 1..n
    let mut ans: i64 = 0;
    for i in 1..=n {
        let tri = tr(i) as usize;
        let term = (a[i] as i128 * fact[tri] as i128 % MOD as i128) as i64;
        let term = (term as i128 * mod_inv(b[i], MOD) as i128 % MOD as i128) as i64;
        ans = (ans + term) % MOD;
    }

    println!("{}", ans);
}
