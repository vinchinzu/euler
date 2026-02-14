// Project Euler 559 - Permutation Matrices
//
// Compute Q(50000) mod 1000000123.

const NVAL: usize = 50_000;
const MOD: i64 = 1_000_000_123;

fn power(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut result: i64 = 1;
    base %= m;
    if base < 0 { base += m; }
    while exp > 0 {
        if exp & 1 == 1 { result = result * base % m; }
        base = base * base % m;
        exp >>= 1;
    }
    result
}

fn main() {
    let n = NVAL;
    let m = MOD;

    let mut factorials = vec![0i64; n + 1];
    let mut inv_factorials = vec![0i64; n + 1];
    let mut pow_inv_fact = vec![0i64; n + 1];

    factorials[0] = 1;
    for i in 1..=n {
        factorials[i] = factorials[i - 1] * i as i64 % m;
    }

    inv_factorials[n] = power(factorials[n], m - 2, m);
    for i in (0..n).rev() {
        inv_factorials[i] = inv_factorials[i + 1] * (i as i64 + 1) % m;
    }

    for i in 0..=n {
        pow_inv_fact[i] = power(inv_factorials[i], n as i64, m);
    }

    let mut ans: i64 = 0;
    let mut dp = vec![0i64; n + 2];
    let mut parts = vec![0usize; n + 2];

    for k in 1..=n {
        let mut np = 0;
        let mut v = 0;
        while v < n {
            parts[np] = v;
            np += 1;
            v += k;
        }
        parts[np] = n;
        np += 1;

        dp[0] = 1;
        for i in 1..np {
            let mut val: i64 = 0;
            for s in 0..i {
                let length = parts[i] - parts[s];
                val = (val - pow_inv_fact[length] % m * (dp[s] % m)) % m;
            }
            dp[i] = (val % m + m) % m;
        }

        let sign: i64 = if (np + 1) % 2 == 0 { 1 } else { -1 };
        let mut pk = (sign * dp[np - 1]) % m;
        if pk < 0 { pk += m; }
        ans = (ans + pk) % m;
    }

    ans = ans * power(factorials[n], n as i64, m) % m;
    println!("{ans}");
}
