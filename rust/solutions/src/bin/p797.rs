// Project Euler 797 - Cyclotomic Polynomials
// Sieve division for F_n(2), then G_n(2) = product of (F_d(2)+1) for d|n.
// Total = sum_{n=1}^N Mertens(N/n) * G_n(2).
//
// F[i] is complete once every proper divisor (all j <= i/2) has been applied, so
// i-ranges (W, 2W] can be inverted and applied as a wave. Updates of a wave
// never touch F-values still in that wave, so the strided stores are parallel
// over disjoint f[] chunks.

use rayon::prelude::*;

const N: usize = 10_000_000;
const MOD: u64 = 1_000_000_007;

fn mod_inv(a: u32) -> u64 {
    let (mut t, mut newt) = (0i32, 1i32);
    let (mut r, mut newr) = (MOD as i32, a as i32);
    while newr != 0 {
        let q = r / newr;
        let tmp = newt;
        newt = t - q * newt;
        t = tmp;
        let tmp = newr;
        newr = r - q * newr;
        r = tmp;
    }
    if t < 0 {
        (t + MOD as i32) as u64
    } else {
        t as u64
    }
}

fn batch_invert(inputs: &[u32], invs: &mut [u32]) {
    let len = inputs.len();
    if len == 0 {
        return;
    }
    let mut prod = 1u64;
    for k in 0..len {
        invs[k] = prod as u32;
        prod = (prod * inputs[k] as u64) % MOD;
    }
    let mut cur_inv = mod_inv(prod as u32);
    for k in (0..len).rev() {
        let x = inputs[k] as u64;
        let pref = invs[k] as u64;
        invs[k] = ((cur_inv * pref) % MOD) as u32;
        cur_inv = ((cur_inv * x) % MOD) as u32 as u64;
    }
}

/// Apply invs[k] to every proper multiple of i = i_start+k, in parallel over
/// disjoint f-chunks. Multiples start at 2i, so a wave (W, 2W] never writes
/// back into itself.
fn apply_invs_parallel(f: &mut [u32], invs: &[u32], i_start: usize) {
    const UCHUNK: usize = 262_144;
    f.par_chunks_mut(UCHUNK).enumerate().for_each(|(ci, chunk)| {
        let start = ci * UCHUNK;
        let end = start + chunk.len();
        for (k, &inv_u) in invs.iter().enumerate() {
            if inv_u <= 1 {
                continue;
            }
            let i = i_start + k;
            let inv = inv_u as u64;
            let min_j = 2 * i;
            let begin = if start > min_j { start } else { min_j };
            if begin >= end {
                continue;
            }
            let rem = begin % i;
            let mut j = if rem == 0 { begin } else { begin + (i - rem) };
            while j < end {
                let slot = unsafe { chunk.get_unchecked_mut(j - start) };
                *slot = ((*slot as u64 * inv) % MOD) as u32;
                j += i;
            }
        }
    });
}

fn invert_slice_parallel(f: &[u32], invs: &mut [u32], i_start: usize) {
    const LCHUNK: usize = 32_768;
    invs.par_chunks_mut(LCHUNK).enumerate().for_each(|(c_idx, c_inv)| {
        let s = i_start + c_idx * LCHUNK;
        let e = s + c_inv.len();
        batch_invert(&f[s..e], c_inv);
    });
}

