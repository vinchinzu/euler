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
// det(F^d-I) = (-1)^d + 1 - L_d, computed mod p^{2e} (fits u64).

use rayon::prelude::*;

const MODV: u64 = 999_999_893;
const KMAX: usize = 22; // 2^21 > 2e6 >= max exponent / Pisano bound

fn main() {
    let m_max: usize = 1_000_000;
    let answer = solve(m_max);
    println!("{}", answer);
}

fn solve(m_max: usize) -> u64 {
    let spf_lim = 2 * m_max + 8;
    let (spf, primes) = linear_sieve(spf_lim);
    let mut mu = vec![0i8; spf_lim + 1];
    mu[1] = 1;
    for i in 2..=spf_lim {
        let p = spf[i] as usize;
        let r = i / p;
        mu[i] = if r % p == 0 { 0 } else { -mu[r] };
    }

    let m_max_u = m_max as u64;
    let n_primes = primes.partition_point(|&p| p <= m_max_u);

    let nested: Vec<Vec<(u64, Vec<(u64, u64)>)>> = primes[..n_primes]
        .par_iter()
        .map(|&p| dists_for_prime(p, m_max_u, &spf, &mu))
        .collect();

    let mut dist_cache: Vec<Vec<(u64, u64)>> = vec![Vec::new(); m_max + 1];
    for inner in nested {
        for (pe, dist) in inner {
            dist_cache[pe as usize] = dist;
        }
    }

    // s(1) = 1; each s(m) is already reduced mod MODV, so the sum fits u64.
    let total: u64 = (2..m_max + 1)
        .into_par_iter()
        .with_min_len(256)
        .map(|m| s_of_m(m as u64, &spf, &dist_cache))
        .sum();
    (total + 1) % MODV
}

fn linear_sieve(limit: usize) -> (Vec<u32>, Vec<u64>) {
    let mut spf = vec![0u32; limit + 1];
    let mut primes = Vec::with_capacity(limit / 10);
    for i in 2..=limit {
        if spf[i] == 0 {
            spf[i] = i as u32;
            primes.push(i as u64);
        }
        for &p in &primes {
            let v = i as u64 * p;
            if v > limit as u64 || p > spf[i] as u64 {
                break;
            }
            spf[v as usize] = p as u32;
        }
    }
    (spf, primes)
}

fn dists_for_prime(p: u64, m_max: u64, spf: &[u32], mu: &[i8]) -> Vec<(u64, Vec<(u64, u64)>)> {
    let tab = make_tab_u64(p);
    let pip = pisano_prime(p, spf, &tab);
    let mut out = Vec::new();
    let mut pe = p;
    let mut e = 1u32;
    loop {
        let dist = if e == 1 {
            period_dist_e1(p, pip, &tab, spf, mu)
        } else {
            period_dist_high(p, e, pe, pip, spf, mu)
        };
        out.push((pe, dist));
        if pe > m_max / p {
            break;
        }
        pe *= p;
        e += 1;
    }
    out
}

fn pisano_prime(p: u64, spf: &[u32], tab: &[(u64, u64); KMAX]) -> u64 {
    if p == 2 {
        return 3;
    }
    if p == 5 {
        return 20;
    }
    let p_minus = matches!(p % 5, 1 | 4);
    let mut n = if p_minus { p - 1 } else { p + 1 };
    let mut order = if p_minus { n } else { 2 * n };
    while n > 1 {
        // SAFETY: n = p±1 <= m_max+1 < spf.len()
        let q = unsafe { *spf.get_unchecked(n as usize) } as u64;
        while n % q == 0 {
            n /= q;
        }
        while order % q == 0 {
            let test = order / q;
            let (f0, f1) = fib_from_tab_u64(test, tab, p);
            if f0 == 0 && f1 == 1 {
                order = test;
            } else {
                break;
            }
        }
    }
    order
}

fn period_dist_e1(
    p: u64,
    pip: u64,
    tab: &[(u64, u64); KMAX],
    spf: &[u32],
    mu: &[i8],
) -> Vec<(u64, u64)> {
    let divs = divisors_of(pip, spf);
    let mut n_vals = Vec::with_capacity(divs.len());
    for &d in &divs {
        let (fd, fd1) = fib_from_tab_u64(d, tab, p);
        n_vals.push(kernel_size(fd, fd1, d, p, 1, p, p));
    }
    invert_exact(&divs, &n_vals, mu)
}

