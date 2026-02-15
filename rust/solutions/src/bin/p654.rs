// Project Euler 654 - Neighbourly Constraints
// Berlekamp-Massey + Kitamasa with 3-prime NTT for T(10^12, 5000).

const MOD: i64 = 1_000_000_007;
const P1: i64 = 998_244_353;
const P2: i64 = 985_661_441;
const P3: i64 = 754_974_721;

fn pw(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut r = 1i64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 { r = (r as i128 * base as i128 % m as i128) as i64; }
        base = (base as i128 * base as i128 % m as i128) as i64;
        exp >>= 1;
    }
    r
}

fn ntt(a: &mut [i64], inv: bool, m: i64, g: i64) {
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
        let w = if inv { pw(g, m - 1 - (m - 1) / len as i64, m) } else { pw(g, (m - 1) / len as i64, m) };
        for i in (0..n).step_by(len) {
            let mut wn = 1i64;
            for jj in 0..len/2 {
                let u = a[i + jj];
                let v = (a[i + jj + len/2] as i128 * wn as i128 % m as i128) as i64;
                a[i + jj] = (u + v) % m;
                a[i + jj + len/2] = (u - v + m) % m;
                wn = (wn as i128 * w as i128 % m as i128) as i64;
            }
        }
        len <<= 1;
    }
    if inv {
        let inv_n = pw(n as i64, m - 2, m);
        for v in a.iter_mut() { *v = (*v as i128 * inv_n as i128 % m as i128) as i64; }
    }
}

fn poly_mul(a: &[i64], b: &[i64]) -> Vec<i64> {
    let nc = a.len() + b.len() - 1;
    let mut n = 1;
    while n < nc { n <<= 1; }
    let mut a1 = vec![0i64; n]; let mut b1 = vec![0i64; n];
    let mut a2 = vec![0i64; n]; let mut b2 = vec![0i64; n];
    let mut a3 = vec![0i64; n]; let mut b3 = vec![0i64; n];
    for i in 0..a.len() { a1[i] = a[i] % P1; a2[i] = a[i] % P2; a3[i] = a[i] % P3; }
    for i in 0..b.len() { b1[i] = b[i] % P1; b2[i] = b[i] % P2; b3[i] = b[i] % P3; }
    ntt(&mut a1, false, P1, 3); ntt(&mut b1, false, P1, 3);
    ntt(&mut a2, false, P2, 3); ntt(&mut b2, false, P2, 3);
    ntt(&mut a3, false, P3, 11); ntt(&mut b3, false, P3, 11);
    for i in 0..n {
        a1[i] = (a1[i] as i128 * b1[i] as i128 % P1 as i128) as i64;
        a2[i] = (a2[i] as i128 * b2[i] as i128 % P2 as i128) as i64;
        a3[i] = (a3[i] as i128 * b3[i] as i128 % P3 as i128) as i64;
    }
    ntt(&mut a1, true, P1, 3); ntt(&mut a2, true, P2, 3); ntt(&mut a3, true, P3, 11);
    let inv12 = pw(P1, P2 - 2, P2);
    let inv13 = pw((P1 as i128 * P2 as i128 % P3 as i128) as i64, P3 - 2, P3);
    let mut res = vec![0i64; nc];
    for i in 0..nc {
        let (r1, r2, r3) = (a1[i], a2[i], a3[i]);
        let x1 = r1;
        let x2 = ((r2 - x1 % P2 + P2) % P2 as i128 * inv12 as i128 % P2 as i128) as i64;
        let val = ((x1 + (x2 as i128 % P3 as i128 * (P1 % P3) as i128 % P3 as i128) as i64) % P3 + P3) % P3;
        let x3 = ((r3 - val + P3) % P3 as i128 * inv13 as i128 % P3 as i128) as i64;
        let result = x1 as i128 + x2 as i128 * P1 as i128 + x3 as i128 * P1 as i128 * P2 as i128;
        res[i] = (result % MOD as i128) as i64;
    }
    res
}

