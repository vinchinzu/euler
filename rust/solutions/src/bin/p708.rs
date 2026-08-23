// Project Euler 708 - Twos Are All You Need
//
// S(N) = sum_{n<=N} 2^{Omega(n)} = sum_{powerful d} 2^{Omega(d)-2 omega(d)} * D(N/d)
// D(n) = sum_{k<=n} tau(k) = sum_{i=1}^n floor(n/i).

use rayon::prelude::*;

const BIG_N: i64 = 100_000_000_000_000; // 10^14
const PAR_Q: i64 = 100_000_000; // parallelize children only for large remaining N/d
const PAR_P: i64 = 20_000; // do not rayon tiny large-prime iterations
const FLOOR_CHUNK: i64 = 262_144;
const TAU_CHUNK: usize = 524_288;

#[inline]
fn isqrt(n: i64) -> i64 {
    if n <= 0 {
        return 0;
    }
    let mut x = (n as f64).sqrt() as i64;
    while x > 0 && x * x > n {
        x -= 1;
    }
    while (x + 1) * (x + 1) <= n {
        x += 1;
    }
    x
}

/// sum_{i=lo..=hi} n/i with independent divisions unrolled.
#[inline]
fn sum_floor_range(n: i64, lo: i64, hi: i64) -> i64 {
    let mut acc = 0i64;
    let mut i = lo;
    while i + 7 <= hi {
        acc += n / i;
        acc += n / (i + 1);
        acc += n / (i + 2);
        acc += n / (i + 3);
        acc += n / (i + 4);
        acc += n / (i + 5);
        acc += n / (i + 6);
        acc += n / (i + 7);
        i += 8;
    }
    while i <= hi {
        acc += n / i;
        i += 1;
    }
    acc
}

struct Ctx<'a> {
    primes: &'a [i32],
    n: i64,
    l: i64,
    small: &'a [i32],
    qs: &'a [i64],
    large: &'a [i64],
}

impl<'a> Ctx<'a> {
    #[inline]
    fn dsum(&self, q: i64) -> i64 {
        if q <= self.l {
            // SAFETY: q >= 1 and q <= l; small has length l+1
            unsafe { *self.small.get_unchecked(q as usize) as i64 }
        } else {
            let i = self.qs.binary_search(&q).unwrap();
            // SAFETY: binary_search succeeded
            unsafe { *self.large.get_unchecked(i) }
        }
    }
}

fn sieve_primes(limit: usize) -> Vec<i32> {
    let mut comp = vec![0u8; limit + 1];
    let sq = isqrt(limit as i64) as usize;
    let mut small: Vec<usize> = Vec::new();
    for i in 2..=sq {
        if comp[i] == 0 {
            small.push(i);
            let mut j = i * i;
            while j <= sq {
                comp[j] = 1;
                j += i;
            }
        }
    }
    if sq + 1 <= limit {
        let tail = &mut comp[sq + 1..=limit];
        const CHUNK: usize = 1 << 18;
        tail.par_chunks_mut(CHUNK).enumerate().for_each(|(ci, slice)| {
            let start = sq + 1 + ci * CHUNK;
            let end = start + slice.len();
            for &p in &small {
                let p2 = p * p;
                let mut j = ((start + p - 1) / p) * p;
                if j < p2 {
                    j = p2;
                }
                while j < end {
                    unsafe {
                        *slice.get_unchecked_mut(j - start) = 1;
                    }
                    j += p;
                }
            }
        });
    }
    let mut primes = Vec::with_capacity(limit / 10);
    for i in 2..=limit {
        if comp[i] == 0 {
            primes.push(i as i32);
        }
    }
    primes
}

fn prefix_tau(limit: usize) -> Vec<i32> {
    let mut num_div = vec![0u16; limit + 1];
    {
        let tail = &mut num_div[1..=limit];
        tail.par_chunks_mut(TAU_CHUNK).enumerate().for_each(|(ci, slice)| {
            let start = 1 + ci * TAU_CHUNK;
            let end = start + slice.len();
            for i in 1..end {
                let mut j = ((start + i - 1) / i) * i;
                if j < start {
                    j += i;
                }
                while j < end {
                    // SAFETY: j in [start, end)
                    unsafe {
                        *slice.get_unchecked_mut(j - start) += 1;
                    }
                    j += i;
                }
            }
        });
    }
    let mut sum = vec![0i32; limit + 1];
    let mut acc = 0i32;
    for i in 1..=limit {
        acc += unsafe { *num_div.get_unchecked(i) as i32 };
        unsafe {
            *sum.get_unchecked_mut(i) = acc;
        }
    }
    sum
}

