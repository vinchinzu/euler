// Project Euler 659 - Largest prime
// P(k) = largest prime dividing (2k)^2+1. Sieve with Tonelli-Shanks.

const N_VAL: usize = 10_000_000;
const M_VAL: i64 = 1_000_000_000_000_000_000;

fn mul_mod(a: i64, b: i64, m: i64) -> i64 {
    ((a as i128 * b as i128) % m as i128) as i64
}

fn pow_mod(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut r = 1i64;
    base = ((base % m) + m) % m;
    while exp > 0 {
        if exp & 1 == 1 { r = mul_mod(r, base, m); }
        base = mul_mod(base, base, m);
        exp >>= 1;
    }
    r
}

fn sqrt_mod_neg1(p: i64) -> i64 {
    if p % 4 == 3 { return -1; }
    let mut q = p - 1;
    let mut s = 0;
    while q % 2 == 0 { q /= 2; s += 1; }
    let mut z = 2i64;
    loop {
        if pow_mod(z, (p - 1) / 2, p) == p - 1 { break; }
        z += 1;
    }
    let mut m_s = s;
    let mut c = pow_mod(z, q, p);
    let mut t = pow_mod(p - 1, q, p);
    let mut r = pow_mod(p - 1, (q + 1) / 2, p);
    while t != 1 {
        let mut i = 1;
        let mut tt = mul_mod(t, t, p);
        while tt != 1 { tt = mul_mod(tt, tt, p); i += 1; }
        let mut b = c;
        for _ in 0..m_s - i - 1 { b = mul_mod(b, b, p); }
        m_s = i;
        c = mul_mod(b, b, p);
        t = mul_mod(t, c, p);
        r = mul_mod(r, b, p);
    }
    r
}

fn main() {
    let n = N_VAL;
    let mut p_arr = vec![0i64; n + 1];
    for k in 1..=n { p_arr[k] = (2 * k as i64) * (2 * k as i64) + 1; }
    let sieve_limit = 2 * n;
    let mut is_prime = vec![true; sieve_limit + 1];
    is_prime[0] = false; is_prime[1] = false;
    let sq = (sieve_limit as f64).sqrt() as usize;
    for i in 2..=sq { if is_prime[i] { for j in (i*i..=sieve_limit).step_by(i) { is_prime[j] = false; } } }
    for p in 5..=sieve_limit {
        if !is_prime[p] || p % 4 != 1 { continue; }
        let pl = p as i64;
        let sv = sqrt_mod_neg1(pl);
        if sv < 0 { continue; }
        let inv2 = (pl + 1) / 2;
        let k_starts = [
            mul_mod(sv, inv2, pl),
            mul_mod(pl - sv, inv2, pl),
        ];
        for &ks in &k_starts {
            let start = if ks == 0 { pl } else { ks };
            let mut k = start as usize;
            while k <= n {
                while p_arr[k] % pl == 0 && p_arr[k] > pl { p_arr[k] /= pl; }
                k += p;
            }
        }
    }
    let mut ans = 0i64;
    for k in 1..=n { ans = (ans + p_arr[k]) % M_VAL; }
    println!("{}", ans);
}
