// Project Euler Problem 953
// Factorisation Nim: S(10^14) mod 10^9+7
// Odd-only bit sieve, inlined SPRP, integer isqrt, rayon on DFS/direct.

use rayon::prelude::*;

const N_VAL: i64 = 100_000_000_000_000;
const MOD: i64 = 1_000_000_007;
const INV6: i64 = 166_666_668; // modular inverse of 6 mod MOD
const LIMIT_PRIME: usize = 22_000_000;
const SMALL_M_LIMIT: usize = 100_000;

#[inline(always)]
fn s2_contribution(k: i64) -> i64 {
    let quot = (N_VAL / k) as u64;
    let m = quot.isqrt() as i64;
    if m == 0 {
        return 0;
    }
    let mm = m % MOD;
    let s2 = mm * (mm + 1) % MOD * ((2 * mm + 1) % MOD) % MOD * INV6 % MOD;
    k % MOD * s2 % MOD
}

/// n < 2^32 so n*n fits in u64; wrapping_mul then % is exact.
#[inline(always)]
fn pow_mod_small(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut r = 1u64;
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
fn mul_mod64(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}

#[inline(always)]
fn pow_mod64(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut r = 1u64;
    while exp > 0 {
        if exp & 1 == 1 {
            r = mul_mod64(r, base, m);
        }
        base = mul_mod64(base, base, m);
        exp >>= 1;
    }
    r
}

#[inline(always)]
fn sprp_small(n: u64, a: u64) -> bool {
    let tz = (n - 1).trailing_zeros();
    let d = (n - 1) >> tz;
    let mut x = pow_mod_small(a, d, n);
    if x == 1 || x == n - 1 {
        return true;
    }
    for _ in 1..tz {
        x = x.wrapping_mul(x) % n;
        if x == n - 1 {
            return true;
        }
    }
    false
}

#[inline(always)]
fn sprp64(n: u64, a: u64) -> bool {
    let tz = (n - 1).trailing_zeros();
    let d = (n - 1) >> tz;
    let mut x = pow_mod64(a, d, n);
    if x == 1 || x == n - 1 {
        return true;
    }
    for _ in 1..tz {
        x = mul_mod64(x, x, n);
        if x == n - 1 {
            return true;
        }
    }
    false
}

/// Deterministic Miller–Rabin. n < 2^32: witnesses 2,7,61.
/// Larger i64: 2,3,5,7,11,13,23 (Jaeschke, n < 3.825e18).
#[inline(always)]
fn is_prime_mr(n: i64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 || n == 5 || n == 7 {
        return true;
    }
    if n & 1 == 0 {
        return false;
    }
    let n = n as u64;
    if n % 3 == 0 || n % 5 == 0 || n % 7 == 0 {
        return n < 8;
    }
    if n <= u32::MAX as u64 {
        return sprp_small(n, 2) && sprp_small(n, 7) && sprp_small(n, 61);
    }
    sprp64(n, 2)
        && sprp64(n, 3)
        && sprp64(n, 5)
        && sprp64(n, 7)
        && sprp64(n, 11)
        && sprp64(n, 13)
        && sprp64(n, 23)
}

/// Odd-only bit-packed sieve. Bit i represents n = 2*i+1.
fn sieve_odd_bits(limit: usize) -> (Vec<u64>, Vec<i32>) {
    let n_bits = limit / 2 + 1;
    let n_words = n_bits.div_ceil(64);
    let mut words = vec![u64::MAX; n_words];
    words[0] &= !1u64; // 1 is not prime
    let rem = n_bits % 64;
    if rem != 0 {
        words[n_words - 1] &= (1u64 << rem) - 1;
    }

    let sqrt_limit = limit.isqrt();
    let mut i = 1usize;
    while 2 * i + 1 <= sqrt_limit {
        if (words[i >> 6] >> (i & 63)) & 1 != 0 {
            let p = 2 * i + 1;
            let mut j = p * p / 2;
            while j < n_bits {
                words[j >> 6] &= !(1u64 << (j & 63));
                j += p;
            }
        }
        i += 1;
    }

    let mut primes = Vec::with_capacity(n_bits / 5);
    primes.push(2);
    for i in 1..n_bits {
        if (words[i >> 6] >> (i & 63)) & 1 != 0 {
            let p = 2 * i + 1;
            if p <= limit {
                primes.push(p as i32);
            }
        }
    }
    (words, primes)
}

#[inline(always)]
fn check_prime(p: i32, words: &[u64]) -> bool {
    if p < 2 {
        return false;
    }
    if p == 2 {
        return true;
    }
    if p & 1 == 0 {
        return false;
    }
    let pu = p as usize;
    if pu <= LIMIT_PRIME {
        let i = pu >> 1;
        // SAFETY: p odd, 3 <= p <= LIMIT_PRIME, i = p/2 is a valid odd-bit index
        unsafe { (*words.get_unchecked(i >> 6) >> (i & 63)) & 1 != 0 }
    } else {
        is_prime_mr(p as i64)
    }
}

fn dfs(
    start: usize,
    current_m: i64,
    current_g: i32,
    q: i32,
    limit_m: i64,
    primes_small: &[i32],
    words: &[u64],
) -> i64 {
    let mut local_sum: i64 = 0;
    let p = current_g ^ q;

    if p > q && check_prime(p, words) {
        // current_m <= limit_m = N/q^2, so cq = current_m*q <= N/q < N, fits i64
        let cq = current_m * q as i64;
        if p as i64 <= N_VAL / cq {
            local_sum = s2_contribution(cq * p as i64);
        }
    }

    let nsp = primes_small.len();
    let mut i = start;
    while i < nsp {
        // SAFETY: i in 0..primes_small.len()
        let next_p = unsafe { *primes_small.get_unchecked(i) };
        // current_m * next_p <= N/q < 10^14, fits i64
        let nm = current_m * next_p as i64;
        if nm > limit_m {
            break;
        }
        local_sum += dfs(
            i + 1,
            nm,
            current_g ^ next_p,
            q,
            limit_m,
            primes_small,
            words,
        );
        i += 1;
    }
    local_sum % MOD
}

fn main() {
    let (sieve_words, primes) = sieve_odd_bits(LIMIT_PRIME);

    let mut lp = vec![0i32; SMALL_M_LIMIT + 1];
    let mut g_arr = vec![0i32; SMALL_M_LIMIT + 1];
    let mut max_p_arr = vec![0i32; SMALL_M_LIMIT + 1];
    let mut sq = vec![true; SMALL_M_LIMIT + 1];
    let mut pr: Vec<i32> = Vec::with_capacity(10000);

    for i in 2..=SMALL_M_LIMIT {
        if lp[i] == 0 {
            lp[i] = i as i32;
            pr.push(i as i32);
            g_arr[i] = i as i32;
            max_p_arr[i] = i as i32;
        }
        for pi in 0..pr.len() {
            let p = pr[pi];
            if p > lp[i] || (i as i64) * (p as i64) > SMALL_M_LIMIT as i64 {
                break;
            }
            let ip = i * p as usize;
            lp[ip] = p;
            max_p_arr[ip] = max_p_arr[i];
            if p == lp[i] {
                sq[ip] = false;
            } else {
                sq[ip] = sq[i];
            }
            g_arr[ip] = g_arr[i] ^ p;
        }
    }

    let mut total_sum: i64 = s2_contribution(1) % MOD;
    let max_q = ((N_VAL / 2) as u64).isqrt() as i64;

    // Split into DFS items (large limit_m) and direct iteration items (small limit_m)
    let mut dfs_items: Vec<(usize, i32)> = Vec::with_capacity(4000);
    let mut direct_items: Vec<(usize, i32)> = Vec::with_capacity(500_000);

    for (qi, &q) in primes.iter().enumerate() {
        if (q as i64) > max_q {
            break;
        }
        let q_sq = q as i64 * q as i64;
        let limit_m = N_VAL / q_sq;
        if limit_m == 0 {
            break;
        }
        if limit_m <= SMALL_M_LIMIT as i64 {
            direct_items.push((qi, q));
        } else if 2 * q_sq <= N_VAL && qi > 0 {
            dfs_items.push((qi, q));
        }
    }

    // Phase 1: DFS items (parallel, heavy per-item)
    let dfs_sum: i64 = dfs_items
        .par_iter()
        .map(|&(qi, q)| {
            let limit_m = N_VAL / (q as i64 * q as i64);
            dfs(0, 1, 0, q, limit_m, &primes[..qi], &sieve_words)
        })
        .reduce(|| 0i64, |a, b| (a + b) % MOD);
    total_sum = (total_sum + dfs_sum) % MOD;

    // Phase 2: Direct iteration items (parallel)
    let direct_sum: i64 = direct_items
        .par_iter()
        .map(|&(_qi, q)| {
            let q_i64 = q as i64;
            let limit_m = N_VAL / (q_i64 * q_i64);
            let mut local_sum: i64 = 0;

            for m in 2..=limit_m as usize {
                // SAFETY: m <= SMALL_M_LIMIT, arrays have SMALL_M_LIMIT+1 entries
                let sqm = unsafe { *sq.get_unchecked(m) };
                if !sqm {
                    continue;
                }
                let maxp = unsafe { *max_p_arr.get_unchecked(m) };
                if maxp >= q {
                    continue;
                }
                let p = unsafe { *g_arr.get_unchecked(m) } ^ q;
                if p <= q {
                    continue;
                }
                if !check_prime(p, &sieve_words) {
                    continue;
                }
                let mq = m as i64 * q_i64;
                if p as i64 <= N_VAL / mq {
                    local_sum += s2_contribution(mq * p as i64);
                }
            }

            local_sum % MOD
        })
        .reduce(|| 0i64, |a, b| (a + b) % MOD);

    total_sum = (total_sum + direct_sum) % MOD;

    println!("{}", total_sum);
}
