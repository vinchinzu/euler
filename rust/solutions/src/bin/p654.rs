// Project Euler 654 - Neighbourly Constraints
// Berlekamp-Massey + Kitamasa with 3-prime NTT for T(10^12, 5000).
// Uses NTT-based Barrett polynomial reduction for O(d log d) poly_mod.
// Optimized: u64 NTT arithmetic.

const MOD: u64 = 1_000_000_007;
const P1: u64 = 998_244_353;
const P2: u64 = 985_661_441;
const P3: u64 = 754_974_721;

#[inline(always)]
fn mulmod(a: u64, b: u64, m: u64) -> u64 {
    (a as u128 * b as u128 % m as u128) as u64
}

fn pw(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut r = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 { r = mulmod(r, base, m); }
        base = mulmod(base, base, m);
        exp >>= 1;
    }
    r
}

fn ntt(a: &mut [u64], inv: bool, m: u64, g: u64) {
    let n = a.len();
    let mut j = 0usize;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 { j ^= bit; bit >>= 1; }
        j ^= bit;
        if i < j { a.swap(i, j); }
    }
    let mut len = 2;
    while len <= n {
        let w = if inv { pw(g, m - 1 - (m - 1) / len as u64, m) } else { pw(g, (m - 1) / len as u64, m) };
        let half = len / 2;
        for i in (0..n).step_by(len) {
            let mut wn = 1u64;
            for jj in 0..half {
                let u = a[i + jj];
                let v = mulmod(a[i + jj + half], wn, m);
                a[i + jj] = if u + v >= m { u + v - m } else { u + v };
                a[i + jj + half] = if u >= v { u - v } else { u + m - v };
                wn = mulmod(wn, w, m);
            }
        }
        len <<= 1;
    }
    if inv {
        let inv_n = pw(n as u64, m - 2, m);
        for v in a.iter_mut() { *v = mulmod(*v, inv_n, m); }
    }
}

/// Multiply two polynomials mod MOD using 3-prime NTT + CRT.
fn poly_mul(a: &[u64], b: &[u64]) -> Vec<u64> {
    if a.is_empty() || b.is_empty() { return vec![]; }
    let nc = a.len() + b.len() - 1;
    let mut n = 1;
    while n < nc { n <<= 1; }
    let inv12 = pw(P1, P2 - 2, P2);
    let inv13 = pw((P1 as u128 * P2 as u128 % P3 as u128) as u64, P3 - 2, P3);
    let mut a1 = vec![0u64; n]; let mut b1 = vec![0u64; n];
    let mut a2 = vec![0u64; n]; let mut b2 = vec![0u64; n];
    let mut a3 = vec![0u64; n]; let mut b3 = vec![0u64; n];
    for i in 0..a.len() { a1[i] = a[i] % P1; a2[i] = a[i] % P2; a3[i] = a[i] % P3; }
    for i in 0..b.len() { b1[i] = b[i] % P1; b2[i] = b[i] % P2; b3[i] = b[i] % P3; }
    ntt(&mut a1, false, P1, 3); ntt(&mut b1, false, P1, 3);
    ntt(&mut a2, false, P2, 3); ntt(&mut b2, false, P2, 3);
    ntt(&mut a3, false, P3, 11); ntt(&mut b3, false, P3, 11);
    for i in 0..n {
        a1[i] = mulmod(a1[i], b1[i], P1);
        a2[i] = mulmod(a2[i], b2[i], P2);
        a3[i] = mulmod(a3[i], b3[i], P3);
    }
    ntt(&mut a1, true, P1, 3); ntt(&mut a2, true, P2, 3); ntt(&mut a3, true, P3, 11);
    let mut res = vec![0u64; nc];
    for i in 0..nc {
        let (r1, r2, r3) = (a1[i], a2[i], a3[i]);
        let x1 = r1;
        let x2 = mulmod((r2 + P2 - x1 % P2) % P2, inv12, P2);
        let val = (x1 as u128 + x2 as u128 * (P1 % P3) as u128) % P3 as u128;
        let x3 = mulmod((r3 + P3 - val as u64 % P3) % P3, inv13, P3);
        let result = x1 as u128 + x2 as u128 * P1 as u128 + x3 as u128 * P1 as u128 * P2 as u128;
        res[i] = (result % MOD as u128) as u64;
    }
    res
}

fn poly_mul_trunc(a: &[u64], b: &[u64], trunc: usize) -> Vec<u64> {
    let mut r = poly_mul(a, b);
    r.truncate(trunc);
    r
}

/// Compute inverse of polynomial f mod x^n via Newton's method.
fn poly_inv(f: &[u64], n: usize) -> Vec<u64> {
    let mut g = vec![pw(f[0], MOD - 2, MOD)];
    let mut cur_len = 1;
    while cur_len < n {
        let next_len = std::cmp::min(cur_len * 2, n);
        let f_trunc: Vec<u64> = f.iter().take(next_len).copied().collect();
        let fg = poly_mul_trunc(&f_trunc, &g, next_len);
        let mut h = vec![0u64; next_len];
        h[0] = (2 + MOD - fg[0]) % MOD;
        for i in 1..fg.len().min(next_len) {
            h[i] = if fg[i] == 0 { 0 } else { MOD - fg[i] };
        }
        g = poly_mul_trunc(&g, &h, next_len);
        cur_len = next_len;
    }
    g.truncate(n);
    g
}