fn main() {
    // Mobius function using linear sieve
    let mut primes = Vec::with_capacity(664_579);
    let mut min_prime = vec![0u32; N + 1];
    let mut mu = vec![0i8; N + 1];
    mu[1] = 1;

    for i in 2..=N {
        let mp_i = min_prime[i];
        let mp = if mp_i == 0 {
            min_prime[i] = i as u32;
            primes.push(i as u32);
            mu[i] = -1;
            i as u32
        } else {
            mp_i
        };

        for &p in &primes {
            let ip = i * p as usize;
            if ip > N {
                break;
            }
            min_prime[ip] = p;
            if p == mp {
                break;
            }
            mu[ip] = -mu[i];
        }
    }

    // Mertens function
    let mut mertens = vec![0i32; N + 1];
    for i in 1..=N {
        mertens[i] = mertens[i - 1] + mu[i] as i32;
    }
    drop(mu);
    drop(min_prime);
    drop(primes);

    // F[n] = cyclotomic polynomial at 2, computed by sieve division
    let mut f = vec![0u32; N + 1];
    {
        let mut p2 = 1u64;
        for i in 0..=N {
            f[i] = (p2 - 1) as u32;
            let next = p2 * 2;
            p2 = if next >= MOD { next - MOD } else { next };
        }
    }

    // Sequential bootstrap: i = 1..BOOT-1 (F[i] for i < 2*BOOT becomes complete).
    const BOOT: usize = 256;
    for i in 1..BOOT {
        if f[i] <= 1 {
            continue;
        }
        let inv = mod_inv(f[i]);
        let mut j = 2 * i;
        while j <= N {
            f[j] = ((f[j] as u64 * inv) % MOD) as u32;
            j += i;
        }
    }

    // Doubling waves: after processing 1..W-1, every F[i] for i < 2W is final,
    // and applying i in [W, 2W) only writes to j >= 2i >= 2W.
    let n_quarter = N / 4;
    let mut wave = BOOT;
    while wave <= n_quarter {
        let w_end = (wave * 2).min(n_quarter + 1);
        let len = w_end - wave;
        let mut invs = vec![0u32; len];
        invert_slice_parallel(&f, &mut invs, wave);
        apply_invs_parallel(&mut f, &invs, wave);
        wave = w_end;
    }

    // Phase 2b: Large divisors i in (N/4, N/2]
    // All f[i] for i > N/4 are already final because proper divisors are <= i/2 <= N/4.
    // Multiples j are only 2*i and 3*i, both > N/2.
    let n_half = N / 2;
    let n_third = N / 3;
    let large_count = n_half - n_quarter;
    let mut large_invs = vec![0u32; large_count];
    invert_slice_parallel(&f, &mut large_invs, n_quarter + 1);

    // Update 3*i for i in (N/4, N/3]
    if n_third > n_quarter {
        let count_3 = n_third - n_quarter;
        let f_ptr = f.as_mut_ptr() as usize;
        (0..count_3).into_par_iter().for_each(|k| {
            let i = n_quarter + 1 + k;
            let inv = large_invs[k] as u64;
            if inv <= 1 {
                return;
            }
            let j = 3 * i;
            unsafe {
                let ptr = (f_ptr as *mut u32).add(j);
                *ptr = ((*ptr as u64 * inv) % MOD) as u32;
            }
        });
    }

    // Update 2*i for i in (N/4, N/2]
    {
        let f_ptr = f.as_mut_ptr() as usize;
        (0..large_count).into_par_iter().for_each(|k| {
            let i = n_quarter + 1 + k;
            let inv = large_invs[k] as u64;
            if inv <= 1 {
                return;
            }
            let j = 2 * i;
            unsafe {
                let ptr = (f_ptr as *mut u32).add(j);
                *ptr = ((*ptr as u64 * inv) % MOD) as u32;
            }
        });
    }
    drop(large_invs);

    // Fold (F[d]+1) into f so the G sieve multiplies by f[d] directly.
    for x in f.iter_mut() {
        let v = *x as u64 + 1;
        *x = (if v >= MOD { v - MOD } else { v }) as u32;
    }

    // G[n] = product of (F[d]+1) for all d|n
    let mut g = vec![1u32; N + 1];
    const GCHUNK: usize = 262_144;
    g[1..].par_chunks_mut(GCHUNK).enumerate().for_each(|(chunk_idx, chunk)| {
        let start = 1 + chunk_idx * GCHUNK;
        let end = start + chunk.len();

        let bound = GCHUNK.min(end - 1);
        for i in 1..=bound {
            let factor = unsafe { *f.get_unchecked(i) } as u64;
            let first = if start <= i { i } else { ((start + i - 1) / i) * i };
            let mut j = first;
            while j < end {
                let slot = unsafe { chunk.get_unchecked_mut(j - start) };
                *slot = ((*slot as u64 * factor) % MOD) as u32;
                j += i;
            }
        }

        let max_k = chunk_idx;
        for k in 1..=max_k {
            let i_min = (start + k - 1) / k;
            let i_max = (end - 1) / k;
            for i in i_min..=i_max {
                let j = k * i;
                let factor = unsafe { *f.get_unchecked(i) } as u64;
                let slot = unsafe { chunk.get_unchecked_mut(j - start) };
                *slot = ((*slot as u64 * factor) % MOD) as u32;
            }
        }
    });
    drop(f);

    // Sum contributions: |Mertens| * G < 1e9 * 1e9, 1e7 terms fit in i128.
    let mut acc: i128 = 0;
    for i in 1..=N {
        acc += mertens[N / i] as i128 * unsafe { *g.get_unchecked(i) } as i128;
    }
    let m = MOD as i128;
    let ans = ((acc % m) + m) % m;

    println!("{}", ans);
}
