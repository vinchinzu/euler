// Project Euler 337 - Totient Stairstep Sequences
// DP with Fenwick tree, sorting by (phi, value).

const TARGET_N: usize = 20_000_000;
const MOD: i64 = 100_000_000;
const START: usize = 6;

fn main() {
    // Linear sieve for phi
    let mut phi_arr = vec![0u32; TARGET_N + 1];
    let mut is_composite = vec![false; TARGET_N + 1];
    let mut primes = Vec::with_capacity(2_000_000);

    phi_arr[0] = 0;
    phi_arr[1] = 1;
    for i in 2..=TARGET_N {
        if !is_composite[i] {
            primes.push(i);
            phi_arr[i] = (i - 1) as u32;
        }
        for &p in &primes {
            let x = i * p;
            if x > TARGET_N {
                break;
            }
            is_composite[x] = true;
            if i % p == 0 {
                phi_arr[x] = phi_arr[i] * p as u32;
                break;
            }
            phi_arr[x] = phi_arr[i] * (p - 1) as u32;
        }
    }

    // Build pairs (phi, idx), sort
    let count = TARGET_N - START + 1;
    let mut pairs: Vec<(u32, u32)> = Vec::with_capacity(count);
    for i in START..=TARGET_N {
        pairs.push((phi_arr[i], i as u32));
    }
    pairs.sort_unstable();

    // Fenwick tree
    let mut bit = vec![0i64; TARGET_N + 2];

    let bit_update = |bit: &mut Vec<i64>, idx: usize, val: i64| {
        let mut i = idx + 1;
        while i <= TARGET_N + 1 {
            bit[i] = (bit[i] + val) % MOD;
            i += i & i.wrapping_neg();
        }
    };

    let bit_query = |bit: &[i64], idx: usize| -> i64 {
        let mut i = idx + 1;
        let mut s = 0i64;
        while i > 0 {
            s += bit[i];
            i -= i & i.wrapping_neg();
        }
        s % MOD
    };

    let bit_range = |bit: &[i64], l: usize, r: usize| -> i64 {
        if l > r {
            return 0;
        }
        let res = bit_query(bit, r) - if l > 0 { bit_query(bit, l - 1) } else { 0 };
        ((res % MOD) + MOD) % MOD
    };

    let mut dp = vec![0i64; TARGET_N + 1];
    let mut total = 0i64;
    let mut pos = 0;

    while pos < count {
        let cur_phi = pairs[pos].0;
        let group_start = pos;
        while pos < count && pairs[pos].0 == cur_phi {
            pos += 1;
        }
        let group_end = pos;

        // Compute dp for each element in the group
        for g in group_start..group_end {
            let j = pairs[g].1 as usize;
            let left = START.max(cur_phi as usize + 1);
            let right = if j > 0 { j - 1 } else { 0 };

            let sum_prev = if left <= right {
                bit_range(&bit, left, right)
            } else {
                0
            };
            let base = if j == START { 1 } else { 0 };
            let value = (base + sum_prev) % MOD;
            dp[j] = value;
            total = (total + value) % MOD;
        }

        // Update BIT with computed dp values
        for g in group_start..group_end {
            let j = pairs[g].1 as usize;
            bit_update(&mut bit, j, dp[j]);
        }
    }

    println!("{}", total);
}
