// Project Euler 767 - Matrix Counting
// FFT-based modular convolution for counting 16xN matrices.

const MOD: i64 = 1_000_000_007;
const BASE: i64 = 32768; // 2^15

fn pow_mod(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut result = 1i64;
    base = ((base % m) + m) % m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result as i128 * base as i128 % m as i128) as i64;
        }
        base = (base as i128 * base as i128 % m as i128) as i64;
        exp >>= 1;
    }
    result
}

fn fft(a: &mut [(f64, f64)], invert: bool) {
    let n = a.len();
    let mut j = 0usize;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j ^= bit;
        if i < j { a.swap(i, j); }
    }

    let mut len = 2;
    while len <= n {
        let ang = 2.0 * std::f64::consts::PI / len as f64 * if invert { -1.0 } else { 1.0 };
        let wlen = (ang.cos(), ang.sin());
        for i in (0..n).step_by(len) {
            let mut w = (1.0, 0.0);
            for jj in 0..len / 2 {
                let u = a[i + jj];
                let v = (
                    a[i + jj + len / 2].0 * w.0 - a[i + jj + len / 2].1 * w.1,
                    a[i + jj + len / 2].0 * w.1 + a[i + jj + len / 2].1 * w.0,
                );
                a[i + jj] = (u.0 + v.0, u.1 + v.1);
                a[i + jj + len / 2] = (u.0 - v.0, u.1 - v.1);
                w = (w.0 * wlen.0 - w.1 * wlen.1, w.0 * wlen.1 + w.1 * wlen.0);
            }
        }
        len <<= 1;
    }

    if invert {
        for x in a.iter_mut() {
            x.0 /= n as f64;
            x.1 /= n as f64;
        }
    }
}

fn convolve_mod(a: &[i64], b: &[i64], result: &mut [i64], n: usize) {
    let mut m = 1;
    while m < 2 * n { m *= 2; }

    let mut fa_lo = vec![(0.0, 0.0); m];
    let mut fa_hi = vec![(0.0, 0.0); m];
    let mut fb_lo = vec![(0.0, 0.0); m];
    let mut fb_hi = vec![(0.0, 0.0); m];

    for i in 0..n {
        fa_lo[i].0 = (a[i] % BASE) as f64;
        fa_hi[i].0 = (a[i] / BASE) as f64;
        fb_lo[i].0 = (b[i] % BASE) as f64;
        fb_hi[i].0 = (b[i] / BASE) as f64;
    }

    fft(&mut fa_lo, false);
    fft(&mut fa_hi, false);
    fft(&mut fb_lo, false);
    fft(&mut fb_hi, false);

    let mut r_ll = vec![(0.0, 0.0); m];
    let mut r_lh = vec![(0.0, 0.0); m];
    let mut r_hh = vec![(0.0, 0.0); m];

    for i in 0..m {
        r_ll[i] = (
            fa_lo[i].0 * fb_lo[i].0 - fa_lo[i].1 * fb_lo[i].1,
            fa_lo[i].0 * fb_lo[i].1 + fa_lo[i].1 * fb_lo[i].0,
        );
        let lh1 = (
            fa_lo[i].0 * fb_hi[i].0 - fa_lo[i].1 * fb_hi[i].1,
            fa_lo[i].0 * fb_hi[i].1 + fa_lo[i].1 * fb_hi[i].0,
        );
        let lh2 = (
            fa_hi[i].0 * fb_lo[i].0 - fa_hi[i].1 * fb_lo[i].1,
            fa_hi[i].0 * fb_lo[i].1 + fa_hi[i].1 * fb_lo[i].0,
        );
        r_lh[i] = (lh1.0 + lh2.0, lh1.1 + lh2.1);
        r_hh[i] = (
            fa_hi[i].0 * fb_hi[i].0 - fa_hi[i].1 * fb_hi[i].1,
            fa_hi[i].0 * fb_hi[i].1 + fa_hi[i].1 * fb_hi[i].0,
        );
    }

    fft(&mut r_ll, true);
    fft(&mut r_lh, true);
    fft(&mut r_hh, true);

    for i in 0..2 * n - 1 {
        let ll = ((r_ll[i].0.round() as i64) % MOD + MOD) % MOD;
        let lh = ((r_lh[i].0.round() as i64) % MOD + MOD) % MOD;
        let hh = ((r_hh[i].0.round() as i64) % MOD + MOD) % MOD;
        result[i] = (ll + (lh as i128 * BASE as i128 % MOD as i128) as i64
            + ((hh as i128 * BASE as i128 % MOD as i128 * BASE as i128 % MOD as i128) as i64))
            % MOD;
    }
}

fn main() {
    let n: i64 = 10_000_000_000_000_000; // 10^16
    let k: usize = 100_000;
    let t: i64 = 16;

    // Precompute factorials
    let mut fact = vec![1i64; k + 1];
    for i in 1..=k {
        fact[i] = (fact[i - 1] as i128 * i as i128 % MOD as i128) as i64;
    }
    let mut inv_fact = vec![1i64; k + 1];
    inv_fact[k] = pow_mod(fact[k], MOD - 2, MOD);
    for i in (0..k).rev() {
        inv_fact[i] = (inv_fact[i + 1] as i128 * (i + 1) as i128 % MOD as i128) as i64;
    }

    // coeffs[i] = (1/i!)^T
    let mut coeffs = vec![0i64; k + 1];
    for i in 0..=k {
        coeffs[i] = pow_mod(inv_fact[i], t, MOD);
    }

    // Convolve coeffs with itself
    let mut p2 = vec![0i64; 2 * k + 1];
    convolve_mod(&coeffs, &coeffs, &mut p2, k + 1);

    // f[i] = i!^T * p2[i]
    let mut f = vec![0i64; k + 1];
    for i in 0..=k {
        f[i] = (pow_mod(fact[i], t, MOD) as i128 * p2[i] as i128 % MOD as i128) as i64;
    }

    let base_val = pow_mod(2, n / k as i64, MOD);
    let term = (base_val - 2 + MOD) % MOD;

    let mut ans: i64 = 0;
    let mut term_pow: i64 = 1;
    for i in 0..=k {
        let ncr = (fact[k] as i128 * inv_fact[i] as i128 % MOD as i128 * inv_fact[k - i] as i128 % MOD as i128) as i64;
        ans = (ans as i128 + ncr as i128 * term_pow as i128 % MOD as i128 * f[k - i] as i128 % MOD as i128) as i64 % MOD;
        term_pow = (term_pow as i128 * term as i128 % MOD as i128) as i64;
    }

    println!("{}", ans);
}
