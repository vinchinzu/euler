// Project Euler 714 - Duodigits
//
// For each k from 1 to 50000, find smallest multiple of k that uses at most
// 2 distinct digits. Uses meet-in-the-middle over digit positions.
//
// Optimizations:
// 1. Rayon parallelism across k values
// 2. Unsafe indexing in hot loops
// 3. Store reduced mod values [0,k) so search avoids expensive modulo
// 4. Early pruning: check num < best before mod check

use rayon::prelude::*;

const NN: usize = 50_000;
const B: usize = 10;

fn d_func(k: usize) -> f64 {
    let mut pows_mod = [0i64; 30];
    let mut pows_f = [0.0f64; 30];
    let ki = k as i64;

    for num_digits in 1.. {
        pows_f[0] = 1.0;
        pows_mod[0] = 1;
        for i in 1..num_digits {
            pows_f[i] = pows_f[i - 1] * B as f64;
            pows_mod[i] = pows_mod[i - 1] * B as i64 % ki;
        }

        let n = 1usize << num_digits;
        let half = n >> 1;

        let mut nums = vec![0.0f64; n * B];
        let mut mods = vec![0i64; n * B];

        for bitset in 1..n {
            let i = bitset.trailing_zeros() as usize;
            let prev_bitset = bitset - (bitset & bitset.wrapping_neg());
            let pb = prev_bitset * B;
            let cb = bitset * B;
            // SAFETY: prev_bitset < bitset < n, all indices within [0, n*B)
            let num = unsafe { *nums.get_unchecked(pb + 1) } + pows_f[i];
            let mut md = unsafe { *mods.get_unchecked(pb + 1) } + pows_mod[i];
            // mods[pb+1] in [0,k), pows_mod[i] in [0,k), so md in [0,2k)
            if md >= ki { md -= ki; }
            // Now md in [0,k). Store d*md % k for each digit d.
            for d in 1..B as i64 {
                unsafe {
                    *nums.get_unchecked_mut(cb + d as usize) = d as f64 * num;
                    *mods.get_unchecked_mut(cb + d as usize) = d * md % ki;
                }
            }
        }

        let mut best = f64::MAX;
        for bitset in 0..half {
            let lb = bitset * B;
            let rb = (n - 1 - bitset) * B;
            for d1 in 0..B {
                // SAFETY: lb + d1 < n * B
                let num1 = unsafe { *nums.get_unchecked(lb + d1) };
                let md1 = unsafe { *mods.get_unchecked(lb + d1) };
                for d2 in 1..B {
                    // SAFETY: rb + d2 < n * B
                    let num = num1 + unsafe { *nums.get_unchecked(rb + d2) };
                    if num < best {
                        let md = md1 + unsafe { *mods.get_unchecked(rb + d2) };
                        // Both in [0,k), so sum in [0,2k). Divisible by k iff == 0 or == k.
                        if md == 0 || md == ki {
                            best = num;
                        }
                    }
                }
            }
        }

        if best < f64::MAX {
            return best;
        }
    }
    unreachable!()
}

fn main() {
    let ans: f64 = (1..=NN).into_par_iter().map(|k| d_func(k)).sum();
    println!("{:.12e}", ans);
}
