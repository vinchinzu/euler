// Problem 780 — Torus Tilings
// Port of the Python G_fast algorithm.

const MOD: i64 = 1_000_000_007;

// ======================== exact sqrt(3) helpers ========================

/// Integer square root: floor(sqrt(n)) for n >= 0.
fn isqrt(n: u128) -> u128 {
    if n == 0 {
        return 0;
    }
    let mut x = (n as f64).sqrt() as u128;
    // Adjust for floating-point imprecision
    while x * x > n {
        x -= 1;
    }
    while (x + 1) * (x + 1) <= n {
        x += 1;
    }
    x
}

/// floor(sqrt(3) * n) exactly for integer n >= 0.
fn floor_sqrt3_mul(n: i64) -> i64 {
    debug_assert!(n >= 0);
    let nu = n as u128;
    isqrt(3 * nu * nu) as i64
}

/// floor(n / (m*sqrt(3))) exactly for n>=0, m>=1.
fn floor_div_sqrt3(n: i64, m: i64) -> i64 {
    debug_assert!(n >= 0 && m >= 1);
    let nn = (n as u128) * (n as u128);
    let den = 3u128 * (m as u128) * (m as u128);
    let mut t = isqrt(nn / den);
    while den * (t + 1) * (t + 1) <= nn {
        t += 1;
    }
    while den * t * t > nn {
        if t == 0 {
            break;
        }
        t -= 1;
    }
    t as i64
}

// ======================== divisor summatory D(n) ========================

