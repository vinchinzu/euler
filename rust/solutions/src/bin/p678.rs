// Project Euler 678 - Fermat-like Equations
// Count (a,b,c,e,f) with a^e + b^e = c^f, a<b, e>=2, f>=3, c^f<=N.
// Uses Gaussian integers for sums of two squares, divisor enumeration
// for cubes, and direct enumeration for e>=5.

use std::collections::{HashMap, HashSet};

const N_VAL: i64 = 1_000_000_000_000_000_000; // 10^18

fn isqrt_i64(n: i64) -> i64 {
    if n <= 0 { return 0; }
    let mut r = (n as f64).sqrt() as i64;
    if r < 0 { r = 0; }
    while r > 0 && r as i128 * r as i128 > n as i128 { r -= 1; }
    while (r + 1) as i128 * (r + 1) as i128 <= n as i128 { r += 1; }
    r
}

fn is_sq(n: i64) -> bool {
    if n < 0 { return false; }
    let r = isqrt_i64(n);
    r as i128 * r as i128 == n as i128
}

#[derive(Clone, Copy)]
struct GI { re: i64, im: i64 }

fn find_gaussian_factor(p: i32) -> GI {
    // Find quadratic non-residue
    let mut x = 2i64;
    loop {
        let mut r = 1i64;
        let mut base = x;
        let mut exp = (p as i64 - 1) / 2;
        let m = p as i64;
        while exp > 0 {
            if exp & 1 == 1 { r = (r as i128 * base as i128 % m as i128) as i64; }
            base = (base as i128 * base as i128 % m as i128) as i64;
            exp >>= 1;
        }
        if r == p as i64 - 1 { break; }
        x += 1;
    }

    // r = x^((p-1)/4) mod p
    let mut rr = 1i64;
    {
        let mut base = x;
        let mut exp = (p as i64 - 1) / 4;
        let m = p as i64;
        while exp > 0 {
            if exp & 1 == 1 { rr = (rr as i128 * base as i128 % m as i128) as i64; }
            base = (base as i128 * base as i128 % m as i128) as i64;
            exp >>= 1;
        }
    }

    // Euclidean algorithm
    let sqrt_p = isqrt_i64(p as i64);
    let mut m = p as i64;
    let mut n = rr;
    while n > sqrt_p {
        let tmp = n;
        n = m % n;
        m = tmp;
    }

    let a = n;
    let b = isqrt_i64(p as i64 - n * n);
    if a < b { GI { re: a, im: b } } else { GI { re: b, im: a } }
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

fn sums_of_two_squares(factors: &[PrimePower], f_mult: i32) -> Vec<(i64, i64)> {
    // Check feasibility: p=3 mod 4 must have even total exponent
    for f in factors {
        if f.p % 4 == 3 && (f.e * f_mult) % 2 == 1 { return Vec::new(); }
    }

    // Build using Gaussian integers
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
            let gf = find_gaussian_factor(p);
            let a = gf.re;
            let b = gf.im;

            // Powers of (a+bi) and (a-bi)
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

    // Deduplicate pairs
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

    // Precompute sums a^e + b^e for e >= 5
    let mut e_counts: Vec<HashMap<i64, i32>> = vec![HashMap::new(); 64];
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
        for i in 0..pows.len() {
            for j in i+1..pows.len() {
                let cf = pows[i] as i128 + pows[j] as i128;
                if cf <= N_VAL as i128 {
                    *e_counts[e as usize].entry(cf as i64).or_insert(0) += 1;
                }
            }
        }
    }

    let mut ans = 0i64;

    // Iterate over all c^f where f >= 3 and c^f <= N
    for f in 3.. {
        if (1i64 << f) > N_VAL { break; }
        for c in 2i64.. {
            // Compute c^f
            let mut cf = 1i128;
            let mut overflow = false;
            for _ in 0..f {
                cf *= c as i128;
                if cf > N_VAL as i128 { overflow = true; break; }
            }
            if overflow || cf > N_VAL as i128 { break; }
            let cf = cf as i64;

            // Get prime factorization of c
            let factors = factorize(c, &ff);

            // e = 2: sums of two squares
            let pairs = sums_of_two_squares(&factors, f);
            for &(x, y) in &pairs {
                if x > 0 && x < y {
                    ans += 1;
                }
            }

            // e = 3: sums of two cubes
            let divs = get_divisors(&factors, f);
            for &d in &divs {
                if d > 0 && (d as i128 * d as i128 * d as i128) < 4 * cf as i128 {
                    if (4 * cf as i128) % d as i128 != 0 { continue; }
                    let disc = (4 * cf as i128 / d as i128 - d as i128 * d as i128) as i64;
                    if disc > 0 && disc < 3 * d * d && is_sq(3 * disc) {
                        ans += 1;
                    }
                }
            }

            // e = 4: sums of two fourth powers
            // Reuse the sums_of_two_squares result
            for &(x, y) in &pairs {
                if x > 0 && x < y && is_sq(x) && is_sq(y) {
                    ans += 1;
                }
            }

            // e >= 5: lookup precomputed
            for e in 5..64 {
                if (1i64 << e) >= cf { break; }
                if let Some(&cnt) = e_counts[e].get(&cf) {
                    ans += cnt as i64;
                }
            }
        }
    }

    println!("{}", ans);
}
