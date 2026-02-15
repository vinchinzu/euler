// Project Euler 741 - Binary Grid Colourings
//
// Burnside's lemma with rotation/reflection symmetries on grid colourings.

const M: i64 = 1_000_000_007;
const N1: i32 = 823543;   // 7^7
const N2: i32 = 16777216; // 8^8

fn pow_mod(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut result: i64 = 1;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result as i128 * base as i128 % m as i128) as i64;
        }
        exp >>= 1;
        base = (base as i128 * base as i128 % m as i128) as i64;
    }
    result
}

fn mod_inv(a: i64, m: i64) -> i64 {
    pow_mod(a, m - 2, m)
}

fn f(n: i32) -> i64 {
    let n = n as usize;
    let mut f_arr = vec![0i64; n + 1];
    let mut fp = vec![0i64; n + 1];
    f_arr[0] = 1;
    for k in 2..=n {
        fp[k] = (k as i64 * ((k - 1) as i64) % M
            * (((k - 1) as i128 * f_arr[k - 2] as i128 % M as i128 + fp[k - 1] as i128) % M as i128) as i64
            % M) % M;
        f_arr[k] = ((M + 1) / 2 as i128 * fp[k] as i128 % M as i128) as i64;
    }
    f_arr[n]
}

fn rotate90(n: i32) -> i64 {
    if n % 2 == 1 { return 0; }
    let n = n as usize;
    let mut f_arr = vec![0i64; n + 1];
    let mut fp = vec![0i64; n + 1];
    f_arr[0] = 1;
    if n >= 2 { f_arr[2] = 1; }
    let mut k = 4;
    while k <= n {
        fp[k] = ((k - 2) as i128 * ((f_arr[k - 4] as i128 + fp[k - 2] as i128) % M as i128) % M as i128) as i64;
        f_arr[k] = (f_arr[k - 2] as i128
            + (k / 2 - 1) as i128 * f_arr[k - 4] as i128 % M as i128
            + (k - 2) as i128 * fp[k - 2] as i128 % M as i128)
            as i64 % M;
        k += 2;
    }
    f_arr[n]
}

fn rotate180(n: i32) -> i64 {
    let n = n as usize;
    let mut f_arr = vec![0i64; n + 1];
    let mut fp = vec![0i64; n + 1];
    f_arr[0] = 1;
    if n >= 2 { f_arr[2] = 1; fp[2] = 2; }
    for k in 3..=n {
        if k % 2 == 0 {
            fp[k] = (k as i128 * (((k - 2) as i128 * f_arr[k - 4] as i128 % M as i128 + fp[k - 2] as i128) % M as i128 * (k - 2) as i128 % M as i128 + f_arr[k - 2] as i128) % M as i128 % M as i128) as i64 % M;
            f_arr[k] = ((M + 1) / 2 as i128 * fp[k] as i128 % M as i128) as i64;
        } else {
            fp[k] = ((k - 1) as i128 * ((f_arr[k - 3] as i128 + (k - 3) as i128 * fp[k - 2] as i128 % M as i128) % M as i128) % M as i128) as i64 % M;
            f_arr[k] = ((k / 2) as i128 * fp[k] as i128 % M as i128) as i64;
        }
    }
    f_arr[n]
}

fn flip_y(n: i32) -> i64 {
    if n % 2 == 1 { return 0; }
    let n = n as usize;
    let mut fact: i64 = 1;
    for i in 1..=n {
        fact = (fact as i128 * i as i128 % M as i128) as i64;
    }
    (fact as i128 * mod_inv(pow_mod(2, n as i64 / 2, M), M) as i128 % M as i128) as i64
}

fn flip_diagonal(n: i32) -> i64 {
    let n = n as usize;
    let mut f_arr = vec![0i64; n + 1];
    let mut fp = vec![0i64; n + 1];
    let mut fpp = vec![0i64; n + 1];
    f_arr[0] = 1;
    if n >= 1 { fp[1] = 1; }
    for k in 2..=n {
        fp[k] = (f_arr[k - 1] as i128 + (k - 1) as i128 * fp[k - 1] as i128 % M as i128) as i64 % M;
        fpp[k] = (f_arr[k - 2] as i128 + fp[k - 1] as i128 + (k - 2) as i128 * fpp[k - 1] as i128 % M as i128) as i64 % M;
        let ncr = (k as i64 - 1) * (k as i64 - 2) / 2 % M;
        f_arr[k] = ((k - 1) as i128 * fp[k - 1] as i128 % M as i128 + ncr as i128 * fpp[k - 1] as i128 % M as i128) as i64 % M;
    }
    f_arr[n]
}

fn g(n: i32) -> i64 {
    let val = (f(n) + 2 * rotate90(n) % M + rotate180(n) + 2 * flip_y(n) % M + 2 * flip_diagonal(n) % M) % M;
    (val as i128 * mod_inv(8, M) as i128 % M as i128) as i64
}

fn main() {
    let ans = (g(N1) + g(N2)) % M;
    println!("{}", ans);
}
