// Problem 947: Fibonacci Residues
// s(m) = sum_{a,b in Z_m} p(a,b,m)^2
// S(M) = sum_{m=1..M} s(m), find S(10^6) mod 999999893
//
// For each m, factor into prime powers. For each prime power p^e,
// compute the period distribution (which periods occur and how many
// (a,b) pairs have each period). Then combine via Cartesian product
// with lcm to get s(m).
//
// Key: |ker(F^d - I mod p^e)| = p^min(v1,e) * p^min(v2,e)
// where v1 = min v_p of entries, v2 = v_p(det(F^d-I)) - v1
// det(F^d-I) must be computed with enough p-adic precision.

use euler_utils::primes::sieve_smallest_factor;
use std::collections::HashMap;

const MODV: u64 = 999_999_893;

fn main() {
    let m_max: usize = 1_000_000;
    let answer = solve(m_max);
    println!("{}", answer);
}

#[derive(Clone, Copy)]
struct Mat2 { a: [u64; 4] }

impl Mat2 {
    fn identity() -> Self { Mat2 { a: [1, 0, 0, 1] } }
    fn fib() -> Self { Mat2 { a: [0, 1, 1, 1] } }

    #[inline]
    fn mul_mod(&self, other: &Mat2, m: u64) -> Mat2 {
        let m128 = m as u128;
        Mat2 { a: [
            ((self.a[0] as u128 * other.a[0] as u128 + self.a[1] as u128 * other.a[2] as u128) % m128) as u64,
            ((self.a[0] as u128 * other.a[1] as u128 + self.a[1] as u128 * other.a[3] as u128) % m128) as u64,
            ((self.a[2] as u128 * other.a[0] as u128 + self.a[3] as u128 * other.a[2] as u128) % m128) as u64,
            ((self.a[2] as u128 * other.a[1] as u128 + self.a[3] as u128 * other.a[3] as u128) % m128) as u64,
        ]}
    }

    fn pow_mod(&self, mut exp: u64, m: u64) -> Mat2 {
        let mut result = Mat2::identity();
        let mut base = *self;
        while exp > 0 {
            if exp & 1 == 1 { result = result.mul_mod(&base, m); }
            base = base.mul_mod(&base, m);
            exp >>= 1;
        }
        result
    }

    fn is_identity(&self, m: u64) -> bool {
        self.a[0] % m == 1 && self.a[1] % m == 0 && self.a[2] % m == 0 && self.a[3] % m == 1
    }

    fn sub_identity(&self, m: u64) -> Mat2 {
        Mat2 { a: [
            (self.a[0] + m - 1) % m,
            self.a[1] % m,
            self.a[2] % m,
            (self.a[3] + m - 1) % m,
        ]}
    }
}

fn gcd(a: u64, b: u64) -> u64 { if b == 0 { a } else { gcd(b, a % b) } }
fn lcm_u64(a: u64, b: u64) -> u64 { if a == 0 || b == 0 { 0 } else { a / gcd(a, b) * b } }

fn pow_u64(mut base: u64, mut exp: u32) -> u64 {
    let mut result = 1u64;
    while exp > 0 {
        if exp & 1 == 1 { result *= base; }
        if exp > 1 { base *= base; }
        exp >>= 1;
    }
    result
}

fn p_adic_val(n: u64, p: u64, cap: u32) -> u64 {
    if n == 0 { return cap as u64; }
    let mut v = 0u64;
    let mut x = n;
    while x % p == 0 { v += 1; x /= p; }
    v
}

fn mod_mul(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}

fn factor_small(mut n: u64) -> Vec<(u64, u32)> {
    let mut res = Vec::new();
    let mut d = 2u64;
    while d * d <= n {
        if n % d == 0 {
            let mut e = 0;
            while n % d == 0 { n /= d; e += 1; }
            res.push((d, e));
        }
        d += 1;
    }
    if n > 1 { res.push((n, 1)); }
    res
}

fn divisors_of(n: u64) -> Vec<u64> {
    let mut divs = Vec::new();
    let mut d = 1u64;
    while d * d <= n {
        if n % d == 0 { divs.push(d); if d != n / d { divs.push(n / d); } }
        d += 1;
    }
    divs.sort_unstable();
    divs
}

