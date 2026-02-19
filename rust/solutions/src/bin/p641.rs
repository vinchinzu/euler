// Project Euler 641 - A Long Row of Dice
// f(10^36) = count of k ≤ 10^36 with τ(k) ≡ 1 (mod 6)
//
// τ(k) ≡ 1 (mod 6) requires k = m² (perfect square) with τ(m²) ≡ 1 (mod 3).
// Writing m = c³·d² (d squarefree), the mod-3 condition becomes μ(d) = 1.
// So f(10^36) = Σ_{d sqfree, μ(d)=1, d ≤ √N} ⌊(N/d²)^{1/3}⌋, N = 10^18.
//
// Split: direct sum for d ≤ T, then group by cbrt value for d > T.
// For each cbrt group, count of {d: sqfree, μ(d)=1} = (Q(b)-Q(a-1)+M(b)-M(a-1))/2
// where Q = squarefree count, M = Mertens function.
// Q(x) = Σ_{k≤√x} μ(k)⌊x/k²⌋, M(x) computed via Lucy DP in O(x^{2/3}).

use std::collections::HashMap;

fn isqrt(n: u64) -> u64 {
    if n == 0 { return 0; }
    let mut x = (n as f64).sqrt() as u64;
    while x > 0 && x * x > n { x -= 1; }
    while (x + 1) * (x + 1) <= n { x += 1; }
    x
}

#[inline(always)]
fn icbrt(n: u64) -> u64 {
    if n == 0 { return 0; }
    let mut x = (n as f64).cbrt() as u64;
    while x > 0 && x * x * x > n { x -= 1; }
    while (x + 1) * (x + 1) * (x + 1) <= n { x += 1; }
    x
}

/// Count of squarefree integers ≤ x: Q(x) = Σ_{k=1}^{√x} μ(k)⌊x/k²⌋
fn squarefree_count(x: u64, mu: &[i8]) -> i64 {
    let sq = isqrt(x) as usize;
    let mut result = 0i64;
    for k in 1..=sq {
        result += mu[k] as i64 * (x / (k as u64 * k as u64)) as i64;
    }
    result
}

/// Mertens function M(x) = Σ_{d=1}^{x} μ(d) via recursive identity with memoization
fn mertens(x: u64, prefix: &[i64], cache: &mut HashMap<u64, i64>) -> i64 {
    if (x as usize) < prefix.len() {
        return prefix[x as usize];
    }
    if let Some(&v) = cache.get(&x) {
        return v;
    }
    let u = isqrt(x);
    let mut result = 1i64;
    // First part: k = 2..u, floor(x/k) may be large → recursive
    for k in 2..=u {
        result -= mertens(x / k, prefix, cache);
    }
    // Second part: floor(x/k) for k > u gives values ≤ u → from prefix table
    let v_max = x / (u + 1);
    for v in 1..=v_max {
        let cnt = (x / v) as i64 - (x / (v + 1)) as i64;
        result -= cnt * prefix[v as usize];
    }
    cache.insert(x, result);
    result
}

fn main() {
    let n: u64 = 1_000_000_000_000_000_000; // 10^18
    let d_max = isqrt(n); // 10^9

    // Linear sieve for μ up to threshold
    let small_lim: usize = 1_100_000;
    let mut mu = vec![0i8; small_lim + 1];
    mu[1] = 1;
    {
        let mut spf = vec![0u32; small_lim + 1];
        let mut primes = Vec::new();
        for i in 2..=small_lim {
            if spf[i] == 0 {
                spf[i] = i as u32;
                primes.push(i as u32);
                mu[i] = -1;
            }
            for &p in &primes {
                let ip = i * p as usize;
                if ip > small_lim { break; }
                spf[ip] = p;
                if (i as u32) % p == 0 {
                    mu[ip] = 0;
                    break;
                }
                mu[ip] = -mu[i];
            }
        }
    }

    // Mertens prefix sums
    let mut mertens_pf = vec![0i64; small_lim + 1];
    for i in 1..=small_lim {
        mertens_pf[i] = mertens_pf[i - 1] + mu[i] as i64;
    }

    // Step 1: Direct sum for d = 1..thresh
    let thresh = 1_000_000u64;
    let mut answer: i64 = 0;
    for d in 1..=thresh {
        if mu[d as usize] == 1 {
            answer += icbrt(n / (d * d)) as i64;
        }
    }

    // Step 2: For d > thresh, group by cbrt value v
    // cbrt(N/(thresh+1)²) gives the max v for the grouped region
    let v_hi = icbrt(n / ((thresh + 1) * (thresh + 1)));
    let mut cache: HashMap<u64, i64> = HashMap::new();

    for v in 1..=v_hi {
        let b = std::cmp::min(isqrt(n / (v * v * v)), d_max);
        let a = isqrt(n / ((v + 1) * (v + 1) * (v + 1))) + 1;
        let a = std::cmp::max(a, thresh + 1);
        if a > b { continue; }

        // Count squarefree d with μ(d) = 1 in [a, b]:
        // F₊(x) = (Q(x) + M(x)) / 2,  count in [a,b] = F₊(b) - F₊(a-1)
        let qb = squarefree_count(b, &mu);
        let qa = squarefree_count(a - 1, &mu);
        let mb = mertens(b, &mertens_pf, &mut cache);
        let ma = mertens(a - 1, &mertens_pf, &mut cache);

        let count = (qb - qa + mb - ma) / 2;
        answer += v as i64 * count;
    }

    println!("{}", answer);
}
