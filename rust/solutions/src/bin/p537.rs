// Project Euler 537 - Counting Tuples
//
// NTT-based polynomial exponentiation.
// MOD = 1004535809 = 479 * 2^21 + 1.
// MOD^2 < 2^64, so mulmod is a u64 multiply-mod.

use rayon::prelude::*;

const MOD: i64 = 1_004_535_809;
const G: i64 = 3;
// Parallelize butterfly blocks only when each block is large enough.
const PAR_LEN: usize = 256;

#[inline(always)]
fn mul_mod(a: i64, b: i64) -> i64 {
    ((a as u64) * (b as u64) % (MOD as u64)) as i64
}

#[inline(always)]
fn add_mod(a: i64, b: i64) -> i64 {
    let s = a + b;
    if s >= MOD { s - MOD } else { s }
}

#[inline(always)]
fn sub_mod(a: i64, b: i64) -> i64 {
    if a >= b { a - b } else { a - b + MOD }
}

fn power(mut base: i64, mut exp: i64) -> i64 {
    let mut result = 1i64;
    base %= MOD;
    while exp > 0 {
        if exp & 1 == 1 {
            result = mul_mod(result, base);
        }
        base = mul_mod(base, base);
        exp >>= 1;
    }
    result
}

fn inv_mod(x: i64) -> i64 {
    power(x, MOD - 2)
}

#[inline(always)]
fn butterfly_block(block: &mut [i64], half: usize, step: usize, w: &[i64]) {
    let mut k = 0usize;
    for jj in 0..half {
        // SAFETY: block.len() == 2*half so jj and jj+half are in-bounds.
        // k = jj * step < w.len() because (half-1)*(n/len) < n = w.len().
        unsafe {
            let u = *block.get_unchecked(jj);
            let v = mul_mod(*block.get_unchecked(jj + half), *w.get_unchecked(k));
            *block.get_unchecked_mut(jj) = add_mod(u, v);
            *block.get_unchecked_mut(jj + half) = sub_mod(u, v);
        }
        k += step;
    }
}

fn ntt(a: &mut [i64], invert: bool, omega: &[i64], omega_inv: &[i64]) {
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

    let w = if invert { omega_inv } else { omega };

    let mut len = 2;
    while len <= n {
        let step = omega.len() / len;
        let half = len / 2;
        let nblocks = n / len;
        if nblocks >= 2 && len >= PAR_LEN {
            a.par_chunks_mut(len).for_each(|block| {
                butterfly_block(block, half, step, w);
            });
        } else {
            let mut i = 0;
            while i < n {
                butterfly_block(&mut a[i..i + len], half, step, w);
                i += len;
            }
        }
        len <<= 1;
    }

    if invert {
        let n_inv = inv_mod(n as i64);
        for x in a.iter_mut() {
            *x = mul_mod(*x, n_inv);
        }
    }
}

fn poly_mul(
    a: &[i64],
    b: &[i64],
    result: &mut [i64],
    n: usize,
    omega: &[i64],
    omega_inv: &[i64],
    fa: &mut [i64],
    fb: &mut [i64],
) {
    fa[..n].copy_from_slice(&a[..n]);
    fa[n..].fill(0);
    fb[..n].copy_from_slice(&b[..n]);
    fb[n..].fill(0);

    rayon::join(
        || ntt(fa, false, omega, omega_inv),
        || ntt(fb, false, omega, omega_inv),
    );

    let m = fa.len();
    for i in 0..m {
        fa[i] = mul_mod(fa[i], fb[i]);
    }

    ntt(fa, true, omega, omega_inv);
    result[..n].copy_from_slice(&fa[..n]);
}

fn poly_square(
    a: &[i64],
    result: &mut [i64],
    n: usize,
    omega: &[i64],
    omega_inv: &[i64],
    fa: &mut [i64],
) {
    fa[..n].copy_from_slice(&a[..n]);
    fa[n..].fill(0);

    ntt(fa, false, omega, omega_inv);

    let m = fa.len();
    for i in 0..m {
        fa[i] = mul_mod(fa[i], fa[i]);
    }

    ntt(fa, true, omega, omega_inv);
    result[..n].copy_from_slice(&fa[..n]);
}

fn poly_pow(
    a: &[i64],
    k: i32,
    result: &mut [i64],
    n: usize,
    omega: &[i64],
    omega_inv: &[i64],
    fa: &mut [i64],
    fb: &mut [i64],
) {
    let mut base = vec![0i64; n];
    let mut temp = vec![0i64; n];
    base[..n].copy_from_slice(&a[..n]);
    for x in result[..n].iter_mut() {
        *x = 0;
    }
    result[0] = 1;

    let mut k = k;
    while k > 0 {
        if k & 1 == 1 {
            poly_mul(result, &base, &mut temp, n, omega, omega_inv, fa, fb);
            result[..n].copy_from_slice(&temp[..n]);
        }
        poly_square(&base, &mut temp, n, omega, omega_inv, fa);
        base[..n].copy_from_slice(&temp[..n]);
        k >>= 1;
    }
}

fn sieve_primes_fn(limit: usize) -> Vec<i32> {
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    if limit >= 1 {
        is_prime[1] = false;
    }
    let mut i = 2;
    while i * i <= limit {
        if is_prime[i] {
            let mut j = i * i;
            while j <= limit {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    let mut primes = vec![0i32];
    for i in 2..=limit {
        if is_prime[i] {
            primes.push(i as i32);
        }
    }
    primes
}

fn main() {
    let n = 20000usize;
    let k = 20000i32;

    let sp = sieve_primes_fn(250000);

    let mut f = vec![0i64; n + 1];
    f[0] = 1;
    for i in 1..=n {
        f[i] = (sp[i + 1] - sp[i]) as i64;
    }

    let mut ntt_size = 1;
    while ntt_size < 2 * (n + 1) {
        ntt_size <<= 1;
    }
    let w = power(G, (MOD - 1) / ntt_size as i64);
    let w_inv = inv_mod(w);
    let mut omega = vec![0i64; ntt_size];
    let mut omega_inv = vec![0i64; ntt_size];
    omega[0] = 1;
    omega_inv[0] = 1;
    for i in 1..ntt_size {
        omega[i] = mul_mod(omega[i - 1], w);
        omega_inv[i] = mul_mod(omega_inv[i - 1], w_inv);
    }

    let mut result = vec![0i64; n + 1];
    let mut fa = vec![0i64; ntt_size];
    let mut fb = vec![0i64; ntt_size];
    poly_pow(&f, k, &mut result, n + 1, &omega, &omega_inv, &mut fa, &mut fb);

    println!("{}", result[n]);
}
