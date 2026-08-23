// Problem 780 — Torus Tilings
// Port of the Python G_fast algorithm.
//
// Hot path is Beatty sums over independent v, plus H(N/4) divisor lookups.
// Alpha coefficients stay i64 (products fit); accumulators use i128 / MOD.

use rayon::prelude::*;

const MOD: i64 = 1_000_000_007;

// ======================== exact sqrt(3) helpers ========================

#[inline(always)]
fn isqrt_u128(n: u128) -> u128 {
    n.isqrt()
}

/// floor(sqrt(3) * n) exactly for n >= 0.
#[inline(always)]
fn floor_sqrt3_mul(n: i64) -> i64 {
    debug_assert!(n >= 0);
    isqrt_u128(3u128 * (n as u128) * (n as u128)) as i64
}

/// floor(n / (m*sqrt(3))) exactly for n>=0, m>=1.
#[inline]
fn floor_div_sqrt3(n: i64, m: i64) -> i64 {
    debug_assert!(n >= 0 && m >= 1);
    let nn = (n as u128) * (n as u128);
    let den = 3u128 * (m as u128) * (m as u128);
    isqrt_u128(nn / den) as i64
}

// ======================== divisor summatory D(n) ========================

/// D(n) = sum_{i=1..n} floor(n/i) = 2 * sum_{i=1..s} floor(n/i) - s^2.
#[inline]
fn divisor_summatory(n: i64) -> i64 {
    if n <= 0 {
        return 0;
    }
    let s = (n as u64).isqrt() as i64;
    let mut sum = 0i64;
    let mut i = 1i64;
    while i <= s {
        sum += n / i;
        i += 1;
    }
    2 * sum - s * s
}

// ======================== sieve: spf + mu ========================

fn sieve_mu_spf(n: usize) -> (Vec<u32>, Vec<i8>) {
    let mut spf = vec![0u32; n + 1];
    let mut mu = vec![0i8; n + 1];
    let mut primes: Vec<u32> = Vec::new();
    if n >= 1 {
        mu[1] = 1;
    }

    for i in 2..=n {
        if spf[i] == 0 {
            spf[i] = i as u32;
            primes.push(i as u32);
            mu[i] = -1;
        }
        for &p in &primes {
            let v = i * p as usize;
            if v > n {
                break;
            }
            spf[v] = p;
            if i % p as usize == 0 {
                mu[v] = 0;
                break;
            }
            mu[v] = -mu[i];
        }
    }
    if n >= 1 && spf[1] == 0 {
        spf[1] = 1;
    }
    (spf, mu)
}

/// Distinct prime factors of x using spf.
fn factor_distinct(mut x: usize, spf: &[u32]) -> Vec<u32> {
    let mut ps = Vec::new();
    while x > 1 {
        let p = spf[x];
        ps.push(p);
        while x > 1 && spf[x] == p {
            x /= p as usize;
        }
    }
    ps
}

/// For each n <= max_n, build list of (d, mu(d)) for squarefree d|n.
fn build_squarefree_divs(max_n: usize, spf: &[u32]) -> Vec<Vec<(i64, i8)>> {
    let mut sf: Vec<Vec<(i64, i8)>> = vec![vec![]; max_n + 1];
    if max_n >= 1 {
        sf[1] = vec![(1, 1)];
    }
    for n in 2..=max_n {
        let ps = factor_distinct(n, spf);
        let mut divs: Vec<(i64, i8)> = vec![(1, 1)];
        for &p in &ps {
            let len = divs.len();
            for i in 0..len {
                let (d, s) = divs[i];
                divs.push((d * p as i64, -s));
            }
        }
        sf[n] = divs;
    }
    sf
}

/// For each n <= max_n, build full divisor list.
fn build_all_divisors(max_n: usize, spf: &[u32]) -> Vec<Vec<i64>> {
    let mut divs: Vec<Vec<i64>> = vec![vec![]; max_n + 1];
    if max_n >= 1 {
        divs[1] = vec![1];
    }
    for n in 2..=max_n {
        let mut x = n;
        let p = spf[x] as usize;
        let mut e = 0;
        while x % p == 0 {
            x /= p;
            e += 1;
        }
        let base = &divs[x];
        let mut out: Vec<i64> = Vec::with_capacity(base.len() * (e + 1));
        let mut pe: i64 = 1;
        for _ in 0..=e {
            for &d in base {
                out.push(d * pe);
            }
            pe *= p as i64;
        }
        divs[n] = out;
    }
    divs
}

