// Project Euler 654 - Neighbourly Constraints
// Berlekamp-Massey + Kitamasa with 3-prime NTT for T(10^12, 5000).
// Uses NTT-based Barrett polynomial reduction for O(d log d) poly_mod.
// Optimized: Precomputed twiddles, bit-reversal, CRT constants, frequency-domain caching and Rayon NTT parallelization.

const MOD: u64 = 1_000_000_007;
const P1: u64 = 998_244_353;
const P2: u64 = 985_661_441;
const P3: u64 = 754_974_721;
const INV12: u64 = 657107549;
const INV13: u64 = 284003040;
const N_NTT: usize = 16384;

#[inline(always)]
fn mulmod(a: u64, b: u64, m: u64) -> u64 {
    a * b % m
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

struct NTTPrime {
    p: u64,
    inv_n: u64,
    twiddles: Vec<u64>,
    inv_twiddles: Vec<u64>,
}

impl NTTPrime {
    fn new(p: u64, g: u64, n: usize) -> Self {
        let inv_n = pw(n as u64, p - 2, p);
        let mut twiddles = Vec::with_capacity(n);
        let mut inv_twiddles = Vec::with_capacity(n);
        let mut len = 2;
        while len <= n {
            let half = len / 2;
            let w = pw(g, (p - 1) / len as u64, p);
            let inv_w = pw(g, p - 1 - (p - 1) / len as u64, p);
            let mut cur_w = 1u64;
            let mut cur_inv_w = 1u64;
            for _ in 0..half {
                twiddles.push(cur_w);
                inv_twiddles.push(cur_inv_w);
                cur_w = mulmod(cur_w, w, p);
                cur_inv_w = mulmod(cur_inv_w, inv_w, p);
            }
            len <<= 1;
        }
        Self { p, inv_n, twiddles, inv_twiddles }
    }
}

struct NTTContext {
    primes: [NTTPrime; 3],
    bit_rev: Vec<u16>,
}

impl NTTContext {
    fn new() -> Self {
        let primes = [
            NTTPrime::new(P1, 3, N_NTT),
            NTTPrime::new(P2, 3, N_NTT),
            NTTPrime::new(P3, 11, N_NTT),
        ];
        let mut bit_rev = vec![0u16; N_NTT];
        let mut j = 0usize;
        for i in 1..N_NTT {
            let mut bit = N_NTT >> 1;
            while j & bit != 0 { j ^= bit; bit >>= 1; }
            j ^= bit;
            bit_rev[i] = j as u16;
        }
        Self { primes, bit_rev }
    }
}

fn ntt_fast(a: &mut [u64], prime: &NTTPrime, bit_rev: &[u16]) {
    let n = a.len();
    for i in 1..n {
        let j = bit_rev[i] as usize;
        if i < j { a.swap(i, j); }
    }
    let mut len = 2;
    let mut tw_offset = 0;
    while len <= n {
        let half = len / 2;
        let tw = &prime.twiddles[tw_offset..tw_offset + half];
        for i in (0..n).step_by(len) {
            for jj in 0..half {
                let u = a[i + jj];
                let v = mulmod(a[i + jj + half], tw[jj], prime.p);
                a[i + jj] = if u + v >= prime.p { u + v - prime.p } else { u + v };
                a[i + jj + half] = if u >= v { u - v } else { u + prime.p - v };
            }
        }
        tw_offset += half;
        len <<= 1;
    }
}

fn intt_fast(a: &mut [u64], prime: &NTTPrime, bit_rev: &[u16]) {
    let n = a.len();
    for i in 1..n {
        let j = bit_rev[i] as usize;
        if i < j { a.swap(i, j); }
    }
    let mut len = 2;
    let mut tw_offset = 0;
    while len <= n {
        let half = len / 2;
        let tw = &prime.inv_twiddles[tw_offset..tw_offset + half];
        for i in (0..n).step_by(len) {
            for jj in 0..half {
                let u = a[i + jj];
                let v = mulmod(a[i + jj + half], tw[jj], prime.p);
                a[i + jj] = if u + v >= prime.p { u + v - prime.p } else { u + v };
                a[i + jj + half] = if u >= v { u - v } else { u + prime.p - v };
            }
        }
        tw_offset += half;
        len <<= 1;
    }
    for v in a.iter_mut() {
        *v = mulmod(*v, prime.inv_n, prime.p);
    }
}

struct NTTPoly {
    p1: Vec<u64>,
    p2: Vec<u64>,
    p3: Vec<u64>,
}

fn transform(a: &[u64], ctx: &NTTContext) -> NTTPoly {
    let mut p1 = vec![0u64; N_NTT];
    let mut p2 = vec![0u64; N_NTT];
    let mut p3 = vec![0u64; N_NTT];
    for i in 0..a.len() {
        let v = a[i];
        p1[i] = if v >= P1 { v - P1 } else { v };
        p2[i] = if v >= P2 { v - P2 } else { v };
        p3[i] = v % P3;
    }
    rayon::join(
        || ntt_fast(&mut p1, &ctx.primes[0], &ctx.bit_rev),
        || rayon::join(
            || ntt_fast(&mut p2, &ctx.primes[1], &ctx.bit_rev),
            || ntt_fast(&mut p3, &ctx.primes[2], &ctx.bit_rev),
        ),
    );
    NTTPoly { p1, p2, p3 }
}

#[inline(always)]
fn crt_combine(a1: &[u64], a2: &[u64], a3: &[u64], res: &mut [u64]) {
    let nc = res.len();
    for i in 0..nc {
        let (r1, r2, r3) = (a1[i], a2[i], a3[i]);
        let x1 = r1;
        let x2 = mulmod((r2 + P2 - x1 % P2) % P2, INV12, P2);
        let val = (x1 as u128 + x2 as u128 * (P1 % P3) as u128) % P3 as u128;
        let x3 = mulmod((r3 + P3 - val as u64 % P3) % P3, INV13, P3);
        let result = x1 as u128 + x2 as u128 * P1 as u128 + x3 as u128 * P1 as u128 * P2 as u128;
        res[i] = (result % MOD as u128) as u64;
    }
}

fn mul_freq(a: &NTTPoly, b: &NTTPoly, out_len: usize, ctx: &NTTContext) -> Vec<u64> {
    let mut c1 = vec![0u64; N_NTT];
    let mut c2 = vec![0u64; N_NTT];
    let mut c3 = vec![0u64; N_NTT];
    for i in 0..N_NTT {
        c1[i] = mulmod(a.p1[i], b.p1[i], P1);
        c2[i] = mulmod(a.p2[i], b.p2[i], P2);
        c3[i] = mulmod(a.p3[i], b.p3[i], P3);
    }
    rayon::join(
        || intt_fast(&mut c1, &ctx.primes[0], &ctx.bit_rev),
        || rayon::join(
            || intt_fast(&mut c2, &ctx.primes[1], &ctx.bit_rev),
            || intt_fast(&mut c3, &ctx.primes[2], &ctx.bit_rev),
        ),
    );
    let mut res = vec![0u64; out_len];
    crt_combine(&c1, &c2, &c3, &mut res);
    res
}

fn square_freq(a: &NTTPoly, out_len: usize, ctx: &NTTContext) -> Vec<u64> {
    let mut c1 = vec![0u64; N_NTT];
    let mut c2 = vec![0u64; N_NTT];
    let mut c3 = vec![0u64; N_NTT];
    for i in 0..N_NTT {
        c1[i] = mulmod(a.p1[i], a.p1[i], P1);
        c2[i] = mulmod(a.p2[i], a.p2[i], P2);
        c3[i] = mulmod(a.p3[i], a.p3[i], P3);
    }
    rayon::join(
        || intt_fast(&mut c1, &ctx.primes[0], &ctx.bit_rev),
        || rayon::join(
            || intt_fast(&mut c2, &ctx.primes[1], &ctx.bit_rev),
            || intt_fast(&mut c3, &ctx.primes[2], &ctx.bit_rev),
        ),
    );
    let mut res = vec![0u64; out_len];
    crt_combine(&c1, &c2, &c3, &mut res);
    res
}

fn poly_mul(a: &[u64], b: &[u64], ctx: &NTTContext) -> Vec<u64> {
    if a.is_empty() || b.is_empty() { return vec![]; }
    let nc = a.len() + b.len() - 1;
    let a_f = transform(a, ctx);
    let b_f = transform(b, ctx);
    mul_freq(&a_f, &b_f, nc, ctx)
}

fn poly_mul_trunc(a: &[u64], b: &[u64], trunc: usize, ctx: &NTTContext) -> Vec<u64> {
    let mut r = poly_mul(a, b, ctx);
    r.truncate(trunc);
    r
}

fn poly_inv(f: &[u64], n: usize, ctx: &NTTContext) -> Vec<u64> {
    let mut g = vec![pw(f[0], MOD - 2, MOD)];
    let mut cur_len = 1;
    while cur_len < n {
        let next_len = std::cmp::min(cur_len * 2, n);
        let f_trunc: Vec<u64> = f.iter().take(next_len).copied().collect();
        let fg = poly_mul_trunc(&f_trunc, &g, next_len, ctx);
        let mut h = vec![0u64; next_len];
        h[0] = (2 + MOD - fg[0]) % MOD;
        for i in 1..fg.len().min(next_len) {
            h[i] = if fg[i] == 0 { 0 } else { MOD - fg[i] };
        }
        g = poly_mul_trunc(&g, &h, next_len, ctx);
        cur_len = next_len;
    }
    g.truncate(n);
    g
}

fn poly_mod_barrett(a: &[u64], cp_freq: &NTTPoly, d: usize, inv_rev_f_freq: &NTTPoly, ctx: &NTTContext) -> Vec<u64> {
    if a.len() <= d {
        let mut res = vec![0u64; d];
        for i in 0..a.len() { res[i] = a[i]; }
        return res;
    }
    let deg_a = a.len() - 1;

    let mut rev_a = a.to_vec();
    rev_a.reverse();

    let q_len = deg_a - d + 1;
    let rev_a_freq = transform(&rev_a, ctx);
    let q_rev = mul_freq(&rev_a_freq, inv_rev_f_freq, q_len, ctx);

    let mut q = q_rev;
    while q.len() < q_len { q.push(0); }
    q.reverse();

    let q_freq = transform(&q, ctx);
    let qf_low = mul_freq(&q_freq, cp_freq, d, ctx);

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

    let ctx = NTTContext::new();

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
    let inv_rev_f = poly_inv(&rev_f, d, &ctx);

    let cp_freq = transform(&char_poly_trunc, &ctx);
    let inv_rev_f_freq = transform(&inv_rev_f, &ctx);

    let mut result = vec![0u64; d]; result[0] = 1;
    let mut base = vec![0u64; d]; if d > 1 { base[1] = 1; }
    let mut exp = n_val - 1;
    while exp > 0 {
        if exp & 1 == 1 {
            let res_freq = transform(&result, &ctx);
            let base_freq = transform(&base, &ctx);
            let prod = mul_freq(&res_freq, &base_freq, 2 * d - 1, &ctx);
            result = poly_mod_barrett(&prod, &cp_freq, d, &inv_rev_f_freq, &ctx);
        }
        let base_freq = transform(&base, &ctx);
        let prod = square_freq(&base_freq, 2 * d - 1, &ctx);
        base = poly_mod_barrett(&prod, &cp_freq, d, &inv_rev_f_freq, &ctx);
        exp >>= 1;
    }
    let mut ans = 0u64;
    for i in 0..d { ans = (ans + mulmod(result[i], seq[i], MOD)) % MOD; }
    println!("{}", ans);
}
