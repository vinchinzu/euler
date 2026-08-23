// Project Euler 699 - Triffle Numbers
// Sum n <= N whose reduced σ(n)/n denominator is a positive power of 3.
//
// Seeds are 2^a 3^b 5^c (b >= 1). New primes p > 5 are attached only when
// p^e already divides the current numerator, so the reduced denominator stays
// {2,3,5}-smooth and never grows. Distinct 2-3-5 kernels make seeds independent.

use euler_utils::factor;
use fxhash::FxHashSet;
use rayon::prelude::*;

const N: u64 = 100_000_000_000_000;

#[inline(always)]
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

/// den is always {2,3,5}-smooth, so it is 3^k (k>0) iff it has no 2 or 5.
#[inline(always)]
fn is_pow3_den(den: u64) -> bool {
    den > 1 && (den & 1) != 0 && den % 5 != 0
}

fn dfs(n: u64, num: u64, den: u64, seen: &mut FxHashSet<u64>) -> u64 {
    if !seen.insert(n) {
        return 0;
    }
    let mut acc = if is_pow3_den(den) { n } else { 0 };
    if den % 3 != 0 || num <= 1 {
        return acc;
    }
    let facs = factor(num);
    for &(p, exp) in &facs {
        if p <= 5 || n % p == 0 {
            continue;
        }
        let mut pp = 1u64;
        let mut sig = 1u64;
        for _ in 0..exp {
            if pp > N / p {
                break;
            }
            pp *= p;
            sig = 1 + sig * p;
            if n > N / pp {
                break;
            }
            let mut new_num = (num / pp) * sig;
            let mut new_den = den;
            let g = gcd(new_num, new_den);
            new_num /= g;
            new_den /= g;
            acc += dfs(n * pp, new_num, new_den, seen);
        }
    }
    acc
}

fn seed_states() -> Vec<(u64, u64, u64)> {
    let mut seeds = Vec::with_capacity(4096);
    let mut p2 = 1u64;
    loop {
        let s2 = if p2 == 1 { 1 } else { p2 * 2 - 1 };
        let mut p3 = 3u64;
        loop {
            if p2 > N / p3 {
                break;
            }
            let base = p2 * p3;
            let s3 = (p3 * 3 - 1) / 2;
            let mut p5 = 1u64;
            loop {
                if p5 > N / base {
                    break;
                }
                let n = base * p5;
                let s5 = if p5 == 1 { 1 } else { (p5 * 5 - 1) / 4 };
                let sig = s2 * s3 * s5;
                let g = gcd(n, sig);
                let den = n / g;
                if den > 1 && den % 3 == 0 {
                    seeds.push((n, sig / g, den));
                }
                if p5 > N / 5 {
                    break;
                }
                p5 *= 5;
            }
            if p3 > N / 3 {
                break;
            }
            p3 *= 3;
        }
        if p2 > N / 2 {
            break;
        }
        p2 *= 2;
    }
    seeds
}

fn main() {
    let seeds = seed_states();
    let answer: u64 = seeds
        .into_par_iter()
        .map(|(n, num, den)| {
            let mut seen = FxHashSet::with_capacity_and_hasher(64, Default::default());
            dfs(n, num, den, &mut seen)
        })
        .sum();
    println!("{}", answer);
}
