// Project Euler 553 - Power Sets of Power Sets
//
// Find C(10^4, 10) mod 10^9+7 using EGF approach with polynomial operations.

const MOD: i64 = 1_000_000_007;

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

fn inv(a: i64, m: i64) -> i64 { power(a, m - 2, m) }

fn poly_mul(a: &[i64], b: &[i64], n: usize) -> Vec<i64> {
    let mut out = vec![0i64; n + 1];
    for i in 0..=n {
        if a[i] == 0 { continue; }
        let ai = a[i];
        for j in 0..=(n - i) {
            out[i + j] = (out[i + j] + ai * b[j]) % MOD;
        }
    }
    out
}

fn poly_inv(f: &[i64], n: usize) -> Vec<i64> {
    let mut g = vec![0i64; n + 1];
    g[0] = inv(f[0], MOD);
    for i in 1..=n {
        let mut s: i64 = 0;
        for j in 1..=i {
            if j <= n && f[j] != 0 {
                s = (s + f[j] * g[i - j]) % MOD;
            }
        }
        g[i] = (MOD - g[0] % MOD * s % MOD) % MOD;
    }
    g
}

fn poly_deriv(f: &[i64], n: usize) -> Vec<i64> {
    let mut df = vec![0i64; n + 1];
    for i in 0..n {
        df[i] = f[i + 1] * ((i + 1) as i64) % MOD;
    }
    df
}

fn poly_integ(f: &[i64], n: usize, inv_table: &[i64]) -> Vec<i64> {
    let mut intf = vec![0i64; n + 1];
    intf[0] = 0;
    for i in 0..n {
        intf[i + 1] = f[i] % MOD * inv_table[i + 1] % MOD;
    }
    intf
}

fn poly_log(f: &[i64], n: usize, inv_table: &[i64]) -> Vec<i64> {
    let df = poly_deriv(f, n);
    let finv = poly_inv(f, n);
    let quot = poly_mul(&df, &finv, n);
    poly_integ(&quot, n, inv_table)
}

fn poly_exp_fast(f: &[i64], n: usize, inv_table: &[i64]) -> Vec<i64> {
    let mut g = vec![0i64; n + 1];
    g[0] = 1;
    let mut kf = vec![0i64; n + 1];
    for k in 1..=n {
        kf[k] = k as i64 % MOD * f[k] % MOD;
    }
    for nn in 1..=n {
        let mut s: i64 = 0;
        for k in 1..=nn {
            s = (s + kf[k] * g[nn - k]) % MOD;
        }
        g[nn] = s % MOD * inv_table[nn] % MOD;
    }
    g
}

fn main() {
    let n = 10_000usize;
    let k = 10usize;

    // Precompute factorials
    let mut fact = vec![0i64; n + 1];
    let mut inv_fact = vec![0i64; n + 1];
    fact[0] = 1;
    for i in 1..=n { fact[i] = fact[i - 1] * i as i64 % MOD; }
    inv_fact[n] = inv(fact[n], MOD);
    for i in (0..n).rev() { inv_fact[i] = inv_fact[i + 1] * (i as i64 + 1) % MOD; }

    // Precompute inverse table
    let mut inv_table = vec![0i64; n + 1];
    inv_table[1] = 1;
    for i in 2..=n {
        inv_table[i] = (MOD - MOD / i as i64) * inv_table[(MOD % i as i64) as usize] % MOD;
    }

    // a_coeff
    let mut a_coeff = vec![0i64; n + 1];
    for i in 0..=n {
        let exp_val = power(2, i as i64, MOD - 1);
        let exp_val = (exp_val - 1 + MOD - 1) % (MOD - 1);
        a_coeff[i] = power(2, exp_val, MOD) % MOD * inv_fact[i] % MOD;
    }

    // e_neg, e_pos
    let mut e_neg = vec![0i64; n + 1];
    let mut e_pos = vec![0i64; n + 1];
    for i in 0..=n {
        e_neg[i] = if i % 2 == 0 { inv_fact[i] } else { (MOD - inv_fact[i]) % MOD };
        e_pos[i] = inv_fact[i];
    }

    let p = poly_mul(&a_coeff, &e_neg, n);
    let logp = poly_log(&p, n, &inv_table);

    // h = logp shifted down by 1
    let mut h = vec![0i64; n + 1];
    for i in 0..n { h[i] = logp[i + 1]; }

    let h0 = h[0];
    let h0_inv = inv(h0, MOD);
    let nk = n - k;
    let mut h_norm = vec![0i64; nk + 1];
    for i in 0..=nk { h_norm[i] = h[i] * h0_inv % MOD; }

    let log_h = poly_log(&h_norm, nk, &inv_table);

    let mut log_h_k = vec![0i64; nk + 1];
    for i in 0..=nk { log_h_k[i] = log_h[i] * k as i64 % MOD; }

    let h_pow = poly_exp_fast(&log_h_k, nk, &inv_table);

    let h0k = power(h0, k as i64, MOD);
    let mut h_pow_scaled = vec![0i64; nk + 1];
    for i in 0..=nk { h_pow_scaled[i] = h_pow[i] * h0k % MOD; }

    let mut logpk = vec![0i64; n + 1];
    for i in k..=n { logpk[i] = h_pow_scaled[i - k]; }

    let result = poly_mul(&logpk, &e_pos, n);

    let ans = result[n] * fact[n] % MOD * inv_fact[k] % MOD;
    println!("{ans}");
}
