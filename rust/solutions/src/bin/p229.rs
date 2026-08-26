// Project Euler 229 - Four Representations using Squares
//
// n has all four representations a^2+k b^2 (k=1,2,3,7, a,b>0) iff n = s^2 * q
// with q square-free, q>1, and every prime of q ≡ 1,25,121 (mod 168); plus those
// squares m^2 whose primes jointly split for all four forms.

use rayon::prelude::*;

const N: u64 = 2_000_000_000;
const WHEEL: u64 = 168;
const RES: [u64; 3] = [1, 25, 121];
const PMIN: u64 = 193;
const CAND_PER_RES: u64 = 32_768;
const SPAN: u64 = WHEEL * CAND_PER_RES;

struct WheelPrime {
    p: u32,
    /// Least n = 168k+RES[i] divisible by p.
    base: [u32; 3],
}

fn main() {
    println!("{}", solve(N));
}

fn solve(n: u64) -> u64 {
    if n < PMIN {
        return 0;
    }
    let sqrt_n = n.isqrt();
    let small = primes_upto(sqrt_n as usize);
    let sieve_ps: Vec<u32> = small
        .iter()
        .copied()
        .filter(|&p| p != 2 && p != 3 && p != 7)
        .collect();
    let wheel = make_wheel_primes(&sieve_ps);

    let store_lim = n / PMIN;
    let specials = special_primes_upto(store_lim, &wheel);
    let mut total = count_nonsquare(n, &specials);

    let start = store_lim + 1;
    if start <= n {
        let n_seg = (n - start + 1).div_ceil(SPAN) as usize;
        total += (0..n_seg)
            .into_par_iter()
            .map(|si| {
                let low = start + si as u64 * SPAN;
                let high = (low + SPAN).min(n + 1);
                segment_special_sum(low, high, n, &wheel)
            })
            .sum::<u64>();
    }

    total + count_good_squares(n)
}

/// Numbers s^2 * q ≤ n with q>1 square-free on special primes.
fn count_nonsquare(n: u64, ps: &[u32]) -> u64 {
    let np = ps.len();
    let mut total = 0u64;

    for &p in ps {
        total += (n / p as u64).isqrt();
    }

    for i in 0..np {
        let p = ps[i] as u64;
        if p > n / p {
            break;
        }
        let maxq = n / p;
        let end = ps.partition_point(|&q| (q as u64) <= maxq);
        if end <= i + 1 {
            continue;
        }
        for &q in &ps[i + 1..end] {
            total += (n / (p * q as u64)).isqrt();
        }
    }

    for i in 0..np {
        let p = ps[i] as u64;
        let Some(p3) = p.checked_mul(p).and_then(|x| x.checked_mul(p)) else {
            break;
        };
        if p3 > n {
            break;
        }
        for j in i + 1..np {
            let q = ps[j] as u64;
            let Some(pq) = p.checked_mul(q) else {
                break;
            };
            if pq > n {
                break;
            }
            let maxr = n / pq;
            let end = ps.partition_point(|&r| (r as u64) <= maxr);
            if end <= j + 1 {
                continue;
            }
            for &r in &ps[j + 1..end] {
                total += (n / (pq * r as u64)).isqrt();
            }
        }
    }

    for i in 0..np {
        let p = ps[i] as u64;
        let Some(p4) = p
            .checked_mul(p)
            .and_then(|x| x.checked_mul(p))
            .and_then(|x| x.checked_mul(p))
        else {
            break;
        };
        if p4 > n {
            break;
        }
        for j in i + 1..np {
            let q = ps[j] as u64;
            let Some(pq) = p.checked_mul(q) else {
                break;
            };
            for k in j + 1..np {
                let r = ps[k] as u64;
                let Some(pqr) = pq.checked_mul(r) else {
                    break;
                };
                if pqr > n {
                    break;
                }
                let maxs = n / pqr;
                let end = ps.partition_point(|&s| (s as u64) <= maxs);
                if end <= k + 1 {
                    continue;
                }
                for &s in &ps[k + 1..end] {
                    total += (n / (pqr * s as u64)).isqrt();
                }
            }
        }
    }

    total
}

/// m^2 works iff m's primes cover all four splitting conditions:
/// k=1: p≡1 (mod 4); k=2: p≡1,3 (mod 8);
/// k=3: even or p≡1 (mod 3); k=7: v2≥2 or odd p≡1,2,4 (mod 7).
fn count_good_squares(n: u64) -> u64 {
    let m_max = n.isqrt() as usize;
    let mut spf = vec![0u32; m_max + 1];
    for i in 2..=m_max {
        if spf[i] == 0 {
            let mut j = i;
            while j <= m_max {
                if spf[j] == 0 {
                    spf[j] = i as u32;
                }
                j += i;
            }
        }
    }
    let mut cnt = 0u64;
    for m in 1..=m_max {
        let mut c1 = false;
        let mut c2 = false;
        let mut c3 = false;
        let mut c7 = false;
        let mut x = m as u32;
        while x > 1 {
            let p = spf[x as usize];
            let mut e = 0u32;
            while x % p == 0 {
                x /= p;
                e += 1;
            }
            if p == 2 {
                c3 = true;
                if e >= 2 {
                    c7 = true;
                }
            } else {
                if p % 4 == 1 {
                    c1 = true;
                }
                let r8 = p & 7;
                if r8 == 1 || r8 == 3 {
                    c2 = true;
                }
                if p % 3 == 1 {
                    c3 = true;
                }
                let r7 = p % 7;
                if r7 == 1 || r7 == 2 || r7 == 4 {
                    c7 = true;
                }
            }
            if c1 && c2 && c3 && c7 {
                break;
            }
        }
        if c1 && c2 && c3 && c7 {
            cnt += 1;
        }
    }
    cnt
}

