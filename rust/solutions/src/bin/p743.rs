// Project Euler 743 - Window into a Matrix
//
// Count 2xN binary matrices where every 2xK sub-matrix sums to K.

const MOD: i64 = 1_000_000_007;

fn pow_mod(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut result: i64 = 1;
    base = ((base % m) + m) % m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result as i128 * base as i128 % m as i128) as i64;
        }
        exp >>= 1;
        base = (base as i128 * base as i128 % m as i128) as i64;
    }
    result
}

fn main() {
    let n: i64 = 10_000_000_000_000_000; // 10^16
    let k: i64 = 100_000_000; // 10^8
    let m = MOD;
    let half_k = k / 2;

    // Precompute modular inverses
    let mut inv = vec![0i64; half_k as usize + 2];
    inv[1] = 1;
    for i in 2..=half_k as usize + 1 {
        inv[i] = (m - (m / i as i64) * inv[(m % i as i64) as usize] % m) % m;
    }

    // base = 2^{-2n/k} mod m
    let base = pow_mod(pow_mod(2, 2 * n / k, m), m - 2, m);

    let mut res = pow_mod(2, n, m);
    let mut ans: i64 = 0;

    for i in 0..=half_k {
        ans = (ans + res) % m;
        if 2 * i < k {
            let inv_ip1 = inv[(i + 1) as usize];
            let inv_sq = (inv_ip1 as i128 * inv_ip1 as i128 % m as i128) as i64;
            res = (res as i128 * inv_sq as i128 % m as i128) as i64;
            res = (res as i128 * ((k - 2 * i) % m) as i128 % m as i128) as i64;
            res = (res as i128 * ((k - 2 * i - 1) % m) as i128 % m as i128) as i64;
            res = (res as i128 * base as i128 % m as i128) as i64;
        }
    }

    println!("{}", ans % m);
}
