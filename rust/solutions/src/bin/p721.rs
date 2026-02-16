// Project Euler 721 - High Powers of Irrational Numbers
//
// Matrix exponentiation for (ceil(sqrt(a)) + sqrt(a))^(a^2).
// Parallelized with rayon; u64 arithmetic (M < 10^9 so M*M < 10^18 < u64::MAX).

use rayon::prelude::*;

const N: u64 = 5_000_000;
const M: u64 = 999_999_937;

#[inline(always)]
fn mat_mult(a: &[u64; 4], b: &[u64; 4]) -> [u64; 4] {
    [
        (a[0] * b[0] % M + a[1] * b[2] % M) % M,
        (a[0] * b[1] % M + a[1] * b[3] % M) % M,
        (a[2] * b[0] % M + a[3] * b[2] % M) % M,
        (a[2] * b[1] % M + a[3] * b[3] % M) % M,
    ]
}

#[inline(always)]
fn mat_pow(mat: &[u64; 4], mut exp: u64) -> [u64; 4] {
    let mut result = [1u64, 0, 0, 1];
    let mut base = [mat[0] % M, mat[1] % M, mat[2] % M, mat[3] % M];
    while exp > 0 {
        if exp & 1 == 1 {
            result = mat_mult(&result, &base);
        }
        base = mat_mult(&base, &base);
        exp >>= 1;
    }
    result
}

#[inline(always)]
fn is_sq(n: u64) -> bool {
    let r = (n as f64).sqrt() as u64;
    // Check r-1, r, r+1 to handle floating point imprecision
    if r >= 1 && r - 1 > 0 && (r - 1) * (r - 1) == n {
        return true;
    }
    if r * r == n {
        return true;
    }
    if (r + 1) * (r + 1) == n {
        return true;
    }
    false
}

fn f(a: u64) -> u64 {
    let c = (a as f64).sqrt().ceil() as u64;
    let mat = [c, a, 1, c];
    let result = mat_pow(&mat, a * a);
    let s = 2 * result[0] % M;
    if !is_sq(a) {
        (s + M - 1) % M
    } else {
        s
    }
}

fn main() {
    let ans: u64 = (1..=N).into_par_iter().map(|a| f(a)).sum::<u64>() % M;
    println!("{}", ans);
}
