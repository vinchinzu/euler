use rayon::prelude::*;

const N: u32 = 1_000_000;
const NUS: usize = N as usize;

/// Binary coprime test. `m` and `n` have opposite parity (exactly one even).
#[inline(always)]
fn coprime(mut u: u32, mut v: u32) -> bool {
    u >>= u.trailing_zeros();
    v >>= v.trailing_zeros();
    while u != v {
        if u > v {
            u -= v;
            u >>= u.trailing_zeros();
        } else {
            v -= u;
            v >>= v.trailing_zeros();
        }
    }
    u == 1
}

#[inline(always)]
fn count_pairs(hs: &[u32]) -> i64 {
    let nh = hs.len();
    if nh < 2 {
        return 0;
    }
    let mut local = 0i64;
    for i in 0..nh {
        // SAFETY: i < nh by loop bound
        let h1 = unsafe { *hs.get_unchecked(i) } as u64;
        let h1sq = h1 * h1;
        for j in (i + 1)..nh {
            // SAFETY: j < nh by loop bound
            let h2 = unsafe { *hs.get_unchecked(j) } as u64;
            // (h1+h2) | h1*h2  iff  (h1+h2) | h1^2
            if h1sq % (h1 + h2) == 0 {
                local += 1;
            }
        }
    }
    local
}

fn main() {
    // Primitive triples with hypotenuse < N.
    let mut prims: Vec<(u32, u32, u32)> = Vec::with_capacity(160_000);
    let mut m = 2u32;
    while m * m + 1 < N {
        let mut n = 1 + (m & 1);
        while n < m {
            if coprime(m, n) {
                let a = m * m - n * n;
                let b = 2 * m * n;
                let c = m * m + n * n;
                if c < N {
                    prims.push((a, b, c));
                }
            }
            n += 2;
        }
        m += 1;
    }

    // Pass 1: count heights per width for CSR offsets.
    let mut counts = vec![0u32; NUS];
    for &(a, b, c) in &prims {
        let max_k = (N - 1) / c;
        for k in 1..=max_k {
            counts[(k * a) as usize] += 1;
            counts[(k * b) as usize] += 1;
        }
    }

    let mut offsets = vec![0u32; NUS + 1];
    for i in 0..NUS {
        offsets[i + 1] = offsets[i] + counts[i];
    }
    let total = offsets[NUS] as usize;
    // Reuse `counts` as insertion cursors.
    counts.copy_from_slice(&offsets[..NUS]);

    let mut heights = vec![0u32; total];
    for &(a, b, c) in &prims {
        let max_k = (N - 1) / c;
        for k in 1..=max_k {
            let ka = (k * a) as usize;
            let kb = (k * b) as usize;
            let ia = counts[ka] as usize;
            unsafe {
                *heights.get_unchecked_mut(ia) = k * b;
            }
            counts[ka] += 1;
            let ib = counts[kb] as usize;
            unsafe {
                *heights.get_unchecked_mut(ib) = k * a;
            }
            counts[kb] += 1;
        }
    }

    const CHUNK: usize = 1024;
    let nchunks = (NUS + CHUNK - 1) / CHUNK;
    let count: i64 = (0..nchunks)
        .into_par_iter()
        .map(|chunk| {
            let start_w = (chunk * CHUNK).max(1);
            let end_w = (chunk * CHUNK + CHUNK).min(NUS);
            let mut local = 0i64;
            for w in start_w..end_w {
                let s = unsafe { *offsets.get_unchecked(w) } as usize;
                let e = unsafe { *offsets.get_unchecked(w + 1) } as usize;
                local += count_pairs(unsafe { heights.get_unchecked(s..e) });
            }
            local
        })
        .sum();

    println!("{}", count);
}
