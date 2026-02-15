// Project Euler 746 - A Messy Dinner
//
// Precompute factorials and inverse factorials, compute M(k) using
// inclusion-exclusion.

const MOD: i64 = 1_000_000_007;
const MAXN: usize = 2021;
const MAX_FACT: usize = 4 * MAXN + 1;

fn pow_mod(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut result: i64 = 1;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result as i128 * base as i128 % m as i128) as i64;
        }
        exp >>= 1;
        base = (base as i128 * base as i128 % m as i128) as i64;
    }
    result
}

fn main() {
    // Precompute factorials
    let mut fact = vec![0i64; MAX_FACT + 1];
    fact[0] = 1;
    for i in 1..=MAX_FACT {
        fact[i] = (fact[i - 1] as i128 * i as i128 % MOD as i128) as i64;
    }
    let mut inv_fact = vec![0i64; MAX_FACT + 1];
    inv_fact[MAX_FACT] = pow_mod(fact[MAX_FACT], MOD - 2, MOD);
    for i in (0..MAX_FACT).rev() {
        inv_fact[i] = (inv_fact[i + 1] as i128 * (i + 1) as i128 % MOD as i128) as i64;
    }

    let ncr = |n: usize, r: usize| -> i64 {
        if r > n { return 0; }
        (fact[n] as i128 * inv_fact[r] as i128 % MOD as i128 * inv_fact[n - r] as i128 % MOD as i128) as i64
    };

    let sq = |n: i64| -> i64 {
        (n as i128 * n as i128 % MOD as i128) as i64
    };

    let mut ans: i64 = 0;
    for k in 2..=MAXN {
        let mut res = sq(fact[2 * k]);
        for r in 1..=k {
            let mut f_k = ncr(k, r);
            f_k = (f_k as i128 * (4 * k as i64) as i128 % MOD as i128) as i64;
            f_k = (f_k as i128 * ncr(4 * k - 3 * r - 1, r - 1) as i128 % MOD as i128) as i64;
            f_k = (f_k as i128 * fact[r - 1] as i128 % MOD as i128) as i64;
            f_k = (f_k as i128 * pow_mod(4, r as i64, MOD) as i128 % MOD as i128) as i64;
            f_k = (f_k as i128 * sq(fact[2 * (k - r)]) as i128 % MOD as i128) as i64;
            if r % 2 == 0 {
                res = (res + f_k) % MOD;
            } else {
                res = (res - f_k + MOD) % MOD;
            }
        }
        ans = (ans + 2 * res) % MOD;
    }

    println!("{}", ans);
}
