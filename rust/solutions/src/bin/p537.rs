// Project Euler 537 - Counting Tuples
//
// NTT-based polynomial exponentiation.
// MOD = 1004535809 = 479 * 2^21 + 1.

const N_MAX: usize = 20_001;
const MOD: i64 = 1_004_535_809;
const G: i64 = 3;

fn power(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut result = 1i64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 { result = (result as i128 * base as i128 % m as i128) as i64; }
        base = (base as i128 * base as i128 % m as i128) as i64;
        exp >>= 1;
    }
    result
}

fn inv_mod(x: i64) -> i64 { power(x, MOD - 2, MOD) }

fn ntt(a: &mut [i64], invert: bool, omega: &[i64], omega_inv: &[i64]) {
    let n = a.len();
    let mut j = 0usize;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 { j ^= bit; bit >>= 1; }
        j ^= bit;
        if i < j { a.swap(i, j); }
    }

    let w = if invert { omega_inv } else { omega };

    let mut len = 2;
    while len <= n {
        let step = omega.len() / len;
        let mut i = 0;
        while i < n {
            let mut k = 0;
            for jj in 0..len / 2 {
                let u = a[i + jj];
                let v = (a[i + jj + len / 2] as i128 * w[k] as i128 % MOD as i128) as i64;
                a[i + jj] = (u + v) % MOD;
                a[i + jj + len / 2] = (u - v + MOD) % MOD;
                k += step;
            }
            i += len;
        }
        len <<= 1;
    }

    if invert {
        let n_inv = inv_mod(n as i64);
        for x in a.iter_mut() {
            *x = (*x as i128 * n_inv as i128 % MOD as i128) as i64;
        }
    }
}

fn poly_mul(a: &[i64], b: &[i64], result: &mut [i64], n: usize,
            omega: &[i64], omega_inv: &[i64]) {
    let mut m = 1;
    while m < 2 * n { m <<= 1; }

    let mut fa = vec![0i64; m];
    let mut fb = vec![0i64; m];
    fa[..n].copy_from_slice(&a[..n]);
    fb[..n].copy_from_slice(&b[..n]);

    ntt(&mut fa, false, omega, omega_inv);
    ntt(&mut fb, false, omega, omega_inv);

    for i in 0..m {
        fa[i] = (fa[i] as i128 * fb[i] as i128 % MOD as i128) as i64;
    }

    ntt(&mut fa, true, omega, omega_inv);
    result[..n].copy_from_slice(&fa[..n]);
}

fn poly_pow(a: &[i64], k: i32, result: &mut [i64], n: usize,
            omega: &[i64], omega_inv: &[i64]) {
    let mut base = vec![0i64; n];
    let mut temp = vec![0i64; n];
    base[..n].copy_from_slice(&a[..n]);
    for x in result[..n].iter_mut() { *x = 0; }
    result[0] = 1;

    let mut k = k;
    while k > 0 {
        if k & 1 == 1 {
            poly_mul(result, &base, &mut temp, n, omega, omega_inv);
            result[..n].copy_from_slice(&temp[..n]);
        }
        poly_mul(&base, &base, &mut temp, n, omega, omega_inv);
        base[..n].copy_from_slice(&temp[..n]);
        k >>= 1;
    }
}

fn sieve_primes_fn(limit: usize) -> Vec<i32> {
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    if limit >= 1 { is_prime[1] = false; }
    let mut i = 2;
    while i * i <= limit {
        if is_prime[i] {
            let mut j = i * i;
            while j <= limit { is_prime[j] = false; j += i; }
        }
        i += 1;
    }
    // Index 0 = 0, then primes
    let mut primes = vec![0i32];
    for i in 2..=limit {
        if is_prime[i] { primes.push(i as i32); }
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

    // Init NTT
    let mut ntt_size = 1;
    while ntt_size < 2 * (n + 1) { ntt_size <<= 1; }
    let w = power(G, (MOD - 1) / ntt_size as i64, MOD);
    let w_inv = inv_mod(w);
    let mut omega = vec![0i64; ntt_size];
    let mut omega_inv = vec![0i64; ntt_size];
    omega[0] = 1; omega_inv[0] = 1;
    for i in 1..ntt_size {
        omega[i] = (omega[i - 1] as i128 * w as i128 % MOD as i128) as i64;
        omega_inv[i] = (omega_inv[i - 1] as i128 * w_inv as i128 % MOD as i128) as i64;
    }

    let mut result = vec![0i64; n + 1];
    poly_pow(&f, k, &mut result, n + 1, &omega, &omega_inv);

    println!("{}", result[n]);
}
