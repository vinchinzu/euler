// Project Euler 678 - Fermat-like Equations
// Count (a,b,c,e,f) with a^e + b^e = c^f, a<b, e>=2, f>=3, c^f<=N.
// Uses Gaussian integers for sums of two squares, divisor enumeration
// for cubes, and direct enumeration for e>=5.
//
// Optimizations:
// - Precompute Gaussian factors for all primes p=1 mod 4 up to limit
// - i64 arithmetic in e3 check (avoid i128)
// - Fast is_sq using u64 arithmetic (avoid i128 multiply in square test)
// - d_max prefilter to skip large divisors early
// - Parallelize with rayon in chunks

use rayon::prelude::*;
use std::collections::HashSet;

const N_VAL: i64 = 1_000_000_000_000_000_000; // 10^18

#[inline]
fn isqrt_i64(n: i64) -> i64 {
    if n <= 0 { return 0; }
    let mut r = (n as f64).sqrt() as i64;
    if r < 0 { r = 0; }
    while r > 0 && (r as i128) * (r as i128) > n as i128 { r -= 1; }
    while ((r + 1) as i128) * ((r + 1) as i128) <= n as i128 { r += 1; }
    r
}

#[inline]
fn is_sq(n: i64) -> bool {
    if n < 0 { return false; }
    let r = isqrt_i64(n);
    (r as i128) * (r as i128) == n as i128
}

/// Fast perfect square test for values known to fit in ~10^13 range
/// (i.e., sqrt fits in u32). Uses pure u64 arithmetic, no i128.
#[inline]
fn is_sq_small(n: i64) -> bool {
    debug_assert!(n >= 0);
    let nu = n as u64;
    let r = (nu as f64).sqrt() as u64;
    if r * r == nu { return true; }
    let r1 = r + 1;
    if r1 * r1 == nu { return true; }
    if r > 0 && (r - 1) * (r - 1) == nu { return true; }
    false
}

#[derive(Clone, Copy)]
struct GI { re: i64, im: i64 }

fn find_gaussian_factor(p: i32) -> GI {
    let m = p as i64;
    let mut x = 2i64;
    loop {
        let mut r = 1i64;
        let mut base = x;
        let mut exp = (m - 1) / 2;
        while exp > 0 {
            if exp & 1 == 1 { r = r * base % m; }
            base = base * base % m;
            exp >>= 1;
        }
        if r == m - 1 { break; }
        x += 1;
    }

    let mut rr = 1i64;
    {
        let mut base = x;
        let mut exp = (m - 1) / 4;
        while exp > 0 {
            if exp & 1 == 1 { rr = rr * base % m; }
            base = base * base % m;
            exp >>= 1;
        }
    }

    let sqrt_p = isqrt_i64(m);
    let mut a = m;
    let mut b = rr;
    while b > sqrt_p {
        let tmp = b;
        b = a % b;
        a = tmp;
    }

    let re = b;
    let im = isqrt_i64(m - re * re);
    if re < im { GI { re, im } } else { GI { re: im, im: re } }
}

#[derive(Clone, Copy)]
struct PrimePower { p: i32, e: i32 }

fn factorize(mut n: i64, ff: &[u32]) -> Vec<PrimePower> {
    let mut factors = Vec::new();
    while n > 1 {
        let p = ff[n as usize] as i32;
        let mut e = 0;
        while n > 0 && n % p as i64 == 0 { n /= p as i64; e += 1; }
        factors.push(PrimePower { p, e });
    }
    factors
}