/// D(n) = sum_{i=1..n} floor(n/i).
fn divisor_summatory(n: i64) -> i64 {
    if n <= 0 {
        return 0;
    }
    let mut res: i64 = 0;
    let mut i: i64 = 1;
    while i <= n {
        let q = n / i;
        let j = n / q;
        res += q * (j - i + 1);
        i = j + 1;
    }
    res
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
            let existing: Vec<(i64, i8)> = divs.clone();
            for (d, s) in existing {
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
        let mut out: Vec<i64> = Vec::new();
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
// Represent alpha as (a + b*sqrt(3)) / c with a: i128, b: i128 >= 0, c: i128 > 0.

type Alpha = (i128, i128, i128);

fn alpha_norm(a: i128, b: i128, c: i128) -> Alpha {
    let (mut a, mut b, mut c) = (a, b, c);
    if c < 0 {
        a = -a;
        b = -b;
        c = -c;
    }
    fn gcd128(x: i128, y: i128) -> i128 {
        let (mut a, mut b) = (x.unsigned_abs(), y.unsigned_abs());
        while b != 0 {
            let t = b;
            b = a % b;
            a = t;
        }
        a as i128
    }
    let g = gcd128(gcd128(a.abs(), b), c);
    if g > 1 {
        a /= g;
        b /= g;
        c /= g;
    }
    (a, b, c)
}

/// floor((a + b*sqrt(3))/c) with b>=0, c>0.
fn floor_qsqrt3(a: i128, b: i128, c: i128) -> i128 {
    if b == 0 {
        return a.div_euclid(c);
    }

    // Underestimate using floor(b*sqrt(3)).
    let fb = isqrt(3u128 * (b as u128) * (b as u128)) as i128;
    let mut x = (a + fb).div_euclid(c);

    let bb3: i128 = 3 * b * b;

    // Adjust upward if needed.
    loop {
        let y = (x + 1) * c - a;
        if y <= 0 || y * y <= bb3 {
            x += 1;
        } else {
            break;
        }
    }

    // Adjust downward if needed.
    loop {
        let y = x * c - a;
        if y <= 0 || y * y <= bb3 {
            break;
        }
        x -= 1;
    }

    x
}

fn alpha_floor(alpha: Alpha) -> i128 {
    let (a, b, c) = alpha;
    floor_qsqrt3(a, b, c)
}

fn alpha_mul_floor(alpha: Alpha, n: i128) -> i128 {
    let (a, b, c) = alpha;
    floor_qsqrt3(a * n, b * n, c)
}

fn alpha_sub_int(alpha: Alpha, k: i128) -> Alpha {
    let (a, b, c) = alpha;
    alpha_norm(a - k * c, b, c)
}

/// Return beta = alpha/(alpha-1), assuming 1 < alpha < 2.
fn alpha_div_alpha_minus1(alpha: Alpha) -> Alpha {
    let (a, b, c) = alpha;
    let ac = a - c;

    // (a+b√3)/(ac+b√3) = (a*ac - 3*b^2 - b*c√3) / (ac^2 - 3*b^2)
    // Wait, let's redo: multiply num and denom by conjugate (ac - b√3):
    // num = (a+b√3)(ac-b√3) = a*ac - 3*b^2 + b*√3*(ac - a) = a*ac - 3*b^2 + b*√3*(-c)
    // Hmm, let me just follow the Python exactly:
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

fn tri(n: i128) -> i128 {
    n * (n + 1) / 2
}

/// S(n, alpha) = sum_{k=1..n} floor(k * alpha), for alpha > 1 irrational.
fn beatty_sum(mut alpha: Alpha, mut n: i128) -> i128 {
    let mut res: i128 = 0;
    let mut sign: i128 = 1;

    while n > 0 {
        let f = alpha_floor(alpha);
        if f > 1 {
            res += sign * (f - 1) * tri(n);
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
fn beatty_sqrt3(c: i64, n: i64) -> i128 {
    if n <= 0 {
        return 0;
    }
    beatty_sum((0, c as i128, 1), n as i128)
}

// ======================== fast strip sum ========================

fn strip_hyperbola_sum(
    n: i64,
    v_max: i64,
    l: i64,
    sf_divs: &[Vec<(i64, i8)>],
    all_divs: &[Vec<i64>],
) -> (i128, i128) {
    let mut s1: i128 = 0;
    let mut s2: i128 = 0;

    for v in 1..=v_max {
        let umax = l / v;
        let dv = &all_divs[v as usize];

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
                cnt += muq as i64 * (hi / q - lo_minus / q);

                let hiq = hi / q;
                let loq = lo_minus / q;
                if hiq > 0 {
                    sm += muq as i128 * (beatty_sqrt3(v * q, hiq) - beatty_sqrt3(v * q, loq));
                }
            }

            s1 += w as i128 * cnt as i128;
            s2 += sm;
        }
    }

    // Apply symmetry
    let mut diag1: i128 = 0;
    let mut diag2: i128 = 0;
    for v in 1..=v_max {
        diag1 += (n / (2 * v)) as i128;
        diag2 += floor_sqrt3_mul(v) as i128;
    }

    let big_s1 = 2 * s1 - diag1;
    let big_s2 = 2 * s2 - diag2;
    (big_s1, big_s2)
}

// ======================== fast hex correction sum ========================

/// Count t in [lo, hi] with t % 3 == r.
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

fn hex_hsum(
    x: i64,
    sf_divs: &[Vec<(i64, i8)>],
) -> i128 {
    if x <= 0 {
        return 0;
    }

    // Use a simple HashMap for D cache since keys are unbounded (X // Q values)
    let mut d_cache = std::collections::HashMap::<i64, i64>::new();
    let d_fn = |n: i64, cache: &mut std::collections::HashMap<i64, i64>| -> i64 {
        if n <= 0 {
            return 0;
        }
        if let Some(&v) = cache.get(&n) {
            return v;
        }
        let v = divisor_summatory(n);
        cache.insert(n, v);
        v
    };

    let v_max = isqrt(x as u128) as i64;
    let mut extra: i128 = 0;

    for v in 1..=v_max {
        let disc0 = 4i128 * x as i128 - 3i128 * (v as i128) * (v as i128);
        if disc0 <= 0 {
            break;
        }
        let umax = (-(v as i128) + isqrt(disc0 as u128) as i128) / 2;

        let mut u = v as i128 + 1;
        let sfv = &sf_divs[v as usize];
        let vmod = v % 3;

        while u <= umax {
            let t = u * u + u * v as i128 + (v as i128) * (v as i128);
            let q = x as i128 / t;
            if q == 0 {
                break;
            }

            // Find max u' with same q
            let big_t = x as i128 / q;
            let disc = 4i128 * big_t - 3i128 * (v as i128) * (v as i128);
            let mut uhi = (-(v as i128) + isqrt(disc as u128) as i128) / 2;
            if uhi > umax {
                uhi = umax;
            }

            let lo1 = u - 1;

            // Count gcd(u,v)=1 in [u, uhi]
            let mut total: i64 = 0;
            for &(d, mud) in sfv {
                let d128 = d as i128;
                total += mud as i64 * ((uhi / d128) as i64 - (lo1 / d128) as i64);
            }

            // Exclude those with (2u+v) % 3 == 0, i.e., u == v (mod 3) when v%3 != 0
            if vmod != 0 {
                let mut bad: i64 = 0;
                let r = vmod as i64;
                for &(d, mud) in sfv {
                    let dm3 = (d % 3) as i64;
                    // d divides v, and v%3 != 0, so d%3 != 0 either.
                    let inv = if dm3 == 1 { 1i64 } else { 2i64 };
                    let tlo = (u as i64 + d as i64 - 1) / d as i64;
                    let thi = uhi as i64 / d as i64;
                    let rr = (r * inv) % 3;
                    bad += mud as i64 * count_mod3_res(tlo, thi, rr);
                }
                total -= bad;
            }

            if total != 0 {
                let dq = d_fn(q as i64, &mut d_cache);
                extra += total as i128 * dq as i128;
            }

            u = uhi + 1;
        }
    }

    let dx = d_fn(x, &mut d_cache) as i128;
    dx + 2 * extra
}

// ======================== G_fast ========================

fn g_fast(n: i64) -> i64 {
    if n <= 0 {
        return 0;
    }

    let m = n / 2;
    let l = floor_div_sqrt3(m, 1);
    let v_strip = isqrt(l as u128) as i64;

    let x = n / 4;
    let v_hex = if x > 0 { isqrt(x as u128) as i64 } else { 0 };

    let max_pre = v_strip.max(v_hex).max(1) as usize;
    let (spf, _mu) = sieve_mu_spf(max_pre);
    let sf_divs = build_squarefree_divs(max_pre, &spf);
    let all_divs = build_all_divisors(max_pre, &spf);

    let base = 2i128 * divisor_summatory(m) as i128;

    let (s1, s2) = strip_hyperbola_sum(n, v_strip, l, &sf_divs, &all_divs);
    let strip_part = base + 4 * (s1 - s2);

    // Hex correction: subtract 4 * H(N//4)
    let h = hex_hsum(x, &sf_divs);

    let total = strip_part - 4 * h;
    (total.rem_euclid(MOD as i128)) as i64
}

fn main() {
    println!("{}", g_fast(1_000_000_000));
}
