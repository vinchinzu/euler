// Project Euler 457 - A square on the hypotenuse
// f(n) = n^2 - 3n - 1, R(p) = smallest positive n with f(n) ≡ 0 (mod p^2).
// Sum R(p) for all primes p <= 10^7.

const MAXN: usize = 10_000_001;

fn pow_mod(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
    let mut result = 1i64;
    base = base.rem_euclid(modulus);
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result as i128 * base as i128 % modulus as i128) as i64;
        }
        base = (base as i128 * base as i128 % modulus as i128) as i64;
        exp >>= 1;
    }
    result
}

fn sqrt_mod(a: i64, p: i64) -> i64 {
    let a = a.rem_euclid(p);
    if a == 0 {
        return 0;
    }
    if p == 2 {
        return a % 2;
    }
    if pow_mod(a, (p - 1) / 2, p) != 1 {
        return -1;
    }

    let mut q = p - 1;
    let mut s = 0;
    while q % 2 == 0 {
        q /= 2;
        s += 1;
    }

    let mut z = 2i64;
    while pow_mod(z, (p - 1) / 2, p) != p - 1 {
        z += 1;
    }

    let mut m_val = s;
    let mut c = pow_mod(z, q, p);
    let mut t = pow_mod(a, q, p);
    let mut r = pow_mod(a, (q + 1) / 2, p);

    while t != 1 {
        let mut tt = t;
        let mut i = 0;
        while i < m_val && tt != 1 {
            tt = (tt as i128 * tt as i128 % p as i128) as i64;
            i += 1;
        }
        let b = pow_mod(c, 1i64 << (m_val - i - 1), p);
        m_val = i;
        c = (b as i128 * b as i128 % p as i128) as i64;
        t = (t as i128 * c as i128 % p as i128) as i64;
        r = (r as i128 * b as i128 % p as i128) as i64;
    }

    r
}

fn mod_inv_fn(a: i64, m: i64) -> i64 {
    let a = a.rem_euclid(m);
    let (mut t, mut new_t) = (0i64, 1i64);
    let (mut r, mut new_r) = (m, a);
    while new_r != 0 {
        let q = r / new_r;
        let tmp = new_t;
        new_t = t - q * new_t;
        t = tmp;
        let tmp = new_r;
        new_r = r - q * new_r;
        r = tmp;
    }
    if t < 0 { t + m } else { t }
}

fn compute_r(p: i64) -> i64 {
    if p <= 13 {
        let p2 = p * p;
        for n in 1..=p2 {
            let fn_val = ((n * n - 3 * n - 1) % p2 + p2) % p2;
            if fn_val == 0 {
                return n;
            }
        }
        return 0;
    }

    let sv = sqrt_mod(13, p);
    if sv < 0 {
        return 0;
    }

    let inv2 = (p + 1) / 2;
    let mut r_min = i64::MAX;

    for sign in [-1i64, 1i64] {
        let n_val = ((3 + sign * sv).rem_euclid(p)) * inv2 % p;
        let fn_full = n_val as i128 * n_val as i128 - 3 * n_val as i128 - 1;
        let k_num = fn_full / p as i128;
        let deriv = (2 * n_val - 3).rem_euclid(p);
        let inv_deriv = mod_inv_fn(deriv, p);
        let k = (((-k_num).rem_euclid(p as i128)) as i64 % p + p) % p * inv_deriv % p;
        let candidate = k * p + n_val;
        if candidate > 0 && candidate < r_min {
            r_min = candidate;
        }
    }

    if r_min < i64::MAX { r_min } else { 0 }
}

fn main() {
    let n = MAXN - 1;

    let mut is_prime = vec![true; MAXN];
    is_prime[0] = false;
    is_prime[1] = false;
    {
        let mut i = 2;
        while i * i < MAXN {
            if is_prime[i] {
                let mut j = i * i;
                while j < MAXN {
                    is_prime[j] = false;
                    j += i;
                }
            }
            i += 1;
        }
    }

    let mut ans: i64 = 0;
    for p in 2..=n {
        if is_prime[p] {
            ans += compute_r(p as i64);
        }
    }

    println!("{}", ans);
}