fn sums_of_two_squares(factors: &[PrimePower], f_mult: i32, gauss_cache: &[GI]) -> Vec<(i64, i64)> {
    for f in factors {
        if f.p % 4 == 3 && (f.e * f_mult) % 2 == 1 { return Vec::new(); }
    }

    let mut results: Vec<GI> = vec![GI { re: 1, im: 0 }];

    for fi in factors {
        let p = fi.p;
        let exp = (fi.e * f_mult) as usize;

        if p == 2 {
            for _ in 0..exp {
                for r in results.iter_mut() {
                    let re = r.re;
                    let im = r.im;
                    r.re = re - im;
                    r.im = re + im;
                }
            }
        } else if p % 4 == 1 {
            let gf = gauss_cache[p as usize];
            let a = gf.re;
            let b = gf.im;

            let mut pow_pos = vec![GI { re: 1, im: 0 }; exp + 1];
            let mut pow_neg = vec![GI { re: 1, im: 0 }; exp + 1];
            for i in 1..=exp {
                pow_pos[i].re = pow_pos[i-1].re * a - pow_pos[i-1].im * b;
                pow_pos[i].im = pow_pos[i-1].re * b + pow_pos[i-1].im * a;
                pow_neg[i].re = pow_neg[i-1].re * a + pow_neg[i-1].im * b;
                pow_neg[i].im = -pow_neg[i-1].re * b + pow_neg[i-1].im * a;
            }

            let mut new_results = Vec::with_capacity(results.len() * (exp + 1));
            for r in &results {
                let re = r.re;
                let im = r.im;
                for k in 0..=exp {
                    let fre = pow_pos[k].re * pow_neg[exp-k].re - pow_pos[k].im * pow_neg[exp-k].im;
                    let fim = pow_pos[k].re * pow_neg[exp-k].im + pow_pos[k].im * pow_neg[exp-k].re;
                    new_results.push(GI {
                        re: re * fre - im * fim,
                        im: re * fim + im * fre,
                    });
                }
            }
            results = new_results;
        } else if p % 4 == 3 {
            let mut scale = 1i64;
            for _ in 0..(exp / 2) { scale *= p as i64; }
            for r in results.iter_mut() {
                r.re *= scale;
                r.im *= scale;
            }
        }
    }

    let mut pairs_set: HashSet<(i64, i64)> = HashSet::new();
    for r in &results {
        let x = r.re.abs();
        let y = r.im.abs();
        let (lo, hi) = if x <= y { (x, y) } else { (y, x) };
        pairs_set.insert((lo, hi));
    }
    pairs_set.into_iter().collect()
}

fn get_divisors(factors: &[PrimePower], f_mult: i32) -> Vec<i64> {
    let mut divs = vec![1i64];
    for fi in factors {
        let p = fi.p as i64;
        let exp = (fi.e * f_mult) as usize;
        let old_n = divs.len();
        let mut pp = 1i64;
        for _ in 1..=exp {
            pp *= p;
            for i in 0..old_n {
                divs.push(divs[i] * pp);
            }
        }
    }
    divs
}

/// Process a single (c, f) pair and return the count of valid equations
fn process_cf(c: i64, f: i32, ff: &[u32], gauss_cache: &[GI], e_counts: &[Vec<(i64, i32)>]) -> i64 {
    // Compute c^f
    let mut cf = 1i128;
    for _ in 0..f {
        cf *= c as i128;
        if cf > N_VAL as i128 { return 0; }
    }
    if cf > N_VAL as i128 { return 0; }
    let cf = cf as i64;

    let factors = factorize(c, ff);
    let mut count = 0i64;

    // e = 2: sums of two squares
    let pairs = sums_of_two_squares(&factors, f, gauss_cache);
    for &(x, y) in &pairs {
        if x > 0 && x < y {
            count += 1;
        }
    }

    // e = 3: sums of two cubes using i64 arithmetic
    let divs = get_divisors(&factors, f);
    let cf4 = 4 * cf; // fits in i64 since cf <= 10^18 and 4*10^18 < i64::MAX
    // Upper bound for d: d < cbrt(4*cf)
    let d_max = (cf4 as f64).cbrt() as i64 + 2;
    for &d in &divs {
        if d <= 0 || d >= d_max { continue; }
        // d < d_max, so d^3 fits in i64
        let d3 = d * d * d;
        if d3 >= cf4 { continue; }
        if cf4 % d != 0 { continue; }
        let disc = cf4 / d - d * d;
        // disc < 3*d^2 ensures 3*disc < 9*d^2. Since d < 1.6*10^6, 9*d^2 < 2.3*10^13,
        // which fits in i64/u64 for is_sq_small.
        if disc > 0 && disc < 3 * d * d && is_sq_small(3 * disc) {
            count += 1;
        }
    }

    // e = 4: sums of two fourth powers
    for &(x, y) in &pairs {
        if x > 0 && x < y && is_sq(x) && is_sq(y) {
            count += 1;
        }
    }

    // e >= 5: lookup precomputed (binary search)
    for e in 5..64usize {
        if (1i64 << e) >= cf { break; }
        let ec = &e_counts[e];
        if let Ok(idx) = ec.binary_search_by_key(&cf, |&(k, _)| k) {
            count += ec[idx].1 as i64;
        }
    }

    count
}

