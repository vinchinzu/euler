// Project Euler 805 - Shifted Multiples
// Sum N(u^3/v^3) mod 10^9+7 for coprime u,v <= 200

use rayon::prelude::*;

const MOD: u64 = 1_000_000_007;

fn gcd(mut a: i64, mut b: i64) -> i64 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

#[inline(always)]
fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result: u64 = 1;
    base %= modulus;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % modulus;
        }
        base = base * base % modulus;
        exp >>= 1;
    }
    result
}

fn euler_totient(mut n: i64) -> i64 {
    let mut result = n;
    let mut p = 2i64;
    while p * p <= n {
        if n % p == 0 {
            while n % p == 0 {
                n /= p;
            }
            result -= result / p;
        }
        p += 1;
    }
    if n > 1 {
        result -= result / n;
    }
    result
}

fn mult_order(base: i64, m: i64) -> i64 {
    if m == 1 {
        return 1;
    }
    let phi = euler_totient(m);

    // Find prime factorization of phi
    let mut temp = phi;
    let mut factors = Vec::new();
    let mut p = 2i64;
    while p * p <= temp {
        if temp % p == 0 {
            factors.push(p);
            while temp % p == 0 {
                temp /= p;
            }
        }
        p += 1;
    }
    if temp > 1 {
        factors.push(temp);
    }

    let mut ord = phi;
    let m_u = m as u64;
    let base_u = base.rem_euclid(m) as u64;
    for &f in &factors {
        while ord % f == 0 {
            if mod_pow(base_u, (ord / f) as u64, m_u) == 1 {
                ord /= f;
            } else {
                break;
            }
        }
    }
    ord
}

fn contrib_for_v(v: i64, n: i64) -> u64 {
    let mut ans: u64 = 0;
    let v3 = v * v * v;

    for u in 1..=n {
        if gcd(u, v) != 1 {
            continue;
        }

        let u3 = u * u * u;
        let c = 10 * v3 - u3;
        if c <= 0 {
            continue;
        }

        let mut best_k: i64 = -1;
        let mut best_a: i64 = 0;

        for a in 1..=9i64 {
            if a * u3 >= c {
                continue;
            }

            let g = gcd(a * v3, c);
            let d = c / g;

            if d == 1 {
                let k = 0;
                if best_k < 0 || k < best_k || (k == best_k && a < best_a) {
                    best_k = k;
                    best_a = a;
                }
                continue;
            }

            if gcd(10, d) != 1 {
                continue;
            }

            let k = mult_order(10, d) - 1;
            if best_k < 0 || k < best_k || (k == best_k && a < best_a) {
                best_k = k;
                best_a = a;
            }
        }

        if best_k >= 0 {
            let k1 = (best_k + 1) as u64;
            let p10 = mod_pow(10, k1, MOD);
            let mut num = (best_a as u64 % MOD) * (v3 as u64 % MOD) % MOD;
            num = num * ((p10 + MOD - 1) % MOD) % MOD;
            let inv_c = mod_pow((c as u64) % MOD, MOD - 2, MOD);
            num = num * inv_c % MOD;
            ans += num;
            if ans >= MOD {
                ans -= MOD;
            }
        }
    }
    ans
}

fn main() {
    let n = 200i64;
    let ans: u64 = (1..=n)
        .into_par_iter()
        .map(|v| contrib_for_v(v, n))
        .reduce(|| 0u64, |a, b| {
            let s = a + b;
            if s >= MOD { s - MOD } else { s }
        });

    println!("{}", ans);
}
