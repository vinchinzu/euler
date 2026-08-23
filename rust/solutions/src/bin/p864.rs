// Project Euler 864 - C(n) = count of squarefree x^2+1 for 1 <= x <= n
// Part A: DFS over d <= D with CRT solutions
// Part B: Pell equation for d > D

use rayon::prelude::*;

const N_VAL: i64 = 123_567_101_113;
const D_LIM: i64 = 100_000_000;
const SIEVE_LIM: usize = 100_000_001;
const MAX_SOLS: usize = 64;
const MU_LIM: usize = 400_000;
const PAR_D: i64 = 50_000;

#[inline(always)]
fn mod_inv_gen(mut a: i64, m: i64) -> i64 {
    let mut t = 0i64;
    let mut newt = 1i64;
    let mut r = m;
    a %= m;
    let mut newr = a;
    while newr != 0 {
        let q = r / newr;
        let tmp = newt;
        newt = t - q * newt;
        t = tmp;
        let tmp = newr;
        newr = r - q * newr;
        r = tmp;
    }
    if t < 0 { t += m; }
    t
}

#[inline(always)]
fn mul_mod(a: i64, b: i64, m: i64) -> i64 {
    debug_assert!(a >= 0 && b >= 0 && m > 0);
    let m = m as u64;
    if m <= 0xFFFF_FFFF {
        (a as u64 * b as u64 % m) as i64
    } else {
        ((a as u128 * b as u128) % m as u128) as i64
    }
}

#[inline(always)]
fn pow_mod_u32(base: u32, mut exp: u32, m: u32) -> u32 {
    let m64 = m as u64;
    let mut r = 1u64;
    let mut b = (base as u64) % m64;
    while exp > 0 {
        if exp & 1 == 1 {
            r = r * b % m64;
        }
        b = b * b % m64;
        exp >>= 1;
    }
    r as u32
}

/// sqrt(-1) mod p for p ≡ 1 (mod 4), Hensel-lifted to mod p^2.
fn mod_sqrt_neg1_p2(p: i32) -> i64 {
    let pu = p as u32;
    let exp = (pu - 1) / 4;
    let r0 = if pu & 7 == 5 {
        pow_mod_u32(2, exp, pu)
    } else {
        let mut z = 3u32;
        loop {
            let t = pow_mod_u32(z, exp, pu);
            if t as u64 * t as u64 % pu as u64 == (pu - 1) as u64 {
                break t;
            }
            z += 2;
        }
    };

    // Lift x0^2 ≡ -1 (mod p) to mod p^2 via Hensel (all arithmetic < p^2 ≤ 1e16).
    let pl = pu as u64;
    let r0u = r0 as u64;
    let t = (r0u * r0u + 1) / pl;
    let inv2x = mod_inv_gen(((2 * r0u) % pl) as i64, pl as i64) as u64;
    let k = (pl - t * inv2x % pl) % pl;
    (r0u + k * pl) as i64
}

#[inline(always)]
fn count_solutions(n: i64, m_sq: i64, sols: &[i64]) -> i64 {
    let mut total = 0i64;
    for &a in sols {
        if a == 0 {
            total += n / m_sq;
        } else if a <= n {
            total += (n - a) / m_sq + 1;
        }
    }
    total
}

#[inline(always)]
fn fill_sols(
    out: &mut [i64; MAX_SOLS],
    sols: &[i64],
    d_sq: i64,
    r0: i64,
    r1: i64,
    p2: i64,
) -> usize {
    let inv = mod_inv_gen(d_sq % p2, p2);
    let mut n = 0usize;
    for &s in sols {
        let s_mod = s % p2;
        let diff0 = if r0 >= s_mod { r0 - s_mod } else { r0 + p2 - s_mod };
        let kv0 = mul_mod(diff0, inv, p2);
        out[n] = s + d_sq * kv0;
        n += 1;
        let diff1 = if r1 >= s_mod { r1 - s_mod } else { r1 + p2 - s_mod };
        let kv1 = mul_mod(diff1, inv, p2);
        out[n] = s + d_sq * kv1;
        n += 1;
    }
    n
}

fn part_a_children(
    idx: usize,
    d: i64,
    sols: &[i64],
    mu: i64,
    primes: &[i32],
    roots: &[i64],
) -> i64 {
    if idx >= primes.len() {
        return 0;
    }
    let max_p = D_LIM / d;
    let end = idx
        + primes[idx..].partition_point(|&p| (p as i64) <= max_p);
    part_a_range(idx, end, d, sols, mu, primes, roots)
}