fn collect_qs(min_index: usize, d: i64, ctx: &Ctx, out: &mut Vec<i64>) {
    let q = ctx.n / d;
    if q <= ctx.l {
        return;
    }
    out.push(q);
    let primes = ctx.primes;
    for index in min_index..primes.len() {
        let p = primes[index] as i64;
        if p > q / p {
            break;
        }
        let mut new_d = d * p;
        while new_d <= ctx.n / p {
            new_d *= p;
            collect_qs(index + 1, new_d, ctx, out);
        }
    }
}

/// Load-balanced D(q) for every distinct q = N/d with q > sqrt(N).
fn compute_large_d(qs: &[i64]) -> Vec<i64> {
    if qs.is_empty() {
        return Vec::new();
    }
    struct Job {
        idx: u32,
        n: i64,
        lo: i64,
        hi: i64,
    }
    let mut jobs: Vec<Job> = Vec::with_capacity(qs.len() * 2);
    let mut sqs = vec![0i64; qs.len()];
    for (idx, &q) in qs.iter().enumerate() {
        let s = isqrt(q);
        sqs[idx] = s;
        let mut lo = 1i64;
        while lo <= s {
            let hi = (lo + FLOOR_CHUNK - 1).min(s);
            jobs.push(Job {
                idx: idx as u32,
                n: q,
                lo,
                hi,
            });
            lo = hi + 1;
        }
    }
    let contrib: Vec<i64> = jobs
        .par_iter()
        .with_min_len(1)
        .map(|j| sum_floor_range(j.n, j.lo, j.hi))
        .collect();
    let mut sums = vec![0i64; qs.len()];
    for (j, acc) in jobs.iter().zip(contrib.iter()) {
        sums[j.idx as usize] += *acc;
    }
    sums.into_iter()
        .zip(sqs.into_iter())
        .map(|(s, sq)| 2 * s - sq * sq)
        .collect()
}

#[inline]
fn expand_prime(index: usize, d: i64, mult: i64, ctx: &Ctx, parallel: bool) -> i64 {
    let p = ctx.primes[index] as i64;
    let mut local = 0i64;
    let mut new_d = d * p;
    let mut e = 2;
    while new_d <= ctx.n / p {
        new_d *= p;
        local += helper(index + 1, new_d, mult << (e - 2), ctx, parallel);
        e += 1;
    }
    local
}

fn seq_primes(min_index: usize, hi: usize, d: i64, mult: i64, ctx: &Ctx) -> i64 {
    let mut ans = 0i64;
    for index in min_index..hi {
        ans += expand_prime(index, d, mult, ctx, false);
    }
    ans
}

fn par_primes(lo: usize, hi: usize, d: i64, mult: i64, ctx: &Ctx) -> i64 {
    if hi - lo <= 1 {
        if lo < hi {
            expand_prime(lo, d, mult, ctx, true)
        } else {
            0
        }
    } else {
        let mid = lo + (hi - lo) / 2;
        let (a, b) = rayon::join(
            || par_primes(lo, mid, d, mult, ctx),
            || par_primes(mid, hi, d, mult, ctx),
        );
        a + b
    }
}

fn helper(min_index: usize, d: i64, mult: i64, ctx: &Ctx, parallel: bool) -> i64 {
    let q = ctx.n / d;
    let mut ans = ctx.dsum(q) * mult;
    let primes = ctx.primes;
    let plen = primes.len();
    if min_index >= plen {
        return ans;
    }

    if parallel && q >= PAR_Q {
        let mut hi = min_index;
        while hi < plen {
            let p = unsafe { *primes.get_unchecked(hi) } as i64;
            if p > q / p {
                break;
            }
            hi += 1;
        }
        let mut par_hi = min_index;
        while par_hi < hi && (unsafe { *primes.get_unchecked(par_hi) } as i64) < PAR_P {
            par_hi += 1;
        }
        if par_hi - min_index >= 2 {
            ans += par_primes(min_index, par_hi, d, mult, ctx);
            ans += seq_primes(par_hi, hi, d, mult, ctx);
        } else {
            ans += seq_primes(min_index, hi, d, mult, ctx);
        }
    } else {
        for index in min_index..plen {
            let p = unsafe { *primes.get_unchecked(index) } as i64;
            if p > q / p {
                break;
            }
            ans += expand_prime(index, d, mult, ctx, false);
        }
    }
    ans
}

fn main() {
    let l = isqrt(BIG_N);
    let limit = l as usize;

    let (primes, small) = rayon::join(|| sieve_primes(limit), || prefix_tau(limit));

    let mut ctx = Ctx {
        primes: &primes,
        n: BIG_N,
        l,
        small: &small,
        qs: &[],
        large: &[],
    };

    let mut qs = Vec::with_capacity(8192);
    collect_qs(0, 1, &ctx, &mut qs);
    qs.sort_unstable();
    qs.dedup();
    let large = compute_large_d(&qs);

    ctx.qs = &qs;
    ctx.large = &large;

    let ans = helper(0, 1, 1, &ctx, true);
    println!("{}", ans);
}