fn mobius(n: u64) -> i32 {
    if n == 1 { return 1; }
    let mut val = n;
    let mut count = 0;
    let mut d = 2;
    while d * d <= val {
        if val % d == 0 { count += 1; val /= d; if val % d == 0 { return 0; } }
        d += 1;
    }
    if val > 1 { count += 1; }
    if count % 2 == 0 { 1 } else { -1 }
}

fn legendre_5(p: u64) -> i64 {
    match p % 5 { 1 | 4 => 1, 2 | 3 => -1, 0 => 0, _ => unreachable!() }
}

fn find_order(f: &Mat2, bound: u64, p: u64) -> u64 {
    let factors = factor_small(bound);
    let mut order = bound;
    for &(q, e) in &factors {
        for _ in 0..e {
            let test = order / q;
            if f.pow_mod(test, p).is_identity(p) { order = test; } else { break; }
        }
    }
    order
}

fn pisano_prime(p: u64) -> u64 {
    if p == 2 { return 3; }
    if p == 5 { return 20; }
    let f = Mat2::fib();
    let leg = legendre_5(p);
    let bound = if leg == 1 { p - 1 } else { 2 * (p + 1) };
    find_order(&f, bound, p)
}

fn pisano_period(p: u64, e: u32, pip: u64) -> u64 {
    pip * pow_u64(p, e - 1)
}

// Compute v_p(det(F^d - I)) with sufficient precision
// det(F^d - I) = (-1)^d + 1 - trace(F^d) = (-1)^d + 1 - L_d
fn det_vp(d: u64, p: u64, needed_e: u32) -> u64 {
    // Need precision >= 2*needed_e since det can have v_p up to 2*e
    // (when F^d - I has entries divisible by p^e, det ~ p^{2e})
    let prec = (2 * needed_e + 2).min(60);
    // Try to compute p^prec; if it overflows u64 or exceeds safe limit, reduce.
    // Limit big_pe < 2^63 to avoid overflow in mat_mul (u128) and trace sum (u64).
    const SAFE_LIMIT: u64 = 1u64 << 63;
    let (big_pe, actual_prec) = {
        let mut pr = prec;
        loop {
            if let Some(v) = checked_pow(p, pr) {
                if v < SAFE_LIMIT {
                    break (v, pr);
                }
            }
            pr -= 1;
            if pr == 0 { break (1, 0); }
        }
    };

    if actual_prec == 0 { return 0; }

    let f = Mat2::fib();
    let fd = f.pow_mod(d, big_pe);
    let trace = (fd.a[0] + fd.a[3]) % big_pe;
    let det = if d % 2 == 0 {
        (2 + big_pe - trace) % big_pe
    } else {
        (big_pe - trace) % big_pe
    };

    p_adic_val(det, p, actual_prec)
}

fn checked_pow(base: u64, exp: u32) -> Option<u64> {
    let mut result = 1u64;
    let mut b = base;
    let mut e = exp;
    while e > 0 {
        if e & 1 == 1 { result = result.checked_mul(b)?; }
        if e > 1 { b = b.checked_mul(b)?; }
        e >>= 1;
    }
    Some(result)
}

// Compute |ker(F^d - I mod p^e)|
fn kernel_size(mat: &Mat2, p: u64, e: u32, d: u64) -> u64 {
    let pe = pow_u64(p, e);
    let v_entries = [
        p_adic_val(mat.a[0] % pe, p, e),
        p_adic_val(mat.a[1] % pe, p, e),
        p_adic_val(mat.a[2] % pe, p, e),
        p_adic_val(mat.a[3] % pe, p, e),
    ];
    let v1 = *v_entries.iter().min().unwrap();
    if v1 >= e as u64 { return pe * pe; }

    let vdet = det_vp(d, p, e);
    let v2 = if vdet >= v1 { (vdet - v1).min(e as u64) } else { 0 };

    pow_u64(p, v1.min(e as u64) as u32) * pow_u64(p, v2 as u32)
}

