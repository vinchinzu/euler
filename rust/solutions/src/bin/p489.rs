// Project Euler 489: Common factors between two sequences
// For given (a, b), find GCD of (n^3+b, (n+a)^3+b) using prime factorization + CRT.
// Sum G(a,b) for a=1..18, b=1..1900.

fn gcd_ll(a: i64, b: i64) -> i64 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 { let t = b; b = a % b; a = t; }
    a
}

#[derive(Clone)]
struct Factors {
    primes: Vec<i64>,
    exps: Vec<i32>,
}

fn factorize(mut n: i64) -> Factors {
    let mut f = Factors { primes: Vec::new(), exps: Vec::new() };
    if n < 0 { n = -n; }
    if n <= 1 { return f; }
    let mut d = 2i64;
    while d * d <= n {
        if n % d == 0 {
            let mut e = 0;
            while n % d == 0 { e += 1; n /= d; }
            f.primes.push(d);
            f.exps.push(e);
        }
        d += 1;
    }
    if n > 1 { f.primes.push(n); f.exps.push(1); }
    f
}

fn merge_primes(f1: &Factors, f2: &Factors) -> Vec<i64> {
    let mut out = Vec::new();
    for &p in &f1.primes { if !out.contains(&p) { out.push(p); } }
    for &p in &f2.primes { if !out.contains(&p) { out.push(p); } }
    out
}

fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 { return (b, 0, 1); }
    let (g, x1, y1) = ext_gcd(b % a, a);
    (g, y1 - (b / a) * x1, x1)
}

fn mod_inv_fn(a: i64, m: i64) -> Option<i64> {
    if m == 1 { return Some(0); }
    let a = ((a % m) + m) % m;
    let (g, x, _) = ext_gcd(a, m);
    if g != 1 { return None; }
    Some(((x % m) + m) % m)
}

fn crt2(r1: i64, m1: i64, r2: i64, m2: i64) -> Option<(i64, i64)> {
    let g = gcd_ll(m1, m2);
    if (r2 - r1) % g != 0 { return None; }
    let lcm = m1 / g * m2;
    let inv = mod_inv_fn(m1 / g, m2 / g)?;
    let r = r1 + (((r2 - r1) / g % (m2 / g) * inv % (m2 / g)) as i128 * (m1 as i128) % lcm as i128) as i64;
    let r = ((r % lcm) + lcm) % lcm;
    Some((r, lcm))
}

fn cb_mod(n: i64, m: i64) -> i64 {
    let n = ((n % m) + m) % m;
    ((n as i128 * n as i128 % m as i128 * n as i128 % m as i128) as i64 + m) % m
}

struct PrimeSolution {
    ns: Vec<i64>,
    modulus: i64,
}

fn find_prime_solutions(a: i64, b: i64, p: i64) -> PrimeSolution {
    let mut prev_ns: Vec<i64> = Vec::new();
    let mut prev_m: i64 = 1;

    let mut m = 1i64;
    loop {
        m = prev_m.checked_mul(p).unwrap_or(i64::MAX);
        if m / p != prev_m || m > 1_000_000_000_000 { break; }

        let mut ns_buf = Vec::new();

        if gcd_ll(p, 6 * a) == 1 {
            let two_a2 = ((2i128 * a as i128 % m as i128 * a as i128 % m as i128) as i64 + m) % m;
            if let Some(inv_val) = mod_inv_fn(two_a2, m) {
                let inner = ((3i128 * b as i128 + (a as i128).pow(3)) % m as i128 + m as i128) as i64 % m;
                let n = ((-(inv_val as i128) % m as i128 * (inner as i128) % m as i128) % m as i128 + 2 * m as i128) as i64 % m;
                let neg_b = ((-b) % m + m) % m;
                if cb_mod(n, m) == neg_b && cb_mod(n + a, m) == neg_b {
                    ns_buf.push(n);
                }
            }
        } else {
            for n in 0..m {
                if ns_buf.len() >= 1000 { break; }
                let v1 = (cb_mod(n, m) + ((b % m) + m) % m) % m;
                let v2 = (cb_mod(n + a, m) + ((b % m) + m) % m) % m;
                if v1 == 0 && v2 == 0 {
                    ns_buf.push(n);
                }
            }
        }

        if ns_buf.is_empty() { break; }

        prev_ns = ns_buf;
        prev_m = m;
    }

    PrimeSolution { ns: prev_ns, modulus: prev_m }
}

fn g_fn(a: i32, b: i32) -> i64 {
    let f1 = factorize(6 * a as i64);
    let expr = (a as i64).pow(6) + 27 * (b as i64) * (b as i64);
    let f2 = factorize(expr);

    let all_primes = merge_primes(&f1, &f2);
    if all_primes.is_empty() { return 0; }

    let mut sols = Vec::new();
    for &p in &all_primes {
        let sol = find_prime_solutions(a as i64, b as i64, p);
        if !sol.ns.is_empty() {
            sols.push(sol);
        }
    }
    if sols.is_empty() { return 0; }

    // CRT combination
    let mut cur: Vec<(i64, i64)> = sols[0].ns.iter().map(|&r| (r, sols[0].modulus)).collect();

    for si in 1..sols.len() {
        let mut new_cur = Vec::new();
        for &(cr, cm) in &cur {
            for &nr in &sols[si].ns {
                if let Some((r, m)) = crt2(cr, cm, nr, sols[si].modulus) {
                    new_cur.push((r, m));
                }
            }
        }
        cur = new_cur;
    }

    let mut best: i64 = -1;
    for &(v, _m) in &cur {
        let v = if v < 0 { v + _m } else { v };
        if best < 0 || v < best { best = v; }
    }

    if best >= 0 { best } else { 0 }
}

fn main() {
    let ma = 18;
    let n = 1900;
    let mut ans: i64 = 0;

    for a in 1..=ma {
        for b in 1..=n {
            ans += g_fn(a, b);
        }
    }

    println!("{}", ans);
}
