// Project Euler 338: Cutting Rectangles

use rayon::prelude::*;

const M: i64 = 100_000_000; // 10^8

#[inline(always)]
fn isqrt(n: i64) -> i64 {
    (n as f64).sqrt() as i64
}

#[inline(always)]
fn icbrt(n: i64) -> i64 {
    let mut x = (n as f64).cbrt() as i64;
    if (x + 1) * (x + 1) * (x + 1) <= n {
        x += 1;
    } else if x * x * x > n {
        x -= 1;
    }
    x
}

#[inline(always)]
fn sum_floor_quotients(m: i64) -> i64 {
    let s = isqrt(m);
    let mut s0 = 0i64;
    let mut s1 = 0i64;
    let mut s2 = 0i64;
    let mut s3 = 0i64;
    let mut k = 1i64;
    while k + 3 <= s {
        s0 += m / k;
        s1 += m / (k + 1);
        s2 += m / (k + 2);
        s3 += m / (k + 3);
        k += 4;
    }
    while k <= s {
        s0 += m / k;
        k += 1;
    }
    let total = s0 + s1 + s2 + s3;
    let sm = s % M;
    ((2 * total - sm * sm) % M + M) % M
}

fn num_triplets_mod(n: i64) -> i64 {
    let k = icbrt(n);

    // Term 1: 3 * sum_{a=1}^k D(n/a)
    let term1_sum: i64 = (1..=k)
        .into_par_iter()
        .map(|a| sum_floor_quotients(n / a))
        .sum::<i64>() % M;
    let term1 = (3 * term1_sum) % M;

    // Term 2: 3 * sum_{a=1}^k sum_{b=1}^k floor(n / (a*b))
    let mut diag = 0i64;
    for a in 1..=k {
        diag = (diag + (n / (a * a))) % M;
    }

    let off_sum: i64 = (1..k)
        .into_par_iter()
        .map(|a| {
            let q = n / a;
            let mut s0 = 0i64;
            let mut s1 = 0i64;
            let mut s2 = 0i64;
            let mut s3 = 0i64;
            let mut b = a + 1;
            while b + 3 <= k {
                s0 += q / b;
                s1 += q / (b + 1);
                s2 += q / (b + 2);
                s3 += q / (b + 3);
                b += 4;
            }
            while b <= k {
                s0 += q / b;
                b += 1;
            }
            (s0 + s1 + s2 + s3) % M
        })
        .sum::<i64>() % M;

    let term2 = (3 * (diag + 2 * off_sum)) % M;
    let km = k % M;
    let term3 = (km * km % M * km) % M;

    ((term1 - term2 + term3) % M + 2 * M) % M
}

fn main() {
    let n: i64 = 1_000_000_000_000; // 10^12
    let l = isqrt(n);

    // Part 1: for k = 2 to L
    let ans1: i64 = (2..=l)
        .into_par_iter()
        .map(|k| {
            let nk = (n / k) % M;
            let nkm1 = (n / (k - 1)) % M;
            (nk * nkm1) % M
        })
        .sum::<i64>() % M;

    // Part 2: for t = 1..(n/l)
    let ans2: i64 = (1..(n / l))
        .into_par_iter()
        .map(|t| {
            let block = (n / t - n / (t + 1)) % M;
            let tm = t % M;
            let tm_sq = (tm * tm) % M;
            let t_tp1 = (tm * ((tm + 1) % M)) % M;
            ((block - 1 + M) % M * tm_sq + t_tp1) % M
        })
        .sum::<i64>() % M;

    let ans = (ans1 + ans2) % M;

    let triplets = num_triplets_mod(n);
    let sfq = sum_floor_quotients(n);

    let result = ((ans - triplets + sfq) % M + 2 * M) % M;
    println!("{}", result);
}
