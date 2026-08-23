// Problem 908 - Clock Sequence II
//
// A clock sequence is a periodic sequence of positive integers that can be
// broken into contiguous segments such that the sum of the n-th segment is n.
//
// C(N) counts distinct clock sequences with minimal period at most N.
// We compute C(10^4) mod 1111211113.
//
// Algorithm:
// 1. For each modulus m, compute k(m) = number of distinct triangular residues mod m.
//    k is multiplicative; for prime powers: k(2^e) = 2^e; for odd p^e, a recurrence.
// 2. B[p] = number of clock seqs with period p (not necessarily minimal) is computed by
//    summing C(m-k, p-k) over all (m, k(m)) pairs with k <= p.
// 3. Mobius inversion: A[p] = sum_{d|p} mu(d) * B[p/d] gives minimal-period count.
// 4. C(N) = sum_{p=1..N} A[p].

use euler_utils::primes::primes_up_to;
use rayon::prelude::*;

const MOD: u64 = 1_111_211_113;
const N: usize = 10_000;

#[inline(always)]
fn mul_mod(a: u64, b: u64) -> u64 {
    a * b % MOD
}

/// Compute Mobius function for 0..=n via linear sieve.
fn mobius_upto(n: usize) -> Vec<i8> {
    let mut mu = vec![0i8; n + 1];
    let mut primes: Vec<usize> = Vec::new();
    let mut is_comp = vec![false; n + 1];
    mu[1] = 1;
    for i in 2..=n {
        if !is_comp[i] {
            primes.push(i);
            mu[i] = -1;
        }
        for &p in &primes {
            let v = i * p;
            if v > n {
                break;
            }
            is_comp[v] = true;
            if i % p == 0 {
                mu[v] = 0;
                break;
            }
            mu[v] = -mu[i];
        }
    }
    mu
}

/// Generate all pairs (n, k) = (m - k(m), k(m)) with k(m) <= max_k.
fn generate_moduli(max_k: usize) -> Vec<(u32, u32)> {
    let primes = primes_up_to(2 * max_k);

    // For each prime, precompute list of (p^e, k(p^e)) for e>=1 with k <= max_k
    let mut options: Vec<Vec<(u64, usize)>> = Vec::with_capacity(primes.len());
    for &p in &primes {
        let mut opts = Vec::new();
        if p == 2 {
            let mut m = 2u64;
            let mut k = 2usize;
            while k <= max_k {
                opts.push((m, k));
                m <<= 1;
                k <<= 1;
            }
        } else {
            let pu = p as u64;
            let mut m = pu;
            let mut k = ((pu + 1) / 2) as usize;
            let mut e = 1u32;
            while k <= max_k {
                opts.push((m, k));
                e += 1;
                m *= pu;
                let kk = k as u64;
                if e % 2 == 0 {
                    k = (pu * kk - (pu - 1)) as usize;
                } else {
                    k = (pu * kk - (pu - 1) / 2) as usize;
                }
            }
        }
        options.push(opts);
    }

    let mut pairs: Vec<(u32, u32)> = Vec::with_capacity(50_000);

    fn dfs(
        start_idx: usize,
        m_cur: u64,
        k_cur: usize,
        max_k: usize,
        options: &[Vec<(u64, usize)>],
        pairs: &mut Vec<(u32, u32)>,
    ) {
        debug_assert!(m_cur >= k_cur as u64);
        pairs.push(((m_cur - k_cur as u64) as u32, k_cur as u32));
        for j in start_idx..options.len() {
            let opts = &options[j];
            if opts.is_empty() {
                continue;
            }
            // Smallest k-factor for this prime
            if k_cur * opts[0].1 > max_k {
                break;
            }
            for &(mp, kp) in opts {
                let k_new = k_cur * kp;
                if k_new > max_k {
                    break;
                }
                dfs(j + 1, m_cur * mp, k_new, max_k, options, pairs);
            }
        }
    }

    dfs(0, 1, 1, max_k, &options, &mut pairs);
    pairs
}

