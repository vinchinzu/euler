// Project Euler 786 - Billiard Ball Bounces
// Mobius function sieve and lattice point counting.
// Segmented parallel Möbius (primes to sqrt(g_limit) + leftover cofactor)
// fused with the lattice accumulation so the 187.5M μ array is never stored.

use rayon::prelude::*;

const BIG_N: i64 = 1_000_000_000;
const TAB3: [i64; 9] = [
    0, 1, 3,
    0, 2, 2,
    0, 0, 1,
];
const TAB9: [i64; 81] = [
    0, 4, 12, 15, 22, 24, 30, 31, 36,
    0, 5, 5, 9, 17, 20, 27, 29, 35,
    0, 6, 7, 12, 12, 16, 24, 27, 34,
    0, 7, 9, 15, 16, 21, 21, 25, 33,
    0, 8, 11, 18, 20, 26, 27, 32, 32,
    0, 0, 4, 12, 15, 22, 24, 30, 31,
    0, 1, 6, 6, 10, 18, 21, 28, 30,
    0, 2, 8, 9, 14, 14, 18, 26, 29,
    0, 3, 10, 12, 18, 19, 24, 24, 28,
];

#[inline(always)]
fn lattice_count_3(t: i64) -> i64 {
    if t < 8 {
        return 0;
    }
    let n = (t - 3) / 5;
    let sum_y = n * t - 5 * n * (n + 1) / 2;
    let q = n / 3;
    let r = (n % 3) as usize;
    let tm = (t % 3) as usize;
    let sum_mod = q * 3 + TAB3[tm * 3 + r];
    (sum_y - sum_mod) / 3
}

#[inline(always)]
fn lattice_count_9(t: i64) -> i64 {
    if t < 14 {
        return 0;
    }
    let n = (t - 9) / 5;
    let sum_y = n * t - 5 * n * (n + 1) / 2;
    let q = n / 9;
    let r = (n % 9) as usize;
    let tm = (t % 9) as usize;
    let sum_mod = q * 36 + TAB9[tm * 9 + r];
    (sum_y - sum_mod) / 9
}

fn sieve_primes(limit: usize) -> Vec<u32> {
    if limit < 2 {
        return Vec::new();
    }
    let mut is_comp = vec![false; limit + 1];
    let mut primes = Vec::with_capacity(limit / 10);
    for i in 2..=limit {
        if !is_comp[i] {
            primes.push(i as u32);
            let mut j = i * i;
            while j <= limit {
                is_comp[j] = true;
                j += i;
            }
        }
    }
    primes
}

fn main() {
    let l = (3 * BIG_N + 5) / 2;
    // For d=3 (g % 3 == 0), t >= 8 => g <= l / 8.
    // For d=9 (g % 3 != 0), t >= 14 => g <= l / 14.
    let g_limit = (l / 8) as usize;
    let g_lim_9 = (l / 14) as usize;

    let sqrt_l = (g_limit as u64).isqrt() as usize;
    let primes = sieve_primes(sqrt_l);

    const SEG: usize = 1 << 20;
    let n_seg = (g_limit + SEG) / SEG; // covers 0..=g_limit

    let ans: i64 = (0..n_seg)
        .into_par_iter()
        .map(|si| {
            let lo = si * SEG;
            let hi = (lo + SEG).min(g_limit + 1);
            if hi <= 1 {
                return 0i64;
            }
            let len = hi - lo;
            let mut rem = vec![0u32; len];
            let mut mu = vec![1i8; len];
            for i in 0..len {
                rem[i] = (lo + i) as u32;
            }
            if lo == 0 {
                mu[0] = 0;
                rem[0] = 1;
            }

            for &p_u in &primes {
                let p = p_u as usize;
                let start = if lo <= p {
                    p
                } else {
                    let r = lo % p;
                    if r == 0 { lo } else { lo + (p - r) }
                };
                let mut j = start;
                while j < hi {
                    let idx = j - lo;
                    unsafe {
                        *rem.get_unchecked_mut(idx) /= p_u;
                        *mu.get_unchecked_mut(idx) = -*mu.get_unchecked(idx);
                    }
                    j += p;
                }
                let p2 = p.saturating_mul(p);
                if p2 != 0 && p2 < hi {
                    let start2 = if lo <= p2 {
                        p2
                    } else {
                        let r = lo % p2;
                        if r == 0 { lo } else { lo + (p2 - r) }
                    };
                    let mut j = start2;
                    while j < hi {
                        unsafe {
                            *mu.get_unchecked_mut(j - lo) = 0;
                        }
                        j += p2;
                    }
                }
            }

            let mut local = 0i64;
            let start_n = if lo < 1 { 1 } else { lo };
            for n in start_n..hi {
                let idx = n - lo;
                let mut m = unsafe { *mu.get_unchecked(idx) } as i64;
                if m == 0 {
                    continue;
                }
                if unsafe { *rem.get_unchecked(idx) } > 1 {
                    m = -m;
                }
                if n > g_lim_9 && n % 3 != 0 {
                    continue;
                }
                let t = l / n as i64;
                let count = if n % 3 == 0 {
                    lattice_count_3(t)
                } else {
                    lattice_count_9(t)
                };
                local += m * count;
            }
            local
        })
        .sum();

    let ans = ans * 4 + 2;
    println!("{}", ans);
}
