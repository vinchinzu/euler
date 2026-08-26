// Project Euler 231: Prime Factorisation of Binomial Coefficients
//
// sopfr(C(N,K)) = S(N) - S(K) - S(N-K)
// where S(n) = sum_{p prime} p * v_p(n!),  v_p(n!) = n/p + n/p^2 + ...
//
// k=1 is Dirichlet floor blocks on grouped odd-prime sums from a
// parallel odd-only sieve; k>=2 only needs p <= sqrt(N).

const N: u64 = 20_000_000;
const K: u64 = 15_000_000;
const M: u64 = N - K;
const LIMIT: usize = N as usize;
const GROUP: usize = 64;
const PERIOD: usize = 105; // 3*5*7
const NTHREADS: usize = 8;

fn main() {
    let sqrt = (LIMIT as f64).sqrt() as usize;
    let small = primes_upto(sqrt);
    let (sieve, gpre) = parallel_odd_sieve(LIMIT, &small);

    let mut ans = sum_p_floor(N, &sieve, &gpre)
        - sum_p_floor(K, &sieve, &gpre)
        - sum_p_floor(M, &sieve, &gpre);

    for &p in &small {
        let pu = p as u64;
        if pu > N / pu {
            break;
        }
        let mut pp = pu * pu;
        loop {
            ans += pu as i64 * ((N / pp - K / pp - M / pp) as i64);
            if pp > N / pu {
                break;
            }
            pp *= pu;
        }
    }

    println!("{}", ans);
}

fn primes_upto(limit: usize) -> Vec<u32> {
    if limit < 2 {
        return Vec::new();
    }
    let mut is_comp = vec![0u8; limit + 1];
    let mut ps = Vec::with_capacity(limit / 8);
    for i in 2..=limit {
        if is_comp[i] == 0 {
            ps.push(i as u32);
            let mut j = i * i;
            while j <= limit {
                is_comp[j] = 1;
                j += i;
            }
        }
    }
    ps
}

fn make_pattern() -> [u8; PERIOD] {
    let mut pat = [0u8; PERIOD];
    for i in 0..PERIOD {
        let n = 2 * i + 1;
        if n % 3 == 0 || n % 5 == 0 || n % 7 == 0 {
            pat[i] = 1;
        }
    }
    pat
}

fn parallel_odd_sieve(limit: usize, small: &[u32]) -> (Vec<u8>, Vec<u64>) {
    let n_odd = (limit + 1) / 2;
    let pat = make_pattern();
    let n_groups = n_odd / GROUP;

    let mut sieve: Vec<u8> = Vec::with_capacity(n_odd);
    // SAFETY: every byte is written by fill_mark_gsum before any read.
    unsafe {
        sieve.set_len(n_odd);
    }
    let mut gsum = vec![0u64; n_groups];

    std::thread::scope(|scope| {
        let g_per = (n_groups + NTHREADS - 1) / NTHREADS;
        let mut s_rest = sieve.as_mut_slice();
        let mut gs_rest = gsum.as_mut_slice();
        let mut start = 0usize;
        let mut handles = Vec::with_capacity(NTHREADS);
        for _ in 0..NTHREADS {
            let ng = g_per.min((n_odd - start) / GROUP);
            if ng == 0 {
                break;
            }
            let nbytes = ng * GROUP;
            let (s_chunk, s_tail) = s_rest.split_at_mut(nbytes);
            let (g_chunk, g_tail) = gs_rest.split_at_mut(ng);
            s_rest = s_tail;
            gs_rest = g_tail;
            let st = start;
            handles.push(scope.spawn(move || {
                fill_mark_gsum(s_chunk, g_chunk, st, small, &pat);
            }));
            start += nbytes;
        }
        for h in handles {
            h.join().unwrap();
        }
    });

    let mut gpre = vec![0u64; n_groups + 1];
    for i in 0..n_groups {
        gpre[i + 1] = gpre[i] + gsum[i];
    }
    (sieve, gpre)
}

