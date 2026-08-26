// Project Euler 428: Necklace of Circles
// T(N) = S3 + S4 + S6. Linearized Du Jiao / Lucy tables indexed by N/i
// (small + large Vec), integer isqrt, rayon over independent phases.

use fxhash::FxHashMap;
use rayon::prelude::*;

const NVAL: i64 = 1_000_000_000;

#[inline(always)]
fn isqrt(n: i64) -> i64 {
    if n <= 0 {
        0
    } else {
        (n as u64).isqrt() as i64
    }
}

/// Prefix of f on 1..=limit, plus f(N/i) for i = 1..=N/(limit+1).
struct LinPref {
    n: i64,
    limit: i64,
    small: Vec<i64>,
    large: Vec<i64>,
}

impl LinPref {
    #[inline(always)]
    fn get(&self, x: i64) -> i64 {
        if x <= 0 {
            0
        } else if x <= self.limit {
            // SAFETY: x in 1..=limit, small.len() == limit+1.
            unsafe { *self.small.get_unchecked(x as usize) }
        } else {
            // SAFETY: x > limit ⇒ n/x <= n/(limit+1) < large.len().
            unsafe { *self.large.get_unchecked((self.n / x) as usize) }
        }
    }
}

struct PiTab {
    n: i64,
    sqrt_n: i64,
    small1: Vec<i64>,
    big1: Vec<i64>,
}

impl PiTab {
    #[inline(always)]
    fn pi1(&self, v: i64) -> i64 {
        if v < 2 {
            0
        } else if v <= self.sqrt_n {
            unsafe { *self.small1.get_unchecked(v as usize) }
        } else {
            unsafe { *self.big1.get_unchecked((self.n / v) as usize) }
        }
    }
}

#[inline(always)]
fn l3(x: i64, l: &LinPref) -> i64 {
    if x <= 0 {
        0
    } else {
        l.get(x) + l.get(x / 3)
    }
}

/// F(X) = Σ |μ(d)| ⌊X/d⌋ and T(X) = Σ 2^ω(d) ⌊X/d⌋ share this loop.
#[inline]
fn dirichlet_from_prefix(x: i64, pref: &LinPref) -> i64 {
    if x <= 0 {
        return 0;
    }
    let mut s = 0i64;
    let mut d = 1i64;
    let mut prev = 0i64;
    while d <= x {
        let q = x / d;
        let dmax = x / q;
        let v = pref.get(dmax);
        s += q * (v - prev);
        prev = v;
        d = dmax + 1;
    }
    s
}

/// L(X) = Σ_{j≤√X} M(⌊X/j²⌋), grouped on constant ⌊X/j²⌋.
fn compute_l(x: i64, m: &LinPref) -> i64 {
    if x <= 0 {
        return 0;
    }
    let s = isqrt(x);
    let mut tot = 0i64;
    let mut j = 1i64;
    while j <= s {
        let q = x / (j * j);
        let mut j_hi = isqrt(x / q);
        if j_hi > s {
            j_hi = s;
        }
        if j_hi < j {
            j_hi = j;
        }
        tot += (j_hi - j + 1) * m.get(q);
        j = j_hi + 1;
    }
    tot
}

fn compute_q_raw(x: i64, mu: &[i8]) -> i64 {
    let s = isqrt(x);
    let mut t = 0i64;
    let mut k = 1i64;
    while k <= s {
        // SAFETY: k <= √x <= √N < sieve_limit.
        let mk = unsafe { *mu.get_unchecked(k as usize) };
        if mk != 0 {
            t += mk as i64 * (x / (k * k));
        }
        k += 1;
    }
    t
}

fn fill_mertens(n: i64, sl: usize, mu_prefix: Vec<i64>) -> LinPref {
    let limit = sl as i64;
    let max_i = (n / (limit + 1)) as usize;
    let mut large = vec![0i64; max_i + 1];
    for i in (1..=max_i).rev() {
        let x = n / i as i64;
        let mut s = 0i64;
        let mut l = 2i64;
        while l <= x {
            let q = x / l;
            let r = x / q;
            let mq = if q <= limit {
                unsafe { *mu_prefix.get_unchecked(q as usize) }
            } else {
                unsafe { *large.get_unchecked((n / q) as usize) }
            };
            s += (r - l + 1) * mq;
            l = r + 1;
        }
        unsafe {
            *large.get_unchecked_mut(i) = 1 - s;
        }
    }
    LinPref {
        n,
        limit,
        small: mu_prefix,
        large,
    }
}

