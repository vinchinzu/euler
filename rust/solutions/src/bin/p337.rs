// Project Euler 337 - Totient Stairstep Sequences
// DP with Fenwick tree, sorting by (phi, value).

const TARGET_N: usize = 20_000_000;
const MOD: i64 = 100_000_000;
const START: usize = 6;

#[inline(always)]
fn bit_update(bit: &mut [i32], idx: usize, val: i32) {
    let mut i = idx + 1;
    let n = bit.len();
    while i < n {
        // SAFETY: i < n guaranteed by while condition
        unsafe {
            let v = *bit.get_unchecked(i);
            *bit.get_unchecked_mut(i) = ((v as i64 + val as i64) % MOD) as i32;
        }
        i += i & i.wrapping_neg();
    }
}

#[inline(always)]
fn bit_query(bit: &[i32], idx: usize) -> i64 {
    let mut i = idx + 1;
    let mut s = 0i64;
    while i > 0 {
        // SAFETY: i starts at idx+1 <= TARGET_N+1 < bit.len(), decreases
        unsafe { s += *bit.get_unchecked(i) as i64; }
        i -= i & i.wrapping_neg();
    }
    s % MOD
}

fn main() {
    // Linear sieve for phi
    let mut phi_arr = vec![0u32; TARGET_N + 1];
    let mut is_composite = vec![false; TARGET_N + 1];
    let mut primes = Vec::with_capacity(2_000_000);

    phi_arr[1] = 1;
    for i in 2..=TARGET_N {
        // SAFETY: i <= TARGET_N, arrays have size TARGET_N+1
        unsafe {
            if !*is_composite.get_unchecked(i) {
                primes.push(i);
                *phi_arr.get_unchecked_mut(i) = (i - 1) as u32;
            }
        }
        for &p in &primes {
            let x = i * p;
            if x > TARGET_N { break; }
            // SAFETY: x <= TARGET_N, arrays have size TARGET_N+1
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

    // Build pairs (phi, idx), sort
    let count = TARGET_N - START + 1;
    let mut pairs: Vec<(u32, u32)> = Vec::with_capacity(count);
    for i in START..=TARGET_N {
        // SAFETY: i <= TARGET_N
        unsafe { pairs.push((*phi_arr.get_unchecked(i), i as u32)); }
    }
    drop(phi_arr);
    drop(is_composite);
    pairs.sort_unstable();

    // Fenwick tree (i32 halves cache footprint; MOD=10^8 fits in i32)
    let mut bit = vec![0i32; TARGET_N + 2];

    let mut group_vals: Vec<(usize, i32)> = Vec::new();
    let mut total = 0i64;
    let mut pos = 0;

    while pos < count {
        let cur_phi = pairs[pos].0;
        group_vals.clear();

        // Hoist the left-boundary query — same for all elements in this group
        let left = START.max(cur_phi as usize + 1);
        let base_sum = if left > 0 { bit_query(&bit, left - 1) } else { 0 };

        while pos < count && pairs[pos].0 == cur_phi {
            let j = pairs[pos].1 as usize;
            let right = j.saturating_sub(1);

            let sum_prev = if left <= right {
                ((bit_query(&bit, right) - base_sum) % MOD + MOD) % MOD
            } else {
                0
            };
            let base = if j == START { 1i64 } else { 0 };
            let value = ((base + sum_prev) % MOD) as i32;
            group_vals.push((j, value));
            total = (total + value as i64) % MOD;
            pos += 1;
        }

        for &(j, value) in &group_vals {
            bit_update(&mut bit, j, value);
        }
    }

    println!("{}", total);
}
