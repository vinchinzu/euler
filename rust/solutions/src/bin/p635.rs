// Project Euler 635 - Subset sums
// For each prime p < 10^8, compute A(2,p)+A(3,p) using factorials mod M
// A(2,p) = (C(2p,p) + 2(p-1)) / p
// A(3,p) = (C(3p,p) + 3(p-1)) / p

use rayon::prelude::*;

const N: usize = 100_000_000;
const M: u64 = 1_000_000_009;

// M < 2^30, so M*M < 2^60 < 2^64: all modular mults fit in u64
#[inline(always)]
fn mulmod(a: u64, b: u64) -> u64 {
    a * b % M
}

#[inline(always)]
fn addmod(a: u64, b: u64) -> u64 {
    let s = a + b;
    if s >= M { s - M } else { s }
}

fn mod_pow(mut base: u64, mut exp: u64) -> u64 {
    let mut result = 1u64;
    base %= M;
    while exp > 0 {
        if exp & 1 == 1 { result = mulmod(result, base); }
        base = mulmod(base, base);
        exp >>= 1;
    }
    result
}

#[inline(always)]
fn mod_inv(a: u64) -> u64 { mod_pow(a, M - 2) }

fn main() {
    // Odd-only bit-packed sieve
    let num_odds = N / 2;
    let num_words = (num_odds + 63) / 64;
    let mut composite = vec![0u64; num_words];
    composite[0] |= 1; // 1 is not prime (k = 0 corresponds to 2*0 + 1 = 1)

    let limit_k = (((N as f64).sqrt() as usize) - 1) / 2;
    for k in 1..=limit_k {
        let word_idx = k / 64;
        let bit_idx = k % 64;
        if (composite[word_idx] & (1 << bit_idx)) == 0 {
            let p = 2 * k + 1;
            let mut step_k = 2 * k * (k + 1);
            while step_k < num_odds {
                composite[step_k / 64] |= 1 << (step_k % 64);
                step_k += p;
            }
        }
    }

    // Collect primes > 2
    let mut primes: Vec<u32> = Vec::with_capacity(5_761_455);
    for (w_idx, &word) in composite.iter().enumerate() {
        let mut free = !word;
        if w_idx == 0 {
            free &= !1;
        }
        let base_k = w_idx * 64;
        while free != 0 {
            let tz = free.trailing_zeros();
            let k = base_k + tz as usize;
            if k < num_odds {
                primes.push((2 * k + 1) as u32);
            }
            free &= free - 1;
        }
    }

    // Precompute factorials mod M up to 3*N in parallel chunks
    let flen = 3 * N + 1;
    let num_chunks = (rayon::current_num_threads().max(1) * 8).min(flen - 1);
    let chunk_size = (flen - 1 + num_chunks - 1) / num_chunks;

    let chunk_prods: Vec<u64> = (0..num_chunks)
        .into_par_iter()
        .map(|c| {
            let start = 1 + c * chunk_size;
            let end = (start + chunk_size).min(flen);
            let mut p = 1u64;
            for x in start..end {
                p = mulmod(p, x as u64);
            }
            p
        })
        .collect();

    let mut chunk_starts = vec![1u64; num_chunks];
    for c in 1..num_chunks {
        chunk_starts[c] = mulmod(chunk_starts[c - 1], chunk_prods[c - 1]);
    }

    let mut fact = vec![0u32; flen];
    fact[0] = 1;
    fact[1..]
        .par_chunks_mut(chunk_size)
        .enumerate()
        .for_each(|(c, chunk)| {
            let mut cur = chunk_starts[c];
            let mut x = 1 + c * chunk_size;
            for elem in chunk.iter_mut() {
                cur = mulmod(cur, x as u64);
                *elem = cur as u32;
                x += 1;
            }
        });

    // For p=2: A(2,2)+A(3,2) = 2+6 = 8
    let base_ans = 8u64;

    // Parallel sum over primes > 2
    // For each prime p:
    //   C(2p,p) = fact[2p] * inv(fact[p])^2
    //   A(2,p) = (C(2p,p) + 2(p-1)) * inv(p) mod M
    //   C(3p,p) = fact[3p] * inv(fact[p]) * inv(fact[2p])
    //   A(3,p) = (C(3p,p) + 3(p-1)) * inv(p) mod M
    //
    // Montgomery simultaneous inversion for fp, f2p, and pp with a single mod_inv
    let chunk_sum: u64 = primes.par_chunks(8192).map(|chunk| {
        let mut local_sum = 0u64;
        for &p in chunk {
            let pp = p as u64;
            let p_idx = p as usize;
            // SAFETY: 2*p < 2*N < 3*N+1 = flen, 3*p < 3*N+1 = flen, p < N < flen
            unsafe {
                let fp = *fact.get_unchecked(p_idx) as u64;
                let f2p = *fact.get_unchecked(2 * p_idx) as u64;
                let f3p = *fact.get_unchecked(3 * p_idx) as u64;

                let xy = mulmod(fp, f2p);
                let xyz = mulmod(xy, pp);
                let inv_xyz = mod_inv(xyz);

                let inv_p = mulmod(xy, inv_xyz);
                let inv_xy = mulmod(pp, inv_xyz);
                let inv_fp = mulmod(f2p, inv_xy);

                // C(2p,p) = f2p * inv_fp^2
                let c2p = mulmod(f2p, mulmod(inv_fp, inv_fp));
                // C(3p,p) = f3p * inv_fp * inv_f2p = f3p * inv_xy
                let c3p = mulmod(f3p, inv_xy);

                let total = addmod(addmod(c2p, c3p), 5 * (pp - 1));
                local_sum = addmod(local_sum, mulmod(total, inv_p));
            }
        }
        local_sum
    }).reduce(|| 0u64, |a, b| addmod(a, b));

    let ans = addmod(base_ans, chunk_sum);
    println!("{}", ans);
}