fn poly_mod(p: &[i64], cp: &[i64], d: usize) -> Vec<i64> {
    if p.len() <= d {
        let mut res = vec![0i64; d];
        for i in 0..p.len() { res[i] = p[i]; }
        return res;
    }
    let mut r = p.to_vec();
    for i in (d..r.len()).rev() {
        let coeff = r[i] % MOD;
        if coeff == 0 { continue; }
        for j in 0..d {
            r[i - d + j] = (r[i - d + j] - (coeff as i128 * cp[j] as i128 % MOD as i128) as i64 + MOD) % MOD;
        }
        r[i] = 0;
    }
    r.truncate(d);
    r
}

fn berlekamp_massey(s: &[i64]) -> Vec<i64> {
    let len = s.len();
    let mut c = vec![0i64; len + 2]; c[0] = 1;
    let mut b_arr = vec![0i64; len + 2]; b_arr[0] = 1;
    let (mut clen, mut blen) = (1usize, 1usize);
    let (mut l, mut m) = (0usize, 1usize);
    let mut bv = 1i64;
    for n in 0..len {
        let mut d = s[n];
        for j in 1..=l { d = (d + (c[j] as i128 * s[n - j] as i128 % MOD as i128) as i64) % MOD; }
        d = (d % MOD + MOD) % MOD;
        if d == 0 { m += 1; continue; }
        if 2 * l <= n {
            let t = c[..clen].to_vec();
            let coef = (d as i128 * pw(bv, MOD - 2, MOD) as i128 % MOD as i128) as i64;
            let new_len = blen + m;
            if new_len > clen { c.resize(new_len + 1, 0); clen = new_len; }
            for i in 0..blen { c[i + m] = (c[i + m] - (coef as i128 * b_arr[i] as i128 % MOD as i128) as i64 + MOD) % MOD; }
            l = n + 1 - l;
            b_arr = t;
            blen = b_arr.len();
            bv = d; m = 1;
        } else {
            let coef = (d as i128 * pw(bv, MOD - 2, MOD) as i128 % MOD as i128) as i64;
            let new_len = blen + m;
            if new_len > clen { c.resize(new_len + 1, 0); clen = new_len; }
            for i in 0..blen { c[i + m] = (c[i + m] - (coef as i128 * b_arr[i] as i128 % MOD as i128) as i64 + MOD) % MOD; }
            m += 1;
        }
    }
    c.truncate(l + 1);
    c
}

fn main() {
    let n_val: i64 = 1_000_000_000_000;
    let k = 5000usize;
    let mut dp = vec![0i64; k]; for i in 1..k { dp[i] = 1; }
    let seq_len = 2 * k - 1;
    let mut seq = vec![0i64; seq_len];
    for iter in 0..seq_len {
        let ti: i64 = dp.iter().sum::<i64>() % MOD;
        seq[iter] = ti;
        let mut new_dp = vec![0i64; k];
        new_dp[1] = ti;
        let mut cum = 0i64;
        for j in (1..k).rev() {
            cum = (cum + dp[j]) % MOD;
            let idx = k - 1 - j;
            if idx < k - 2 { new_dp[idx + 2] = (ti - cum + MOD) % MOD; }
        }
        dp = new_dp;
    }
    let c_poly = berlekamp_massey(&seq);
    let d = c_poly.len() - 1;
    let mut cp_trunc = vec![0i64; d];
    for i in 0..d { cp_trunc[i] = (MOD - c_poly[d - i]) % MOD; }
    // Reverse cp_trunc to match the C code's convention
    // Actually the C code stores char_poly differently. Let me match it:
    // char_poly[d] = 1, char_poly[d-1-i] = C_poly[i+1]
    // cp_trunc[i] = char_poly[i] for i < d
    let mut char_poly_trunc = vec![0i64; d];
    for i in 0..d {
        char_poly_trunc[i] = c_poly[d - i]; // This is C[i+1] mapped
    }

    let mut result = vec![0i64; d]; result[0] = 1;
    let mut base = vec![0i64; d]; if d > 1 { base[1] = 1; }
    let mut exp = n_val - 1;
    while exp > 0 {
        if exp & 1 == 1 {
            let prod = poly_mul(&result, &base);
            result = poly_mod(&prod, &char_poly_trunc, d);
        }
        let prod = poly_mul(&base, &base);
        base = poly_mod(&prod, &char_poly_trunc, d);
        exp >>= 1;
    }
    let mut ans = 0i64;
    for i in 0..d { ans = (ans + (result[i] as i128 * seq[i] as i128 % MOD as i128) as i64) % MOD; }
    println!("{}", ans);
}