/// Compute modular inverses inv[1..n] where inv[i] = i^{-1} mod MOD.
fn prepare_inverses(n: usize) -> Vec<u64> {
    let mut inv = vec![0u64; n + 1];
    inv[1] = 1;
    for i in 2..=n {
        let qi = MOD / (i as u64);
        let ri = (MOD % (i as u64)) as usize;
        inv[i] = (MOD - qi % MOD * inv[ri] % MOD) % MOD;
    }
    inv
}

/// Add binomial row C(n, r) into b[k + r] for r = 0..=rmax (no reduction).
fn add_binomial_row(b: &mut [u64], n: u32, k: u32, max_period: usize, inv: &[u64]) {
    let k = k as usize;
    let rmax = (max_period - k).min(n as usize);

    // SAFETY: k <= max_period, rmax <= max_period - k, so k + r <= max_period.
    // inv has length max_period + 1 and r in 1..=rmax.
    unsafe {
        *b.get_unchecked_mut(k) += 1;
        let mut c = 1u64;
        let mut numer = n as u64;
        let mut idx = k;
        let mut inv_ptr = inv.as_ptr().add(1);
        for _ in 0..rmax {
            c = mul_mod(c, numer);
            c = mul_mod(c, *inv_ptr);
            inv_ptr = inv_ptr.add(1);
            numer -= 1;
            idx += 1;
            *b.get_unchecked_mut(idx) += c;
        }
    }
}

/// Compute B[p] = number of clock sequences with period p (not necessarily minimal).
fn compute_b(max_period: usize) -> Vec<u64> {
    let moduli = generate_moduli(max_period);
    let inv = prepare_inverses(max_period);

    // Thread-local B arrays: inner-loop writes would race on a shared buffer.
    // Each pair contributes at most one add per index and |pairs| * MOD fits in u64,
    // so we defer % MOD until after the merge.
    let mut b = moduli
        .par_iter()
        .with_min_len(16)
        .fold(
            || vec![0u64; max_period + 1],
            |mut local, &(n, k)| {
                add_binomial_row(&mut local, n, k, max_period, &inv);
                local
            },
        )
        .reduce(
            || vec![0u64; max_period + 1],
            |mut a, b| {
                for (x, y) in a.iter_mut().zip(&b) {
                    *x += *y;
                }
                a
            },
        );

    for x in &mut b {
        *x %= MOD;
    }
    b
}

/// A[p] = number of clock sequences with minimal period exactly p.
/// A[p] = sum_{d|p} mu(d) * B[p/d]
fn compute_a_from_b(b: &[u64], mu: &[i8]) -> Vec<u64> {
    let n = b.len() - 1;
    let mut a = vec![0u64; n + 1];

    for d in 1..=n {
        let md = mu[d];
        if md == 0 {
            continue;
        }
        if md == 1 {
            for q in 1..=(n / d) {
                let p = d * q;
                let mut v = a[p] + b[q];
                if v >= MOD {
                    v -= MOD;
                }
                a[p] = v;
            }
        } else {
            // md == -1
            for q in 1..=(n / d) {
                let p = d * q;
                let v = if a[p] >= b[q] {
                    a[p] - b[q]
                } else {
                    a[p] + MOD - b[q]
                };
                a[p] = v;
            }
        }
    }

    a
}

fn main() {
    let b = compute_b(N);
    let mu = mobius_upto(N);
    let a = compute_a_from_b(&b, &mu);

    // C(N) = sum A[1..N]
    let mut s = 0u64;
    for i in 1..=N {
        s += a[i];
        s %= MOD;
    }

    // Sanity checks from problem statement
    {
        // Compute C(3), C(4), C(10) via prefix sums of a
        let mut c = vec![0u64; N + 1];
        let mut ps = 0u64;
        for i in 1..=N {
            ps += a[i];
            ps %= MOD;
            c[i] = ps;
        }
        debug_assert_eq!(c[3], 3, "C(3) should be 3");
        debug_assert_eq!(c[4], 7, "C(4) should be 7");
        debug_assert_eq!(c[10], 561, "C(10) should be 561");
    }

    println!("{}", s);
}
