// Project Euler 351: Hexagonal Orchards
// Hidden = 3*n*(n+1) - 6*sum_{k=1..n} phi(k), n=10^8.
// Summatory totient via Du Jiao: Φ(n) = n(n+1)/2 - Σ_{d=2}^n Φ(⌊n/d⌋).
// Sieve Φ to ~n^{2/3}; remaining Φ(⌊n/i⌋) filled bottom-up (array hits, no HashMap).

const N: u64 = 100_000_000;

/// Φ(x) from the identity Σ_{d≤x} Φ(⌊x/d⌋) = x(x+1)/2.
#[inline(always)]
fn compute_phi(x: u64, limit: u64, small: &[u64], large: &[u64], parent: u64) -> u64 {
    let mut f = x * (x + 1) / 2;
    let mut l = 2u64;
    while l <= x {
        let q = x / l;
        let r = x / q;
        let sq = if q <= limit {
            // SAFETY: q <= limit and small.len() == limit+1.
            unsafe { *small.get_unchecked(q as usize) }
        } else {
            // SAFETY: q > limit ⇒ parent/q <= parent/(limit+1) = max_i < large.len().
            unsafe { *large.get_unchecked((parent / q) as usize) }
        };
        f -= (r - l + 1) * sq;
        l = r + 1;
    }
    f
}

fn main() {
    // Sieve ~ N^{2/3} so Du Jiao work is O(N^{2/3}) rather than O(N^{3/4}).
    let cbrt_n = {
        let mut x = (N as f64).cbrt() as u64;
        while x.saturating_mul(x).saturating_mul(x) > N {
            x -= 1;
        }
        while (x + 1).saturating_mul(x + 1).saturating_mul(x + 1) <= N {
            x += 1;
        }
        x
    };
    let limit = (cbrt_n * cbrt_n) as usize;
    let limit_u = limit as u64;

    let mut phi = vec![0u32; limit + 1];
    let mut lp = vec![0u32; limit + 1];
    let mut primes: Vec<u32> = Vec::with_capacity(limit / 16);
    phi[1] = 1;
    for i in 2..=limit {
        // SAFETY: i <= limit; lp/phi have length limit+1.
        let lpi = unsafe { *lp.get_unchecked(i) };
        if lpi == 0 {
            unsafe {
                *lp.get_unchecked_mut(i) = i as u32;
                *phi.get_unchecked_mut(i) = (i - 1) as u32;
            }
            primes.push(i as u32);
        }
        let lpi = unsafe { *lp.get_unchecked(i) };
        let phi_i = unsafe { *phi.get_unchecked(i) };
        for &p in &primes {
            let ip = i as u64 * p as u64;
            if ip > limit_u || p > lpi {
                break;
            }
            let j = ip as usize;
            // SAFETY: j = i*p <= limit.
            unsafe {
                *lp.get_unchecked_mut(j) = p;
                *phi.get_unchecked_mut(j) = if p == lpi {
                    phi_i.wrapping_mul(p)
                } else {
                    phi_i.wrapping_mul(p - 1)
                };
            }
            if p == lpi {
                break;
            }
        }
    }
    drop(lp);
    drop(primes);

    let mut small = vec![0u64; limit + 1];
    let mut acc = 0u64;
    for k in 1..=limit {
        // SAFETY: 1 <= k <= limit.
        acc += unsafe { *phi.get_unchecked(k) } as u64;
        unsafe {
            *small.get_unchecked_mut(k) = acc;
        }
    }
    drop(phi);

    let max_i = (N / (limit_u + 1)) as usize;
    let mut large = vec![0u64; max_i + 1];
    // Increasing floors: i = max_i .. 1 ⇒ x = N/i runs from just above the
    // sieve through N, so every Φ(x/d) is already in `small` or `large`.
    for i in (1..=max_i).rev() {
        let x = N / i as u64;
        let val = compute_phi(x, limit_u, &small, &large, N);
        // SAFETY: i in 1..=max_i.
        unsafe {
            *large.get_unchecked_mut(i) = val;
        }
    }

    let totient_sum = unsafe { *large.get_unchecked(1) };
    let hidden = 3 * N * (N + 1) - 6 * totient_sum;
    println!("{hidden}");
}
