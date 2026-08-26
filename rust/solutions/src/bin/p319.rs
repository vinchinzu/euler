// Project Euler 319: Bounded Sequences
// t(n) = 1 + sum_{d=1}^n μ(d) G(floor(n/d)),  G(m) = sum_{k=1}^m (3^k-2^k-1).
// Mertens via linear sieve + Lucy DP on remaining floor(N/k).

const N: u64 = 10_000_000_000;
const MOD: u64 = 1_000_000_000;
const MOD2: u64 = 2 * MOD;
const SIEVE_LIMIT: usize = 3_000_000;

#[inline(always)]
fn pow_mod(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut r = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            r = r.wrapping_mul(base) % m;
        }
        base = base.wrapping_mul(base) % m;
        exp >>= 1;
    }
    r
}

#[inline(always)]
fn g_large(m: u64) -> u64 {
    let p3 = pow_mod(3, m + 1, MOD2);
    let sum3 = ((p3 + MOD2 - 3) % MOD2) >> 1;
    let p2 = pow_mod(2, m + 1, MOD);
    let sum2 = (p2 + MOD - 2) % MOD;
    let s = (sum3 + MOD - sum2) % MOD;
    (s + MOD - m % MOD) % MOD
}

fn main() {
    let limit = SIEVE_LIMIT;
    let mut mu = vec![0i8; limit + 1];
    let mut minp = vec![0u32; limit + 1];
    let mut primes: Vec<u32> = Vec::with_capacity(limit / 10);
    mu[1] = 1;
    // SAFETY: i, j stay in 1..=limit; mu/minp have length limit+1.
    for i in 2..=limit {
        let minp_i = unsafe { *minp.get_unchecked(i) };
        if minp_i == 0 {
            unsafe {
                *minp.get_unchecked_mut(i) = i as u32;
                *mu.get_unchecked_mut(i) = -1;
            }
            primes.push(i as u32);
        }
        let mu_i = unsafe { *mu.get_unchecked(i) };
        let minp_i = unsafe { *minp.get_unchecked(i) };
        for &p in &primes {
            let v = i as u64 * p as u64;
            if v > limit as u64 {
                break;
            }
            let j = v as usize;
            unsafe {
                *minp.get_unchecked_mut(j) = p;
            }
            if p == minp_i {
                unsafe {
                    *mu.get_unchecked_mut(j) = 0;
                }
                break;
            }
            unsafe {
                *mu.get_unchecked_mut(j) = -mu_i;
            }
        }
    }
    drop(minp);
    drop(primes);

    let mut small = vec![0i32; limit + 1];
    let mut acc_m = 0i32;
    for i in 1..=limit {
        acc_m += unsafe { *mu.get_unchecked(i) } as i32;
        unsafe {
            *small.get_unchecked_mut(i) = acc_m;
        }
    }
    drop(mu);

    let mut g_small = vec![0u32; limit + 1];
    {
        let mut p3 = 1u64;
        let mut p2 = 1u64;
        let mut g = 0u64;
        for m in 1..=limit {
            p3 = p3 * 3 % MOD;
            p2 = p2 * 2 % MOD;
            g += p3 + MOD;
            g -= p2 + 1;
            g %= MOD;
            unsafe {
                *g_small.get_unchecked_mut(m) = g as u32;
            }
        }
    }

    let max_i = (N / (limit as u64 + 1)) as usize;
    let mut large = vec![0i32; max_i + 1];
    let lim = limit as u64;

    for ii in (1..=max_i).rev() {
        let x = N / ii as u64;
        let sq = x.isqrt();
        let mut s = 1i64;

        // i <= x/(L+1) ⇒ floor(x/i) > L, stored in `large`.
        let last_large = x / (lim + 1);
        let end_large = if last_large < sq { last_large } else { sq };

        let mut i = 2u64;
        // SAFETY: q > L ⇒ N/q <= max_i; after end_large, x/i <= L so small[] is in range.
        while i + 3 <= end_large {
            let q0 = x / i;
            let q1 = x / (i + 1);
            let q2 = x / (i + 2);
            let q3 = x / (i + 3);
            unsafe {
                s -= *large.get_unchecked((N / q0) as usize) as i64;
                s -= *large.get_unchecked((N / q1) as usize) as i64;
                s -= *large.get_unchecked((N / q2) as usize) as i64;
                s -= *large.get_unchecked((N / q3) as usize) as i64;
            }
            i += 4;
        }
        while i <= end_large {
            let q = x / i;
            unsafe {
                s -= *large.get_unchecked((N / q) as usize) as i64;
            }
            i += 1;
        }
        while i + 3 <= sq {
            unsafe {
                s -= *small.get_unchecked((x / i) as usize) as i64;
                s -= *small.get_unchecked((x / (i + 1)) as usize) as i64;
                s -= *small.get_unchecked((x / (i + 2)) as usize) as i64;
                s -= *small.get_unchecked((x / (i + 3)) as usize) as i64;
            }
            i += 4;
        }
        while i <= sq {
            unsafe {
                s -= *small.get_unchecked((x / i) as usize) as i64;
            }
            i += 1;
        }

        let max_q = if sq < x { x / (sq + 1) } else { 0 };
        let mut hi = x;
        let mut q = 1u64;
        while q <= max_q {
            let lo_m1 = x / (q + 1);
            let left_raw = lo_m1 + 1;
            let left = if left_raw > sq + 1 { left_raw } else { sq + 1 };
            if left <= hi {
                unsafe {
                    s -= (hi - left + 1) as i64 * *small.get_unchecked(q as usize) as i64;
                }
            }
            hi = lo_m1;
            q += 1;
        }
        large[ii] = s as i32;
    }

    let mut ans: i128 = 1;
    let mut l = 1u64;
    while l <= N {
        let q = N / l;
        let r = N / q;
        let mr = if r <= lim {
            unsafe { *small.get_unchecked(r as usize) }
        } else {
            unsafe { *large.get_unchecked((N / r) as usize) }
        } as i128;
        let ml = if l == 1 {
            0
        } else if l - 1 <= lim {
            unsafe { *small.get_unchecked((l - 1) as usize) }
        } else {
            unsafe { *large.get_unchecked((N / (l - 1)) as usize) }
        } as i128;
        let g = if q <= lim {
            unsafe { *g_small.get_unchecked(q as usize) as i128 }
        } else {
            g_large(q) as i128
        };
        ans += (mr - ml) * g;
        l = r + 1;
    }
    let mut out = (ans % MOD as i128) as i64;
    if out < 0 {
        out += MOD as i64;
    }
    println!("{}", out);
}
