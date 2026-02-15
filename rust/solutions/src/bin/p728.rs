// Project Euler 728 - Circle of Coins
//
// Sum 2^{(i-1)*g} * mult(i) for g=1..N/i, using Euler totient sieve.

const N: usize = 10_000_000;
const MOD: i64 = 1_000_000_007;

fn pow_mod(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut result: i64 = 1;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result as i128 * base as i128 % m as i128) as i64;
        }
        base = (base as i128 * base as i128 % m as i128) as i64;
        exp >>= 1;
    }
    result
}

fn mod_inv(a: i64, m: i64) -> i64 {
    let mut t: i64 = 0;
    let mut new_t: i64 = 1;
    let mut r: i64 = m;
    let mut new_r: i64 = a % m;
    while new_r != 0 {
        let q = r / new_r;
        let tmp = new_t;
        new_t = t - q * new_t;
        t = tmp;
        let tmp = new_r;
        new_r = r - q * new_r;
        r = tmp;
    }
    if t < 0 {
        t += m;
    }
    t
}

fn main() {
    // Sieve phi
    let mut phi = vec![0i64; N + 1];
    for i in 0..=N {
        phi[i] = i as i64;
    }
    for i in 2..=N {
        if phi[i] == i as i64 {
            // prime
            let mut j = i;
            while j <= N {
                phi[j] -= phi[j] / i as i64;
                j += i;
            }
        }
    }

    // Precompute powers of 2 mod M
    let mut pow2s = vec![0i64; N + 1];
    pow2s[0] = 1;
    for i in 1..=N {
        pow2s[i] = pow2s[i - 1] * 2 % MOD;
    }

    let l = N / 30;

    let mut ans: i64 = 0;
    for i in 1..=N {
        let mut res: i64 = 0;
        if i > 1 && i < l {
            // Geometric series
            let base = pow2s[i - 1];
            let max_g = N / i;
            if base == 1 {
                res = max_g as i64 % MOD;
            } else {
                let numerator = (pow_mod(base, max_g as i64 + 1, MOD) - 1 + MOD) % MOD;
                let denominator = mod_inv((base - 1 + MOD) % MOD, MOD);
                res = ((numerator as i128 * denominator as i128 % MOD as i128) as i64 - 1 + MOD) % MOD;
            }
        } else {
            for g in 1..=N / i {
                let idx = (i as i64 - 1) * g as i64;
                if (idx as usize) <= N {
                    res = (res + pow2s[idx as usize]) % MOD;
                } else {
                    res = (res + pow_mod(2, idx, MOD)) % MOD;
                }
            }
        }

        let multiplier = if i == 1 || i % 2 == 0 {
            2 * phi[i] % MOD
        } else {
            3 * phi[i] / 2 % MOD
        };

        ans = (ans + (res as i128 % MOD as i128 * multiplier as i128) as i64 % MOD) % MOD;
    }

    println!("{}", ans);
}
