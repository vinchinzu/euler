// Project Euler 354: Honeycomb distance distribution
//
// B(L)=450 iff R(n)=75 for n=L^2/3, R multiplicative via primes ≡ 1 (mod 3).
// Patterns from 75: (24,2), (14,4), (4,4,2). Remainder 3^a * b^2 with
// primes of b all ≡ 2 (mod 3). Heavy r-sum uses Dirichlet groups on C/r^2.

use rayon::prelude::*;

const LMAX: u128 = 500_000_000_000;

fn sieve_primes(limit: usize) -> Vec<u32> {
    if limit < 2 {
        return Vec::new();
    }
    // index i -> odd number 2*i+1
    let sz = (limit + 1) / 2;
    let mut comp = vec![0u8; sz];
    comp[0] = 1;
    let mut i = 1usize;
    loop {
        let p = 2 * i + 1;
        if p * p > limit {
            break;
        }
        // SAFETY: i < sz by construction of the odd-index loop.
        if unsafe { *comp.get_unchecked(i) } == 0 {
            let mut j = (p * p) / 2;
            while j < sz {
                // SAFETY: j < sz from the loop bound.
                unsafe { *comp.get_unchecked_mut(j) = 1 };
                j += p;
            }
        }
        i += 1;
    }
    let mut primes = Vec::with_capacity(sz / 5 + 8);
    primes.push(2);
    for i in 1..sz {
        // SAFETY: i in 1..sz.
        if unsafe { *comp.get_unchecked(i) } == 0 {
            let p = 2 * i + 1;
            if p > limit {
                break;
            }
            primes.push(p as u32);
        }
    }
    primes
}

#[inline(always)]
fn count_primes1(primes1: &[u32], lo: u32, hi: u32) -> i32 {
    let r = primes1.partition_point(|&p| p <= hi);
    let l = primes1.partition_point(|&p| p < lo);
    (r - l) as i32
}

/// #{ b <= sqrt(m / 3^a) : primes of b are all ≡ 2 (mod 3) }, summed over a >= 0.
#[inline(always)]
fn g(mut m: u64, prefix: &[i32]) -> i64 {
    let bmax = prefix.len() - 1;
    let bmax2 = bmax as u64 * bmax as u64;
    let mut total = 0i64;
    while m > bmax2 {
        // SAFETY: bmax = prefix.len() - 1.
        total += unsafe { *prefix.get_unchecked(bmax) } as i64;
        m /= 3;
    }
    while m != 0 {
        let s = m.isqrt() as usize;
        // SAFETY: m <= bmax^2 so s <= bmax.
        total += unsafe { *prefix.get_unchecked(s) } as i64;
        m /= 3;
    }
    total
}

/// Sum_r G(C / r^2) over primes r ≡ 1 (mod 3), r != excl1, excl2.
fn sum_over_r(c: u64, excl1: u32, excl2: u32, primes1: &[u32], prefix: &[i32]) -> i64 {
    let rmax = c.isqrt();
    if rmax < 7 {
        return 0;
    }
    let last = *primes1.last().unwrap() as u64;
    let rmax = rmax.min(last);
    let mut r_low = 7u64;
    let mut total = 0i64;
    while r_low <= rmax {
        let t = c / (r_low * r_low);
        if t == 0 {
            break;
        }
        let mut r_high = (c / t).isqrt();
        if r_high > rmax {
            r_high = rmax;
        }
        if r_high < r_low {
            break;
        }
        let mut cnt = count_primes1(primes1, r_low as u32, r_high as u32);
        if r_low <= excl1 as u64 && (excl1 as u64) <= r_high {
            cnt -= 1;
        }
        if excl2 != excl1 && r_low <= excl2 as u64 && (excl2 as u64) <= r_high {
            cnt -= 1;
        }
        if cnt > 0 {
            total += cnt as i64 * g(t, prefix);
        }
        r_low = r_high + 1;
    }
    total
}

fn main() {
    let n: u128 = LMAX * LMAX / 3;

    let p7_4 = 7u128.pow(4);
    let p13_4 = 13u128.pow(4);
    let p19_2 = 19u128.pow(2);
    let max_r = (n / (p7_4 * p13_4)).isqrt() as usize;
    let bmax = (n / (p7_4 * p13_4 * p19_2)).isqrt() as usize;

    let primes = sieve_primes(max_r);
    let primes1: Vec<u32> = primes.iter().copied().filter(|&p| p % 3 == 1).collect();

    // prefix[x] = #{ b <= x : every prime factor of b is ≡ 2 (mod 3) } (includes 1).
    let mut good = vec![1u8; bmax + 1];
    good[0] = 0;
    for &p in &primes {
        let pu = p as usize;
        if pu > bmax {
            break;
        }
        if pu == 3 || p % 3 == 1 {
            let mut j = pu;
            while j <= bmax {
                // SAFETY: j <= bmax = good.len() - 1.
                unsafe { *good.get_unchecked_mut(j) = 0 };
                j += pu;
            }
        }
    }
    let mut prefix = vec![0i32; bmax + 1];
    let mut acc = 0i32;
    for i in 1..=bmax {
        // SAFETY: i in 1..=bmax, both arrays have length bmax+1.
        acc += unsafe { *good.get_unchecked(i) } as i32;
        unsafe { *prefix.get_unchecked_mut(i) = acc };
    }

    let mut ans = 0i64;

    // Pattern (24, 2): 7^24 * q^2, q ≡ 1 (mod 3), q != 7.
    {
        let p24 = 7u128.pow(24);
        if p24 <= n {
            let max_q2 = n / p24;
            for &q in &primes1 {
                if q == 7 {
                    continue;
                }
                let q2 = q as u128 * q as u128;
                if q2 > max_q2 {
                    break;
                }
                ans += g((n / (p24 * q2)) as u64, &prefix);
            }
        }
    }

    // Pattern (14, 4): p^14 * q^4, p != q.
    for &p in &primes1 {
        let Some(p14) = (p as u128).checked_pow(14) else { break };
        if p14 > n {
            break;
        }
        let max_q4 = n / p14;
        for &q in &primes1 {
            if q == p {
                continue;
            }
            let q4 = (q as u128).pow(4);
            if q4 > max_q4 {
                break;
            }
            ans += g((n / p14 / q4) as u64, &prefix);
        }
    }

    // Pattern (4, 4, 2): p^4 * q^4 * r^2 with p < q, r distinct.
    let mut pairs: Vec<(u32, u32, u64)> = Vec::new();
    for (i, &p) in primes1.iter().enumerate() {
        let p4 = (p as u128).pow(4);
        let mut any = false;
        for &q in &primes1[i + 1..] {
            let q4 = (q as u128).pow(4);
            let Some(base) = p4.checked_mul(q4) else { break };
            if base > n {
                break;
            }
            pairs.push((p, q, (n / base) as u64));
            any = true;
        }
        if !any {
            break;
        }
    }

    ans += pairs
        .par_iter()
        .map(|&(p, q, c)| sum_over_r(c, p, q, &primes1, &prefix))
        .sum::<i64>();

    println!("{ans}");
}
