// Project Euler 657 - Incomplete Words
// Inclusion-exclusion for words of length <= N using K letters, N=10^12, K=10^7.

const MOD: i64 = 1_000_000_007;

fn power_mod(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut r = 1i64;
    base %= m; if base < 0 { base += m; }
    while exp > 0 {
        if exp & 1 == 1 { r = (r as i128 * base as i128 % m as i128) as i64; }
        base = (base as i128 * base as i128 % m as i128) as i64;
        exp >>= 1;
    }
    r
}

fn main() {
    let n: i64 = 1_000_000_000_000;
    let k = 10_000_000usize;
    let mut pows = vec![0i64; k + 1];
    for i in 1..=k { pows[i] = power_mod(i as i64, n + 1, MOD); }
    let mut invs = vec![0i64; k + 1];
    invs[1] = 1;
    for i in 2..=k { invs[i] = (MOD - (MOD / i as i64) * invs[(MOD % i as i64) as usize] % MOD) % MOD; }
    let mut ans = 0i64;
    let mut num_choices = 1i64;
    for t in 0..k {
        let num_words = if t == 0 { 1i64 }
            else if t == 1 { (n + 1) % MOD }
            else { (pows[t] - 1 + MOD) % MOD * invs[t - 1] % MOD };
        let sign = if (k - t) % 2 == 0 { 1i64 } else { MOD - 1 };
        ans = (ans - (sign as i128 * num_words as i128 % MOD as i128 * num_choices as i128 % MOD as i128) as i64 % MOD + MOD) % MOD;
        if t < k - 1 {
            num_choices = (num_choices as i128 * ((k - t) as i64 % MOD) as i128 % MOD as i128) as i64;
            num_choices = (num_choices as i128 * invs[t + 1] as i128 % MOD as i128) as i64;
        }
    }
    println!("{}", ans % MOD);
}
