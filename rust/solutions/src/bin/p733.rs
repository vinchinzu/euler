// Project Euler 733 - Ascending Subsequences
//
// BIT (Fenwick tree) with coordinate compression for K-term ascending subsequences.

const MOD: i64 = 1_000_000_007;

#[inline(always)]
fn add_mod(a: i64, b: i64) -> i64 {
    let s = a + b;
    if s >= MOD {
        s - MOD
    } else {
        s
    }
}

#[inline]
fn bit_add(tree: &mut [i64], mut i: usize, val: i64, n: usize) {
    // SAFETY: fenwick indices stay in 1..=n; tree.len() = n+1
    while i <= n {
        unsafe {
            let e = tree.get_unchecked_mut(i);
            *e = add_mod(*e, val);
        }
        i += i & i.wrapping_neg();
    }
}

#[inline]
fn bit_sum(tree: &[i64], mut i: usize) -> i64 {
    let mut s: i64 = 0;
    while i > 0 {
        unsafe {
            s = add_mod(s, *tree.get_unchecked(i));
        }
        i -= i & i.wrapping_neg();
    }
    s
}

fn main() {
    let n = 1_000_000usize;

    let mut seq = vec![0i64; n];
    let mut a: i64 = 153;
    for i in 0..n {
        seq[i] = a;
        a = a * 153 % 10_000_019;
    }

    // Values are in 0..10_000_019 — direct rank table beats binary_search
    let mut order: Vec<i32> = seq.iter().map(|&x| x as i32).collect();
    order.sort_unstable();
    order.dedup();
    let mut rank_of = vec![0u32; 10_000_019];
    for (i, &v) in order.iter().enumerate() {
        rank_of[v as usize] = (i + 1) as u32;
    }
    let rank_map: Vec<usize> = seq.iter().map(|&x| rank_of[x as usize] as usize).collect();

    // K=4 fixed trees — no Vec-of-Vec indirection in the hot path
    let mut c1 = vec![0i64; n + 1];
    let mut c2 = vec![0i64; n + 1];
    let mut c3 = vec![0i64; n + 1];
    let mut c4 = vec![0i64; n + 1];
    let mut s1 = vec![0i64; n + 1];
    let mut s2 = vec![0i64; n + 1];
    let mut s3 = vec![0i64; n + 1];
    let mut s4 = vec![0i64; n + 1];

    for i in 0..n {
        let r = rank_map[i];
        let val = seq[i] % MOD;

        // k=4 from k=3
        {
            let cnt = bit_sum(&c3, r - 1);
            let sm = bit_sum(&s3, r - 1);
            if cnt != 0 || sm != 0 {
                bit_add(&mut c4, r, cnt, n);
                let prod = (cnt as i128 * val as i128 % MOD as i128) as i64;
                bit_add(&mut s4, r, add_mod(prod, sm), n);
            }
        }
        // k=3 from k=2
        {
            let cnt = bit_sum(&c2, r - 1);
            let sm = bit_sum(&s2, r - 1);
            if cnt != 0 || sm != 0 {
                bit_add(&mut c3, r, cnt, n);
                let prod = (cnt as i128 * val as i128 % MOD as i128) as i64;
                bit_add(&mut s3, r, add_mod(prod, sm), n);
            }
        }
        // k=2 from k=1
        {
            let cnt = bit_sum(&c1, r - 1);
            let sm = bit_sum(&s1, r - 1);
            if cnt != 0 || sm != 0 {
                bit_add(&mut c2, r, cnt, n);
                let prod = (cnt as i128 * val as i128 % MOD as i128) as i64;
                bit_add(&mut s2, r, add_mod(prod, sm), n);
            }
        }
        bit_add(&mut c1, r, 1, n);
        bit_add(&mut s1, r, val, n);
    }

    println!("{}", bit_sum(&s4, n));
}