fn fill_q(n: i64, sl: usize, absmu: Vec<i64>, mu: &[i8]) -> LinPref {
    let limit = sl as i64;
    let max_i = (n / (limit + 1)) as usize;
    let mut large = vec![0i64; max_i + 1];
    large.par_iter_mut().enumerate().for_each(|(i, slot)| {
        if i == 0 {
            return;
        }
        *slot = compute_q_raw(n / i as i64, mu);
    });
    LinPref {
        n,
        limit,
        small: absmu,
        large,
    }
}

fn fill_f(n: i64, sl: usize, f_small: Vec<i64>, qtab: &LinPref) -> LinPref {
    let limit = sl as i64;
    let max_i = (n / (limit + 1)) as usize;
    let mut large = vec![0i64; max_i + 1];
    large.par_iter_mut().enumerate().for_each(|(i, slot)| {
        if i == 0 {
            return;
        }
        *slot = dirichlet_from_prefix(n / i as i64, qtab);
    });
    LinPref {
        n,
        limit,
        small: f_small,
        large,
    }
}

fn fill_l(n: i64, sqrt_n: usize, m: &LinPref) -> LinPref {
    let mut small = vec![0i64; sqrt_n + 1];
    let mut large = vec![0i64; sqrt_n + 1];
    rayon::join(
        || {
            small.par_iter_mut().enumerate().for_each(|(x, slot)| {
                if x > 0 {
                    *slot = compute_l(x as i64, m);
                }
            });
        },
        || {
            large.par_iter_mut().enumerate().for_each(|(k, slot)| {
                if k > 0 {
                    *slot = compute_l(n / k as i64, m);
                }
            });
        },
    );
    LinPref {
        n,
        limit: sqrt_n as i64,
        small,
        large,
    }
}

fn lucy_dp(n: i64, sqrt_n: usize, primes_small: &[i32]) -> PiTab {
    let mut small1 = vec![0i64; sqrt_n + 1];
    let mut small2 = vec![0i64; sqrt_n + 1];
    let mut big1 = vec![0i64; sqrt_n + 2];
    let mut big2 = vec![0i64; sqrt_n + 2];
    let mut vlo = vec![0i64; sqrt_n + 1];

    for v in 1..=sqrt_n {
        let vi = v as i64;
        small1[v] = (vi + 2) / 3 - 1;
        small2[v] = (vi + 1) / 3;
        vlo[v] = n / vi;
    }
    for k in 1..=sqrt_n {
        let v = unsafe { *vlo.get_unchecked(k) };
        big1[k] = (v + 2) / 3 - 1;
        big2[k] = (v + 1) / 3;
    }

    let sqrt_n_i = sqrt_n as i64;
    for &p in primes_small {
        if p == 3 {
            continue;
        }
        let p64 = p as i64;
        let pp = p64 * p64;
        let p1 = unsafe { *small1.get_unchecked((p as usize) - 1) };
        let p2 = unsafe { *small2.get_unchecked((p as usize) - 1) };
        let max_k = std::cmp::min(sqrt_n_i, n / pp) as usize;
        let pmod1 = p % 3 == 1;

        if pmod1 {
            for k in 1..=max_k {
                let vp = unsafe { *vlo.get_unchecked(k) } / p64;
                let (c1, c2) = if vp <= sqrt_n_i {
                    unsafe {
                        (
                            *small1.get_unchecked(vp as usize),
                            *small2.get_unchecked(vp as usize),
                        )
                    }
                } else {
                    let idx = (n / vp) as usize;
                    unsafe { (*big1.get_unchecked(idx), *big2.get_unchecked(idx)) }
                };
                unsafe {
                    *big1.get_unchecked_mut(k) -= c1 - p1;
                    *big2.get_unchecked_mut(k) -= c2 - p2;
                }
            }
            if pp <= sqrt_n_i {
                for v in (pp as usize..=sqrt_n).rev() {
                    let vp = v as i64 / p64;
                    unsafe {
                        let c1 = *small1.get_unchecked(vp as usize);
                        let c2 = *small2.get_unchecked(vp as usize);
                        *small1.get_unchecked_mut(v) -= c1 - p1;
                        *small2.get_unchecked_mut(v) -= c2 - p2;
                    }
                }
            }
        } else {
            for k in 1..=max_k {
                let vp = unsafe { *vlo.get_unchecked(k) } / p64;
                let (c1, c2) = if vp <= sqrt_n_i {
                    unsafe {
                        (
                            *small1.get_unchecked(vp as usize),
                            *small2.get_unchecked(vp as usize),
                        )
                    }
                } else {
                    let idx = (n / vp) as usize;
                    unsafe { (*big1.get_unchecked(idx), *big2.get_unchecked(idx)) }
                };
                unsafe {
                    *big1.get_unchecked_mut(k) -= c2 - p2;
                    *big2.get_unchecked_mut(k) -= c1 - p1;
                }
            }
            if pp <= sqrt_n_i {
                for v in (pp as usize..=sqrt_n).rev() {
                    let vp = v as i64 / p64;
                    unsafe {
                        let c1 = *small1.get_unchecked(vp as usize);
                        let c2 = *small2.get_unchecked(vp as usize);
                        *small1.get_unchecked_mut(v) -= c2 - p2;
                        *small2.get_unchecked_mut(v) -= c1 - p1;
                    }
                }
            }
        }
    }

    drop(small2);
    drop(big2);
    PiTab {
        n,
        sqrt_n: sqrt_n_i,
        small1,
        big1,
    }
}

