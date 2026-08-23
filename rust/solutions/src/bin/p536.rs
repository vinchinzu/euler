// Project Euler 536 - Modulo Power Identity
//
// Find sum of all m <= N such that a^{m+4} = a (mod m) for all a.
// Condition: m squarefree, lambda(m) | m+3.
//
// Recursively build squarefree m by multiplying primes, tracking
// carmichael = lcm(p_i - 1). CRT optimization when search space is small.
// CRT leaves are independent; collect them then scan in parallel.

use rayon::prelude::*;

const N: u64 = 1_000_000_000_000;
const SPF_LIMIT: usize = 100_000_001;
const CRT_CHUNK: u64 = 8192;

fn sieve_primes(limit: usize) -> Vec<u64> {
    let mut is_p = vec![true; limit + 1];
    is_p[0] = false;
    if limit >= 1 {
        is_p[1] = false;
    }
    let mut i = 2;
    while i * i <= limit {
        if is_p[i] {
            let mut j = i * i;
            while j <= limit {
                is_p[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    (2..=limit).filter(|&i| is_p[i]).map(|i| i as u64).collect()
}

/// Parallel SPF table. Each segment is initialized to identity then marked
/// with primes <= sqrt(limit) in increasing order, so the first hit is SPF.
fn sieve_spf_par(limit: usize) -> Vec<u32> {
    let mut spf = Vec::<u32>::with_capacity(limit + 1);
    // SAFETY: every slot is written before any read (head sequentially, tail
    // in disjoint par_chunks).
    unsafe {
        spf.set_len(limit + 1);
    }

    let s = (limit as f64).sqrt() as usize;
    let s = s.min(limit);

    for i in 0..=s {
        spf[i] = i as u32;
    }
    if limit >= 1 {
        spf[1] = 1;
    }

    let mut small_primes: Vec<usize> = Vec::new();
    for i in 2..=s {
        if spf[i] == i as u32 {
            small_primes.push(i);
            let mut j = i * i;
            while j <= s {
                if spf[j] == j as u32 {
                    spf[j] = i as u32;
                }
                j += i;
            }
        }
    }

    let start = s + 1;
    if start <= limit {
        let tail = &mut spf[start..=limit];
        const CHUNK: usize = 1 << 18;
        tail.par_chunks_mut(CHUNK).enumerate().for_each(|(ci, slice)| {
            let base = start + ci * CHUNK;
            let end = base + slice.len();
            for (k, v) in slice.iter_mut().enumerate() {
                *v = (base + k) as u32;
            }
            for &p in &small_primes {
                let p2 = p * p;
                let mut j = if base <= p2 {
                    p2
                } else {
                    let r = base % p;
                    if r == 0 { base } else { base + (p - r) }
                };
                while j < end {
                    let idx = j - base;
                    // SAFETY: j < end = base + slice.len()
                    unsafe {
                        let slot = slice.get_unchecked_mut(idx);
                        if *slot == j as u32 {
                            *slot = p as u32;
                        }
                    }
                    j += p;
                }
            }
        });
    }
    spf
}

#[inline]
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

#[inline]
fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

fn mod_inv(a: i64, m: i64) -> i64 {
    if m == 1 {
        return 0;
    }
    let (mut t, mut new_t) = (0i64, 1i64);
    let (mut r, mut new_r) = (m, a);
    while new_r != 0 {
        let q = r / new_r;
        let tmp_t = new_t;
        new_t = t - q * new_t;
        t = tmp_t;
        let tmp_r = new_r;
        new_r = r - q * new_r;
        r = tmp_r;
    }
    if t < 0 {
        t += m;
    }
    t
}

#[inline]
fn imod(a: i64, m: i64) -> i64 {
    ((a % m) + m) % m
}

/// Check if m*r satisfies conditions where r > 1 is the remaining factor.
/// CRT callers guarantee r < spf.len().
#[inline(always)]
fn good(m: u64, mut r: u64, max_p: u64, spf: &[u32]) -> bool {
    let m3 = m + 3;
    while r > 1 {
        // SAFETY: r decreases and starts < SPF_LIMIT == spf.len()
        let p = unsafe { *spf.get_unchecked(r as usize) } as u64;
        if m3 % (p - 1) != 0 {
            return false;
        }
        if p >= max_p {
            return false;
        }
        r /= p;
        if r % p == 0 {
            return false;
        }
    }
    true
}

struct CrtJob {
    m: u64,
    r0: u64,
    r_end: u64,
    modv: u64,
    max_p: u64,
}

fn collect(
    max_index: usize,
    m: u64,
    carmichael: u64,
    primes: &[u64],
    jobs: &mut Vec<CrtJob>,
    ans: &mut u64,
) {
    let g = gcd(m, carmichael);
    if 3 % g != 0 {
        return;
    }
    if (m + 3) % carmichael == 0 {
        *ans += m;
    }

    let nm = N / m;
    if nm < SPF_LIMIT as u64 && nm / carmichael < (1u64 << max_index.min(63)) {
        let modv = carmichael / g;
        if modv > 0 {
            let inv = mod_inv((m / g) as i64, modv as i64);
            let r_start = imod((-3i64 / g as i64) * inv, modv as i64) as u64;
            let max_p = if max_index < primes.len() {
                primes[max_index]
            } else {
                N + 1
            };
            let mut r = r_start;
            while r <= nm {
                let terms = (nm - r) / modv + 1;
                let take = terms.min(CRT_CHUNK);
                jobs.push(CrtJob {
                    m,
                    r0: r,
                    r_end: r + take * modv,
                    modv,
                    max_p,
                });
                r += take * modv;
            }
        }
        return;
    }

    for index in 0..max_index {
        let p = primes[index];
        if m > N / p {
            break;
        }
        collect(index, m * p, lcm(carmichael, p - 1), primes, jobs, ans);
    }
}

fn crt_job(job: &CrtJob, spf: &[u32]) -> u64 {
    let mut ans = 0u64;
    let m = job.m;
    let modv = job.modv;
    let max_p = job.max_p;
    let r_end = job.r_end;
    let mut r = job.r0;
    while r < r_end {
        if r > 1 && good(m * r, r, max_p, spf) {
            ans += m * r;
        }
        r += modv;
    }
    ans
}

fn main() {
    let sqrt_n = (N as f64).sqrt() as usize + 1;
    let primes = sieve_primes(sqrt_n);
    let spf = sieve_spf_par(SPF_LIMIT);

    let mut jobs = Vec::with_capacity(1 << 17);
    let mut ans = 0u64;
    collect(primes.len(), 1, 1, &primes, &mut jobs, &mut ans);

    ans += jobs.par_iter().map(|job| crt_job(job, &spf)).sum::<u64>();
    println!("{}", ans);
}
