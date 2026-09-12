// Project Euler 467: Superstring of prime and composite digital roots
//
// Build shortest common supersequence of first N=10000 digital roots
// of primes and composites. Return its digits interpreted as base-10
// number mod 10^9+7.

use rayon::prelude::*;

const NN: usize = 10000;
const MOD: i64 = 1_000_000_007;
// Block size for anti-diagonal wavefront. NN is divisible by 100.
const B: usize = 100;

fn digital_root(n: usize) -> i32 {
    if n == 0 {
        return 0;
    }
    1 + ((n - 1) % 9) as i32
}

fn main() {
    // Sieve
    let limit = 120000;
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut i = 2;
    while i * i <= limit {
        if is_prime[i] {
            let mut j = i * i;
            while j <= limit {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }

    let mut p = vec![0i32; NN];
    let mut c = vec![0i32; NN];
    let mut np = 0usize;
    let mut nc = 0usize;

    let mut n = 2;
    while np < NN || nc < NN {
        if is_prime[n] {
            if np < NN {
                p[np] = digital_root(n);
                np += 1;
            }
        } else if nc < NN {
            c[nc] = digital_root(n);
            nc += 1;
        }
        n += 1;
    }

    // DP arrays
    let stride = NN + 1;
    let mut dp = vec![0i32; stride * stride];
    let mut move_i = vec![0u8; stride * stride];

    // Base cases
    for j in 0..=NN {
        dp[NN * stride + j] = (NN - j) as i32;
    }
    for i in 0..=NN {
        dp[i * stride + NN] = (NN - i) as i32;
    }
    for i in 0..NN {
        move_i[i * stride + NN] = 1;
    }

    let nblocks = (NN + B - 1) / B;
    let dp_addr = dp.as_mut_ptr() as usize;
    let mv_addr = move_i.as_mut_ptr() as usize;
    let p_addr = p.as_ptr() as usize;
    let c_addr = c.as_ptr() as usize;

    // Cells with equal i+j are independent. Wave by block anti-diagonal so
    // each rayon task has B*B cells (cell-level tasks are too cheap).
    for diag in (0..2 * nblocks - 1).rev() {
        let bi_lo = if diag + 1 > nblocks {
            diag + 1 - nblocks
        } else {
            0
        };
        let bi_hi = if diag < nblocks { diag } else { nblocks - 1 };
        (bi_lo..bi_hi + 1).into_par_iter().for_each(|bi| {
            let bj = diag - bi;
            let i_start = bi * B;
            let i_end = (i_start + B).min(NN);
            let j_start = bj * B;
            let j_end = (j_start + B).min(NN);
            let dp_ptr = dp_addr as *mut i32;
            let mv_ptr = mv_addr as *mut u8;
            let p_ptr = p_addr as *const i32;
            let c_ptr = c_addr as *const i32;

            // SAFETY: blocks on one anti-diagonal cover disjoint (i,j) so
            // writes do not alias. Reads of (i+1,*) / (*,j+1) / (i+1,j+1)
            // hit already-finished higher diagonals or earlier cells in this
            // block. i,j < NN and stride = NN+1 keep all indices in-bounds.
            unsafe {
                let mut i = i_end;
                while i > i_start {
                    i -= 1;
                    let pi = *p_ptr.add(i);
                    let row = i * stride;
                    let row1 = row + stride;
                    let mut j = j_end;
                    while j > j_start {
                        j -= 1;
                        let idx = row + j;
                        if pi == *c_ptr.add(j) {
                            *dp_ptr.add(idx) = 1 + *dp_ptr.add(row1 + j + 1);
                        } else {
                            let val_i = *dp_ptr.add(row1 + j);
                            let val_j = *dp_ptr.add(idx + 1);
                            if val_i <= val_j {
                                *dp_ptr.add(idx) = 1 + val_i;
                                if val_i < val_j || pi < *c_ptr.add(j) {
                                    *mv_ptr.add(idx) = 1;
                                }
                            } else {
                                *dp_ptr.add(idx) = 1 + val_j;
                            }
                        }
                    }
                }
            }
        });
    }

    // Reconstruct
    let mut ans: i64 = 0;
    let mut ii = 0usize;
    let mut jj = 0usize;
    while ii < NN || jj < NN {
        let idx = ii * stride + jj;
        let digit;
        if ii < NN && jj < NN && p[ii] == c[jj] {
            digit = p[ii] as i64;
            ii += 1;
            jj += 1;
        } else if move_i[idx] != 0 {
            digit = p[ii] as i64;
            ii += 1;
        } else {
            digit = c[jj] as i64;
            jj += 1;
        }
        ans = (10 * ans + digit) % MOD;
    }

    println!("{}", ans);
}
