// Project Euler 570 - Snowflakes
//
// GCD(2*4^{n-2} - 3^{n-2}, 7n+3) summed for n=3..10^7, times 6.

fn mod_pow(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
    let mut result: i64 = 1;
    base = base.rem_euclid(modulus);
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result as i128 * base as i128 % modulus as i128) as i64;
        }
        base = (base as i128 * base as i128 % modulus as i128) as i64;
        exp >>= 1;
    }
    result
}

fn gcd_ll(mut a: i64, mut b: i64) -> i64 {
    if a < 0 { a = -a; }
    if b < 0 { b = -b; }
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn main() {
    use rayon::prelude::*;
    let n_max: i64 = 10_000_000;
    let ans: i64 = (3..=n_max).into_par_iter().map(|n| {
        let m = 7 * n + 3;
        let t1 = mod_pow(4, n - 2, m);
        let t2 = mod_pow(3, n - 2, m);
        let term = (2 * t1 - t2 + m) % m;
        let g = gcd_ll(term, m);
        6 * g
    }).sum();
    println!("{}", ans);
}
