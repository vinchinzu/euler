// Project Euler 921 - Fibonacci-based sum with Pisano period and matrix exponentiation
// M = 398874989 (prime), compute S = sum_{i=2}^{1618034} (p^5 + q^5) mod M

const M_VAL: u64 = 398_874_989;

fn mod_pow(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut result = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result as u128 * base as u128 % m as u128) as u64;
        }
        base = (base as u128 * base as u128 % m as u128) as u64;
        exp >>= 1;
    }
    result
}

/// Returns (F_n mod m, F_{n+1} mod m) via matrix exponentiation
fn fib_mod(n: u64, m: u64) -> (u64, u64) {
    if n == 0 { return (0, 1); }
    // Matrix [[1,1],[1,0]]^n
    let mut a = [[1u64, 1], [1, 0]]; // matrix A
    let mut r = [[1u64, 0], [0, 1]]; // result = identity
    let mut p = n;

    while p > 0 {
        if p & 1 == 1 {
            let t00 = ((r[0][0] as u128 * a[0][0] as u128 + r[0][1] as u128 * a[1][0] as u128) % m as u128) as u64;
            let t01 = ((r[0][0] as u128 * a[0][1] as u128 + r[0][1] as u128 * a[1][1] as u128) % m as u128) as u64;
            let t10 = ((r[1][0] as u128 * a[0][0] as u128 + r[1][1] as u128 * a[1][0] as u128) % m as u128) as u64;
            let t11 = ((r[1][0] as u128 * a[0][1] as u128 + r[1][1] as u128 * a[1][1] as u128) % m as u128) as u64;
            r = [[t00, t01], [t10, t11]];
        }
        let t00 = ((a[0][0] as u128 * a[0][0] as u128 + a[0][1] as u128 * a[1][0] as u128) % m as u128) as u64;
        let t01 = ((a[0][0] as u128 * a[0][1] as u128 + a[0][1] as u128 * a[1][1] as u128) % m as u128) as u64;
        let t10 = ((a[1][0] as u128 * a[0][0] as u128 + a[1][1] as u128 * a[1][0] as u128) % m as u128) as u64;
        let t11 = ((a[1][0] as u128 * a[0][1] as u128 + a[1][1] as u128 * a[1][1] as u128) % m as u128) as u64;
        a = [[t00, t01], [t10, t11]];
        p >>= 1;
    }
    // F_n = r[1][0], F_{n+1} = r[0][0]
    (r[1][0], r[0][0])
}

fn find_pisano_period(p: u64) -> u64 {
    let candidate = if p % 5 == 1 || p % 5 == 4 {
        p - 1
    } else {
        2 * (p + 1)
    };

    // Find divisors of candidate
    let mut divs = Vec::new();
    let mut i = 1u64;
    while i * i <= candidate {
        if candidate % i == 0 {
            divs.push(i);
            if i * i != candidate {
                divs.push(candidate / i);
            }
        }
        i += 1;
    }
    divs.sort();

    for d in divs {
        let (fn_val, _) = fib_mod(d, p);
        let (fn1_val, _) = fib_mod(d + 1, p);
        if fn_val == 0 && fn1_val == 1 {
            return d;
        }
    }
    candidate
}

fn find_order(a: u64, m: u64, phi_m: u64) -> u64 {
    // Factor phi_m
    let mut temp = phi_m;
    let mut pfactors = Vec::new();
    let mut d = 2u64;
    while d * d <= temp {
        if temp % d == 0 {
            pfactors.push(d);
            while temp % d == 0 { temp /= d; }
        }
        d += 1;
    }
    if temp > 1 { pfactors.push(temp); }

    let mut order = phi_m;
    for &pf in &pfactors {
        while order % pf == 0 && mod_pow(a, order / pf, m) == 1 {
            order /= pf;
        }
    }
    order
}

fn main() {
    let pi_m = find_pisano_period(M_VAL);

    // Factor pi_M to compute phi(pi_M)
    let mut temp = pi_m;
    let mut factors = Vec::new();
    let mut exps = Vec::new();
    let mut d = 2u64;
    while d * d <= temp {
        if temp % d == 0 {
            let mut e = 0;
            while temp % d == 0 { e += 1; temp /= d; }
            factors.push(d);
            exps.push(e);
        }
        d += 1;
    }
    if temp > 1 { factors.push(temp); exps.push(1); }

    let mut phi_pi = 1u64;
    for i in 0..factors.len() {
        let mut pw = 1u64;
        for _ in 0..exps[i] - 1 { pw *= factors[i]; }
        phi_pi *= (factors[i] - 1) * pw;
    }

    let l = find_order(5, pi_m, phi_pi);
    let inv_2 = (M_VAL + 1) / 2;
    let mut total_s = 0u64;
    let mut a_fib = 1u64; // F_1 mod L
    let mut b_fib = 1u64; // F_2 mod L
    let m_limit = 1_618_034;

    for _ in 2..=m_limit {
        let k = 3 * mod_pow(5, b_fib, pi_m) % pi_m;
        let (f_k, f_k1) = fib_mod(k, M_VAL);
        // L_K = 2*F_{K+1} - F_K mod M
        let l_k = (2 * f_k1 % M_VAL + M_VAL - f_k) % M_VAL;

        let p = (f_k as u128 * inv_2 as u128 % M_VAL as u128) as u64;
        let q = (l_k as u128 * inv_2 as u128 % M_VAL as u128) as u64;

        let term = (mod_pow(p, 5, M_VAL) + mod_pow(q, 5, M_VAL)) % M_VAL;
        total_s = (total_s + term) % M_VAL;

        let new_b = (a_fib + b_fib) % l;
        a_fib = b_fib;
        b_fib = new_b;
    }

    println!("{}", total_s);
}
