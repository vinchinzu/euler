// Project Euler 733 - Ascending Subsequences
//
// BIT (Fenwick tree) with coordinate compression for K-term ascending subsequences.

const MAXN: usize = 1_000_001;
const MOD: i64 = 1_000_000_007;
const K: usize = 4;

fn bit_add(tree: &mut [i64], mut i: usize, val: i64, n: usize) {
    while i <= n {
        tree[i] = (tree[i] + val) % MOD;
        i += i & i.wrapping_neg();
    }
}

fn bit_sum(tree: &[i64], mut i: usize) -> i64 {
    let mut s: i64 = 0;
    while i > 0 {
        s = (s + tree[i]) % MOD;
        i -= i & i.wrapping_neg();
    }
    s
}

fn main() {
    let n = 1_000_000usize;

    // Generate sequence
    let mut seq = vec![0i64; n];
    let mut a: i64 = 153;
    for i in 0..n {
        seq[i] = a;
        a = a * 153 % 10_000_019;
    }

    // Coordinate compression
    let mut sorted_vals: Vec<i32> = seq.iter().map(|&x| x as i32).collect();
    sorted_vals.sort_unstable();
    sorted_vals.dedup();

    let mut rank_map = vec![0usize; n];
    for i in 0..n {
        let target = seq[i] as i32;
        rank_map[i] = sorted_vals.binary_search(&target).unwrap() + 1;
    }

    let mut count_bit = vec![vec![0i64; n + 1]; K + 1];
    let mut sum_bit = vec![vec![0i64; n + 1]; K + 1];

    for i in 0..n {
        let r = rank_map[i];
        let val = seq[i] % MOD;

        for k in (2..=K).rev() {
            let cnt = bit_sum(&count_bit[k - 1], r - 1);
            let sm = bit_sum(&sum_bit[k - 1], r - 1);
            if cnt > 0 || sm > 0 {
                bit_add(&mut count_bit[k], r, cnt, n);
                bit_add(&mut sum_bit[k], r, (cnt % MOD * val % MOD + sm) % MOD, n);
            }
        }
        bit_add(&mut count_bit[1], r, 1, n);
        bit_add(&mut sum_bit[1], r, val, n);
    }

    let ans = ((bit_sum(&sum_bit[K], n) % MOD) + MOD) % MOD;
    println!("{}", ans);
}