fn period_dist(p: u64, e: u32, pip: u64) -> Vec<(u64, u64)> {
    let pe = pow_u64(p, e);
    let pi_pe = pisano_period(p, e, pip);
    let divs = divisors_of(pi_pe);

    let f = Mat2::fib();

    let mut n_vals: Vec<u64> = Vec::with_capacity(divs.len());
    for &d in &divs {
        let fd = f.pow_mod(d, pe);
        let fd_mi = fd.sub_identity(pe);
        let ks = kernel_size(&fd_mi, p, e, d);
        n_vals.push(ks);
    }

    let mut result: Vec<(u64, u64)> = Vec::new();
    for i in 0..divs.len() {
        let d = divs[i];
        let mut exact: i64 = 0;
        for j in 0..divs.len() {
            let d2 = divs[j];
            if d % d2 == 0 {
                let mu = mobius(d / d2);
                if mu != 0 { exact += mu as i64 * n_vals[j] as i64; }
            }
        }
        if exact > 0 { result.push((d, exact as u64)); }
    }
    result
}

fn factorize(mut n: u64, spf: &[u32]) -> Vec<(u64, u32)> {
    let mut factors = Vec::new();
    while n > 1 {
        let p = spf[n as usize] as u64;
        let mut e = 0u32;
        while n % p == 0 { n /= p; e += 1; }
        factors.push((p, e));
    }
    factors
}

fn solve(m_max: usize) -> u64 {
    let spf = sieve_smallest_factor(m_max);

    // Precompute Pisano periods for all primes
    let mut pip_cache: Vec<u64> = vec![0; m_max + 1];
    for p in 2..=m_max {
        if spf[p] == p as u32 { pip_cache[p] = pisano_prime(p as u64); }
    }

    // Cache period distributions for all prime powers p^e <= m_max
    let mut dist_cache: HashMap<(u64, u32), Vec<(u64, u64)>> = HashMap::new();
    for p in 2..=m_max {
        if spf[p] != p as u32 { continue; }
        let mut pe_val = p as u64;
        let mut e = 1u32;
        while pe_val <= m_max as u64 {
            let dist = period_dist(p as u64, e, pip_cache[p]);
            dist_cache.insert((p as u64, e), dist);
            if pe_val > m_max as u64 / p as u64 { break; }
            pe_val *= p as u64;
            e += 1;
        }
    }

    let mut total: u64 = 1; // s(1) = 1

    for m in 2..=m_max {
        let factors = factorize(m as u64, &spf);
        let s_m = compute_s_m_cached(&factors, &dist_cache);
        total = (total + s_m) % MODV;
    }

    total
}

fn compute_s_m_cached(factors: &[(u64, u32)], cache: &HashMap<(u64, u32), Vec<(u64, u64)>>) -> u64 {
    if factors.is_empty() { return 1; }

    let first = &cache[&factors[0]];
    let mut partial: Vec<(u64, u64)> = first.iter().map(|&(d, c)| (d, c % MODV)).collect();

    for &(p, e) in factors.iter().skip(1) {
        let dist = &cache[&(p, e)];
        let mut new_partial: Vec<(u64, u64)> = Vec::with_capacity(partial.len() * dist.len());
        for &(lcm_so_far, cnt_so_far) in &partial {
            for &(d, c) in dist {
                let new_lcm = lcm_u64(lcm_so_far, d);
                let new_cnt = mod_mul(cnt_so_far, c % MODV, MODV);
                new_partial.push((new_lcm, new_cnt));
            }
        }
        new_partial.sort_unstable_by_key(|&(l, _)| l);
        let mut merged: Vec<(u64, u64)> = Vec::new();
        for &(l, c) in &new_partial {
            if let Some(last) = merged.last_mut() {
                if last.0 == l { last.1 = (last.1 + c) % MODV; continue; }
            }
            merged.push((l, c));
        }
        partial = merged;
    }

    let mut s: u64 = 0;
    for &(lcm_val, cnt) in &partial {
        let lcm_mod = lcm_val % MODV;
        let lcm_sq = mod_mul(lcm_mod, lcm_mod, MODV);
        s = (s + mod_mul(lcm_sq, cnt, MODV)) % MODV;
    }
    s
}
