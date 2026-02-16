// Project Euler 614 - Special partitions II
// Compute sum of P(i) for i=1..10^7, where P(n) counts partitions of n into
// distinct parts not congruent to 2 mod 4.
//
// Uses the identity: F(x) * K(x) = B(x), where
//   K(x) = P(x)*P(x^4)/P(x^2) has sparse support on triangular numbers
//   B(x) = P(x^2)*P(x^8)/P(x^4) has sparse support on pronic numbers
//
// Single blocked recurrence: F(n) = B(n) + sum_{t>=1} s_t * F(n - T_t)
// where T_t = t(t+1)/2, s_t = (-1)^floor((t-1)/2)
//
// Phase 1 (large offsets) parallelized with rayon.
// Accumulates in i64 to skip per-term modular reduction.

use rayon::prelude::*;

const N: usize = 10_000_000;
const MOD: u64 = 1_000_000_007;
const MODI: i64 = MOD as i64;
const BLOCK: usize = 1 << 14; // 16384

fn triangular_terms(max_val: usize) -> Vec<(usize, i64)> {
    let mut terms = Vec::new();
    let mut t: usize = 1;
    loop {
        let tri = t * (t + 1) / 2;
        if tri > max_val { break; }
        // s_t = (-1)^floor((t-1)/2): +,+,-,-,+,+,-,-,...
        let sign: i64 = if ((t - 1) / 2) % 2 == 0 { 1 } else { -1 };
        terms.push((tri, sign));
        t += 1;
    }
    terms
}

#[inline(always)]
fn reduce_mod(x: i64) -> u64 {
    ((x % MODI) + MODI) as u64 % MOD
}

fn main() {
    let tri_terms = triangular_terms(N);
    let split = tri_terms.iter().position(|&(w, _)| w >= BLOCK).unwrap_or(tri_terms.len());
    let small_tri = &tri_terms[..split];
    let large_tri = &tri_terms[split..];

    // Precompute base case B[n]: nonzero only at pronic numbers m(m+1)
    // Sign: (-1)^floor((m+1)/2)
    let mut base = vec![0i64; N + 1];
    {
        let mut m: usize = 0;
        loop {
            let p = m * (m + 1);
            if p > N { break; }
            base[p] = if ((m + 1) / 2) % 2 == 0 { 1 } else { -1 };
            m += 1;
        }
    }

    let mut f = vec![0u64; N + 1];
    let num_blocks = (N + BLOCK) / BLOCK;

    // Pre-allocate thread-local temp buffers
    let nthreads = rayon::current_num_threads();
    let chunk_size = std::cmp::max(1, (large_tri.len() + nthreads - 1) / nthreads);
    let mut thread_temps: Vec<Vec<i64>> = (0..nthreads).map(|_| vec![0i64; BLOCK]).collect();
    let mut temp = vec![0i64; BLOCK];

    for b in 0..num_blocks {
        let bstart = b * BLOCK;
        let bend = std::cmp::min(bstart + BLOCK, N + 1);
        let blen = bend - bstart;

        // Phase 1: Parallel accumulation of large-offset contributions
        let f_ref: &[u64] = &f;
        thread_temps.par_iter_mut().enumerate().for_each(|(tid, ptmp)| {
            let chunk_start = tid * chunk_size;
            let chunk_end = std::cmp::min(chunk_start + chunk_size, large_tri.len());
            // SAFETY: ptmp has length BLOCK >= blen
            for i in 0..blen {
                unsafe { *ptmp.get_unchecked_mut(i) = 0; }
            }
            for idx in chunk_start..chunk_end {
                let (w, sign) = unsafe { *large_tri.get_unchecked(idx) };
                let n_lo = bstart.max(w);
                if n_lo >= bend { break; }
                let src_base = n_lo - w;
                let dst_base = n_lo - bstart;
                let len = bend - n_lo;
                for j in 0..len {
                    unsafe {
                        let prev = *f_ref.get_unchecked(src_base + j) as i64;
                        *ptmp.get_unchecked_mut(dst_base + j) += sign * prev;
                    }
                }
            }
        });

        // Merge: sum all thread temps into temp
        for i in 0..blen {
            let mut s = 0i64;
            for ptmp in &thread_temps {
                s += unsafe { *ptmp.get_unchecked(i) };
            }
            unsafe { *temp.get_unchecked_mut(i) = s; }
        }

        // Phase 2: Forward sweep within block (sequential)
        for i in 0..blen {
            let n = bstart + i;
            let mut acc = unsafe { *base.get_unchecked(n) }
                        + unsafe { *temp.get_unchecked(i) };

            for &(w, sign) in small_tri {
                if w > n { break; }
                let prev = unsafe { *f.get_unchecked(n - w) } as i64;
                acc += sign * prev;
            }

            unsafe { *f.get_unchecked_mut(n) = reduce_mod(acc); }
        }
    }

    // Sum f[1..=N]
    let mut ans = 0u64;
    for i in 1..=N {
        ans += unsafe { *f.get_unchecked(i) };
        if ans >= MOD { ans -= MOD; }
    }

    println!("{}", ans);
}
