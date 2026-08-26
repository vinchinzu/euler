// Project Euler 916 - Permutation Subsequences
// P(n) = C_n^2 * (1 + (3n/(n+2))^2) mod 10^9+7
// where C_n is the nth Catalan number.

use rayon::prelude::*;

const MOD: u64 = 1_000_000_007;

#[inline(always)]
fn mulm(a: u64, b: u64) -> u64 {
    a * b % MOD
}

fn power(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut res = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            res = res * base % m;
        }
        base = base * base % m;
        exp >>= 1;
    }
    res
}

fn inverse(a: u64, m: u64) -> u64 {
    power(a, m - 2, m)
}

/// Product lo*...*hi (mod MOD). Independent streams hide mul-mod latency.
fn range_prod(lo: u64, hi: u64) -> u64 {
    if lo > hi {
        return 1;
    }
    let mut i = lo;
    let mut a0 = 1u64;
    let mut a1 = 1u64;
    let mut a2 = 1u64;
    let mut a3 = 1u64;
    let mut a4 = 1u64;
    let mut a5 = 1u64;
    let mut a6 = 1u64;
    let mut a7 = 1u64;
    while i + 7 <= hi {
        a0 = mulm(a0, i);
        a1 = mulm(a1, i + 1);
        a2 = mulm(a2, i + 2);
        a3 = mulm(a3, i + 3);
        a4 = mulm(a4, i + 4);
        a5 = mulm(a5, i + 5);
        a6 = mulm(a6, i + 6);
        a7 = mulm(a7, i + 7);
        i += 8;
    }
    while i <= hi {
        a0 = mulm(a0, i);
        i += 1;
    }
    mulm(
        mulm(mulm(a0, a1), mulm(a2, a3)),
        mulm(mulm(a4, a5), mulm(a6, a7)),
    )
}

fn main() {
    let n: u64 = 100_000_000; // 10^8
    let two_n = 2 * n;

    // 2n < MOD, so the loop index itself needs no reduction.
    // Split 1..=2n into thread chunks; each task multiplies millions of
    // terms, then we reduce. Parts fully in 1..=n also contribute to n!.
    let nthreads = rayon::current_num_threads().max(1);
    let chunk = (two_n + nthreads as u64 - 1) / nthreads as u64;
    let parts: Vec<(u64, u64)> = (0..nthreads)
        .into_par_iter()
        .map(|t| {
            let lo = t as u64 * chunk + 1;
            let hi = ((t as u64 + 1) * chunk).min(two_n);
            if lo > hi {
                return (1u64, 1u64);
            }
            if hi <= n {
                let p = range_prod(lo, hi);
                (p, p)
            } else if lo > n {
                (range_prod(lo, hi), 1)
            } else {
                let pn = range_prod(lo, n);
                (mulm(pn, range_prod(n + 1, hi)), pn)
            }
        })
        .collect();

    let mut fact_2n = 1u64;
    let mut fact_n = 1u64;
    for &(p2, pn) in &parts {
        fact_2n = mulm(fact_2n, p2);
        fact_n = mulm(fact_n, pn);
    }

    // C_n = (2n)! / ((n+1)! * n!) = (2n)! * inv(n+1) * inv(n!)^2
    let inv_fact_n = inverse(fact_n, MOD);
    let inv_n_plus_1 = inverse(n + 1, MOD);

    let mut cn = mulm(fact_2n, inv_n_plus_1);
    cn = mulm(cn, inv_fact_n);
    cn = mulm(cn, inv_fact_n);

    // term2_val = 3n / (n+2) mod p
    let term2_num = 3 * n % MOD;
    let term2_den = inverse(n + 2, MOD);
    let term2_val = mulm(term2_num, term2_den);

    // P(n) = Cn^2 * (1 + term2_val^2)
    let cn_sq = mulm(cn, cn);
    let term2_sq = mulm(term2_val, term2_val);
    let bracket = (1 + term2_sq) % MOD;

    let ans = mulm(cn_sq, bracket);
    println!("{}", ans);
}
