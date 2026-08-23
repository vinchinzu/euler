// Project Euler 502 - Counting Castles
// Column DP (even block-count) + Berlekamp-Massey + Kitamasa.
// u64 mulmod (MOD^2 fits u64); no per-column vec zeroing.

use rayon::prelude::*;

const MOD: u64 = 1_000_000_007;
const MOD2: u64 = 2 * MOD;
const SEQ_LEN: usize = 500;

#[inline(always)]
fn addmod(a: u64, b: u64) -> u64 {
    let s = a + b;
    if s >= MOD { s - MOD } else { s }
}

#[inline(always)]
fn submod(a: u64, b: u64) -> u64 {
    if a >= b { a - b } else { a + MOD - b }
}

#[inline(always)]
fn mulmod(a: u64, b: u64) -> u64 {
    a * b % MOD
}

/// Reduce v ∈ [0, 4·MOD) → [0, MOD). Written for cmov.
#[inline(always)]
fn red4(v: u64) -> u64 {
    let v = if v >= MOD2 { v - MOD2 } else { v };
    if v >= MOD { v - MOD } else { v }
}

#[inline(always)]
fn madd3_sub(a: u64, b: u64, c: u64, sub: u64) -> u64 {
    red4(a + b + c + (MOD - sub))
}

fn powmod(mut base: u64, mut exp: u64) -> u64 {
    let mut r = 1u64;
    while exp > 0 {
        if exp & 1 == 1 { r = mulmod(r, base); }
        base = mulmod(base, base);
        exp >>= 1;
    }
    r
}

#[inline(always)]
fn modinv(x: u64) -> u64 {
    powmod(x, MOD - 2)
}

/// Even-block castles of width `w`, height at most `h`.
/// After each column, `out[x]` is set if `out` is non-empty.
fn castle_dp(w: usize, h: usize, out: &mut [u64]) -> u64 {
    if h == 0 || w == 0 {
        return 0;
    }
    let n = h + 1;
    let mut prev0 = vec![0u64; n];
    let mut prev1 = vec![0u64; n];
    let mut curr0 = vec![0u64; n];
    let mut curr1 = vec![0u64; n];

    for y in (0..n).step_by(2) {
        prev0[y] = 1;
    }

    let record = !out.is_empty();
    let mut last = 0u64;
    for x in 0..w {
        // SAFETY: indices stay in 0..=h; all four arrays have length h+1.
        unsafe {
            let p0 = prev0.as_ptr();
            let p1 = prev1.as_ptr();
            let c0 = curr0.as_mut_ptr();
            let c1 = curr1.as_mut_ptr();

            *c0 = 0;
            *c1 = 0;

            let s0 = addmod(*p0.add(h), *p0.add(h - 1));
            let s1 = addmod(*p1.add(h), *p1.add(h - 1));

            *c0.add(1) = madd3_sub(s0, *p1, 0, *p0);
            *c1.add(1) = madd3_sub(s1, *p0, 0, *p1);

            if h >= 2 {
                *c0.add(2) = madd3_sub(s0, *p1.add(1), 0, *p0.add(1));
                *c1.add(2) = madd3_sub(s1, *p0.add(1), 0, *p1.add(1));
            }

            // Four independent chains: (even/odd) × (parity 0/1).
            let mut y = 3usize;
            while y + 1 <= h {
                *c0.add(y) = madd3_sub(s0, *p1.add(y - 1), *c0.add(y - 2), *p0.add(y - 1));
                *c1.add(y) = madd3_sub(s1, *p0.add(y - 1), *c1.add(y - 2), *p1.add(y - 1));
                *c0.add(y + 1) = madd3_sub(s0, *p1.add(y), *c0.add(y - 1), *p0.add(y));
                *c1.add(y + 1) = madd3_sub(s1, *p0.add(y), *c1.add(y - 1), *p1.add(y));
                y += 2;
            }
            if y <= h {
                *c0.add(y) = madd3_sub(s0, *p1.add(y - 1), *c0.add(y - 2), *p0.add(y - 1));
                *c1.add(y) = madd3_sub(s1, *p0.add(y - 1), *c1.add(y - 2), *p1.add(y - 1));
            }
        }

        std::mem::swap(&mut prev0, &mut curr0);
        std::mem::swap(&mut prev1, &mut curr1);
        last = addmod(prev0[h], prev0[h - 1]);
        if record {
            out[x] = last;
        }
    }
    last
}

#[inline]
fn num_castles(w: usize, h: usize) -> u64 {
    castle_dp(w, h, &mut [])
}

#[inline]
fn dp_width_seq(w: usize, h: usize) -> Vec<u64> {
    let mut out = vec![0u64; w];
    castle_dp(w, h, &mut out);
    out
}

