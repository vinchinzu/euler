// Project Euler 743 - Window into a Matrix
// Parallel blocked recurrence with Montgomery batch inversion and Barrett reduction.

use rayon::prelude::*;

const MOD: u64 = 1_000_000_007;
const BARRETT_M: u128 = 18446743944u128; // floor(2^64 / MOD)

#[inline(always)]
fn fast_mod(x: u64) -> u64 {
    let q = ((x as u128 * BARRETT_M) >> 64) as u64;
    let mut r = x - q * MOD;
    if r >= MOD {
        r -= MOD;
    }
    r
}

#[inline(always)]
fn mul(a: u64, b: u64) -> u64 {
    fast_mod(a * b)
}

fn pow_mod(mut base: u64, mut exp: u64) -> u64 {
    let mut result = 1u64;
    base %= MOD;
    while exp > 0 {
        if exp & 1 == 1 {
            result = mul(result, base);
        }
        exp >>= 1;
        base = mul(base, base);
    }
    result
}

#[inline(always)]
fn mod_inv(x: u64) -> u64 {
    pow_mod(x, MOD - 2)
}

const BLOCK_SIZE: usize = 4096;

fn main() {
    let n: u64 = 10_000_000_000_000_000; // 10^16
    let k: u64 = 100_000_000; // 10^8
    let half_k = (k / 2) as usize;

    let base = mod_inv(pow_mod(2, 2 * n / k));

    let num_chunks = 128;
    let chunk_size = (half_k + num_chunks - 1) / num_chunks;

    let chunk_results: Vec<(u64, u64)> = (0..num_chunks)
        .into_par_iter()
        .map(|c| {
            let c_start = c * chunk_size;
            let c_end = (c_start + chunk_size).min(half_k);
            if c_start >= c_end {
                return (0, 1);
            }

            let mut prefix = [0u64; BLOCK_SIZE];
            let mut invs = [0u64; BLOCK_SIZE];

            let mut y = 1u64;
            let mut c_sum = 0u64;
            let mut cur_i = c_start;

            while cur_i < c_end {
                let b_end = (cur_i + BLOCK_SIZE).min(c_end);
                let m = b_end - cur_i;

                let mut cur = 1u64;
                for idx in 0..m {
                    let val = (cur_i + idx + 1) as u64;
                    cur = mul(cur, val);
                    prefix[idx] = cur;
                }

                let mut inv_all = mod_inv(cur);
                for idx in (1..m).rev() {
                    invs[idx] = mul(inv_all, prefix[idx - 1]);
                    let val = (cur_i + idx + 1) as u64;
                    inv_all = mul(inv_all, val);
                }
                invs[0] = inv_all;

                let mut blk_sum = 0u64;
                for idx in 0..m {
                    let i = (cur_i + idx) as u64;
                    blk_sum += y;

                    let inv_ip1 = invs[idx];
                    let inv_sq = mul(inv_ip1, inv_ip1);
                    let term1 = k - 2 * i;
                    let term2 = term1 - 1;
                    let t12 = mul(term1, term2);
                    let step_mult = mul(mul(t12, inv_sq), base);
                    y = mul(y, step_mult);
                }
                c_sum = fast_mod(c_sum + fast_mod(blk_sum));

                cur_i = b_end;
            }

            (c_sum, y)
        })
        .collect();

    let mut ans = 0u64;
    let mut scale = pow_mod(2, n);
    for (c_sum, prod) in chunk_results {
        ans = (ans + mul(scale, c_sum)) % MOD;
        scale = mul(scale, prod);
    }

    ans = (ans + scale) % MOD;

    println!("{}", ans);
}
