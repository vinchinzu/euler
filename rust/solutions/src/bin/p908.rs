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

const MOD: u64 = 1_111_211_113;
const N: usize = 10_000;

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

/// Generate all pairs (m, k(m)) with k(m) <= max_k, where k is multiplicative.
fn generate_moduli(max_k: usize) -> Vec<(u64, usize)> {
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

    let mut pairs: Vec<(u64, usize)> = Vec::new();

    // DFS to enumerate all squarefree-extended products
    fn dfs(
        start_idx: usize,
        m_cur: u64,
        k_cur: usize,
        max_k: usize,
        options: &[Vec<(u64, usize)>],
        pairs: &mut Vec<(u64, usize)>,
    ) {
        pairs.push((m_cur, k_cur));
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

/// Compute modular inverses inv[1..n] where inv[i] = i^{-1} mod m.
fn prepare_inverses(n: usize, m: u64) -> Vec<u64> {
    let mut inv = vec![0u64; n + 1];
    inv[1] = 1;
    for i in 2..=n {
        // inv[i] = -(m / i) * inv[m % i] mod m
        let qi = m / (i as u64);
        let ri = (m % (i as u64)) as usize;
        inv[i] = (m - qi % m * inv[ri] % m) % m;
    }
    inv
}

/// Compute B[p] = number of clock sequences with period p (not necessarily minimal).
fn compute_b(max_period: usize, modulus: u64) -> Vec<u64> {
    let moduli = generate_moduli(max_period);
    let inv = prepare_inverses(max_period, modulus);

    let mut b = vec![0u64; max_period + 1];

    for &(m, k) in &moduli {
        if k > max_period {
            continue;
        }
        let mu = m as usize;
        if mu < k {
            continue;
        }
        let n = mu - k; // n = m - k
        let mut rmax = max_period - k;
        if n < rmax {
            rmax = n;
        }

        // r = 0: C(n, 0) = 1
        let idx0 = k;
        let mut v = b[idx0] + 1;
        if v >= modulus {
            v -= modulus;
        }
        b[idx0] = v;

        // r = 1..rmax: C(n, r) computed iteratively
        let mut c = 1u64;
        for r in 1..=rmax {
            // c = c * (n - r + 1) / r mod modulus
            let nr = (n - r + 1) as u64;
            c = c % modulus * (nr % modulus) % modulus;
            c = c * inv[r] % modulus;
            let idx = idx0 + r;
            let mut val = b[idx] + c;
            if val >= modulus {
                val -= modulus;
            }
            b[idx] = val;
        }
    }

    b
}

/// A[p] = number of clock sequences with minimal period exactly p.
/// A[p] = sum_{d|p} mu(d) * B[p/d]
fn compute_a_from_b(b: &[u64], mu: &[i8], modulus: u64) -> Vec<u64> {
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
                if v >= modulus {
                    v -= modulus;
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
                    a[p] + modulus - b[q]
                };
                a[p] = v;
            }
        }
    }

    a
}

fn main() {
    let b = compute_b(N, MOD);
    let mu = mobius_upto(N);
    let a = compute_a_from_b(&b, &mu, MOD);

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
