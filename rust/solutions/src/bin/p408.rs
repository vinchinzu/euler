// Project Euler 408: Admissible paths through a grid
// Count paths from (0,0) to (N,N) that avoid inadmissible points.
// Inadmissible points are (a^2, b^2) where a^2+b^2 is a perfect square.

use rayon::prelude::*;

const N: usize = 10_000_000;
const MOD: u64 = 1_000_000_007;
const MAX_FACT: usize = 2 * N + 1;
const CHUNK: usize = 1 << 16;

#[inline(always)]
fn mul(a: u64, b: u64) -> u64 {
    a * b % MOD
}

fn pow_mod(mut base: u64, mut exp: u64) -> u64 {
    let mut r = 1u64;
    while exp > 0 {
        if exp & 1 != 0 {
            r = r * base % MOD;
        }
        base = base * base % MOD;
        exp >>= 1;
    }
    r
}

fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

/// fact[i] = i! mod MOD, filled with a parallel product-scan.
fn fill_fact(fact: &mut [u32]) {
    let n = fact.len();
    let nchunks = n.div_ceil(CHUNK);
    let mut prods = vec![1u64; nchunks];
    prods.par_iter_mut().enumerate().for_each(|(ci, p)| {
        let lo = ci * CHUNK;
        let hi = ((ci + 1) * CHUNK).min(n);
        let mut acc = 1u64;
        for i in lo.max(1)..hi {
            acc = acc * i as u64 % MOD;
        }
        *p = acc;
    });
    let mut start = vec![1u64; nchunks];
    for ci in 1..nchunks {
        start[ci] = mul(start[ci - 1], prods[ci - 1]);
    }
    fact.par_chunks_mut(CHUNK).enumerate().for_each(|(ci, slice)| {
        let lo = ci * CHUNK;
        let mut f = start[ci];
        for (off, cell) in slice.iter_mut().enumerate() {
            let i = lo + off;
            if i == 0 {
                *cell = 1;
            } else {
                f = f * i as u64 % MOD;
                *cell = f as u32;
            }
        }
    });
}

/// inv_fact[i] = (i!)^{-1} mod MOD, reverse parallel product-scan.
fn fill_inv_fact(inv_fact: &mut [u32], inv_last: u64) {
    let n = inv_fact.len();
    let nchunks = n.div_ceil(CHUNK);
    let mut prods = vec![1u64; nchunks];
    prods.par_iter_mut().enumerate().for_each(|(ci, p)| {
        let lo = ci * CHUNK;
        let hi = ((ci + 1) * CHUNK).min(n);
        let from = if hi == n { n - 1 } else { hi };
        let mut acc = 1u64;
        for m in (lo + 1)..=from {
            acc = acc * m as u64 % MOD;
        }
        *p = acc;
    });
    let mut right = vec![1u64; nchunks];
    right[nchunks - 1] = inv_last;
    for ci in (0..nchunks - 1).rev() {
        right[ci] = mul(right[ci + 1], prods[ci + 1]);
    }
    inv_fact.par_chunks_mut(CHUNK).enumerate().for_each(|(ci, slice)| {
        let lo = ci * CHUNK;
        let hi = lo + slice.len();
        if hi == n {
            let mut cur = inv_last;
            let last_off = slice.len() - 1;
            slice[last_off] = cur as u32;
            for off in (0..last_off).rev() {
                let i = lo + off;
                cur = cur * (i + 1) as u64 % MOD;
                slice[off] = cur as u32;
            }
        } else {
            let mut cur = right[ci];
            for off in (0..slice.len()).rev() {
                let i = lo + off;
                cur = cur * (i + 1) as u64 % MOD;
                slice[off] = cur as u32;
            }
        }
    });
}

