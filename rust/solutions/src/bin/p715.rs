// Project Euler 715 - Sextuplet Norms
//
// Lucy DP of μ'(n) = μ(n) χ₄(n) (Gaussian Möbius) plus sum-of-cubes.

const M: i64 = 1_000_000_007;
const INV2: i64 = (M + 1) / 2;

#[inline(always)]
fn imod(a: i64) -> i64 {
    let r = a % M;
    if r < 0 { r + M } else { r }
}

#[inline(always)]
fn sum_cubes(n: i64) -> i64 {
    // (n(n+1)/2)^2 mod M; M^2 fits in i64 so no i128.
    let n = n % M;
    let s = n * ((n + 1) % M) % M * INV2 % M;
    s * s % M
}

#[inline(always)]
fn isqrt(n: u64) -> u64 {
    let mut x = (n as f64).sqrt() as u64;
    while x.saturating_mul(x) > n {
        x -= 1;
    }
    while x + 1 <= n / (x + 1) {
        x += 1;
    }
    x
}

/// sum_{k=1}^n χ₄(k) ∈ {0,1} for n ≥ 0.
#[inline(always)]
fn cum_chi(n: u64) -> i64 {
    ((n + 1) >> 1) as i64 & 1
}

fn main() {
    let big_n: u64 = 1_000_000_000_000;
    let mut l1 = (big_n as f64).cbrt() as u64;
    while l1 * l1 * l1 > big_n {
        l1 -= 1;
    }
    while (l1 + 1) * (l1 + 1) * (l1 + 1) <= big_n {
        l1 += 1;
    }
    let l1 = l1 as usize;
    let l2 = (big_n / l1 as u64) as usize;

    // Linear sieve: SPF as u32 (~400MB vs 800MB usize) and μ' in the same pass.
    let mut ff = vec![0u32; l2 + 1];
    let mut primes: Vec<u32> = Vec::with_capacity(6_000_000);
    let mut small = vec![0i32; l2 + 1];
    small[1] = 1;

    for i in 2..=l2 {
        // SAFETY: i ∈ [2, l2], arrays sized l2+1
        unsafe {
            if *ff.get_unchecked(i) == 0 {
                *ff.get_unchecked_mut(i) = i as u32;
                primes.push(i as u32);
                if i != 2 {
                    *small.get_unchecked_mut(i) = if i & 3 == 1 { -1 } else { 1 };
                }
            }
            let mu_i = *small.get_unchecked(i);
            let iu = i as u32;
            for &p in &primes {
                let v = iu as u64 * p as u64;
                if v > l2 as u64 {
                    break;
                }
                let vu = v as usize;
                *ff.get_unchecked_mut(vu) = p;
                if iu % p == 0 {
                    break;
                }
                *small.get_unchecked_mut(vu) = mu_i * *small.get_unchecked(p as usize);
            }
        }
    }
    drop(ff);
    drop(primes);

    // Prefix of μ'. |sum| ≤ l2 < M, so no modular reduction.
    for i in 1..=l2 {
        small[i] += small[i - 1];
    }

    let mut big = vec![0i64; l1 + 2];
    for i in (1..=l1).rev() {
        let ni = big_n / i as u64;
        let sqrtni = isqrt(ni) as usize;
        let mut acc = 1i64;

        // First k with i*k ≥ l1. χ₄(even)=0 so only odd k.
        let k_split = (l1 + i - 1) / i;
        let k_lim = sqrtni;

        // k < k_split: use already-computed big[i*k]
        let k_big_end = k_lim.min(k_split);
        let mut k = 3usize;
        if k < k_big_end && (k & 3) == 1 {
            let val = unsafe { *big.get_unchecked(i * k) };
            acc -= val;
            k += 2;
        }
        while k + 2 < k_big_end {
            // SAFETY: k < k_split ⇒ i*k < l1; k+2 < k_split similarly
            unsafe {
                acc += *big.get_unchecked(i * k);
                acc -= *big.get_unchecked(i * (k + 2));
            }
            k += 4;
        }
        if k < k_big_end {
            acc += unsafe { *big.get_unchecked(i * k) };
            k += 2;
        }

        // k ≥ k_split: use small[ni/k]
        if k < 3 {
            k = 3;
        }
        if k < k_lim && (k & 1) == 0 {
            k += 1;
        }
        if k < k_lim && (k & 3) == 1 {
            let idx = (ni / k as u64) as usize;
            // SAFETY: k ≥ k_split ⇒ ni/k ≤ l2; k < sqrt(ni) ⇒ idx ≥ 1
            acc -= unsafe { *small.get_unchecked(idx) } as i64;
            k += 2;
        }
        while k + 2 < k_lim {
            unsafe {
                let idx0 = (ni / k as u64) as usize;
                let idx1 = (ni / (k as u64 + 2)) as usize;
                acc += *small.get_unchecked(idx0) as i64;
                acc -= *small.get_unchecked(idx1) as i64;
            }
            k += 4;
        }
        if k < k_lim {
            let idx = (ni / k as u64) as usize;
            acc += unsafe { *small.get_unchecked(idx) } as i64;
        }

        // Tail: group k ≥ sqrt(ni) by t = floor(ni/k). Reuse ni/(t+1).
        let max_t = (ni / sqrtni as u64) as usize;
        let mut nit = ni;
        for t in 1..=max_t {
            let nit1 = ni / (t as u64 + 1);
            let diff = cum_chi(nit) - cum_chi(nit1);
            // SAFETY: t ≤ max_t ≈ sqrt(ni) ≤ l2
            acc -= diff * unsafe { *small.get_unchecked(t) } as i64;
            nit = nit1;
        }

        // |acc| ≪ 2^63: at most ~sqrt(ni) terms of size ≤ l2
        big[i] = imod(acc);
    }

    let mut ans: i64 = 0;

    // sum_{i=1}^{l2} μ'(i) * sum_cubes(N/i) via floor blocks (no i128)
    let l2u = l2 as u64;
    let mut i = 1u64;
    while i <= l2u {
        let q = big_n / i;
        let mut r = big_n / q;
        if r > l2u {
            r = l2u;
        }
        let mu_sum = unsafe {
            *small.get_unchecked(r as usize) - *small.get_unchecked((i - 1) as usize)
        } as i64;
        ans = (ans + sum_cubes(q as i64) * mu_sum) % M;
        i = r + 1;
    }

    for t in 1..l1 {
        let sc = sum_cubes(t as i64);
        let diff = big[t] - big[t + 1];
        ans = (ans + sc * diff) % M;
    }

    println!("{}", imod(ans));
}