fn primes_upto(limit: usize) -> Vec<u32> {
    if limit < 2 {
        return Vec::new();
    }
    let mut comp = vec![false; limit + 1];
    let mut ps = Vec::with_capacity(limit / 10);
    let mut i = 2usize;
    while i * i <= limit {
        if !comp[i] {
            ps.push(i as u32);
            let mut j = i * i;
            while j <= limit {
                comp[j] = true;
                j += i;
            }
        }
        i += 1;
    }
    while i <= limit {
        if !comp[i] {
            ps.push(i as u32);
        }
        i += 1;
    }
    ps
}

fn modinv(a: u64, m: u64) -> u64 {
    let mut t = 0i64;
    let mut newt = 1i64;
    let mut r = m as i64;
    let mut newr = (a % m) as i64;
    while newr != 0 {
        let q = r / newr;
        (t, newt) = (newt, t - q * newt);
        (r, newr) = (newr, r - q * newr);
    }
    if t < 0 {
        t += m as i64;
    }
    t as u64
}

fn make_wheel_primes(ps: &[u32]) -> Vec<WheelPrime> {
    ps.iter()
        .map(|&p| {
            let p64 = p as u64;
            let inv = modinv(WHEEL % p64, p64);
            let mut base = [0u32; 3];
            for (i, &r) in RES.iter().enumerate() {
                let k = ((p64 - r % p64) % p64) * inv % p64;
                base[i] = (WHEEL * k + r) as u32;
            }
            WheelPrime { p, base }
        })
        .collect()
}

fn first_hit_pre(wp: &WheelPrime, ri: usize, start: u64) -> u64 {
    let stride = WHEEL * wp.p as u64;
    let mut n = wp.base[ri] as u64;
    if n < start {
        n += (start - n).div_ceil(stride) * stride;
    }
    n
}

fn mark_ap(comp: &mut [u8], first: u64, high: u64, start: u64, wp: &WheelPrime, ri: usize) {
    let n0 = first_hit_pre(wp, ri, start.max(first));
    if n0 >= high {
        return;
    }
    let mut idx = ((n0 - first) / WHEEL) as usize;
    let step = wp.p as usize;
    let n_cand = comp.len();
    // SAFETY: idx starts < n_cand (n0 < high ⇒ idx in range) and grows by step.
    while idx < n_cand {
        unsafe {
            *comp.get_unchecked_mut(idx) = 1;
        }
        idx += step;
    }
}

fn collect_ap(comp: &[u8], first: u64, limit: u64, out: &mut Vec<u32>) {
    for (i, &c) in comp.iter().enumerate() {
        if c == 0 {
            let pr = first + i as u64 * WHEEL;
            if pr <= limit {
                out.push(pr as u32);
            }
        }
    }
}

fn special_primes_upto(limit: u64, wheel: &[WheelPrime]) -> Vec<u32> {
    if limit < PMIN {
        return Vec::new();
    }
    let mut out = Vec::with_capacity((limit as usize / 16) / 10 + 8);
    for (ri, &r) in RES.iter().enumerate() {
        let mut first = r;
        if first < PMIN {
            first += WHEEL * (PMIN - first).div_ceil(WHEEL);
        }
        if first > limit {
            continue;
        }
        let n_cand = ((limit - first) / WHEEL + 1) as usize;
        let mut comp = vec![0u8; n_cand];
        for wp in wheel {
            let p = wp.p as u64;
            let pp = p.saturating_mul(p);
            if pp > limit {
                break;
            }
            mark_ap(&mut comp, first, limit + 1, pp, wp, ri);
        }
        collect_ap(&comp, first, limit, &mut out);
    }
    out.sort_unstable();
    out
}

fn segment_special_sum(low: u64, high: u64, n: u64, wheel: &[WheelPrime]) -> u64 {
    let mut sum = 0u64;
    for (ri, &r) in RES.iter().enumerate() {
        let rem = low % WHEEL;
        let mut first = if rem <= r {
            low + (r - rem)
        } else {
            low + (WHEEL - rem + r)
        };
        if first < PMIN {
            first += WHEEL * (PMIN - first).div_ceil(WHEEL);
        }
        if first >= high {
            continue;
        }
        let n_cand = ((high - 1 - first) / WHEEL + 1) as usize;
        let mut comp = vec![0u8; n_cand];
        for wp in wheel {
            let p = wp.p as u64;
            let pp = p.saturating_mul(p);
            if pp >= high {
                break;
            }
            mark_ap(&mut comp, first, high, low.max(pp), wp, ri);
        }
        for i in 0..n_cand {
            if unsafe { *comp.get_unchecked(i) } == 0 {
                let pr = first + i as u64 * WHEEL;
                if pr <= n {
                    sum += (n / pr).isqrt();
                }
            }
        }
    }
    sum
}
