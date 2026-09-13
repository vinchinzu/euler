// Project Euler 709 - Even Stevens
//
// f(n) is the Euler zigzag number A000111(n). EGF F(x) = sec(x) + tan(x)
// = (1 + sin x) / cos x. Compute F mod M via 3-prime NTT inversion of cos,
// then f(n) = n! [x^n] F(x). N = 24680, M = 1020202009.

const N: usize = 24680;
const M: u64 = 1_020_202_009;

// NTT-friendly primes (2^16 | p-1) with primitive root 3.
const P1: u64 = 998_244_353;
const P2: u64 = 1_004_535_809;
const P3: u64 = 469_762_049;
const G: u64 = 3;

#[inline(always)]
fn mulmod(a: u64, b: u64, m: u64) -> u64 {
    a * b % m
}

fn pw(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut r = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            r = mulmod(r, base, m);
        }
        base = mulmod(base, base, m);
        exp >>= 1;
    }
    r
}

fn ntt(a: &mut [u64], invert: bool, m: u64, g: u64) {
    let n = a.len();
    let mut j = 0usize;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j ^= bit;
        if i < j {
            a.swap(i, j);
        }
    }

    let mut len = 2;
    while len <= n {
        let wlen = if invert {
            pw(g, m - 1 - (m - 1) / len as u64, m)
        } else {
            pw(g, (m - 1) / len as u64, m)
        };
        let half = len / 2;
        for i in (0..n).step_by(len) {
            let mut w = 1u64;
            for jj in 0..half {
                // SAFETY: i + jj + half < i + len <= n.
                unsafe {
                    let u = *a.get_unchecked(i + jj);
                    let v = mulmod(*a.get_unchecked(i + jj + half), w, m);
                    *a.get_unchecked_mut(i + jj) = if u + v >= m { u + v - m } else { u + v };
                    *a.get_unchecked_mut(i + jj + half) =
                        if u >= v { u - v } else { u + m - v };
                }
                w = mulmod(w, wlen, m);
            }
        }
        len <<= 1;
    }

    if invert {
        let n_inv = pw(n as u64, m - 2, m);
        for v in a.iter_mut() {
            *v = mulmod(*v, n_inv, m);
        }
    }
}

struct Crt {
    inv12: u64,
    inv123: u64,
    p12_mod_m: u64,
}

impl Crt {
    fn new() -> Self {
        let inv12 = pw(P1, P2 - 2, P2);
        let p12_mod_p3 = (P1 as u128 * P2 as u128 % P3 as u128) as u64;
        Self {
            inv12,
            inv123: pw(p12_mod_p3, P3 - 2, P3),
            p12_mod_m: (P1 as u128 * P2 as u128 % M as u128) as u64,
        }
    }

    #[inline(always)]
    fn combine(&self, r1: u64, r2: u64, r3: u64) -> u64 {
        let k1 = mulmod((r2 + P2 - r1 % P2) % P2, self.inv12, P2);
        // r1 + k1*P1 <= P1*P2 - 1 < 2^64.
        let x12 = r1 + k1 * P1;
        let k2 = mulmod((r3 + P3 - x12 % P3) % P3, self.inv123, P3);
        let s = x12 % M + mulmod(k2, self.p12_mod_m, M);
        if s >= M { s - M } else { s }
    }
}

fn poly_mul_naive(a: &[u64], b: &[u64], trunc: usize) -> Vec<u64> {
    let mut r = vec![0u64; trunc];
    for (i, &ai) in a.iter().enumerate() {
        if ai == 0 || i >= trunc {
            continue;
        }
        let ai = ai as u128;
        let jmax = b.len().min(trunc - i);
        for j in 0..jmax {
            r[i + j] = ((r[i + j] as u128 + ai * b[j] as u128) % M as u128) as u64;
        }
    }
    r
}

