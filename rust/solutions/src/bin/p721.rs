// Project Euler 721 - High Powers of Irrational Numbers
//
// Matrix exponentiation for (ceil(sqrt(a)) + sqrt(a))^(a^2).

const N: i64 = 5_000_000;
const M: i64 = 999_999_937;

fn is_sq(n: i64) -> bool {
    let r = (n as f64).sqrt() as i64;
    for rr in (r - 1).max(0)..=r + 1 {
        if rr * rr == n {
            return true;
        }
    }
    false
}

fn mat_mult(a: &[i64; 4], b: &[i64; 4]) -> [i64; 4] {
    [
        (a[0] as i128 * b[0] as i128 + a[1] as i128 * b[2] as i128).rem_euclid(M as i128) as i64,
        (a[0] as i128 * b[1] as i128 + a[1] as i128 * b[3] as i128).rem_euclid(M as i128) as i64,
        (a[2] as i128 * b[0] as i128 + a[3] as i128 * b[2] as i128).rem_euclid(M as i128) as i64,
        (a[2] as i128 * b[1] as i128 + a[3] as i128 * b[3] as i128).rem_euclid(M as i128) as i64,
    ]
}

fn mat_pow(mat: &[i64; 4], mut exp: i64) -> [i64; 4] {
    let mut result = [1, 0, 0, 1];
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

fn f(a: i64, n: i64) -> i64 {
    let c = (a as f64).sqrt().ceil() as i64;
    let mat = [c, a, 1, c];
    let result = mat_pow(&mat, n);
    let mut s = 2 * result[0] % M;
    if !is_sq(a) {
        s = (s - 1 + M) % M;
    }
    s
}

fn main() {
    let mut ans: i64 = 0;
    for a in 1..=N {
        ans = (ans + f(a, a * a)) % M;
    }
    println!("{}", ans);
}
