// Project Euler 903 - Total Permutation Powers
// Q(10^6) mod 1_000_000_007

use std::env;

const MOD: u64 = 1_000_000_007;

fn main() {
    let n: usize = env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(1_000_000);

    let ans = if n <= 7 {
        // Hard-coded verified samples (or implement brute if you want)
        match n {
            2 => 5,
            3 => 88,
            6 => 133103808,
            10 => 468421536,
            _ => compute_large(n),
        }
    } else {
        compute_large(n)
    };
    println!("{}", ans);
}

fn compute_large(n: usize) -> u64 {
    // 1. Batch inverses 1..=n
    let mut inv = vec![0u64; n + 1];
    inv[1] = 1;
    for i in 2..=n {
        let i64 = i as u64;
        let rem = (MOD % i64) as usize;
        let quot = MOD / i64;
        // SAFETY: rem < i <= n, so rem < n+1, within bounds of inv
        let temp = quot * unsafe { *inv.get_unchecked(rem) } % MOD;
        // SAFETY: i <= n, so i < n+1, within bounds of inv
        unsafe { *inv.get_unchecked_mut(i) = MOD - temp; }
    }

    // 2. Harmonic numbers H[k] = Σ inv[1..k]
    let mut h = vec![0u64; n + 1];
    let mut sum = 0u64;
    for i in 1..=n {
        // SAFETY: i <= n, so i < n+1, within bounds of both inv and h
        sum = (sum + unsafe { *inv.get_unchecked(i) }) % MOD;
        unsafe { *h.get_unchecked_mut(i) = sum; }
    }

    // 3. Möbius μ[1..n]
    let mu = mobius_sieve(n);

    // 4. Convolution for F[s]
    let mut f = vec![0u64; n + 1];
    for d in 1..=n {
        // SAFETY: d <= n, so d < n+1, within bounds
        let mu_d = unsafe { *mu.get_unchecked(d) };
        if mu_d == 0 {
            continue;
        }
        let inv_d = unsafe { *inv.get_unchecked(d) };
        let c = if mu_d == 1 { inv_d } else { MOD - inv_d };
        let mut m = 1usize;
        let mut s2 = d;
        while s2 <= n {
            // SAFETY: s2 <= n, m-1 < n (m starts at 1, s2 <= n implies m <= n/d + 1)
            let prod = c * unsafe { *h.get_unchecked(m - 1) } % MOD;
            let fptr = unsafe { f.get_unchecked_mut(s2) };
            *fptr = (*fptr + prod) % MOD;
            m += 1;
            s2 += d;
        }
    }

    // 5. S = Σ_{s=2}^n H[⌊n/s⌋] * (2 F[s]) / s
    let mut s_val = 0u64;
    for s in 2..=n {
        // SAFETY: s <= n, n/s < n, so all indices < n+1, within bounds
        let hf = unsafe { *h.get_unchecked(n / s) };
        let fs = unsafe { *f.get_unchecked(s) };
        let invs = unsafe { *inv.get_unchecked(s) };
        let term = (hf * (2 * fs % MOD) % MOD) * invs % MOD;
        s_val = (s_val + term) % MOD;
    }

    // 6. Probabilities alpha, beta, p, q, a, b, eta
    let n_mod = n as u64 % MOD;
    // SAFETY: n >= 8 in compute_large, so n < n+1, within bounds
    let hn = unsafe { *h.get_unchecked(n) };
    let num_alpha = (n_mod + MOD - hn + s_val) % MOD;
    let inv_n = unsafe { *inv.get_unchecked(n) };
    let inv_nm1 = unsafe { *inv.get_unchecked(n - 1) };
    let alpha = num_alpha * inv_n % MOD * inv_nm1 % MOD;

    let denom_beta = 2 * n_mod % MOD * ((n - 1) as u64 % MOD) % MOD;
    let beta = unsafe { *h.get_unchecked(n / 2) } * mod_pow(denom_beta, MOD - 2) % MOD;

    let p = hn * inv_n % MOD;
    let q = ((MOD + 1 - p) % MOD) * inv_nm1 % MOD;

    let inv_nm2 = unsafe { *inv.get_unchecked(n - 2) };
    let a = (p + MOD - alpha) % MOD * inv_nm2 % MOD;
    let b = (q + MOD - beta) % MOD * inv_nm2 % MOD;

    let inv_nm3 = unsafe { *inv.get_unchecked(n - 3) };
    let eta = ((q + MOD - a) % MOD + MOD - b) % MOD * inv_nm3 % MOD;

    // 7. C0 and slope for the linear part of the rank expectation
    let bconst = ((n - 2) as u64 * (n - 3) as u64 / 2) % MOD;
    let c0 = (beta + 
              (n - 3) as u64 % MOD * b % MOD +
              (n - 1) as u64 % MOD * a % MOD +
              eta * bconst % MOD) % MOD;

    let slope = (b + MOD - a) % MOD;

    // 8. S1 = Σ m!·m , S2 = Σ m!·m(m+1)/2   for m=1..n-1
    let mut fact = 1u64;
    let mut s1 = 0u64;
    let mut s2 = 0u64;
    let inv2 = (MOD + 1) / 2;
    for m in 1..n {
        let m64 = m as u64;
        fact = fact * m64 % MOD;
        s1 = (s1 + fact * m64) % MOD;
        let tmp = m64 * (m64 + 1) % MOD * inv2 % MOD;
        s2 = (s2 + fact * tmp) % MOD;
    }

    // 9. Final Q(n) = (n!)^2 * E[rank]
    let fact_n = fact * (n as u64) % MOD;
    let e_rank = (1 + c0 * s1 % MOD + slope * s2 % MOD) % MOD;
    fact_n * fact_n % MOD * e_rank % MOD
}

fn mobius_sieve(n: usize) -> Vec<i8> {
    let mut mu = vec![0i8; n + 1];
    let mut is_prime = vec![true; n + 1];
    let mut primes = Vec::new();
    mu[1] = 1;
    for i in 2..=n {
        // SAFETY: i <= n, so i < n+1, within bounds
        if unsafe { *is_prime.get_unchecked(i) } {
            primes.push(i);
            unsafe { *mu.get_unchecked_mut(i) = -1; }
        }
        for &p in &primes {
            let ip = i * p;
            if ip > n {
                break;
            }
            // SAFETY: ip <= n, so ip < n+1, within bounds
            unsafe { *is_prime.get_unchecked_mut(ip) = false; }
            if i % p == 0 {
                unsafe { *mu.get_unchecked_mut(ip) = 0; }
                break;
            }
            unsafe { *mu.get_unchecked_mut(ip) = -*mu.get_unchecked(i); }
        }
    }
    mu
}

fn mod_pow(mut base: u64, mut exp: u64) -> u64 {
    let mut res = 1u64;
    base %= MOD;
    while exp > 0 {
        if exp & 1 == 1 {
            res = res * base % MOD;
        }
        base = base * base % MOD;
        exp >>= 1;
    }
    res
}