fn poly_mul(crt: &Crt, a: &[u64], b: &[u64], trunc: usize) -> Vec<u64> {
    let na = a.len().min(trunc);
    let nb = b.len().min(trunc);
    if na == 0 || nb == 0 {
        return vec![0u64; trunc];
    }
    let need = (na + nb - 1).min(trunc);
    if need <= 64 {
        return poly_mul_naive(&a[..na], &b[..nb], trunc);
    }

    let mut n = 1usize;
    while n < na + nb - 1 {
        n <<= 1;
    }

    let mut a1 = vec![0u64; n];
    let mut b1 = vec![0u64; n];
    let mut a2 = vec![0u64; n];
    let mut b2 = vec![0u64; n];
    let mut a3 = vec![0u64; n];
    let mut b3 = vec![0u64; n];
    for i in 0..na {
        let x = a[i];
        a1[i] = x % P1;
        a2[i] = x % P2;
        a3[i] = x % P3;
    }
    for i in 0..nb {
        let x = b[i];
        b1[i] = x % P1;
        b2[i] = x % P2;
        b3[i] = x % P3;
    }

    ntt(&mut a1, false, P1, G);
    ntt(&mut b1, false, P1, G);
    ntt(&mut a2, false, P2, G);
    ntt(&mut b2, false, P2, G);
    ntt(&mut a3, false, P3, G);
    ntt(&mut b3, false, P3, G);
    for i in 0..n {
        a1[i] = mulmod(a1[i], b1[i], P1);
        a2[i] = mulmod(a2[i], b2[i], P2);
        a3[i] = mulmod(a3[i], b3[i], P3);
    }
    ntt(&mut a1, true, P1, G);
    ntt(&mut a2, true, P2, G);
    ntt(&mut a3, true, P3, G);

    let mut res = vec![0u64; trunc];
    let out = need.min(n);
    for i in 0..out {
        res[i] = crt.combine(a1[i], a2[i], a3[i]);
    }
    res
}

fn poly_inv(crt: &Crt, f: &[u64], n: usize) -> Vec<u64> {
    let mut g = vec![pw(f[0], M - 2, M)];
    let mut cur = 1usize;
    while cur < n {
        let nxt = (cur * 2).min(n);
        let f_trunc = &f[..nxt.min(f.len())];
        let fg = poly_mul(crt, f_trunc, &g, nxt);
        let mut h = vec![0u64; nxt];
        h[0] = (2 + M - fg[0]) % M;
        for i in 1..nxt {
            h[i] = if fg[i] == 0 { 0 } else { M - fg[i] };
        }
        g = poly_mul(crt, &g, &h, nxt);
        cur = nxt;
    }
    g
}

fn main() {
    let crt = Crt::new();
    let n = N;

    let mut fact = vec![1u64; n + 1];
    for i in 1..=n {
        fact[i] = mulmod(fact[i - 1], i as u64, M);
    }
    let mut invfact = vec![1u64; n + 1];
    invfact[n] = pw(fact[n], M - 2, M);
    for i in (1..=n).rev() {
        invfact[i - 1] = mulmod(invfact[i], i as u64, M);
    }

    // cos x = sum (-1)^m x^{2m}/(2m)!, sin x = sum (-1)^m x^{2m+1}/(2m+1)!
    let mut cos = vec![0u64; n + 1];
    let mut sin = vec![0u64; n + 1];
    for m in 0..=n / 2 {
        let even = 2 * m;
        let odd = even + 1;
        let term_e = invfact[even];
        cos[even] = if m % 2 == 0 { term_e } else { M - term_e };
        if odd <= n {
            let term_o = invfact[odd];
            sin[odd] = if m % 2 == 0 { term_o } else { M - term_o };
        }
    }

    let inv_cos = poly_inv(&crt, &cos, n + 1);
    sin[0] = 1; // 1 + sin x
    let f = poly_mul(&crt, &sin, &inv_cos, n + 1);
    println!("{}", mulmod(f[n], fact[n], M));
}