fn fill_mark_gsum(
    chunk: &mut [u8],
    gsum: &mut [u64],
    start: usize,
    small: &[u32],
    pat: &[u8; PERIOD],
) {
    let len = chunk.len();
    let end = start + len;

    let mut src = start % PERIOD;
    let mut off = 0usize;
    while off < len {
        let n = (PERIOD - src).min(len - off);
        chunk[off..off + n].copy_from_slice(&pat[src..src + n]);
        off += n;
        src = 0;
    }
    if start == 0 {
        chunk[0] = 1; // 1 is not prime
    }

    for &sp in small {
        if sp <= 7 {
            continue;
        }
        let p = sp as usize;
        let mut i = start.max(p * p / 2);
        if i >= end {
            continue;
        }
        // Odd index i is a multiple of p iff i ≡ (p-1)/2 (mod p).
        let need = (p - 1) / 2;
        i += (need + p - i % p) % p;
        let mut j = i - start;
        // SAFETY: j < len = chunk.len(); unrolled stride stays in-bounds via the guard.
        unsafe {
            let ptr = chunk.as_mut_ptr();
            while j + 3 * p < len {
                *ptr.add(j) = 1;
                *ptr.add(j + p) = 1;
                *ptr.add(j + 2 * p) = 1;
                *ptr.add(j + 3 * p) = 1;
                j += 4 * p;
            }
            while j < len {
                *ptr.add(j) = 1;
                j += p;
            }
        }
    }

    for (g, slot) in gsum.iter_mut().enumerate() {
        let lo = g * GROUP;
        let hi = lo + GROUP;
        let mut s = 0u64;
        let mut i = lo;
        // SAFETY: [lo, hi) sits inside chunk (GROUP-aligned); unaligned u64 loads of 0/1 bytes.
        unsafe {
            let ptr = chunk.as_ptr();
            while i + 8 <= hi {
                let v = std::ptr::read_unaligned(ptr.add(i) as *const u64);
                let packed =
                    ((v & 0x0101_0101_0101_0101).wrapping_mul(0x0102_0408_1020_4080)) >> 56;
                let mut bits = (!packed) & 0xFF;
                while bits != 0 {
                    let b = bits.trailing_zeros() as usize;
                    bits &= bits - 1;
                    s += (2 * (start + i + b) + 1) as u64;
                }
                i += 8;
            }
        }
        *slot = s;
    }
}

fn sum_odds(sieve: &[u8], gpre: &[u64], i_lo: usize, i_hi: usize) -> u64 {
    if i_lo > i_hi {
        return 0;
    }
    let n_odd = sieve.len();
    let i_hi = i_hi.min(n_odd - 1);
    let g0 = i_lo / GROUP;
    let g1 = i_hi / GROUP;
    // SAFETY: i_lo..=i_hi are odd-indices < n_odd.
    if g0 == g1 {
        let mut s = 0u64;
        for i in i_lo..=i_hi {
            if unsafe { *sieve.get_unchecked(i) } == 0 {
                s += (2 * i + 1) as u64;
            }
        }
        return s;
    }
    let mut s = 0u64;
    let g0_end = (g0 + 1) * GROUP;
    for i in i_lo..g0_end {
        if unsafe { *sieve.get_unchecked(i) } == 0 {
            s += (2 * i + 1) as u64;
        }
    }
    s += gpre[g1] - gpre[g0 + 1];
    for i in (g1 * GROUP)..=i_hi {
        if unsafe { *sieve.get_unchecked(i) } == 0 {
            s += (2 * i + 1) as u64;
        }
    }
    s
}

/// sum_{p <= n} p * floor(n/p)
fn sum_p_floor(n: u64, sieve: &[u8], gpre: &[u64]) -> i64 {
    if n < 2 {
        return 0;
    }
    let mut ans = 0i64;
    let mut l = 2u64;
    while l <= n {
        let v = n / l;
        let r = (n / v).min(n);
        let mut s = 0u64;
        if l <= 2 && r >= 2 {
            s += 2;
        }
        let a = l.max(3);
        if a <= r {
            let i_lo = (a / 2) as usize;
            let i_hi = ((r - 1) / 2) as usize;
            s += sum_odds(sieve, gpre, i_lo, i_hi);
        }
        ans += v as i64 * s as i64;
        l = r + 1;
    }
    ans
}
