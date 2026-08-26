// Project Euler 299: Three similar triangles
//
// Two families, parametrized by coprime opposite-parity (m, n):
//   f1 = m^2 - n^2 + 2mn,  contribute 2 * floor((N-1)/f1)
//   f2 = 2(m^2 + n^2),     contribute     floor((N-1)/f2)
// Both are homogeneous of degree 2, so Möbius drops the gcd:
//   sum_{gcd=1} floor(L/f(m,n)) = sum_d μ(d) sum floor(L/(d^2 f(m',n'))).
// Opposite parity forces d odd. Inner f-values step as quadratics.

use rayon::prelude::*;

fn linear_sieve_mu(limit: usize) -> Vec<i8> {
    let mut mu = vec![0i8; limit + 1];
    let mut composite = vec![false; limit + 1];
    let mut primes = Vec::with_capacity(limit / 10);
    mu[1] = 1;
    for i in 2..=limit {
        if !composite[i] {
            primes.push(i);
            mu[i] = -1;
        }
        for &p in &primes {
            let ip = i * p;
            if ip > limit {
                break;
            }
            composite[ip] = true;
            if i % p == 0 {
                mu[ip] = 0;
                break;
            } else {
                mu[ip] = -mu[i];
            }
        }
    }
    mu
}

fn main() {
    const N: u64 = 100_000_000;
    let nm1 = N - 1;

    // f1(2,1) = 7 is the smallest admissible value, so d^2 * 7 <= N-1.
    let dmax = (nm1 / 7).isqrt() as usize;
    let mu = linear_sieve_mu(dmax);

    // (μ(d), floor((N-1)/d^2)) for odd square-free d, increasing d (decreasing L).
    let ds: Vec<(i64, u64)> = (1..=dmax)
        .step_by(2)
        .filter(|&d| mu[d] != 0)
        .map(|d| {
            let d2 = d as u64 * d as u64;
            (mu[d] as i64, nm1 / d2)
        })
        .collect();

    // 2n^2 + 4n + 1 <= N-1  <=>  n <= isqrt((N)/2) - 1
    let nmax1 = (N / 2).isqrt() as u64 - 1;
    // 4n^2 + 4n + 2 <= N-1  <=>  n <= (isqrt(N-2) - 1)/2
    let nmax2 = ((nm1 - 1).isqrt() - 1) / 2;

    let case1: i64 = (1..=nmax1)
        .into_par_iter()
        .map(|n| {
            let f0 = 2 * n * n + 4 * n + 1;
            let inc0 = 8 * n + 8;
            let mut local = 0i64;
            for &(mu_d, l) in &ds {
                if f0 > l {
                    break;
                }
                let mut f = f0;
                let mut inc = inc0;
                while f <= l {
                    local += mu_d * (l / f) as i64;
                    f += inc;
                    inc += 8;
                }
            }
            local
        })
        .sum();

    let case2: i64 = (1..=nmax2)
        .into_par_iter()
        .map(|n| {
            let f0 = 4 * n * n + 4 * n + 2;
            let inc0 = 8 * n + 16;
            let mut local = 0i64;
            for &(mu_d, l) in &ds {
                if f0 > l {
                    break;
                }
                let mut f = f0;
                let mut inc = inc0;
                while f <= l {
                    local += mu_d * (l / f) as i64;
                    f += inc;
                    inc += 16;
                }
            }
            local
        })
        .sum();

    println!("{}", case1 * 2 + case2);
}