fn t_c(x: i64, f: &LinPref, cache: &mut FxHashMap<i64, i64>) -> i64 {
    if x <= 0 {
        return 0;
    }
    if let Some(&v) = cache.get(&x) {
        return v;
    }
    let result = dirichlet_from_prefix(x, f);
    cache.insert(x, result);
    result
}

fn t_odd(
    x: i64,
    f: &LinPref,
    t_cache: &mut FxHashMap<i64, i64>,
    to_cache: &mut FxHashMap<i64, i64>,
) -> i64 {
    if x <= 0 {
        return 0;
    }
    if let Some(&v) = to_cache.get(&x) {
        return v;
    }
    let mut result = t_c(x, f, t_cache);
    let mut a = 1i64;
    let mut pw = 2i64;
    while pw <= x {
        result -= (2 * a + 1) * t_odd(x / pw, f, t_cache, to_cache);
        a += 1;
        pw *= 2;
    }
    to_cache.insert(x, result);
    result
}

fn t_on3(
    x: i64,
    f: &LinPref,
    t_cache: &mut FxHashMap<i64, i64>,
    to_cache: &mut FxHashMap<i64, i64>,
    ton3_cache: &mut FxHashMap<i64, i64>,
) -> i64 {
    if x <= 0 {
        return 0;
    }
    if let Some(&v) = ton3_cache.get(&x) {
        return v;
    }
    let mut result = t_odd(x, f, t_cache, to_cache);
    let mut c = 1i64;
    let mut pw = 3i64;
    while pw <= x {
        result -= (2 * c + 1) * t_on3(x / pw, f, t_cache, to_cache, ton3_cache);
        c += 1;
        pw *= 3;
    }
    ton3_cache.insert(x, result);
    result
}

