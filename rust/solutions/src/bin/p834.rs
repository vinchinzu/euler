// Project Euler 834 - Add and Divide
// f(m) via divisors of n(n-1)
// Optimized: SPF-based factorization of n and n-1 separately, pre-allocated buffers, rayon.

use rayon::prelude::*;

const NMAX: usize = 1_234_567;

fn main() {
    // SPF sieve
    let mut spf = vec![0u32; NMAX + 1];
    for i in 2..=NMAX {
        if spf[i] == 0 {
            spf[i] = i as u32;
            if (i as u64) * (i as u64) <= NMAX as u64 {
                let mut j = i * i;
                while j <= NMAX {
                    if spf[j] == 0 {
                        spf[j] = i as u32;
                    }
                    j += i;
                }
            }
        }
    }

    // Factorize a single number using SPF sieve into a stack-allocated buffer.
    // Returns the number of factors written.
    #[inline]
    fn factorize_spf(mut n: usize, spf: &[u32], factors: &mut [(u32, u32); 20]) -> usize {
        let mut nf = 0;
        while n > 1 {
            let p = unsafe { *spf.get_unchecked(n) } as usize;
            let mut e = 0u32;
            while n > 1 {
                // SAFETY: n is always in range [2..NMAX]
                if unsafe { *spf.get_unchecked(n) } as usize != p {
                    break;
                }
                // Divide out p. Since spf[n] = p, n is divisible by p.
                n /= p;
                e += 1;
            }
            factors[nf] = (p as u32, e);
            nf += 1;
        }
        nf
    }

    // Merge two sorted factor lists (by prime). Both are sorted because SPF
    // extracts factors in increasing order.
    #[inline]
    fn merge_factors(
        a: &[(u32, u32); 20], na: usize,
        b: &[(u32, u32); 20], nb: usize,
        out: &mut [(u32, u32); 20],
    ) -> usize {
        let mut i = 0;
        let mut j = 0;
        let mut k = 0;
        while i < na && j < nb {
            let (pa, ea) = a[i];
            let (pb, eb) = b[j];
            if pa == pb {
                out[k] = (pa, ea + eb);
                i += 1;
                j += 1;
            } else if pa < pb {
                out[k] = (pa, ea);
                i += 1;
            } else {
                out[k] = (pb, eb);
                j += 1;
            }
            k += 1;
        }
        while i < na {
            out[k] = a[i];
            i += 1;
            k += 1;
        }
        while j < nb {
            out[k] = b[j];
            j += 1;
            k += 1;
        }
        k
    }

    // Process a chunk of n values and return partial sum
    let chunk_size = 10000;
    let n_chunks = (NMAX - 2 + chunk_size - 1) / chunk_size; // n ranges from 3..=NMAX

    let ans: i64 = (0..n_chunks).into_par_iter().map(|ci| {
        let n_start = 3 + ci * chunk_size;
        let n_end = std::cmp::min(n_start + chunk_size - 1, NMAX);

        let mut local_ans: i64 = 0;
        let mut fa: [(u32, u32); 20] = [(0, 0); 20];
        let mut fb: [(u32, u32); 20] = [(0, 0); 20];
        let mut merged: [(u32, u32); 20] = [(0, 0); 20];
        let mut divs = [0i64; 10000];

        for n in n_start..=n_end {
            // Factorize n and n-1 separately using SPF
            let na = factorize_spf(n, &spf, &mut fa);
            let nb = factorize_spf(n - 1, &spf, &mut fb);
            let nm = merge_factors(&fa, na, &fb, nb, &mut merged);

            // Generate divisors
            let mut nd: usize = 1;
            divs[0] = 1;
            for fi in 0..nm {
                let (p, e) = merged[fi];
                let p = p as i64;
                let cur = nd;
                let mut pk = 1i64;
                for _ in 0..e {
                    pk *= p;
                    for k in 0..cur {
                        // SAFETY: nd < 10000 guaranteed by divisor count bounds
                        unsafe {
                            *divs.get_unchecked_mut(nd) = *divs.get_unchecked(k) * pk;
                        }
                        nd += 1;
                    }
                }
            }

            let ni64 = n as i64;
            let nn1 = ni64 * (ni64 - 1);

            for i in 0..nd {
                // SAFETY: i < nd <= 10000
                let d = unsafe { *divs.get_unchecked(i) };
                if d > ni64 {
                    let m = d - ni64;
                    let val = d + 1 - nn1 / d;
                    if val & 1 == 0 {
                        local_ans += m;
                    }
                }
            }
        }
        local_ans
    }).sum();

    println!("{}", ans);
}
