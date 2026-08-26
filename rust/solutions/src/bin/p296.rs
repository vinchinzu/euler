// Project Euler 296: Angular Bisector and Tangent, n = 100_000.
// Farey next-neighbor walk over reduced (p, q), split into independent
// coarse-Farey intervals and processed in parallel. Inner k-sum is closed
// form (O(1) half-sum + lattice floor_sum) when kk is large.

use rayon::prelude::*;

const N: i32 = 100_000;
const L: i32 = N / 6;
const COARSE: i32 = 80;

/// sum_{i=0}^{n-1} floor((a*i + b) / m)
#[inline(always)]
fn floor_sum(mut n: i64, mut m: i64, mut a: i64, mut b: i64) -> i64 {
    let mut ans = 0i64;
    loop {
        if a >= m {
            ans += n * (n - 1) / 2 * (a / m);
            a %= m;
        }
        if b >= m {
            ans += n * (b / m);
            b %= m;
        }
        let y_max = a * n + b;
        if y_max < m {
            break;
        }
        n = y_max / m;
        b = y_max % m;
        core::mem::swap(&mut m, &mut a);
    }
    ans
}

/// sum_{j=1}^n floor(j/2); 0 for n <= 0.
#[inline(always)]
fn sum_floor_half(n: i64) -> i64 {
    if n <= 0 {
        0
    } else {
        (n >> 1) * ((n + 1) >> 1)
    }
}

#[inline(always)]
fn inner_closed(k_big: i32, kk: i32, p: i32, q: i32) -> i32 {
    let k_big = k_big as i64;
    let kk = kk as i64;
    let p = p as i64;
    let q = q as i64;
    let sum_max_y = sum_floor_half(k_big - 1) - sum_floor_half(k_big - kk - 1);
    let sum_min_y = floor_sum(kk, p, q, q + p - 1);
    (sum_max_y - sum_min_y + kk) as i32
}

#[inline(always)]
fn process(p: i32, q: i32) -> i32 {
    if p > q {
        return 0;
    }
    let pq = p + q;
    let p2q = p + q + q;
    if (pq as i64) * (p2q as i64) > (p as i64) * (N as i64) {
        return 0;
    }
    let k_big = N / pq;
    let kk = p * k_big / p2q;
    if kk > 8 {
        inner_closed(k_big, kk, p, q)
    } else if kk > 0 {
        let mut s = 0i32;
        let mut k = 1i32;
        while k <= kk {
            s += (k_big - k) / 2 - (k * q + p - 1) / p + 1;
            k += 1;
        }
        s
    } else {
        0
    }
}

fn ext_gcd(mut a: i32, mut b: i32) -> (i32, i32) {
    let (mut x, mut x1) = (1i32, 0i32);
    while b != 0 {
        let q = a / b;
        let t = b;
        b = a - q * b;
        a = t;
        let t = x1;
        x1 = x - q * x1;
        x = t;
    }
    (a, x)
}

/// Next term after a/b in the Farey sequence of order n.
fn successor(a: i32, b: i32, n: i32) -> (i32, i32) {
    if a == 0 {
        return (1, n);
    }
    let (_, inv) = ext_gcd(a, b);
    let inv = ((inv % b) + b) % b;
    let d0 = (b - inv) % b;
    let d = d0 + ((n - d0) / b) * b;
    let c = ((1 + a as i64 * d as i64) / b as i64) as i32;
    (c, d)
}

fn walk_segment(old_p0: i32, old_q0: i32, end_p: i32, end_q: i32) -> i32 {
    let (mut old_p, mut old_q) = (old_p0, old_q0);
    let (mut p, mut q) = successor(old_p, old_q, L);
    let mut ans = 0i32;
    loop {
        ans += process(p, q);
        if p == end_p && q == end_q {
            break;
        }
        let num = L + old_q;
        let med = if num < q + q {
            1
        } else if num < q * 3 {
            2
        } else {
            num / q
        };
        let new_p = med * p - old_p;
        let new_q = med * q - old_q;
        old_p = p;
        old_q = q;
        p = new_p;
        q = new_q;
    }
    ans
}

fn main() {
    let mut segs: Vec<(i32, i32, i32, i32)> = Vec::with_capacity(4000);
    let (mut old_p, mut old_q) = (0i32, 1i32);
    let (mut p, mut q) = (1i32, COARSE);
    while p <= q {
        segs.push((old_p, old_q, p, q));
        let med = (COARSE + old_q) / q;
        let new_p = med * p - old_p;
        let new_q = med * q - old_q;
        old_p = p;
        old_q = q;
        p = new_p;
        q = new_q;
    }

    let ans: i32 = segs
        .into_par_iter()
        .map(|(op, oq, ep, eq)| walk_segment(op, oq, ep, eq))
        .sum();

    println!("{}", ans);
}