fn period_dist_high(p: u64, e: u32, pe: u64, pip: u64, spf: &[u32], mu: &[i8]) -> Vec<(u64, u64)> {
    let pi = pip * p.pow(e - 1);
    let divs = divisors_of(pi, spf);
    let m = pe * pe; // p^{2e} <= 1e12, enough to read v_p(det) up to 2e
    let mut n_vals = Vec::with_capacity(divs.len());
    if m <= 1_000_000 {
        let tab = make_tab_u64(m);
        for &d in &divs {
            let (fd, fd1) = fib_from_tab_u64(d, &tab, m);
            n_vals.push(kernel_size(fd, fd1, d, p, e, m, pe));
        }
    } else {
        let tab = make_tab_wide(m);
        for &d in &divs {
            let (fd, fd1) = fib_from_tab_wide(d, &tab, m);
            n_vals.push(kernel_size(fd, fd1, d, p, e, m, pe));
        }
    }
    invert_exact(&divs, &n_vals, mu)
}

/// |ker(F^d - I)| from (F_d, F_{d+1}) mod m, with m = p^e or p^{2e}.
#[inline]
fn kernel_size(fd: u64, fd1: u64, d: u64, p: u64, e: u32, m: u64, pe: u64) -> u64 {
    let fdm1 = (fd1 + m - fd) % m;
    let e00 = (fdm1 + m - 1) % m;
    let e11 = (fd1 + m - 1) % m;
    let v1 = val_p(e00, p, e)
        .min(val_p(fd, p, e))
        .min(val_p(e11, p, e));
    if v1 >= e {
        return pe * pe;
    }
    let trace = (fdm1 + fd1) % m;
    let det = if d & 1 == 0 {
        (2 + m - trace) % m
    } else {
        (m - trace) % m
    };
    let vdet = val_p(det, p, 2 * e);
    let v2 = if vdet >= v1 { (vdet - v1).min(e) } else { 0 };
    p.pow(v1) * p.pow(v2)
}

#[inline(always)]
fn val_p(mut x: u64, p: u64, cap: u32) -> u32 {
    if x == 0 {
        return cap;
    }
    let mut v = 0u32;
    while x % p == 0 && v < cap {
        x /= p;
        v += 1;
    }
    v
}

fn invert_exact(divs: &[u64], n_vals: &[u64], mu: &[i8]) -> Vec<(u64, u64)> {
    let mut result = Vec::new();
    for i in 0..divs.len() {
        let d = divs[i];
        let mut exact = 0i64;
        for j in 0..divs.len() {
            if d % divs[j] == 0 {
                exact += mu[(d / divs[j]) as usize] as i64 * n_vals[j] as i64;
            }
        }
        if exact > 0 {
            result.push((d, exact as u64));
        }
    }
    result
}

fn factorize(mut n: u64, spf: &[u32]) -> Vec<(u64, u32)> {
    let mut f = Vec::with_capacity(8);
    while n > 1 {
        let p = unsafe { *spf.get_unchecked(n as usize) } as u64;
        let mut e = 0u32;
        while n % p == 0 {
            n /= p;
            e += 1;
        }
        f.push((p, e));
    }
    f
}

fn divisors_of(n: u64, spf: &[u32]) -> Vec<u64> {
    let fac = factorize(n, spf);
    let mut d = vec![1u64];
    for &(p, e) in &fac {
        let len = d.len();
        let mut pe = 1u64;
        for _ in 0..e {
            pe *= p;
            for i in 0..len {
                d.push(d[i] * pe);
            }
        }
    }
    d.sort_unstable();
    d
}

/// tab[k] = (F_{2^k}, F_{2^k+1}) mod m. m <= 1e6 so products fit u64.
fn make_tab_u64(m: u64) -> [(u64, u64); KMAX] {
    let mut tab = [(0u64, 0u64); KMAX];
    tab[0] = (1, 1 % m);
    for k in 1..KMAX {
        let (a, b) = tab[k - 1];
        let two_b = (b << 1) % m;
        let t0 = a * ((two_b + m - a) % m) % m;
        let t1 = (a * a + b * b) % m;
        tab[k] = (t0, t1);
    }
    tab
}

#[inline]
fn fib_from_tab_u64(n: u64, tab: &[(u64, u64); KMAX], m: u64) -> (u64, u64) {
    if n == 0 {
        return (0, 1 % m);
    }
    let mut rest = n;
    let k0 = 63 - rest.leading_zeros();
    let (mut fx, mut fx1) = tab[k0 as usize];
    rest ^= 1u64 << k0;
    while rest != 0 {
        let k = 63 - rest.leading_zeros();
        let (fy, fy1) = tab[k as usize];
        let fxm1 = (fx1 + m - fx) % m;
        let fxy = (fx * fy1 + fxm1 * fy) % m;
        let fxy1 = (fx1 * fy1 + fx * fy) % m;
        fx = fxy;
        fx1 = fxy1;
        rest ^= 1u64 << k;
    }
    (fx, fx1)
}