fn berlekamp_massey(s: &[u64]) -> Vec<u64> {
    let n = s.len();
    let mut c = vec![0u64; n + 2];
    let mut b = vec![0u64; n + 2];
    c[0] = 1;
    b[0] = 1;
    let mut rec_len = 0usize;
    let mut m = 1usize;
    let mut bv = 1u64;

    for i in 0..n {
        let mut d = s[i];
        for j in 1..=rec_len {
            d = addmod(d, mulmod(c[j], s[i - j]));
        }
        if d == 0 {
            m += 1;
        } else if 2 * rec_len <= i {
            let t = c.clone();
            let coef = mulmod(d, modinv(bv));
            for j in m..=n {
                c[j] = submod(c[j], mulmod(coef, b[j - m]));
            }
            b = t;
            rec_len = i + 1 - rec_len;
            bv = d;
            m = 1;
        } else {
            let coef = mulmod(d, modinv(bv));
            for j in m..=n {
                c[j] = submod(c[j], mulmod(coef, b[j - m]));
            }
            m += 1;
        }
    }

    let mut coeffs = vec![0u64; rec_len + 1];
    for i in 1..=rec_len {
        coeffs[i] = submod(0, c[i]);
    }
    coeffs
}

fn poly_mul_mod(out: &mut [u64], x: &[u64], y: &[u64], coeffs: &[u64], tmp: &mut [u64]) {
    let k = out.len();
    tmp[..2 * k].fill(0);
    for i in 0..k {
        let xi = x[i];
        if xi == 0 { continue; }
        for j in 0..k {
            tmp[i + j] = addmod(tmp[i + j], mulmod(xi, y[j]));
        }
    }
    if k >= 2 {
        for i in (k..=2 * k - 2).rev() {
            let t = tmp[i];
            if t == 0 { continue; }
            for j in 1..=k {
                tmp[i - j] = addmod(tmp[i - j], mulmod(t, coeffs[j]));
            }
        }
    }
    out.copy_from_slice(&tmp[..k]);
}

fn linear_recurrence(coeffs: &[u64], a: &[u64], n: u64) -> u64 {
    let rec_len = coeffs.len() - 1;
    if (n as usize) < rec_len {
        return a[n as usize];
    }
    if rec_len == 0 {
        return 0;
    }

    let mut q = vec![0u64; rec_len];
    let mut r = vec![0u64; rec_len];
    let mut tmp = vec![0u64; 2 * rec_len + 2];
    let mut buf = vec![0u64; rec_len];
    q[0] = 1;
    if rec_len > 1 {
        r[1] = 1;
    } else {
        r[0] = coeffs[1] % MOD;
    }

    let mut exp = n;
    while exp > 0 {
        if exp & 1 == 1 {
            poly_mul_mod(&mut buf, &q, &r, coeffs, &mut tmp);
            std::mem::swap(&mut q, &mut buf);
        }
        poly_mul_mod(&mut buf, &r, &r, coeffs, &mut tmp);
        std::mem::swap(&mut r, &mut buf);
        exp >>= 1;
    }

    let mut result = 0u64;
    for i in 0..rec_len {
        result = addmod(result, mulmod(q[i], a[i]));
    }
    result
}

fn extrapolate(values: &[u64], x: u64) -> u64 {
    let n = values.len() as u64;
    if x <= n {
        return values[(x - 1) as usize];
    }
    let coeffs = berlekamp_massey(values);
    linear_recurrence(&coeffs, values, x - 1)
}

fn extrapolate_two(values: &[u64], x1: u64, x2: u64) -> (u64, u64) {
    let n = values.len() as u64;
    if x1 <= n && x2 <= n {
        return (values[(x1 - 1) as usize], values[(x2 - 1) as usize]);
    }
    let coeffs = berlekamp_massey(values);
    (
        if x1 <= n { values[(x1 - 1) as usize] } else { linear_recurrence(&coeffs, values, x1 - 1) },
        if x2 <= n { values[(x2 - 1) as usize] } else { linear_recurrence(&coeffs, values, x2 - 1) },
    )
}

fn f_small_w(w: usize, h: u64) -> u64 {
    let values: Vec<u64> = (1..SEQ_LEN + 1)
        .into_par_iter()
        .map(|hh| num_castles(w, hh))
        .collect();
    let (c_h, c_hm1) = extrapolate_two(&values, h, h - 1);
    submod(c_h, c_hm1)
}

fn f_small_h(w: u64, h: usize) -> u64 {
    let (seq_h, seq_hm1) = rayon::join(
        || dp_width_seq(SEQ_LEN, h),
        || dp_width_seq(SEQ_LEN, h - 1),
    );
    submod(extrapolate(&seq_h, w), extrapolate(&seq_hm1, w))
}

fn main() {
    // Two independent 10k column DPs are the wall-time bottleneck.
    let ((c10k, c9999), (a, c)) = rayon::join(
        || rayon::join(
            || num_castles(10_000, 10_000),
            || num_castles(10_000, 9_999),
        ),
        || rayon::join(
            || f_small_h(1_000_000_000_000, 100),
            || f_small_w(100, 1_000_000_000_000),
        ),
    );
    let b = submod(c10k, c9999);
    println!("{}", addmod(addmod(a, b), c));
}