fn compute_s3_s4_s6(n: i64, f: &LinPref) -> (i64, i64, i64, i64) {
    let mut t_cache: FxHashMap<i64, i64> = FxHashMap::with_capacity_and_hasher(1024, Default::default());
    let mut to_cache: FxHashMap<i64, i64> = FxHashMap::with_capacity_and_hasher(1024, Default::default());
    let mut ton3_cache: FxHashMap<i64, i64> =
        FxHashMap::with_capacity_and_hasher(1024, Default::default());

    let mut needed = Vec::with_capacity(512);
    let mut pw2 = 1i64;
    for _ in 0..61 {
        if pw2 > n {
            break;
        }
        let mut pw3 = 1i64;
        for _ in 0..40 {
            let pw = pw2.saturating_mul(pw3);
            if pw > n {
                break;
            }
            needed.push(n / pw);
            pw3 = pw3.saturating_mul(3);
        }
        pw2 = pw2.saturating_mul(2);
    }
    needed.sort_unstable();
    needed.dedup();
    for &x in &needed {
        t_on3(x, f, &mut t_cache, &mut to_cache, &mut ton3_cache);
    }

    let mut s4 = 0i64;
    {
        let mut a = 0i64;
        let mut pw = 1i64;
        while pw <= n {
            s4 += (2 * a + 2) * t_odd(n / pw, f, &mut t_cache, &mut to_cache);
            a += 1;
            pw *= 2;
        }
    }

    let mut s3 = 0i64;
    {
        let mut a = 0i64;
        let mut pw2 = 1i64;
        while pw2 <= n {
            let mut c = 0i64;
            let mut pw3 = 1i64;
            while pw2 * pw3 <= n {
                s3 += (2 * a + 3)
                    * (2 * c + 2)
                    * t_on3(n / (pw2 * pw3), f, &mut t_cache, &mut to_cache, &mut ton3_cache);
                c += 1;
                pw3 *= 3;
            }
            a += 1;
            pw2 *= 2;
        }
    }

    let mut s6_div3 = 0i64;
    {
        let mut v = 1i64;
        let mut pw3 = 3i64;
        while pw3 <= n {
            let mut a = 0i64;
            let mut pw2 = 1i64;
            while pw2 * pw3 <= n {
                s6_div3 += (2 * v - 1)
                    * (2 * a + 3)
                    * t_on3(n / (pw2 * pw3), f, &mut t_cache, &mut to_cache, &mut ton3_cache);
                a += 1;
                pw2 *= 2;
            }
            v += 1;
            pw3 *= 3;
        }
    }

    let mut s6_tau = 0i64;
    {
        let mut a = 0i64;
        let mut pw = 1i64;
        while pw <= n {
            s6_tau += (2 * a + 3) * t_on3(n / pw, f, &mut t_cache, &mut to_cache, &mut ton3_cache);
            a += 1;
            pw *= 2;
        }
    }

    (s3, s4, s6_div3, s6_tau)
}

fn large_prime_sum(d_val: i64, last_prime: i64, n: i64, sqrt_n: i64, pi: &PiTab, l: &LinPref) -> i64 {
    let upper = n / d_val;
    let lower = last_prime.max(sqrt_n);
    if upper <= lower {
        return 0;
    }
    let mut p = lower + 1;
    let mut large_sum = 0i64;
    while p <= upper {
        let q = upper / p;
        let p_hi = if q > 0 {
            upper.min(upper / q)
        } else {
            upper
        };
        let p_lo = (lower + 1).max(upper / (q + 1) + 1);
        let cnt = pi.pi1(p_hi) - pi.pi1(p_lo - 1);
        if cnt > 0 {
            large_sum += cnt * l3(q, l);
        }
        p = p_hi + 1;
    }
    large_sum
}

fn node_contrib(d_val: i64, b_val: i64, last_prime: i64, n: i64, sqrt_n: i64, pi: &PiTab, l: &LinPref) -> i64 {
    let ls = large_prime_sum(d_val, last_prime, n, sqrt_n, pi, l);
    b_val * (l3(n / d_val, l) + 4 * ls)
}

fn dfs_subtree(
    idx: usize,
    d_val: i64,
    b_val: i64,
    last_prime: i64,
    n: i64,
    sqrt_n: i64,
    p1: &[i32],
    pi: &PiTab,
    l: &LinPref,
) -> i64 {
    let mut s = node_contrib(d_val, b_val, last_prime, n, sqrt_n, pi, l);
    let cap = n / d_val;
    let maxp = cap.min(sqrt_n);
    let end = p1.partition_point(|&p| (p as i64) <= maxp);
    for i in idx..end {
        let pr = unsafe { *p1.get_unchecked(i) } as i64;
        let mut pk = pr;
        let mut k = 1i64;
        loop {
            s += dfs_subtree(i + 1, d_val * pk, b_val * (4 * k), pr, n, sqrt_n, p1, pi, l);
            if pk > cap / pr {
                break;
            }
            pk *= pr;
            k += 1;
        }
    }
    s
}