fn part_a_range(
    lo: usize,
    hi: usize,
    d: i64,
    sols: &[i64],
    mu: i64,
    primes: &[i32],
    roots: &[i64],
) -> i64 {
    if lo >= hi {
        return 0;
    }
    if d < PAR_D && hi - lo > 8 {
        let mid = (lo + hi) / 2;
        let (a, b) = rayon::join(
            || part_a_range(lo, mid, d, sols, mu, primes, roots),
            || part_a_range(mid, hi, d, sols, mu, primes, roots),
        );
        return a + b;
    }

    let d_sq = d * d;
    let mut result = 0i64;
    for i in lo..hi {
        // SAFETY: lo..hi is a subrange of primes; roots is 1-1 with primes.
        let p = unsafe { *primes.get_unchecked(i) } as i64;
        let new_d = d * p;
        let p2 = p * p;
        let r0 = unsafe { *roots.get_unchecked(i) };
        let r1 = p2 - r0;

        let mut new_sols = [0i64; MAX_SOLS];
        let nsols = fill_sols(&mut new_sols, sols, d_sq, r0, r1, p2);
        let child_mu = -mu;
        let cnt = count_solutions(N_VAL, new_d * new_d, &new_sols[..nsols]);
        result += child_mu * cnt
            + part_a_children(i + 1, new_d, &new_sols[..nsols], child_mu, primes, roots);
    }
    result
}

fn part_a_work(primes: &[i32], roots: &[i64]) -> i64 {
    let split = primes.partition_point(|&p| (p as i64) * (p as i64) <= D_LIM);
    let (heavy, light) = rayon::join(
        || {
            (0..split)
                .into_par_iter()
                .map(|i| {
                    let p = primes[i] as i64;
                    let r0 = roots[i];
                    let r1 = p * p - r0;
                    let sols = [r0, r1];
                    -count_solutions(N_VAL, p * p, &sols)
                        + part_a_children(i + 1, p, &sols, -1, primes, roots)
                })
                .sum::<i64>()
        },
        || {
            (split..primes.len())
                .into_par_iter()
                .with_min_len(32768)
                .map(|i| {
                    let p = primes[i] as i64;
                    let r0 = roots[i];
                    let r1 = p * p - r0;
                    -count_solutions(N_VAL, p * p, &[r0, r1])
                })
                .sum::<i64>()
        },
    );
    N_VAL + heavy + light
}

fn sieve_mu(limit: usize) -> Vec<i8> {
    let mut mu = vec![0i8; limit + 1];
    let mut vis = vec![false; limit + 1];
    let mut primes: Vec<usize> = Vec::with_capacity(limit / 10);
    mu[1] = 1;
    for i in 2..=limit {
        if !vis[i] {
            primes.push(i);
            mu[i] = -1;
        }
        for &p in &primes {
            let ip = i * p;
            if ip > limit {
                break;
            }
            vis[ip] = true;
            if i % p == 0 {
                mu[ip] = 0;
                break;
            }
            mu[ip] = -mu[i];
        }
    }
    mu
}

#[inline(always)]
fn mul_mod_u64(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}

#[inline(always)]
fn pow_mod_u64(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut r = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            r = mul_mod_u64(r, base, m);
        }
        base = mul_mod_u64(base, base, m);
        exp >>= 1;
    }
    r
}

/// Deterministic Miller-Rabin for n < 341_550_071_728_321.
#[inline]
fn miller_rabin_u64(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    const SMALL: [u64; 8] = [2, 3, 5, 7, 11, 13, 17, 19];
    for &p in &SMALL {
        if n % p == 0 {
            return n == p;
        }
    }
    let mut d = n - 1;
    let s = d.trailing_zeros();
    d >>= s;
    const WIT: [u64; 7] = [2, 3, 5, 7, 11, 13, 17];
    'a: for &a in &WIT {
        if a >= n {
            return true;
        }
        let mut x = pow_mod_u64(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 1..s {
            x = mul_mod_u64(x, x, n);
            if x == n - 1 {
                continue 'a;
            }
        }
        return false;
    }
    true
}

fn mu_squarefree(n: i64, mu_small: &[i8], primes: &[i32]) -> Option<i32> {
    let lim = mu_small.len() as i64;
    if n < lim {
        let m = unsafe { *mu_small.get_unchecked(n as usize) };
        return if m == 0 { None } else { Some(m as i32) };
    }
    if miller_rabin_u64(n as u64) {
        return Some(-1);
    }
    let mut mu = 1i32;
    let mut temp = n;
    for &p in primes {
        let pl = p as i64;
        if pl * pl > temp {
            break;
        }
        if temp % pl == 0 {
            temp /= pl;
            if temp % pl == 0 {
                return None;
            }
            mu = -mu;
            if temp < lim {
                let m = unsafe { *mu_small.get_unchecked(temp as usize) };
                if m == 0 {
                    return None;
                }
                return Some(mu * m as i32);
            }
            if miller_rabin_u64(temp as u64) {
                return Some(-mu);
            }
        }
    }
    if temp > 1 {
        mu = -mu;
    }
    Some(mu)
}

#[inline]
fn isqrt(n: i64) -> i64 {
    let mut s = (n as f64).sqrt() as i64;
    while s * s > n {
        s -= 1;
    }
    while (s + 1) * (s + 1) <= n {
        s += 1;
    }
    s
}

