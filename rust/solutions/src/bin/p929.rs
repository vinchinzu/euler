// Project Euler Problem 929 - Compositions with Odd-Length Runs
// F(10^5) mod 1111124111
// Uses NTT (3 primes + CRT) for polynomial inversion.
// h[m] = sum_{d|m} (-1)^(d-1) * F_d (Fibonacci)
// P(x) = 1 - H(x), answer = [x^N] P(x)^{-1}

const FINAL_MOD: i64 = 1111124111;
const P1: i64 = 998244353;
const P2: i64 = 1004535809;
const P3: i64 = 469762049;
const G: i64 = 3;

fn power(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
    let mut res: i64 = 1;
    base %= modulus;
    while exp > 0 {
        if exp & 1 == 1 {
            res = (res as i128 * base as i128 % modulus as i128) as i64;
        }
        base = (base as i128 * base as i128 % modulus as i128) as i64;
        exp >>= 1;
    }
    res
}

fn mod_inverse(n: i64, modulus: i64) -> i64 {
    power(n, modulus - 2, modulus)
}

fn ntt(a: &mut [i64], invert: bool, modulus: i64, root: i64) {
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
        let mut wlen = power(root, (modulus - 1) / len as i64, modulus);
        if invert {
            wlen = mod_inverse(wlen, modulus);
        }
        let half = len / 2;
        for i in (0..n).step_by(len) {
            let mut w: i64 = 1;
            for jj in 0..half {
                let u = a[i + jj];
                let v = (a[i + jj + half] as i128 * w as i128 % modulus as i128) as i64;
                let sum = u + v;
                a[i + jj] = if sum < modulus { sum } else { sum - modulus };
                let diff = u - v;
                a[i + jj + half] = if diff >= 0 { diff } else { diff + modulus };
                w = (w as i128 * wlen as i128 % modulus as i128) as i64;
            }
        }
        len <<= 1;
    }

    if invert {
        let n_inv = mod_inverse(n as i64, modulus);
        for val in a.iter_mut() {
            *val = (*val as i128 * n_inv as i128 % modulus as i128) as i64;
        }
    }
}

fn crt3(r1: i64, r2: i64, r3: i64) -> i64 {
    let inv_m1_m2 = mod_inverse(P1 % P2, P2);
    let x1 = r1;
    let k1 = ((r2 - x1 + P2) % P2 as i128 as i64 + P2) % P2;
    let k1 = (k1 as i128 * inv_m1_m2 as i128 % P2 as i128) as i64;
    let m1: i128 = P1 as i128;
    let x12: i128 = x1 as i128 + m1 * k1 as i128;

    let m1m2: i128 = m1 * P2 as i128;
    let inv_m1m2_m3 = mod_inverse((m1m2 % P3 as i128) as i64, P3);
    let r3_minus = ((r3 as i128 - x12 % P3 as i128) % P3 as i128 + P3 as i128) % P3 as i128;
    let k2 = (r3_minus * inv_m1m2_m3 as i128 % P3 as i128) as i64;

    let res = ((x12 % FINAL_MOD as i128) + (m1m2 % FINAL_MOD as i128) * k2 as i128 % FINAL_MOD as i128) % FINAL_MOD as i128;
    res as i64
}

/// Multiply two polynomials using 3-prime NTT + CRT
fn multiply(a: &[i64], b: &[i64]) -> Vec<i64> {
    let na = a.len();
    let nb = b.len();
    let mut n = 1;
    while n < na + nb {
        n <<= 1;
    }

    // NTT with P1
    let mut fa1 = vec![0i64; n];
    let mut fb1 = vec![0i64; n];
    fa1[..na].copy_from_slice(a);
    fb1[..nb].copy_from_slice(b);
    ntt(&mut fa1, false, P1, G);
    ntt(&mut fb1, false, P1, G);
    let mut c1 = vec![0i64; n];
    for i in 0..n {
        c1[i] = (fa1[i] as i128 * fb1[i] as i128 % P1 as i128) as i64;
    }
    ntt(&mut c1, true, P1, G);

    // NTT with P2
    let mut fa2 = vec![0i64; n];
    let mut fb2 = vec![0i64; n];
    fa2[..na].copy_from_slice(a);
    fb2[..nb].copy_from_slice(b);
    ntt(&mut fa2, false, P2, G);
    ntt(&mut fb2, false, P2, G);
    let mut c2 = vec![0i64; n];
    for i in 0..n {
        c2[i] = (fa2[i] as i128 * fb2[i] as i128 % P2 as i128) as i64;
    }
    ntt(&mut c2, true, P2, G);

    // NTT with P3
    let mut fa3 = vec![0i64; n];
    let mut fb3 = vec![0i64; n];
    fa3[..na].copy_from_slice(a);
    fb3[..nb].copy_from_slice(b);
    ntt(&mut fa3, false, P3, G);
    ntt(&mut fb3, false, P3, G);
    let mut c3 = vec![0i64; n];
    for i in 0..n {
        c3[i] = (fa3[i] as i128 * fb3[i] as i128 % P3 as i128) as i64;
    }
    ntt(&mut c3, true, P3, G);

    // CRT
    let mut res = vec![0i64; n];
    for i in 0..n {
        res[i] = crt3(c1[i], c2[i], c3[i]);
    }
    res
}

/// Polynomial inverse: given P[0..n-1], compute Q s.t. P*Q = 1 mod x^n
fn poly_inv(p: &[i64], n: usize) -> Vec<i64> {
    if n == 1 {
        return vec![mod_inverse(p[0], FINAL_MOD)];
    }

    let half_n = (n + 1) / 2;
    let q = poly_inv(p, half_n);

    // T = P_trunc * Q (length n)
    let p_trunc: Vec<i64> = p[..n].to_vec();
    let t = multiply(&p_trunc, &q);

    // R[0] = 2 - T[0], R[i] = -T[i] for i>0
    let mut r = vec![0i64; n];
    for i in 0..n {
        let ti = if i < t.len() { t[i] } else { 0 };
        if i == 0 {
            r[i] = (2 - ti + FINAL_MOD) % FINAL_MOD;
        } else {
            r[i] = (FINAL_MOD - ti) % FINAL_MOD;
        }
    }

    let res_full = multiply(&q, &r);

    // Truncate to n terms
    let mut result = vec![0i64; n];
    for i in 0..n.min(res_full.len()) {
        result[i] = res_full[i];
    }
    result
}

fn main() {
    let n = 100_000usize;

    // Compute Fibonacci mod FINAL_MOD
    let mut f = vec![0i64; n + 1];
    f[1] = 1;
    if n >= 2 {
        f[2] = 1;
    }
    for i in 3..=n {
        f[i] = (f[i - 1] + f[i - 2]) % FINAL_MOD;
    }

    // Compute h[m] = sum_{d|m} (-1)^{d-1} * F[d]
    let mut h = vec![0i64; n + 1];
    for d in 1..=n {
        let val = if (d - 1) % 2 == 1 {
            (FINAL_MOD - f[d]) % FINAL_MOD
        } else {
            f[d]
        };
        let mut m = d;
        while m <= n {
            h[m] = (h[m] + val) % FINAL_MOD;
            m += d;
        }
    }

    // P[0] = 1, P[i] = -h[i] for i >= 1
    let mut p = vec![0i64; n + 1];
    p[0] = 1;
    for i in 1..=n {
        p[i] = (FINAL_MOD - h[i]) % FINAL_MOD;
    }

    // Compute Q = P^{-1} mod x^{N+1}
    let q = poly_inv(&p, n + 1);

    println!("{}", q[n]);
}
