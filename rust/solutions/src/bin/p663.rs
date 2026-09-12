// Project Euler 663 - Sums of Subarrays
// Segment tree for max subarray sum with tribonacci updates.
// Packed AoS nodes; parallel bottom-up build after the first L1 updates.

use rayon::prelude::*;

#[derive(Clone, Copy)]
struct Node {
    sum: i64,
    pre: i64,
    suf: i64,
    sub: i64,
}

#[inline(always)]
fn merge(l: Node, r: Node) -> Node {
    Node {
        sum: l.sum + r.sum,
        pre: l.pre.max(l.sum + r.pre),
        suf: r.suf.max(l.suf + r.sum),
        sub: l.sub.max(r.sub).max(l.suf + r.pre),
    }
}

fn main() {
    let n: usize = 10_000_003;
    let l1: usize = 10_000_000;
    let l2: usize = 10_200_000;
    let n_i64 = n as i64;

    let mut seg_l = 1usize;
    while seg_l < n {
        seg_l *= 2;
    }
    let mut tree = vec![Node { sum: 0, pre: 0, suf: 0, sub: 0 }; 2 * seg_l];
    let mut arr = vec![0i64; n];

    let (mut a, mut b, mut c): (i64, i64, i64) = (0, 0, 1);
    let mut ans: i64 = 0;

    for i in 1..=l2 {
        // SAFETY: a is always in 0..n (tribonacci % n)
        unsafe {
            *arr.get_unchecked_mut(a as usize) += 2 * b - n_i64 + 1;
        }
        if i == l1 {
            let leaves = &mut tree[seg_l..seg_l + n];
            leaves
                .par_iter_mut()
                .zip(arr.par_iter())
                .with_min_len(4096)
                .for_each(|(node, &v)| {
                    *node = Node {
                        sum: v,
                        pre: v,
                        suf: v,
                        sub: v,
                    };
                });
            let mut start = seg_l / 2;
            loop {
                let (rest, children) = tree.split_at_mut(start * 2);
                let parents = &mut rest[start..];
                parents
                    .par_iter_mut()
                    .enumerate()
                    .with_min_len(1024)
                    .for_each(|(k, node)| {
                        // SAFETY: children is tree[2*start..]; 2k, 2k+1 < children.len()
                        // because k < start and children covers [2*start, 4*start).
                        let l = unsafe { *children.get_unchecked(2 * k) };
                        let r = unsafe { *children.get_unchecked(2 * k + 1) };
                        *node = merge(l, r);
                    });
                if start == 1 {
                    break;
                }
                start /= 2;
            }
        } else if i > l1 {
            let ai = a as usize;
            let val = unsafe { *arr.get_unchecked(ai) };
            let mut idx = seg_l + ai;
            tree[idx] = Node {
                sum: val,
                pre: val,
                suf: val,
                sub: val,
            };
            idx /= 2;
            while idx > 0 {
                // SAFETY: idx >= 1, 2*idx+1 < 2*seg_l
                let l = unsafe { *tree.get_unchecked(2 * idx) };
                let r = unsafe { *tree.get_unchecked(2 * idx + 1) };
                unsafe {
                    *tree.get_unchecked_mut(idx) = merge(l, r);
                }
                idx /= 2;
            }
            ans += unsafe { tree.get_unchecked(1).sub };
        }
        let new_a = c;
        let new_b = (a + b + new_a) % n_i64;
        let new_c = (b + c + new_b) % n_i64;
        a = new_a;
        b = new_b;
        c = new_c;
    }
    println!("{}", ans);
}
