// Project Euler 638 - Weighted Paths in a Grid
// q-binomial coefficients for k=1..7

const M: i64 = 1_000_000_007;

fn powmod(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut result = 1i64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 { result = (result as i128 * base as i128 % m as i128) as i64; }
        base = (base as i128 * base as i128 % m as i128) as i64;
        exp >>= 1;
    }
    result
}

fn modinv(a: i64) -> i64 { powmod(a, M - 2, M) }

fn qbinom(a: i64, b: i64, k: i64) -> i64 {
    let n = a + b;
    if k == 1 {
        let mut fact = vec![1i64; (n + 1) as usize];
        for i in 1..=n as usize { fact[i] = (fact[i-1] as i128 * i as i128 % M as i128) as i64; }
        let r = (fact[n as usize] as i128 * modinv(fact[a as usize]) as i128 % M as i128) as i64;
        return (r as i128 * modinv(fact[b as usize]) as i128 % M as i128) as i64;
    }

    let mut qfact = vec![1i64; (n + 1) as usize];
    let mut pow_k = 1i64;
    let mut sum_pow_k = 0i64;
    for i in 1..=n as usize {
        sum_pow_k = (sum_pow_k + pow_k) % M;
        qfact[i] = (qfact[i-1] as i128 * sum_pow_k as i128 % M as i128) as i64;
        pow_k = (pow_k as i128 * k as i128 % M as i128) as i64;
    }
    let r = (qfact[n as usize] as i128 * modinv(qfact[a as usize]) as i128 % M as i128) as i64;
    (r as i128 * modinv(qfact[b as usize]) as i128 % M as i128) as i64
}

fn main() {
    let mut ans = 0i64;
    for k in 1..=7i64 {
        let base = 10i64.pow(k as u32) + k;
        ans = (ans + qbinom(base, base, k)) % M;
    }
    println!("{}", ans);
}