fn solve_pell_and_count(k: i64, mu_small: &[i8], primes_mu: &[i32]) -> i64 {
    let a0 = isqrt(k);
    if a0 * a0 == k {
        return 0;
    }

    let (mut m, mut d, mut a) = (0i64, 1i64, a0);
    let (mut num2, mut num1) = (0i64, 1i64);
    let (mut den2, mut den1) = (1i64, 0i64);
    let mut fund_x = 0i64;
    let mut fund_y = 0i64;

    for iter in 0..10000 {
        let num = a * num1 + num2;
        let den = a * den1 + den2;
        if num > N_VAL || num < 0 {
            return 0;
        }

        let n2 = num as i128 * num as i128;
        let kd2 = k as i128 * den as i128 * den as i128;
        let val = n2 - kd2;

        if val == -1 {
            fund_x = num;
            fund_y = den;
            break;
        }
        if val == 1 && iter > 0 {
            return 0;
        }

        num2 = num1;
        num1 = num;
        den2 = den1;
        den1 = den;
        m = d * a - m;
        d = (k - m * m) / d;
        if d == 0 {
            return 0;
        }
        a = (a0 + m) / d;
    }

    if fund_x == 0 {
        return 0;
    }

    let mul_x: i128 = 2 * fund_x as i128 * fund_x as i128 + 1;
    let mul_y: i128 = 2 * fund_x as i128 * fund_y as i128;
    let k128 = k as i128;

    let mut result = 0i64;
    let mut cx = fund_x as i128;
    let mut cy = fund_y as i128;
    while cx <= N_VAL as i128 {
        let y = cy as i64;
        if y > D_LIM {
            if let Some(mu) = mu_squarefree(y, mu_small, primes_mu) {
                result += mu as i64;
            }
        }
        let nx = cx * mul_x + cy * mul_y * k128;
        if nx > N_VAL as i128 {
            break;
        }
        let ny = cx * mul_y + cy * mul_x;
        cx = nx;
        cy = ny;
    }
    result
}

fn collect_ks(primes: &[i32], k_limit: i64) -> Vec<i64> {
    let end = primes.partition_point(|&p| p as i64 <= k_limit);
    let primes = &primes[..end];
    let mut ks = Vec::with_capacity(500_000);
    fn rec(idx: usize, k: i64, k_limit: i64, primes: &[i32], ks: &mut Vec<i64>) {
        ks.push(k);
        for i in idx..primes.len() {
            let p = unsafe { *primes.get_unchecked(i) } as i64;
            if k > k_limit / p {
                break;
            }
            rec(i, k * p, k_limit, primes, ks);
        }
    }
    rec(0, 1, k_limit, primes, &mut ks);
    rec(0, 2, k_limit, primes, &mut ks);
    ks
}

fn part_b_work(primes_1mod4: &[i32], mu_small: &[i8], primes_mu: &[i32], k_limit: i64) -> i64 {
    let ks = collect_ks(primes_1mod4, k_limit);
    ks.par_iter()
        .map(|&k| solve_pell_and_count(k, mu_small, primes_mu))
        .sum()
}

fn main() {
    let half = SIEVE_LIM / 2 + 1;
    let mut sieve_bits = vec![0u8; half];
    sieve_bits[0] = 1;

    let lim = (SIEVE_LIM as f64).sqrt() as usize;
    for i in 1..half {
        let p = 2 * i + 1;
        if p > lim {
            break;
        }
        if sieve_bits[i] == 0 {
            let mut j = p * p;
            while j <= SIEVE_LIM {
                sieve_bits[(j - 1) / 2] = 1;
                j += 2 * p;
            }
        }
    }

    let mut primes_1mod4: Vec<i32> = Vec::with_capacity(3_000_000);
    for i in 1..half {
        if sieve_bits[i] == 0 {
            let p = (2 * i + 1) as i32;
            if p % 4 == 1 {
                primes_1mod4.push(p);
            }
        }
    }
    drop(sieve_bits);

    let n128: i128 = N_VAL as i128;
    let k_limit = ((n128 * n128 + 1) / (D_LIM as i128 * D_LIM as i128)) as i64 + 1;

    let mu_small = sieve_mu(MU_LIM);
    let pmu_lim = isqrt(N_VAL) as i32 + 1;
    let primes_mu: Vec<i32> = primes_1mod4
        .iter()
        .copied()
        .take_while(|&p| p <= pmu_lim)
        .collect();

    let (part_a_result, part_b_result) = rayon::join(
        || {
            let roots: Vec<i64> = primes_1mod4
                .par_iter()
                .map(|&p| mod_sqrt_neg1_p2(p))
                .collect();
            part_a_work(&primes_1mod4, &roots)
        },
        || part_b_work(&primes_1mod4, &mu_small, &primes_mu, k_limit),
    );

    let total = part_a_result + part_b_result;
    println!("{}", total);
}
