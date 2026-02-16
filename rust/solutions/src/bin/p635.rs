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
    // Bit-packed sieve
    let bytes = (N + 7) / 8;
    let mut sieve = vec![0u8; bytes + 1];
    sieve[0] |= 3; // mark 0 and 1
    let mut i = 2;
    while i * i <= N {
        if sieve[i >> 3] & (1 << (i & 7)) == 0 {
            let mut j = i * i;
            while j <= N { sieve[j >> 3] |= 1 << (j & 7); j += i; }
        }
        i += 1;
    }

    // Collect primes > 2
    let mut primes: Vec<usize> = Vec::with_capacity(6_000_000);
    for p in 3..N {
        if sieve[p >> 3] & (1 << (p & 7)) == 0 {
            primes.push(p);
        }
    }

    // Precompute factorials mod M up to 3*N
    let flen = 3 * N + 1;
    let mut fact = vec![1u64; flen];
    for i in 1..flen {
        // SAFETY: i < flen, i-1 < flen
        unsafe {
            let prev = *fact.get_unchecked(i - 1);
            *fact.get_unchecked_mut(i) = prev * (i as u64) % M;
        }
    }

    // For p=2: A(2,2)+A(3,2) = 2+6 = 8
    let base_ans = 8u64;

    // Parallel sum over primes > 2
    // For each prime p:
    //   C(2p,p) = fact[2p] * inv(fact[p])^2
    //   A(2,p) = (C(2p,p) + 2(p-1)) * inv(p) mod M
    //   C(3p,p) = fact[3p] * inv(fact[p]) * inv(fact[2p])
    //   A(3,p) = (C(3p,p) + 3(p-1)) * inv(p) mod M
    //
    // We compute mod_inv for fact[p], fact[2p], and p directly.
    // This avoids the 2.4GB inv_fact precomputation.

    let chunk_sum: u64 = primes.par_chunks(8192).map(|chunk| {
        let mut local_sum = 0u64;
        for &p in chunk {
            let pp = p as u64;
            // SAFETY: 2*p < 2*N < 3*N+1 = flen, 3*p < 3*N+1 = flen, p < N < flen
            unsafe {
                let fp = *fact.get_unchecked(p);
                let f2p = *fact.get_unchecked(2 * p);
                let f3p = *fact.get_unchecked(3 * p);

                let inv_fp = mod_inv(fp);
                let inv_f2p = mod_inv(f2p);
                let inv_p = mod_inv(pp);

                // C(2p,p) = f2p * inv_fp^2
                let c2p = mulmod(f2p, mulmod(inv_fp, inv_fp));
                let a2 = mulmod(addmod(c2p, 2 * (pp - 1) % M), inv_p);

                // C(3p,p) = f3p * inv_fp * inv_f2p
                let c3p = mulmod(f3p, mulmod(inv_fp, inv_f2p));
                let a3 = mulmod(addmod(c3p, 3 * (pp - 1) % M), inv_p);

                local_sum = addmod(local_sum, addmod(a2, a3));
            }
        }
        local_sum
    }).reduce(|| 0u64, |a, b| addmod(a, b));

    let ans = addmod(base_ans, chunk_sum);
    println!("{}", ans);
}
