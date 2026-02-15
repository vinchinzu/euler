// Project Euler 805 - Shifted Multiples
// Sum N(u^3/v^3) mod 10^9+7 for coprime u,v <= 200

const MOD: i64 = 1_000_000_007;

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

fn mod_pow(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
    let mut result: i64 = 1;
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

fn euler_totient(mut n: i64) -> i64 {
    let mut result = n;
    let mut p = 2i64;
    while p * p <= n {
        if n % p == 0 {
            while n % p == 0 { n /= p; }
            result -= result / p;
        }
        p += 1;
    }
    if n > 1 { result -= result / n; }
    result
}

fn mult_order(base: i64, m: i64) -> i64 {
    if m == 1 { return 1; }
    let phi = euler_totient(m);

    // Find prime factorization of phi
    let mut temp = phi;
    let mut factors = Vec::new();
    let mut p = 2i64;
    while p * p <= temp {
        if temp % p == 0 {
            factors.push(p);
            while temp % p == 0 { temp /= p; }
        }
        p += 1;
    }
    if temp > 1 { factors.push(temp); }

    let mut ord = phi;
    for &f in &factors {
        while ord % f == 0 {
            if mod_pow(base, ord / f, m) == 1 {
                ord /= f;
            } else {
                break;
            }
        }
    }
    ord
}

fn main() {
    let n = 200;
    let mut ans: i64 = 0;

    for v in 1..=n {
        for u in 1..=n {
            if gcd(u, v) != 1 { continue; }

            let u3 = u * u * u;
            let v3 = v * v * v;
            let c = 10 * v3 - u3;
            if c <= 0 { continue; }

            let mut best_k: i64 = -1;
            let mut best_a: i64 = 0;

            for a in 1..=9i64 {
                if a * u3 >= c { continue; }

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

                if gcd(10, d) != 1 { continue; }

                let k = mult_order(10, d) - 1;
                if best_k < 0 || k < best_k || (k == best_k && a < best_a) {
                    best_k = k;
                    best_a = a;
                }
            }

            if best_k >= 0 {
                let k1 = best_k + 1;
                let p10 = mod_pow(10, k1, MOD);
                let mut num = (best_a % MOD) * (v3 % MOD) % MOD;
                num = num * ((p10 - 1 + MOD) % MOD) % MOD;
                let inv_c = mod_pow(c % MOD, MOD - 2, MOD);
                num = (num as i128 * inv_c as i128 % MOD as i128) as i64;
                ans = (ans + num) % MOD;
            }
        }
    }

    println!("{}", ans);
}