/// Barrett polynomial reduction: compute a mod f using precomputed inverse.
fn poly_mod_barrett(a: &[u64], cp: &[u64], d: usize, inv_rev_f: &[u64]) -> Vec<u64> {
    if a.len() <= d {
        let mut res = vec![0u64; d];
        for i in 0..a.len() { res[i] = a[i]; }
        return res;
    }
    let deg_a = a.len() - 1;

    let mut rev_a = a.to_vec();
    rev_a.reverse();

    let q_len = deg_a - d + 1;
    let q_rev = poly_mul_trunc(&rev_a, inv_rev_f, q_len);

    let mut q = q_rev;
    while q.len() < q_len { q.push(0); }
    q.reverse();

    let qf_low = poly_mul_trunc(&q, cp, d);

    let mut r = vec![0u64; d];
    for i in 0..d {
        let ai = if i < a.len() { a[i] } else { 0 };
        let qfi = if i < qf_low.len() { qf_low[i] } else { 0 };
        r[i] = (ai + MOD - qfi) % MOD;
    }
    r
}

fn berlekamp_massey(s: &[u64]) -> Vec<u64> {
    let len = s.len();
    let mut c = vec![0u64; len + 2]; c[0] = 1;
    let mut b_arr = vec![0u64; len + 2]; b_arr[0] = 1;
    let (mut clen, mut blen) = (1usize, 1usize);
    let (mut l, mut m) = (0usize, 1usize);
    let mut bv = 1u64;
    for n in 0..len {
        let mut d = s[n];
        for j in 1..=l { d = (d + mulmod(c[j], s[n - j], MOD)) % MOD; }
        d %= MOD;
        if d == 0 { m += 1; continue; }
        if 2 * l <= n {
            let t = c[..clen].to_vec();
            let coef = mulmod(d, pw(bv, MOD - 2, MOD), MOD);
            let new_len = blen + m;
            if new_len > clen { c.resize(new_len + 1, 0); clen = new_len; }
            for i in 0..blen { c[i + m] = (c[i + m] + MOD - mulmod(coef, b_arr[i], MOD)) % MOD; }
            l = n + 1 - l;
            b_arr = t;
            blen = b_arr.len();
            bv = d; m = 1;
        } else {
            let coef = mulmod(d, pw(bv, MOD - 2, MOD), MOD);
            let new_len = blen + m;
            if new_len > clen { c.resize(new_len + 1, 0); clen = new_len; }
            for i in 0..blen { c[i + m] = (c[i + m] + MOD - mulmod(coef, b_arr[i], MOD)) % MOD; }
            m += 1;
        }
    }
    c.truncate(l + 1);
    c
}

fn main() {
    let n_val: u64 = 1_000_000_000_000;
    let k = 5000usize;
    let mut dp = vec![0u64; k]; for i in 1..k { dp[i] = 1; }
    let seq_len = 2 * k - 1;
    let mut seq = vec![0u64; seq_len];
    for iter in 0..seq_len {
        let ti: u64 = dp.iter().sum::<u64>() % MOD;
        seq[iter] = ti;
        let mut new_dp = vec![0u64; k];
        new_dp[1] = ti;
        let mut cum = 0u64;
        for j in (1..k).rev() {
            cum = (cum + dp[j]) % MOD;
            let idx = k - 1 - j;
            if idx < k - 2 { new_dp[idx + 2] = (ti + MOD - cum) % MOD; }
        }
        dp = new_dp;
    }
    let c_poly = berlekamp_massey(&seq);
    let d = c_poly.len() - 1;

    let mut char_poly_trunc = vec![0u64; d];
    for i in 0..d {
        char_poly_trunc[i] = c_poly[d - i];
    }

    // Precompute inverse of rev(f) for Barrett reduction
    let mut rev_f = vec![0u64; d + 1];
    rev_f[0] = 1;
    for i in 0..d {
        rev_f[i + 1] = char_poly_trunc[d - 1 - i];
    }
    let inv_rev_f = poly_inv(&rev_f, d);

    let mut result = vec![0u64; d]; result[0] = 1;
    let mut base = vec![0u64; d]; if d > 1 { base[1] = 1; }
    let mut exp = n_val - 1;
    while exp > 0 {
        if exp & 1 == 1 {
            let prod = poly_mul(&result, &base);
            result = poly_mod_barrett(&prod, &char_poly_trunc, d, &inv_rev_f);
        }
        let prod = poly_mul(&base, &base);
        base = poly_mod_barrett(&prod, &char_poly_trunc, d, &inv_rev_f);
        exp >>= 1;
    }
    let mut ans = 0u64;
    for i in 0..d { ans = (ans + mulmod(result[i], seq[i], MOD)) % MOD; }
    println!("{}", ans);
}
