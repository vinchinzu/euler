// Project Euler 338: Cutting Rectangles

use rayon::prelude::*;

const M: i64 = 100_000_000; // 10^8

fn isqrt(n: i64) -> i64 {
    if n < 0 { return 0; }
    if n < 2 { return n; }
    let mut x = n;
    let mut y = (x + 1) / 2;
    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }
    x
}

fn sum_floor_quotients(m: i64) -> i64 {
    let s = isqrt(m);
    let mut result: i64 = 0;
    for k in 1..=s {
        result = (result + m / k) % M;
    }
    result = ((2 * result - (s % M) * (s % M) % M) % M + M) % M;
    result
}

fn num_triplets_mod(n: i64) -> i64 {
    // Collect quotient blocks into work units for parallelism
    let mut blocks: Vec<(i64, i64)> = Vec::new();
    let mut a: i64 = 1;
    while a <= n {
        let v = n / a;
        let a_end = n / v;
        blocks.push((a, a_end));
        a = a_end + 1;
    }

    let total: i64 = blocks.par_iter().map(|&(a, a_end)| {
        let v = n / a;
        let count = (a_end - a + 1) % M;
        let dfq = sum_floor_quotients(v);
        (count as i128 * dfq as i128 % M as i128) as i64 % M
    }).reduce(|| 0, |a, b| (a + b) % M);
    total
}

fn main() {
    let n: i64 = 1_000_000_000_000; // 10^12
    let l = isqrt(n);

    // Part 1: for k = 2 to L
    let ans1: i64 = (2..=l).into_par_iter().map(|k| {
        let nk = n / k % M;
        let nkm1 = n / (k - 1) % M;
        (nk as i128 * nkm1 as i128 % M as i128) as i64 % M
    }).reduce(|| 0, |a, b| (a + b) % M);

    // Part 2: for t = 1..(n/l)
    let ans2: i64 = (1..(n / l)).into_par_iter().map(|t| {
        let block = n / t - n / (t + 1);
        let val = ((((block - 1) % M) as i128 * ((t % M) as i128 * (t % M) as i128 % M as i128) % M as i128
            + (t % M) as i128 * ((t + 1) % M) as i128 % M as i128) % M as i128 + M as i128) % M as i128;
        val as i64
    }).reduce(|| 0, |a, b| (a + b) % M);

    let ans = (ans1 + ans2) % M;

    let triplets = num_triplets_mod(n);
    let sfq = sum_floor_quotients(n);

    let result = ((ans - triplets + sfq) % M + 2 * M) % M;
    println!("{}", result);
}
