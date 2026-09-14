// Project Euler 788 - Dominating Numbers
// Count numbers with up to N digits where more than half the digits are equal.

const MOD: i64 = 1_000_000_007;

fn main() {
    let n = 2022;
    let b = 10i64;
    let max_n = 2023usize;

    let mut fact = vec![1i64; max_n];
    for i in 1..max_n { fact[i] = fact[i - 1] * i as i64 % MOD; }
    
    let mut inv_fact = vec![1i64; max_n];
    let mut temp = fact[max_n - 1];
    let mut inv = 1i64;
    let exp = MOD - 2;
    let mut e = exp;
    while e > 0 {
        if e & 1 == 1 { inv = inv * temp % MOD; }
        temp = temp * temp % MOD;
        e >>= 1;
    }
    inv_fact[max_n - 1] = inv;
    for i in (0..max_n - 1).rev() { inv_fact[i] = inv_fact[i + 1] * (i as i64 + 1) % MOD; }

    let ncr = |n: usize, k: usize| -> i64 {
        if k > n { return 0; }
        fact[n] * inv_fact[k] % MOD * inv_fact[n - k] % MOD
    };

    let mut pow9 = vec![1i64; max_n];
    for i in 1..max_n {
        pow9[i] = pow9[i - 1] * (b - 1) % MOD;
    }

    let mut ans: i64 = 0;

    for l in 1..=n {
        for k in (l / 2 + 1)..=l {
            let lk = l - k;
            
            ans = (ans + (b - 1) * ncr(l - 1, k - 1) % MOD * pow9[lk] % MOD) % MOD;

            if lk > 0 {
                ans = (ans + (b - 1) * ncr(l - 1, k) % MOD * (b - 2) % MOD * pow9[lk - 1] % MOD) % MOD;
            }

            ans = (ans + ncr(l - 1, k) * pow9[lk] % MOD) % MOD;
        }
    }

    println!("{}", ans % MOD);
}
