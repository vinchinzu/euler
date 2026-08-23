// Project Euler 733 - Ascending Subsequences
//
// 3-layer interleaved u32 Fenwick (k=1,2,3). k=4 is a scalar accumulator.

const MOD: u32 = 1_000_000_007;
const MOD64: u64 = 1_000_000_007;
const N: usize = 1_000_000;
const SEQ_MOD: u64 = 10_000_019;
const SEQ_MOD_US: usize = 10_000_019;

#[repr(C)]
#[derive(Clone, Copy)]
struct Node {
    c1: u32,
    s1: u32,
    c2: u32,
    s2: u32,
    c3: u32,
    s3: u32,
}

#[inline(always)]
fn add_mod(a: u32, b: u32) -> u32 {
    let s = a + b;
    if s >= MOD {
        s - MOD
    } else {
        s
    }
}

#[inline(always)]
fn madd(cnt: u32, val: u32, sm: u32) -> u32 {
    ((cnt as u64 * val as u64 + sm as u64) % MOD64) as u32
}

#[inline(always)]
unsafe fn bit_query(tree: *const Node, mut i: u32) -> Node {
    let mut c1 = 0u64;
    let mut s1 = 0u64;
    let mut c2 = 0u64;
    let mut s2 = 0u64;
    let mut c3 = 0u64;
    let mut s3 = 0u64;
    while i > 0 {
        // SAFETY: Fenwick index i is in 1..=m; tree has m+1 nodes
        let e = unsafe { &*tree.add(i as usize) };
        c1 += e.c1 as u64;
        s1 += e.s1 as u64;
        c2 += e.c2 as u64;
        s2 += e.s2 as u64;
        c3 += e.c3 as u64;
        s3 += e.s3 as u64;
        i -= i & i.wrapping_neg();
    }
    Node {
        c1: (c1 % MOD64) as u32,
        s1: (s1 % MOD64) as u32,
        c2: (c2 % MOD64) as u32,
        s2: (s2 % MOD64) as u32,
        c3: (c3 % MOD64) as u32,
        s3: (s3 % MOD64) as u32,
    }
}

#[inline(always)]
unsafe fn bit_update(tree: *mut Node, mut i: u32, n: u32, d: Node) {
    while i <= n {
        // SAFETY: Fenwick index i stays in 1..=n; tree has n+1 nodes
        let e = unsafe { &mut *tree.add(i as usize) };
        e.c1 = add_mod(e.c1, d.c1);
        e.s1 = add_mod(e.s1, d.s1);
        e.c2 = add_mod(e.c2, d.c2);
        e.s2 = add_mod(e.s2, d.s2);
        e.c3 = add_mod(e.c3, d.c3);
        e.s3 = add_mod(e.s3, d.s3);
        i += i & i.wrapping_neg();
    }
}

fn main() {
    let mut seq = vec![0u32; N];
    {
        let mut x = 153u64;
        for i in 0..N {
            seq[i] = x as u32;
            x = x * 153 % SEQ_MOD;
        }
    }

    // Direct rank via bitset prefix — values live in 0..SEQ_MOD
    let mut bits = vec![0u64; (SEQ_MOD_US + 63) / 64];
    for &v in &seq {
        bits[v as usize >> 6] |= 1u64 << (v & 63);
    }
    let mut prefix = vec![0u32; bits.len() + 1];
    for i in 0..bits.len() {
        prefix[i + 1] = prefix[i] + bits[i].count_ones();
    }
    let m = prefix[bits.len()] as usize;

    let mut ranks = vec![0u32; N];
    for i in 0..N {
        let v = seq[i] as usize;
        let hi = v >> 6;
        let lo = v & 63;
        ranks[i] = prefix[hi] + (bits[hi] & ((1u64 << lo) - 1)).count_ones() + 1;
    }
    drop(bits);
    drop(prefix);

    let mut tree = vec![0u32; (m + 1) * 6];
    let tp = tree.as_mut_ptr() as *mut Node;
    let m32 = m as u32;
    let mut ans = 0u32;

    for i in 0..N {
        let val = unsafe { *seq.get_unchecked(i) };
        let r = unsafe { *ranks.get_unchecked(i) };
        let q = unsafe { bit_query(tp, r - 1) };

        ans = add_mod(ans, madd(q.c3, val, q.s3));

        let d = Node {
            c1: 1,
            s1: val,
            c2: q.c1,
            s2: madd(q.c1, val, q.s1),
            c3: q.c2,
            s3: madd(q.c2, val, q.s2),
        };
        unsafe {
            bit_update(tp, r, m32, d);
        }
    }

    println!("{}", ans);
}
