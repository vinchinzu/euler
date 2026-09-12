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
// 5. Thread-local grow-only scratch (no per-length malloc)
// 6. D(k)=k for k<100 (every such k is already a duodigit)
// 7. Start at the digit-length of k (D(k) >= k)
// 8. Multiples of 10 must use digits {0,d} and an odd left bitset with d1=0

use rayon::prelude::*;
use std::cell::RefCell;

const NN: usize = 50_000;
const B: usize = 10;

thread_local! {
    static SCRATCH: RefCell<(Vec<f64>, Vec<u32>)> = RefCell::new((Vec::new(), Vec::new()));
}

fn ndigits(mut k: usize) -> usize {
    let mut n = 1;
    while k >= 10 {
        k /= 10;
        n += 1;
    }
    n
}

fn d_func(k: usize) -> f64 {
    if k < 100 {
        return k as f64;
    }

    SCRATCH.with(|cell| {
        let (nums, mods) = &mut *cell.borrow_mut();
        let mut pows_mod = [0u32; 30];
        let mut pows_f = [0.0f64; 30];
        let ki = k as u32;
        let mul10 = k % 10 == 0;
        let start = ndigits(k);

        for num_digits in start.. {
            pows_f[0] = 1.0;
            pows_mod[0] = 1;
            for i in 1..num_digits {
                pows_f[i] = pows_f[i - 1] * B as f64;
                pows_mod[i] = ((pows_mod[i - 1] as u64) * B as u64 % ki as u64) as u32;
            }

            let n = 1usize << num_digits;
            let half = n >> 1;
            let need = n * B;
            if nums.len() < need {
                nums.resize(need, 0.0);
                mods.resize(need, 0);
            }
            // empty bitset stays 0
            unsafe {
                *nums.get_unchecked_mut(0) = 0.0;
                *mods.get_unchecked_mut(0) = 0;
            }

            for bitset in 1..n {
                let i = bitset.trailing_zeros() as usize;
                let prev_bitset = bitset - (bitset & bitset.wrapping_neg());
                let pb = prev_bitset * B;
                let cb = bitset * B;
                let num = unsafe { *nums.get_unchecked(pb + 1) } + pows_f[i];
                let mut md = unsafe { *mods.get_unchecked(pb + 1) } as u32 + pows_mod[i];
                if md >= ki {
                    md -= ki;
                }
                for d in 1..B as u32 {
                    unsafe {
                        *nums.get_unchecked_mut(cb + d as usize) = d as f64 * num;
                        *mods.get_unchecked_mut(cb + d as usize) =
                            ((d as u64) * (md as u64) % (ki as u64)) as u32;
                    }
                }
            }

            let mut best = f64::MAX;
            if mul10 {
                // LSD is d1 iff bitset is odd; d2 is MSD so never 0.
                // Multiples of 10 need last digit 0 => odd bitset, d1=0, digits {0,d2}.
                let mut bitset = 1usize;
                while bitset < half {
                    let rb = (n - 1 - bitset) * B;
                    for d2 in 1..B {
                        let num = unsafe { *nums.get_unchecked(rb + d2) };
                        if num < best && unsafe { *mods.get_unchecked(rb + d2) } == 0 {
                            best = num;
                        }
                    }
                    bitset += 2;
                }
            } else {
                for bitset in 0..half {
                    let lb = bitset * B;
                    let rb = (n - 1 - bitset) * B;
                    for d1 in 0..B {
                        let num1 = unsafe { *nums.get_unchecked(lb + d1) };
                        let md1 = unsafe { *mods.get_unchecked(lb + d1) };
                        for d2 in 1..B {
                            let num = num1 + unsafe { *nums.get_unchecked(rb + d2) };
                            if num < best {
                                let md = md1 + unsafe { *mods.get_unchecked(rb + d2) };
                                if md == 0 || md == ki {
                                    best = num;
                                }
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
    })
}

fn main() {
    let ans: f64 = (1..=NN).into_par_iter().map(|k| d_func(k)).sum();
    println!("{:.12e}", ans);
}
