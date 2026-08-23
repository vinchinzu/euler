// Project Euler 373 - Circumscribed circles
// Integer-sided triangles with integer circumradius r.
// N(r) depends only on the exponents e_i of primes ≡ 1 (mod 4):
//   N(r) = (2∏(3e²+3e+1) - 3∏(2e+1) + 3∏(2⌊e/2⌋+1) - 2) / 6
// Empty products are 1, so N(r)=0 when r has no such primes.

use rayon::prelude::*;

const N: usize = 10_000_000;

fn main() {
    let mut spf = vec![0u32; N + 1];
    for i in 2..=N {
        if spf[i] == 0 {
            spf[i] = i as u32;
            if (i as u64) * (i as u64) <= N as u64 {
                let mut j = i * i;
                while j <= N {
                    if spf[j] == 0 {
                        spf[j] = i as u32;
                    }
                    j += i;
                }
            }
        }
    }

    // Independent ranges: peel SPF and evaluate N(r) per radius.
    let n_chunks = rayon::current_num_threads().max(1) * 8;
    let chunk_size = (N + n_chunks - 1) / n_chunks;
    let spf = &spf;

    let total: i64 = (0..n_chunks)
        .into_par_iter()
        .map(|ci| {
            let lo = ci * chunk_size + 1;
            let hi = ((ci + 1) * chunk_size).min(N);
            if lo > hi {
                return 0;
            }
            let mut local = 0i64;
            for r in lo..=hi {
                let mut x = r;
                let mut a = 1i64;
                let mut b = 1i64;
                let mut c = 1i64;
                while x > 1 {
                    // SAFETY: x starts at r ∈ [1, N] and strictly decreases toward 1.
                    let p = unsafe { *spf.get_unchecked(x) };
                    let mut e = 0i64;
                    while x > 1 {
                        let s = unsafe { *spf.get_unchecked(x) };
                        if s != p {
                            break;
                        }
                        x /= p as usize;
                        e += 1;
                    }
                    if p & 3 == 1 {
                        a *= 3 * e * e + 3 * e + 1;
                        b *= 2 * e + 1;
                        c *= 2 * (e >> 1) + 1;
                    }
                }
                let cnt = (2 * a - 3 * b + 3 * c - 2) / 6;
                local += r as i64 * cnt;
            }
            local
        })
        .sum();

    println!("{}", total);
}
