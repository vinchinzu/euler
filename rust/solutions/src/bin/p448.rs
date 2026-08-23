// Project Euler 448: Average least common multiple
// S(N) = (N + sum_{k=1}^N floor(N/k) * k * phi(k)) / 2  (mod MOD)
//
// Prefix of k*phi(k) is sieved to ~N^{2/3}. Remaining S(floor(N/i))
// values are filled bottom-up (Du Jiao linearization): each is O(sqrt)
// with array hits, no recursive HashMap.

const N: u64 = 99_999_999_019;
const MOD: u64 = 999_999_017;
const INV2: u64 = (MOD + 1) / 2;

fn mod_inv(mut a: i64, m: i64) -> i64 {
    let (mut g, mut x, mut y) = (m, 0i64, 1i64);
    while a != 0 {
        let q = g / a;
        let t = g - q * a;
        g = a;
        a = t;
        let t = x - q * y;
        x = y;
        y = t;
    }
    ((x % m) + m) % m
}

#[inline(always)]
fn mul_mod(a: u64, b: u64) -> u64 {
    a * b % MOD
}

#[inline(always)]
fn add_mod(a: u64, b: u64) -> u64 {
    let s = a + b;
    if s >= MOD { s - MOD } else { s }
}

#[inline(always)]
fn sub_mod(a: u64, b: u64) -> u64 {
    if a >= b { a - b } else { a + MOD - b }
}

/// n(n+1)/2 mod MOD.
#[inline(always)]
fn p1(n: u64) -> u64 {
    let n = n % MOD;
    let np1 = n + 1;
    let np1 = if np1 >= MOD { 0 } else { np1 };
    (n * np1 / 2) % MOD
}

/// n(n+1)(2n+1)/6 mod MOD.
#[inline(always)]
fn p2(n: u64, inv6: u64) -> u64 {
    let n = n % MOD;
    let np1 = n + 1;
    let np1 = if np1 >= MOD { 0 } else { np1 };
    mul_mod(mul_mod(mul_mod(n, np1), (2 * n + 1) % MOD), inv6)
}

/// S(x) = sum_{k<=x} k phi(k) from the identity
/// sum_{d<=x} d S(floor(x/d)) = P2(x).
fn compute_s(x: u64, limit: u64, small: &[u32], large: &[u64], parent: u64, inv6: u64) -> u64 {
    let mut f = p2(x, inv6);
    // Skip d=1 (that term is S(x) itself). prev_p1 = p1(1) = 1.
    let mut prev_p1 = 1u64;
    let mut l = 2u64;
    while l <= x {
        let q = x / l;
        let r = x / q;
        let pr1 = p1(r);
        let sum_d = sub_mod(pr1, prev_p1);
        let sq = if q <= limit {
            // SAFETY: q <= limit and small.len() == limit+1.
            unsafe { *small.get_unchecked(q as usize) as u64 }
        } else {
            // SAFETY: q > limit ⇒ parent/q <= parent/(limit+1) = max_i < large.len().
            unsafe { *large.get_unchecked((parent / q) as usize) }
        };
        f = sub_mod(f, mul_mod(sum_d, sq));
        prev_p1 = pr1;
        l = r + 1;
    }
    f
}

fn main() {
    let inv6 = mod_inv(6, MOD as i64) as u64;

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

    let mut acc = 0u64;
    for k in 1..=limit {
        // SAFETY: 1 <= k <= limit.
        let ph = unsafe { *phi.get_unchecked(k) } as u64;
        acc += ph * k as u64;
        acc %= MOD;
        unsafe {
            *phi.get_unchecked_mut(k) = acc as u32;
        }
    }
    let small = phi;

    let max_i = (N / (limit_u + 1)) as usize;
    let mut large = vec![0u64; max_i + 1];
    // Increasing floors: i = max_i .. 1 ⇒ x = N/i runs from just above the
    // sieve through N, so every S(x/d) is already in `small` or `large`.
    for i in (1..=max_i).rev() {
        let x = N / i as u64;
        let val = compute_s(x, limit_u, &small, &large, N, inv6);
        // SAFETY: i in 1..=max_i.
        unsafe {
            *large.get_unchecked_mut(i) = val;
        }
    }

    // sum_k floor(N/k) * k phi(k) via floor blocks; S(l-1) is the previous S(r).
    let mut ans = 0u64;
    let mut l = 1u64;
    let mut sprev = 0u64;
    while l <= N {
        let q = N / l;
        let r = N / q;
        let sr = if r <= limit_u {
            // SAFETY: r <= limit.
            unsafe { *small.get_unchecked(r as usize) as u64 }
        } else {
            // SAFETY: r > limit ⇒ q = N/l <= N/(limit+1) = max_i.
            unsafe { *large.get_unchecked(q as usize) }
        };
        ans = add_mod(ans, mul_mod(q % MOD, sub_mod(sr, sprev)));
        sprev = sr;
        l = r + 1;
    }

    ans = mul_mod(add_mod(ans, N % MOD), INV2);
    println!("{ans}");
}