#[inline(always)]
fn mul_wide(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128) * (b as u128) % (m as u128)) as u64
}

fn make_tab_wide(m: u64) -> [(u64, u64); KMAX] {
    let mut tab = [(0u64, 0u64); KMAX];
    tab[0] = (1, 1);
    for k in 1..KMAX {
        let (a, b) = tab[k - 1];
        let two_b = mul_wide(b, 2, m);
        let t0 = mul_wide(a, (two_b + m - a) % m, m);
        let t1 = (mul_wide(a, a, m) + mul_wide(b, b, m)) % m;
        tab[k] = (t0, t1);
    }
    tab
}

fn fib_from_tab_wide(n: u64, tab: &[(u64, u64); KMAX], m: u64) -> (u64, u64) {
    if n == 0 {
        return (0, 1);
    }
    let mut rest = n;
    let k0 = 63 - rest.leading_zeros();
    let (mut fx, mut fx1) = tab[k0 as usize];
    rest ^= 1u64 << k0;
    while rest != 0 {
        let k = 63 - rest.leading_zeros();
        let (fy, fy1) = tab[k as usize];
        let fxm1 = (fx1 + m - fx) % m;
        let fxy = (mul_wide(fx, fy1, m) + mul_wide(fxm1, fy, m)) % m;
        let fxy1 = (mul_wide(fx1, fy1, m) + mul_wide(fx, fy, m)) % m;
        fx = fxy;
        fx1 = fxy1;
        rest ^= 1u64 << k;
    }
    (fx, fx1)
}

#[inline(always)]
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

#[inline(always)]
fn lcm_u64(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        a / gcd(a, b) * b
    }
}

fn s_of_m(mut x: u64, spf: &[u32], cache: &[Vec<(u64, u64)>]) -> u64 {
    let mut pes = [0u64; 8];
    let mut n = 0usize;
    while x > 1 {
        // SAFETY: x starts at m <= m_max and strictly decreases; spf.len() > m_max
        let p = unsafe { *spf.get_unchecked(x as usize) } as u64;
        let mut pe = 1u64;
        while x % p == 0 {
            x /= p;
            pe *= p;
        }
        pes[n] = pe;
        n += 1;
    }
    s_from_pes(&pes[..n], cache)
}

fn s_from_pes(pes: &[u64], cache: &[Vec<(u64, u64)>]) -> u64 {
    match pes.len() {
        0 => 1,
        1 => s_from_dist(unsafe { cache.get_unchecked(pes[0] as usize) }),
        2 => s_two(
            unsafe { cache.get_unchecked(pes[0] as usize) },
            unsafe { cache.get_unchecked(pes[1] as usize) },
        ),
        _ => s_combine(pes, cache, 0, 1, 1),
    }
}

#[inline]
fn s_from_dist(dist: &[(u64, u64)]) -> u64 {
    let mut s = 0u64;
    for &(d, c) in dist {
        s = (s + (d % MODV) * (d % MODV) % MODV * (c % MODV)) % MODV;
    }
    s
}

#[inline]
fn s_two(a: &[(u64, u64)], b: &[(u64, u64)]) -> u64 {
    let mut s = 0u64;
    for &(d1, c1) in a {
        let c1m = c1 % MODV;
        for &(d2, c2) in b {
            let l = lcm_u64(d1, d2) % MODV;
            let cm = c1m * (c2 % MODV) % MODV;
            s = (s + l * l % MODV * cm) % MODV;
        }
    }
    s
}

fn s_combine(pes: &[u64], cache: &[Vec<(u64, u64)>], idx: usize, lcm_so_far: u64, cnt: u64) -> u64 {
    let dist = unsafe { cache.get_unchecked(pes[idx] as usize) };
    let last = idx + 1 == pes.len();
    let mut s = 0u64;
    for &(d, c) in dist {
        let l = lcm_u64(lcm_so_far, d);
        let cm = cnt * (c % MODV) % MODV;
        if last {
            let lm = l % MODV;
            s = (s + lm * lm % MODV * cm) % MODV;
        } else {
            s = (s + s_combine(pes, cache, idx + 1, l, cm)) % MODV;
        }
    }
    s
}
