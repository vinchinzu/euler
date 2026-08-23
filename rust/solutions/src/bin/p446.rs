// Project Euler 446 - Retractions B
// F(N) = sum_{n=1}^N R(n^4 + 4), using Sophie Germain identity and sieve.

use rayon::prelude::*;

const N: usize = 10_000_000;
const MOD: u64 = 1_000_000_007;

#[inline(always)]
fn mod_pow(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut result = 1u64;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % m;
        }
        base = base * base % m;
        exp >>= 1;
    }
    result
}

/// Inverse of odd `m` modulo 2^64.
#[inline(always)]
fn inv_odd_u64(odd: u64) -> u64 {
    let mut y = odd;
    y = y.wrapping_mul(2u64.wrapping_sub(odd.wrapping_mul(y)));
    y = y.wrapping_mul(2u64.wrapping_sub(odd.wrapping_mul(y)));
    y = y.wrapping_mul(2u64.wrapping_sub(odd.wrapping_mul(y)));
    y = y.wrapping_mul(2u64.wrapping_sub(odd.wrapping_mul(y)));
    y = y.wrapping_mul(2u64.wrapping_sub(odd.wrapping_mul(y)));
    y
}

/// Jacobi(a/n) for odd n > 0. Returns -1, 0, or 1.
#[inline(always)]
fn jacobi(mut a: u64, mut n: u64) -> i32 {
    let mut t = 1i32;
    a %= n;
    while a != 0 {
        while a & 1 == 0 {
            a >>= 1;
            let r = n & 7;
            if r == 3 || r == 5 {
                t = -t;
            }
        }
        core::mem::swap(&mut a, &mut n);
        if a & 3 == 3 && n & 3 == 3 {
            t = -t;
        }
        a %= n;
    }
    if n == 1 { t } else { 0 }
}

/// sqrt(-1) mod p for p ≡ 1 (mod 4). p^2 fits in u64.
fn sqrt_neg1(p: u64) -> u64 {
    let exp = (p - 1) / 4;
    if p & 7 == 5 {
        return mod_pow(2, exp, p);
    }
    // p ≡ 1 (mod 8): z^{(p-1)/4} for a quadratic non-residue z.
    let mut z = 3u64;
    loop {
        if jacobi(z, p) == -1 {
            return mod_pow(z, exp, p);
        }
        z += 2;
    }
}

fn main() {
    let mut is_prime = vec![true; N + 2];
    is_prime[0] = false;
    is_prime[1] = false;
    {
        let mut i = 2usize;
        while i * i <= N + 1 {
            if is_prime[i] {
                let mut j = i * i;
                while j <= N + 1 {
                    is_prime[j] = false;
                    j += i;
                }
            }
            i += 1;
        }
    }

    let mut primes = Vec::with_capacity(350_000);
    for p in 5..=N + 1 {
        if is_prime[p] && p % 4 == 1 {
            primes.push(p as u32);
        }
    }
    drop(is_prime);

    // (p, r, p-r, p^{-1} mod 2^64, (2^64-1)/p)
    let roots: Vec<(u32, u32, u32, u64, u64)> = primes
        .par_iter()
        .with_min_len(32)
        .map(|&p| {
            let p64 = p as u64;
            let r = sqrt_neg1(p64) as u32;
            (p, r, p - r, inv_odd_u64(p64), u64::MAX / p64)
        })
        .collect();
    drop(primes);

    let mut factors = vec![0u64; N + 2];
    for k in 0..=N + 1 {
        let ku = k as u64;
        let mut v = ku * ku + 1;
        if k & 1 == 1 {
            v >>= 1;
        }
        factors[k] = v;
    }

    let mut res = vec![1u32; N + 3];
    for i in (2..=N).step_by(2) {
        res[i] = 5;
    }

    let n1 = N + 1;
    for &(p, r1, r2, pinv, max_q) in &roots {
        let pu = p as usize;
        let p64 = p as u64;
        let nroots = 1 + (r1 != r2) as usize;
        let starts = [r1 as usize, r2 as usize];
        for &start in &starts[..nroots] {
            let mut k = start;
            while k <= n1 {
                // SAFETY: k in 1..=N+1; res has length N+3; p | factors[k] at least once.
                unsafe {
                    let f = factors.get_unchecked_mut(k);
                    *f = f.wrapping_mul(pinv);
                    let mut pw = p64;
                    loop {
                        let q = f.wrapping_mul(pinv);
                        if q > max_q {
                            break;
                        }
                        *f = q;
                        pw *= p64;
                    }
                    let term = (1 + pw) % MOD;
                    let a = res.get_unchecked_mut(k - 1);
                    *a = ((*a as u64 * term) % MOD) as u32;
                    let b = res.get_unchecked_mut(k + 1);
                    *b = ((*b as u64 * term) % MOD) as u32;
                }
                k += pu;
            }
        }
    }

    for k in 0..=n1 {
        let f = factors[k];
        if f > 1 {
            let term = (1 + f) % MOD;
            if k >= 1 {
                res[k - 1] = ((res[k - 1] as u64 * term) % MOD) as u32;
            }
            res[k + 1] = ((res[k + 1] as u64 * term) % MOD) as u32;
        }
    }

    let mut ans = 0u64;
    for i in 1..=N {
        let n = i as u64;
        let n2 = n * n % MOD;
        let n4 = n2 * n2 % MOD;
        let n4p4 = (n4 + 4) % MOD;
        ans += res[i] as u64 + MOD - n4p4;
        if ans >= 2 * MOD {
            ans -= 2 * MOD;
        } else if ans >= MOD {
            ans -= MOD;
        }
    }

    println!("{}", ans);
}
