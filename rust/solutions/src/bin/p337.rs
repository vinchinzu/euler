// Project Euler 337 - Totient Stairstep Sequences
// DP with Fenwick tree, sorting by (phi, value).
//
// Optimizations:
// - Parallel rayon unstable sort on packed u64 (phi << 32 | idx)
// - Branchless u32 accumulation in bit_query
// - Elimination of 64-bit integer divisions (% MOD) in bit_update and DP query loop
// - Direct pointer accesses in inner loops

use rayon::prelude::*;

const TARGET_N: usize = 20_000_000;
const MOD: i64 = 100_000_000;
const START: usize = 6;

#[inline(always)]
fn bit_update(b_ptr: *mut i32, n: usize, idx: usize, val: i32) {
    let mut i = idx + 1;
    while i < n {
        unsafe {
            let p = b_ptr.add(i);
            let mut sum = *p + val;
            if sum >= 100_000_000 {
                sum -= 100_000_000;
            }
            *p = sum;
        }
        i += i & i.wrapping_neg();
    }
}

#[inline(always)]
fn bit_query(b_ptr: *const i32, idx: usize) -> i64 {
    let mut i = idx + 1;
    let mut s = 0u32;
    while i > 0 {
        unsafe { s += *b_ptr.add(i) as u32; }
        i -= i & i.wrapping_neg();
    }
    (s % 100_000_000) as i64
}

fn main() {
    // Linear sieve for phi
    let mut phi_arr = vec![0u32; TARGET_N + 1];
    let mut is_composite = vec![false; TARGET_N + 1];
    let mut primes = Vec::with_capacity(2_000_000);

    phi_arr[1] = 1;
    for i in 2..=TARGET_N {
        unsafe {
            if !*is_composite.get_unchecked(i) {
                primes.push(i);
                *phi_arr.get_unchecked_mut(i) = (i - 1) as u32;
            }
        }
        for &p in &primes {
            let x = i * p;
            if x > TARGET_N { break; }
            unsafe {
                *is_composite.get_unchecked_mut(x) = true;
                if i % p == 0 {
                    *phi_arr.get_unchecked_mut(x) = *phi_arr.get_unchecked(i) * p as u32;
                } else {
                    *phi_arr.get_unchecked_mut(x) = *phi_arr.get_unchecked(i) * (p - 1) as u32;
                }
            }
            if i % p == 0 { break; }
        }
    }

    // Build packed u64 pairs (phi << 32 | idx), sort with rayon
    let count = TARGET_N - START + 1;
    let mut pairs: Vec<u64> = Vec::with_capacity(count);
    for i in START..=TARGET_N {
        unsafe {
            let phi = *phi_arr.get_unchecked(i) as u64;
            pairs.push((phi << 32) | (i as u64));
        }
    }
    drop(phi_arr);
    drop(is_composite);
    pairs.par_sort_unstable();

    // Fenwick tree (i32 halves cache footprint; MOD=10^8 fits in i32)
    let mut bit = vec![0i32; TARGET_N + 2];
    let b_ptr = bit.as_mut_ptr();
    let n_bit = bit.len();

    let mut group_vals: Vec<(usize, i32)> = Vec::new();
    let mut total = 0i64;
    let mut pos = 0;

    while pos < count {
        let cur_phi = (pairs[pos] >> 32) as u32;
        group_vals.clear();

        // Hoist the left-boundary query — same for all elements in this group
        let left = START.max(cur_phi as usize + 1);
        let base_sum = if left > 0 { bit_query(b_ptr, left - 1) } else { 0 };

        while pos < count && ((pairs[pos] >> 32) as u32) == cur_phi {
            let j = (pairs[pos] as u32) as usize;
            let right = j.saturating_sub(1);

            let sum_prev = if left <= right {
                let diff = bit_query(b_ptr, right) - base_sum;
                if diff < 0 { diff + MOD } else { diff }
            } else {
                0
            };
            let base = if j == START { 1i64 } else { 0 };
            let mut value = base + sum_prev;
            if value >= MOD {
                value -= MOD;
            }
            let val_i32 = value as i32;
            group_vals.push((j, val_i32));
            total += value;
            if total >= MOD {
                total -= MOD;
            }
            pos += 1;
        }

        for &(j, value) in &group_vals {
            bit_update(b_ptr, n_bit, j, value);
        }
    }

    println!("{}", total);
}
