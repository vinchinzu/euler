// Project Euler 663 - Sums of Subarrays
// Segment tree for max subarray sum with tribonacci updates.

fn main() {
    let n: usize = 10_000_003;
    let l1: usize = 10_000_000;
    let l2: usize = 10_200_000;

    let mut seg_l = 1;
    while seg_l < n { seg_l *= 2; }
    let sz = 2 * seg_l;
    let mut sums = vec![0i64; sz];
    let mut max_pre = vec![0i64; sz];
    let mut max_suf = vec![0i64; sz];
    let mut max_sub = vec![0i64; sz];

    let mut arr = vec![0i64; n];

    fn merge(idx: usize, sums: &mut [i64], max_pre: &mut [i64], max_suf: &mut [i64], max_sub: &mut [i64]) {
        let l = 2 * idx;
        let r = 2 * idx + 1;
        sums[idx] = sums[l] + sums[r];
        max_pre[idx] = max_pre[l].max(sums[l] + max_pre[r]);
        max_suf[idx] = max_suf[r].max(max_suf[l] + sums[r]);
        max_sub[idx] = max_sub[l].max(max_sub[r]).max(max_suf[l] + max_pre[r]);
    }

    let (mut a, mut b, mut c): (i64, i64, i64) = (0, 0, 1);
    let mut ans: i64 = 0;

    for i in 1..=l2 {
        arr[a as usize] += 2 * b - n as i64 + 1;
        if i == l1 {
            for j in 0..n {
                let idx = seg_l + j;
                sums[idx] = arr[j]; max_pre[idx] = arr[j];
                max_suf[idx] = arr[j]; max_sub[idx] = arr[j];
            }
            for j in (1..seg_l).rev() { merge(j, &mut sums, &mut max_pre, &mut max_suf, &mut max_sub); }
        } else if i > l1 {
            let idx = seg_l + a as usize;
            sums[idx] = arr[a as usize]; max_pre[idx] = arr[a as usize];
            max_suf[idx] = arr[a as usize]; max_sub[idx] = arr[a as usize];
            let mut j = idx / 2;
            while j > 0 { merge(j, &mut sums, &mut max_pre, &mut max_suf, &mut max_sub); j /= 2; }
            ans += max_sub[1];
        }
        let new_a = c;
        let new_b = (a + b + new_a) % n as i64;
        let new_c = (b + c + new_b) % n as i64;
        a = new_a; b = new_b; c = new_c;
    }
    println!("{}", ans);
}