// ======================== Beatty sum in Q(sqrt(3)) ========================
// Represent alpha as (a + b*sqrt(3)) / c with a: i64, b: i64 >= 0, c: i64 > 0.
// Intermediate products use i128; after gcd they fit in i64 for this problem.

type Alpha = (i64, i64, i64);

#[inline(always)]
fn gcd64(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn alpha_norm(a: i128, b: i128, c: i128) -> Alpha {
    let (mut a, mut b, mut c) = (a, b, c);
    if c < 0 {
        a = -a;
        b = -b;
        c = -c;
    }
    let ga = a.unsigned_abs();
    let gb = b.unsigned_abs();
    let gc = c.unsigned_abs();
    // Coefficients stay well below 2^64 after the sign fix.
    let g = gcd64(gcd64(ga as u64, gb as u64), gc as u64) as i128;
    if g > 1 {
        a /= g;
        b /= g;
        c /= g;
    }
    (a as i64, b as i64, c as i64)
}

/// floor((a + b*sqrt(3))/c) with b>=0, c>0.
#[inline]
fn floor_qsqrt3(a: i128, b: i128, c: i128) -> i128 {
    if b == 0 {
        return a.div_euclid(c);
    }

    let bb3: u128 = 3u128 * (b as u128) * (b as u128);
    let fb = isqrt_u128(bb3) as i128;
    let mut x = (a + fb).div_euclid(c);

    loop {
        let y = (x + 1) * c - a;
        if y <= 0 || (y as u128) * (y as u128) <= bb3 {
            x += 1;
        } else {
            break;
        }
    }

    loop {
        let y = x * c - a;
        if y <= 0 || (y as u128) * (y as u128) <= bb3 {
            break;
        }
        x -= 1;
    }

    x
}

#[inline]
fn alpha_floor(alpha: Alpha) -> i64 {
    let (a, b, c) = alpha;
    floor_qsqrt3(a as i128, b as i128, c as i128) as i64
}

#[inline]
fn alpha_mul_floor(alpha: Alpha, n: i64) -> i64 {
    let (a, b, c) = alpha;
    let n = n as i128;
    floor_qsqrt3(a as i128 * n, b as i128 * n, c as i128) as i64
}

#[inline]
fn alpha_sub_int(alpha: Alpha, k: i64) -> Alpha {
    let (a, b, c) = alpha;
    alpha_norm(a as i128 - k as i128 * c as i128, b as i128, c as i128)
}

/// Return beta = alpha/(alpha-1), assuming 1 < alpha < 2.
fn alpha_div_alpha_minus1(alpha: Alpha) -> Alpha {
    let (a, b, c) = alpha;
    let a = a as i128;
    let b = b as i128;
    let c = c as i128;
    let ac = a - c;

    let big_a = a * ac - 3 * b * b;
    let big_b = -b * c;
    let big_d = ac * ac - 3 * b * b;

    let (mut ra, mut rb, mut rd) = (big_a, big_b, big_d);
    if rd < 0 {
        ra = -ra;
        rb = -rb;
        rd = -rd;
    }
    if rb < 0 {
        ra = -ra;
        rb = -rb;
    }

    alpha_norm(ra, rb, rd)
}

#[inline(always)]
fn tri(n: i64) -> i128 {
    (n as i128) * (n as i128 + 1) / 2
}

/// S(n, alpha) = sum_{k=1..n} floor(k * alpha), for alpha > 1 irrational.
fn beatty_sum(mut alpha: Alpha, mut n: i64) -> i128 {
    let mut res: i128 = 0;
    let mut sign: i128 = 1;

    while n > 0 {
        let f = alpha_floor(alpha);
        if f > 1 {
            res += sign * (f as i128 - 1) * tri(n);
            alpha = alpha_sub_int(alpha, f - 1);
        }

        let m = alpha_mul_floor(alpha, n);
        res += sign * tri(m);

        n = m - n;
        if n <= 0 {
            break;
        }

        alpha = alpha_div_alpha_minus1(alpha);
        sign = -sign;
    }

    res
}

/// sum_{k=1..n} floor(k * c * sqrt(3)).
#[inline]
fn beatty_sqrt3(c: i64, n: i64) -> i128 {
    if n <= 0 {
        return 0;
    }
    beatty_sum((0, c, 1), n)
}

// ======================== fast strip sum ========================

fn strip_one_v(
    n: i64,
    l: i64,
    v: i64,
    sf_divs: &[Vec<(i64, i8)>],
    all_divs: &[Vec<i64>],
) -> (i128, i128) {
    let umax = l / v;
    let dv = &all_divs[v as usize];
    let mut s1: i128 = 0;
    let mut s2: i128 = 0;

    for &d in dv {
        let m = v / d;
        let hi = umax / d;
        if hi < m {
            continue;
        }

        let w = n / (2 * d);
        let lo_minus = m - 1;

        let mut cnt: i64 = 0;
        let mut sm: i128 = 0;

        let sf = &sf_divs[m as usize];
        for &(q, muq) in sf {
            let hiq = hi / q;
            let loq = lo_minus / q;
            cnt += muq as i64 * (hiq - loq);

            if hiq > loq {
                sm += muq as i128 * (beatty_sqrt3(v * q, hiq) - beatty_sqrt3(v * q, loq));
            }
        }

        s1 += w as i128 * cnt as i128;
        s2 += sm;
    }

    let diag1 = (n / (2 * v)) as i128;
    let diag2 = floor_sqrt3_mul(v) as i128;
    (2 * s1 - diag1, 2 * s2 - diag2)
}

fn strip_hyperbola_sum(
    n: i64,
    v_max: i64,
    l: i64,
    sf_divs: &[Vec<(i64, i8)>],
    all_divs: &[Vec<i64>],
) -> (i128, i128) {
    if v_max <= 0 {
        return (0, 0);
    }
    let nthreads = rayon::current_num_threads().max(1) as i64;
    (0..nthreads)
        .into_par_iter()
        .map(|tid| {
            let mut s1: i128 = 0;
            let mut s2: i128 = 0;
            let mut v = tid + 1;
            while v <= v_max {
                let (a, b) = strip_one_v(n, l, v, sf_divs, all_divs);
                s1 += a;
                s2 += b;
                v += nthreads;
            }
            (s1, s2)
        })
        .reduce(|| (0i128, 0i128), |a, b| (a.0 + b.0, a.1 + b.1))
}

// ======================== fast hex correction sum ========================

/// Count t in [lo, hi] with t % 3 == r.
#[inline(always)]
fn count_mod3_res(lo: i64, hi: i64, r: i64) -> i64 {
    if hi < lo {
        return 0;
    }
    let rem = lo.rem_euclid(3);
    let delta = (r - rem).rem_euclid(3);
    let first = lo + delta;
    if first > hi {
        return 0;
    }
    (hi - first) / 3 + 1
}

/// Precompute D(k) for k <= sqrt(x) and D(x/k) for k <= sqrt(x).
fn precompute_d_tables(x: i64) -> (i64, Vec<i64>, Vec<i64>) {
    let s = (x as u64).isqrt() as i64;
    let su = s as usize;
    let mut tau = vec![0i32; su + 1];
    for i in 1..=su {
        let mut j = i;
        while j <= su {
            tau[j] += 1;
            j += i;
        }
    }
    let mut small = vec![0i64; su + 1];
    for i in 1..=su {
        small[i] = small[i - 1] + tau[i] as i64;
    }

    let mut large = vec![0i64; su + 1];
    // Independent hyperbola evaluations; each is O(sqrt(x/k)).
    large
        .par_iter_mut()
        .enumerate()
        .skip(1)
        .for_each(|(k, slot)| {
            *slot = divisor_summatory(x / k as i64);
        });

    (s, small, large)
}

#[inline(always)]
fn d_get(n: i64, x: i64, s: i64, small: &[i64], large: &[i64]) -> i64 {
    if n <= 0 {
        return 0;
    }
    if n <= s {
        small[n as usize]
    } else {
        large[(x / n) as usize]
    }
}

fn hex_one_v(
    x: i64,
    v: i64,
    s: i64,
    small: &[i64],
    large: &[i64],
    sf_divs: &[Vec<(i64, i8)>],
) -> i128 {
    let vv = v * v;
    let disc0 = 4 * x - 3 * vv;
    if disc0 <= 0 {
        return 0;
    }
    let umax = (-v + (disc0 as u64).isqrt() as i64) / 2;

    let mut extra: i128 = 0;
    let mut u = v + 1;
    let sfv = &sf_divs[v as usize];
    let vmod = v % 3;

    while u <= umax {
        let t = u * u + u * v + vv;
        let q = x / t;
        if q == 0 {
            break;
        }

        let big_t = x / q;
        let disc = 4 * big_t - 3 * vv;
        let mut uhi = if disc <= 0 {
            umax
        } else {
            (-v + (disc as u64).isqrt() as i64) / 2
        };
        if uhi > umax {
            uhi = umax;
        }

        let lo1 = u - 1;

        let mut total: i64 = 0;
        for &(d, mud) in sfv {
            total += mud as i64 * (uhi / d - lo1 / d);
        }

        if vmod != 0 {
            let mut bad: i64 = 0;
            let r = vmod;
            for &(d, mud) in sfv {
                let dm3 = d % 3;
                let inv = if dm3 == 1 { 1i64 } else { 2i64 };
                let tlo = (u + d - 1) / d;
                let thi = uhi / d;
                let rr = (r * inv) % 3;
                bad += mud as i64 * count_mod3_res(tlo, thi, rr);
            }
            total -= bad;
        }

        if total != 0 {
            extra += total as i128 * d_get(q, x, s, small, large) as i128;
        }

        u = uhi + 1;
    }

    extra
}

fn hex_hsum(x: i64, sf_divs: &[Vec<(i64, i8)>]) -> i128 {
    if x <= 0 {
        return 0;
    }

    let (s, small, large) = precompute_d_tables(x);
    let v_max = s;
    if v_max <= 0 {
        return d_get(x, x, s, &small, &large) as i128;
    }

    let nthreads = rayon::current_num_threads().max(1) as i64;
    let extra: i128 = (0..nthreads)
        .into_par_iter()
        .map(|tid| {
            let mut extra = 0i128;
            let mut v = tid + 1;
            while v <= v_max {
                extra += hex_one_v(x, v, s, &small, &large, sf_divs);
                v += nthreads;
            }
            extra
        })
        .sum();

    let dx = d_get(x, x, s, &small, &large) as i128;
    dx + 2 * extra
}

// ======================== G_fast ========================

fn g_fast(n: i64) -> i64 {
    if n <= 0 {
        return 0;
    }

    let m = n / 2;
    let l = floor_div_sqrt3(m, 1);
    let v_strip = (l as u64).isqrt() as i64;

    let x = n / 4;
    let v_hex = if x > 0 { (x as u64).isqrt() as i64 } else { 0 };

    let max_pre = v_strip.max(v_hex).max(1) as usize;
    let (spf, _mu) = sieve_mu_spf(max_pre);
    let sf_divs = build_squarefree_divs(max_pre, &spf);
    let all_divs = build_all_divisors(max_pre, &spf);

    let ((s1, s2), (base, h)) = rayon::join(
        || strip_hyperbola_sum(n, v_strip, l, &sf_divs, &all_divs),
        || {
            let base = 2i128 * divisor_summatory(m) as i128;
            let h = hex_hsum(x, &sf_divs);
            (base, h)
        },
    );

    let strip_part = base + 4 * (s1 - s2);
    let total = strip_part - 4 * h;
    total.rem_euclid(MOD as i128) as i64
}

fn main() {
    println!("{}", g_fast(1_000_000_000));
}
