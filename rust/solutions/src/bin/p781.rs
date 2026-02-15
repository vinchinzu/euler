// Project Euler 781 - Feynman Diagrams
// EGF convolution: f(m) = t(m) - sum s(j)*f(m-j), with derangements.

fn main() {
    const MOD: i64 = 1_000_000_007;
    let n = 50000;
    let m = n / 2;

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

    // Derangements D[k]
    let mut d = vec![0i64; n + 1];
    d[0] = 1;
    d[1] = 0;
    for k in 2..=n {
        d[k] = ((k as i64 * d[k - 1]) % MOD + if k % 2 == 0 { 1 } else { MOD - 1 }) % MOD;
    }

    // Factorials and inverse factorials
    let mut fact = vec![1i64; m + 1];
    for i in 1..=m { fact[i] = fact[i - 1] * i as i64 % MOD; }
    let mut inv_fact = vec![1i64; m + 1];
    inv_fact[m] = powmod(fact[m], MOD - 2, MOD);
    for i in (0..m).rev() { inv_fact[i] = inv_fact[i + 1] * (i as i64 + 1) % MOD; }

    // Powers of inv(2)
    let inv2 = powmod(2, MOD - 2, MOD);
    let mut inv_pow2 = vec![1i64; m + 1];
    for i in 1..=m { inv_pow2[i] = inv_pow2[i - 1] * inv2 % MOD; }

    // Compute t[j] and s[j]
    let mut t = vec![0i64; m + 1];
    let mut s = vec![0i64; m + 1];
    for j in 0..=m {
        let idx = 2 * j;
        let b_val = if idx == 0 { 1 }
        else { ((idx as i64 + 1) * d[idx] % MOD + idx as i64 * d[idx - 1] % MOD) % MOD };
        let coeff = inv_pow2[j] * inv_fact[j] % MOD;
        t[j] = b_val * coeff % MOD;
        s[j] = d[idx] * coeff % MOD;
    }

    // Compute f[j] = t[j] - sum_{k=1}^{j} s[k] * f[j-k]
    let mut f = vec![0i64; m + 1];
    for j in 0..=m {
        let mut val = t[j];
        for k in 1..=j {
            val = ((val - s[k] * f[j - k] % MOD) % MOD + MOD) % MOD;
        }
        f[j] = val;
    }

    println!("{}", f[m] % MOD);
}
