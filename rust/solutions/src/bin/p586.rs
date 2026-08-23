// Project Euler 586 - Binary Quadratic Forms
//
// Count numbers expressible as a^2 + 3ab + b^2 up to N = 10^15.
// Uses multiplicative structure and DFS over prime factorizations.

const N: i64 = 1_000_000_000_000_000;
const K: usize = 40;

struct Ctx<'a> {
    primes: &'a [u32],
    min_power: &'a [usize],
    inert_prefix: &'a [u32],
    split_prefix: &'a [u32],
    n: i64,
}

#[inline]
fn exceeds(mut prod: i64, p: i64, exp: usize, n: i64) -> bool {
    for _ in 0..exp {
        if p != 0 && prod > n / p {
            return true;
        }
        prod *= p;
    }
    false
}

#[inline]
fn isqrt(n: i64) -> i64 {
    if n <= 0 {
        return 0;
    }
    let mut s = (n as f64).sqrt() as i64;
    while s > 0 && s > n / s {
        s -= 1;
    }
    while s < i64::MAX && s + 1 <= n / (s + 1) {
        s += 1;
    }
    s
}

fn first_exceeds(start: usize, prod: i64, exp: usize, n: i64, primes: &[u32]) -> usize {
    let mut lo = start;
    let mut hi = primes.len();
    while lo < hi {
        let mid = (lo + hi) / 2;
        if exceeds(prod, primes[mid] as i64, exp, n) {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo
}

fn count_prefix(from_idx: usize, max_p: i64, primes: &[u32], prefix: &[u32]) -> i64 {
    if from_idx >= primes.len() || primes[from_idx] as i64 > max_p {
        return 0;
    }
    let mut lo = from_idx;
    let mut hi = primes.len();
    while lo < hi {
        let mid = (lo + hi) / 2;
        if primes[mid] as i64 <= max_p {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    (prefix[lo] - prefix[from_idx]) as i64
}

/// k == 1: extra factors are 5^a and even powers of inert primes (p % 5 = 2 or 3).
/// Once p^2 is a leaf, every later inert prime is also a leaf — count them.
fn helper_k1(last_index: i32, prod: i64, ctx: &Ctx) -> i64 {
    let mut ans = 1i64;
    let start = (last_index + 1) as usize;
    let primes = ctx.primes;
    let n = ctx.n;
    if start >= primes.len() || prod <= 0 {
        return ans;
    }
    let max_p = isqrt(n / prod);
    let mut i = start;
    while i < primes.len() {
        let p = primes[i] as i64;
        if p > max_p {
            break;
        }
        let pmod = p % 5;
        if p == 5 {
            let mut new_prod = prod;
            while new_prod <= n / 5 {
                new_prod *= 5;
                ans += helper_k1(i as i32, new_prod, ctx);
            }
            i += 1;
            continue;
        }
        if pmod == 1 || pmod == 4 {
            i += 1;
            continue;
        }
        if prod > n / p / p {
            break;
        }
        let p2 = p * p;
        let after = prod * p2;
        let higher = after <= n / p2;
        let more_prime = isqrt(n / after) > p;
        if !higher && !more_prime {
            ans += count_prefix(i, max_p, primes, ctx.inert_prefix);
            break;
        }
        let mut new_prod = prod;
        let mut e = 0usize;
        while new_prod <= n / p {
            new_prod *= p;
            e += 1;
            if e % 2 == 0 {
                ans += helper_k1(i as i32, new_prod, ctx);
            }
        }
        i += 1;
    }
    ans
}

/// k == 2: one more split prime p^1 reduces to k=1. Tail split primes are +1 each.
fn helper_k2(last_index: i32, prod: i64, ctx: &Ctx) -> i64 {
    let mut ans = 0i64;
    let start = (last_index + 1) as usize;
    let primes = ctx.primes;
    let n = ctx.n;
    if start >= primes.len() || prod <= 0 {
        return ans;
    }
    let max_p = n / prod;
    let mut i = start;
    while i < primes.len() {
        let p = primes[i] as i64;
        if p > max_p {
            break;
        }
        let p2_fits = p <= n / p && prod <= n / (p * p);
        let more_after_e1 = {
            let after = prod * p;
            isqrt(n / after) > p || (p < 5 && after <= n / 5)
        };
        if !p2_fits && !more_after_e1 {
            if p == 5 {
                ans += helper(2, i as i32, prod * 5, ctx);
                i += 1;
                continue;
            }
            ans += count_prefix(i, max_p, primes, ctx.split_prefix);
            break;
        }
        ans += process_prime(2, i, prod, ctx);
        i += 1;
    }
    ans
}

fn process_prime(k: usize, index: usize, prod: i64, ctx: &Ctx) -> i64 {
    let p = ctx.primes[index] as i64;
    let n = ctx.n;
    let mut ans = 0i64;
    let mut new_prod = prod;
    let mut e = 1usize;
    while new_prod <= n / p {
        new_prod *= p;
        if p % 5 == 1 || p % 5 == 4 {
            if k % (e + 1) == 0 {
                ans += helper(k / (e + 1), index as i32, new_prod, ctx);
            }
        } else if p == 5 || e % 2 == 0 {
            ans += helper(k, index as i32, new_prod, ctx);
        }
        e += 1;
    }
    ans
}

fn helper(k: usize, last_index: i32, prod: i64, ctx: &Ctx) -> i64 {
    if k == 1 {
        return helper_k1(last_index, prod, ctx);
    }
    if k == 2 {
        return helper_k2(last_index, prod, ctx);
    }

    let start = (last_index + 1) as usize;
    if start >= ctx.primes.len() {
        return 0;
    }
    let end = first_exceeds(start, prod, ctx.min_power[k], ctx.n, ctx.primes);
    let mut ans = 0i64;
    for index in start..end {
        ans += process_prime(k, index, prod, ctx);
    }
    ans
}

fn sieve_primes(limit: usize) -> Vec<u32> {
    if limit < 2 {
        return Vec::new();
    }
    let nbits = limit / 2 + 1;
    let mut bits = vec![u64::MAX; nbits.div_ceil(64)];
    bits[0] &= !1u64;
    let sqrt_lim = (limit as f64).sqrt() as usize + 1;
    let mut p = 3usize;
    while p <= sqrt_lim {
        let bi = p >> 1;
        if (bits[bi >> 6] >> (bi & 63)) & 1 == 1 {
            let mut j = p * p;
            while j <= limit {
                let bj = j >> 1;
                bits[bj >> 6] &= !(1u64 << (bj & 63));
                j += p << 1;
            }
        }
        p += 2;
    }
    let mut primes = Vec::with_capacity(limit / 10);
    primes.push(2);
    let mut x = 3usize;
    while x <= limit {
        let bi = x >> 1;
        if (bits[bi >> 6] >> (bi & 63)) & 1 == 1 {
            primes.push(x as u32);
        }
        x += 2;
    }
    primes
}

fn main() {
    let limit = 2 * K + 2;
    let mut ff = vec![0u32; limit + 1];
    for i in 2..=limit {
        if ff[i] == 0 {
            ff[i] = i as u32;
            let mut j = i * i;
            while j <= limit {
                if ff[j] == 0 {
                    ff[j] = i as u32;
                }
                j += i;
            }
        }
    }

    let mut min_power = vec![0usize; limit + 1];
    for k in 2..=limit {
        let mut m = k;
        let mut mp = 0;
        while m > 1 {
            let p = ff[m] as usize;
            let mut e = 0;
            while m % p == 0 {
                m /= p;
                e += 1;
            }
            mp += e * (p - 1);
        }
        min_power[k] = mp;
    }
    min_power[1] = 2;

    let min_mp = min_power[2 * K].min(min_power[2 * K + 1]);
    let mut max_prime: i64 = N;
    for _ in 0..min_mp - 1 {
        max_prime /= 11;
        if max_prime == 0 {
            break;
        }
    }

    let primes = sieve_primes(max_prime as usize);
    let mut inert_prefix = vec![0u32; primes.len() + 1];
    let mut split_prefix = vec![0u32; primes.len() + 1];
    for i in 0..primes.len() {
        let p = primes[i];
        let r = p % 5;
        inert_prefix[i + 1] = inert_prefix[i] + u32::from(p != 5 && (r == 2 || r == 3));
        split_prefix[i + 1] = split_prefix[i] + u32::from(r == 1 || r == 4);
    }

    let ctx = Ctx {
        primes: &primes,
        min_power: &min_power,
        inert_prefix: &inert_prefix,
        split_prefix: &split_prefix,
        n: N,
    };

    let (a, b) = rayon::join(
        || helper(2 * K, -1, 1, &ctx),
        || helper(2 * K + 1, -1, 1, &ctx),
    );
    println!("{}", a + b);
}