fn main() {
    let limit = (N_VAL as f64).cbrt() as usize + 100;

    // Smallest prime factor sieve
    let mut ff = vec![0u32; limit + 1];
    for i in 0..=limit { ff[i] = i as u32; }
    for i in 2..=limit {
        if (i as u64) * (i as u64) > limit as u64 { break; }
        if ff[i] == i as u32 {
            for j in (i*i..=limit).step_by(i) {
                if ff[j] == j as u32 { ff[j] = i as u32; }
            }
        }
    }

    // Precompute Gaussian factors for all primes p = 1 mod 4 up to limit
    let mut gauss_cache = vec![GI { re: 0, im: 0 }; limit + 1];
    for p in 2..=limit {
        if ff[p] == p as u32 && p % 4 == 1 {
            gauss_cache[p] = find_gaussian_factor(p as i32);
        }
    }

    // Precompute sums a^e + b^e for e >= 5 using sorted Vec for binary search
    let mut e_counts: Vec<Vec<(i64, i32)>> = vec![Vec::new(); 64];
    for e in 5..64u32 {
        if (1i64 << e) >= N_VAL { break; }
        let mut pows = Vec::new();
        for a in 1.. {
            let mut ae = 1i128;
            let mut overflow = false;
            for _ in 0..e {
                ae *= a as i128;
                if ae >= N_VAL as i128 { overflow = true; break; }
            }
            if overflow || ae >= N_VAL as i128 { break; }
            pows.push(ae as i64);
        }
        let mut map: Vec<(i64, i32)> = Vec::new();
        for i in 0..pows.len() {
            for j in i+1..pows.len() {
                let cf = pows[i] as i128 + pows[j] as i128;
                if cf <= N_VAL as i128 {
                    map.push((cf as i64, 1));
                }
            }
        }
        map.sort_unstable_by_key(|&(k, _)| k);
        let mut merged: Vec<(i64, i32)> = Vec::new();
        for (k, v) in map {
            if let Some(last) = merged.last_mut() {
                if last.0 == k {
                    last.1 += v;
                    continue;
                }
            }
            merged.push((k, v));
        }
        e_counts[e as usize] = merged;
    }

    let mut ans = 0i64;

    // Process each f value
    for f in 3.. {
        if (1i64 << f) > N_VAL { break; }

        // Find max c for this f
        let mut c_max = 1i64;
        loop {
            let next = c_max + 1;
            let mut cf = 1i128;
            let mut overflow = false;
            for _ in 0..f {
                cf *= next as i128;
                if cf > N_VAL as i128 { overflow = true; break; }
            }
            if overflow || cf > N_VAL as i128 { break; }
            c_max = next;
        }

        if c_max < 2 { continue; }

        // Parallelize over c values in chunks for better load balancing
        let chunk_size = 1000i64;
        let n_chunks = ((c_max - 1) + chunk_size - 1) / chunk_size;
        let f_ans: i64 = (0..n_chunks).into_par_iter().map(|chunk_idx| {
            let c_lo = 2 + chunk_idx * chunk_size;
            let c_hi = std::cmp::min(c_lo + chunk_size - 1, c_max);
            let mut chunk_sum = 0i64;
            for c in c_lo..=c_hi {
                chunk_sum += process_cf(c, f, &ff, &gauss_cache, &e_counts);
            }
            chunk_sum
        }).sum();

        ans += f_ans;
    }

    println!("{}", ans);
}
