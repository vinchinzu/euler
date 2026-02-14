// Project Euler 487 - Sums of power sums
// Lagrange interpolation for sum of powers, iterated over primes in range.

use euler_utils::is_prime;

fn mulmod(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128) * (b as u128) % (m as u128)) as u64
}

fn powmod(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut result = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = mulmod(result, base, m);
        }
        base = mulmod(base, base, m);
        exp >>= 1;
    }
    result
}

const MAXM: usize = 10003;

fn sum_powers(n: u64, k: usize, p: u64) -> u64 {
    let m = k + 1;
    let x = n % p;

    let mut y = vec![0u64; m + 1];
    for i in 1..=m {
        y[i] = (y[i - 1] + powmod(i as u64, k as u64, p)) % p;
    }

    if n <= m as u64 {
        return y[n as usize];
    }

    let mut fact = vec![0u64; m + 1];
    let mut inv_fact = vec![0u64; m + 1];
    fact[0] = 1;
    for i in 1..=m {
        fact[i] = mulmod(fact[i - 1], i as u64, p);
    }
    inv_fact[m] = powmod(fact[m], p - 2, p);
    for i in (1..=m).rev() {
        inv_fact[i - 1] = mulmod(inv_fact[i], i as u64, p);
    }

    let mut pre = vec![0u64; m + 2];
    let mut suf = vec![0u64; m + 2];
    pre[0] = 1;
    for i in 0..=m {
        let diff = if x >= i as u64 {
            x - i as u64
        } else {
            (p - (i as u64 - x) % p) % p
        };
        pre[i + 1] = mulmod(pre[i], diff, p);
    }
    suf[m + 1] = 1;
    for i in (0..=m).rev() {
        let diff = if x >= i as u64 {
            x - i as u64
        } else {
            (p - (i as u64 - x) % p) % p
        };
        suf[i] = mulmod(suf[i + 1], diff, p);
    }

    let mut result = 0u64;
    for i in 0..=m {
        let num = mulmod(pre[i], suf[i + 1], p);
        let den = mulmod(inv_fact[i], inv_fact[m - i], p);
        let term = mulmod(mulmod(y[i], num, p), den, p);
        if (m - i) & 1 == 1 {
            result = (result + p - term) % p;
        } else {
            result = (result + term) % p;
        }
    }
    result
}

fn main() {
    let n_val: u64 = 1_000_000_000_000;
    let k: usize = 10000;
    let l: u64 = 2_000_000_000;
    let h: u64 = 2_000_002_000;

    let mut ans: u64 = 0;
    for p in l..h {
        if !is_prime(p) {
            continue;
        }
        let s_k = sum_powers(n_val, k, p);
        let s_k1 = sum_powers(n_val, k + 1, p);
        let np1 = (n_val + 1) % p;
        let term = (mulmod(np1, s_k, p) + p - s_k1) % p;
        ans += term;
    }

    println!("{}", ans);
}