fn gen_points() -> Vec<(u32, u32)> {
    let sq_limit = (N as u32).isqrt() as u64;
    let m_limit = (4 * sq_limit as u32).isqrt() as usize + 1;
    let kmax = 4 * sq_limit;
    let nu = N as u64;

    (2..=m_limit)
        .into_par_iter()
        .flat_map_iter(|m| {
            let mut local = Vec::new();
            for n in 1..m {
                if (m + n) & 1 == 0 || gcd(m as u32, n as u32) != 1 {
                    continue;
                }
                let a = (m * m - n * n) as u64;
                let b = (2 * m * n) as u64;
                let c = (m * m + n * n) as u64;
                let mut k = 1u64;
                while k * c <= kmax {
                    let ax = (k * a) * (k * a);
                    let bx = (k * b) * (k * b);
                    if ax <= nu && bx <= nu {
                        local.push((ax as u32, bx as u32));
                        if ax != bx {
                            local.push((bx as u32, ax as u32));
                        }
                    }
                    k += 1;
                }
            }
            local
        })
        .collect()
}

#[inline(always)]
fn ncr(fact: &[u32], inv: &[u32], n: usize, r: usize) -> u64 {
    // SAFETY: n < MAX_FACT, r <= n, n-r < MAX_FACT by construction of queries.
    unsafe {
        let a = *fact.get_unchecked(n) as u64;
        let b = *inv.get_unchecked(r) as u64;
        let c = *inv.get_unchecked(n - r) as u64;
        a * b % MOD * c % MOD
    }
}

fn sub_range(
    qi_lo: usize,
    qi_hi: usize,
    px: u32,
    py: u32,
    xs: &[u32],
    ys: &[u32],
    adm: &[u32],
    fact: &[u32],
    inv: &[u32],
) -> u64 {
    let mut acc = 0u64;
    let mut hits = 0u32;
    for qi in qi_lo..qi_hi {
        // Sorted by x then y, so xs[qi] <= px for all qi < pi.
        // SAFETY: qi_lo..qi_hi ⊆ 0..pi ⊆ xs/ys/adm lengths.
        let qy = unsafe { *ys.get_unchecked(qi) };
        if qy <= py {
            let dx = (px - unsafe { *xs.get_unchecked(qi) }) as usize;
            let dy = (py - qy) as usize;
            let ways = ncr(fact, inv, dx + dy, dx);
            acc = acc.wrapping_add(unsafe { *adm.get_unchecked(qi) } as u64 * ways);
            hits += 1;
            if hits == 16 {
                acc %= MOD;
                hits = 0;
            }
        }
    }
    acc % MOD
}

fn main() {
    let (fact, mut pts) = rayon::join(
        || {
            let mut fact = vec![0u32; MAX_FACT];
            fill_fact(&mut fact);
            fact
        },
        gen_points,
    );

    let inv_last = pow_mod(fact[MAX_FACT - 1] as u64, MOD - 2);
    let (inv_fact, _) = rayon::join(
        || {
            let mut inv_fact = vec![0u32; MAX_FACT];
            fill_inv_fact(&mut inv_fact, inv_last);
            inv_fact
        },
        || {
            pts.sort_unstable();
            pts.dedup();
            pts.push((N as u32, N as u32));
        },
    );

    let npts = pts.len();
    let mut xs = vec![0u32; npts];
    let mut ys = vec![0u32; npts];
    for (i, &(x, y)) in pts.iter().enumerate() {
        xs[i] = x;
        ys[i] = y;
    }
    drop(pts);

    let mut adm = vec![0u32; npts];

    for pi in 0..npts {
        let px = unsafe { *xs.get_unchecked(pi) };
        let py = unsafe { *ys.get_unchecked(pi) };
        let mut total = ncr(&fact, &inv_fact, (px + py) as usize, px as usize);

        let sub = if pi >= 2048 {
            let nthr = rayon::current_num_threads().max(1);
            let chunk = pi.div_ceil(nthr).max(512);
            (0..pi)
                .into_par_iter()
                .step_by(chunk)
                .map(|lo| {
                    let hi = (lo + chunk).min(pi);
                    sub_range(lo, hi, px, py, &xs, &ys, &adm, &fact, &inv_fact)
                })
                .reduce(|| 0, |a, b| a + b)
                % MOD
        } else {
            sub_range(0, pi, px, py, &xs, &ys, &adm, &fact, &inv_fact)
        };

        total = (total + MOD - sub) % MOD;
        unsafe {
            *adm.get_unchecked_mut(pi) = total as u32;
        }
    }

    println!("{}", adm[npts - 1]);
}