fn main() {
    let n = NVAL;
    let sqrt_n = isqrt(n) as usize;

    let cbrt = {
        let mut c = (n as f64).cbrt() as i64;
        while c.saturating_mul(c).saturating_mul(c) > n {
            c -= 1;
        }
        while (c + 1).saturating_mul(c + 1).saturating_mul(c + 1) <= n {
            c += 1;
        }
        c as usize
    };
    let sl = (cbrt * cbrt).max(sqrt_n + 1);

    let mut mu = vec![0i8; sl + 1];
    mu[1] = 1;
    let mut is_comp = vec![0u8; sl + 1];
    let mut primes: Vec<i32> = Vec::with_capacity(sl / 10);
    let mut tw = vec![0i32; sl + 1];
    tw[1] = 1;

    for i in 2..=sl {
        if is_comp[i] == 0 {
            primes.push(i as i32);
            mu[i] = -1;
            tw[i] = 2;
        }
        let mu_i = mu[i];
        let tw_i = tw[i];
        for &p in &primes {
            let v = p as usize * i;
            if v > sl {
                break;
            }
            is_comp[v] = 1;
            if i % p as usize == 0 {
                mu[v] = 0;
                tw[v] = tw_i;
                break;
            }
            mu[v] = -mu_i;
            tw[v] = tw_i * 2;
        }
    }
    drop(is_comp);

    let mut mu_prefix = vec![0i64; sl + 1];
    let mut absmu = vec![0i64; sl + 1];
    let mut f_small = vec![0i64; sl + 1];
    let mut macc = 0i64;
    let mut qacc = 0i64;
    let mut facc = 0i64;
    for i in 1..=sl {
        macc += mu[i] as i64;
        qacc += mu[i].unsigned_abs() as i64;
        facc += tw[i] as i64;
        mu_prefix[i] = macc;
        absmu[i] = qacc;
        f_small[i] = facc;
    }
    drop(tw);

    let primes_small: Vec<i32> = primes
        .iter()
        .copied()
        .take_while(|&p| p as usize <= sqrt_n)
        .collect();
    let primes_1mod3: Vec<i32> = primes_small.iter().copied().filter(|&p| p % 3 == 1).collect();
    drop(primes);

    let ((pi, ltab), (s3, s4, s6_div3, s6_tau)) = rayon::join(
        || {
            rayon::join(
                || lucy_dp(n, sqrt_n, &primes_small),
                || {
                    let mtab = fill_mertens(n, sl, mu_prefix);
                    fill_l(n, sqrt_n, &mtab)
                },
            )
        },
        || {
            let qtab = fill_q(n, sl, absmu, &mu);
            let ftab = fill_f(n, sl, f_small, &qtab);
            compute_s3_s4_s6(n, &ftab)
        },
    );

    let sqrt_n_i = sqrt_n as i64;
    let p1 = &primes_1mod3;
    let root = node_contrib(1, 1, 0, n, sqrt_n_i, &pi, &ltab);
    let rest: i64 = (0..p1.len())
        .into_par_iter()
        .map(|i| {
            let pr = p1[i] as i64;
            let mut s = 0i64;
            let mut pk = pr;
            let mut k = 1i64;
            loop {
                s += dfs_subtree(i + 1, pk, 4 * k, pr, n, sqrt_n_i, p1, &pi, &ltab);
                if pk > n / pr {
                    break;
                }
                pk *= pr;
                k += 1;
            }
            s
        })
        .sum();

    let s6_chi = -(root + rest);
    let s6 = s6_div3 + (s6_tau + s6_chi) / 2;
    println!("{}", s3 + s4 + s6);
}
