// Project Euler 711 - Binary Blackboard
//
// Count starting values n <= 2^N for which Eric can guarantee even popcount
// at sum 2n. N = 12345678, mod = 10^9+7.

const MOD: i64 = 1_000_000_007;

fn pow_mod(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut result: i64 = 1;
    base = ((base % m) + m) % m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result as i128 * base as i128 % m as i128) as i64;
        }
        base = (base as i128 * base as i128 % m as i128) as i64;
        exp >>= 1;
    }
    result
}

fn main() {
    let n: i64 = 12345678;
    let m = MOD;

    let mut ans: i64 = 0;

    // For even i from 2 to n-1 (step 2)
    let mut pow2_i: i64 = 4; // 2^2
    let mut pow2_half: i64 = 2; // 2^1
    let mut i = 2i64;
    while i < n {
        ans = (2 * ans % m
            + ((pow2_half - 2 + m) % m) as i128 * pow2_i as i128 % m as i128) as i64
            % m;
        ans = (ans + pow2_i - 1 + pow2_i + m) % m;

        pow2_i = pow2_i as i128 * 4 % m as i128 as i64;
        pow2_half = pow2_half * 2 % m;
        i += 2;
    }

    // For odd i from 1 to n-1 (step 2)
    let mut pow2_odd: i64 = 2; // 2^1
    let mut i = 1i64;
    while i < n {
        ans = (ans + pow2_odd - 1 + m) % m;
        pow2_odd = pow2_odd as i128 * 4 % m as i128 as i64;
        i += 2;
    }

    // Add pow2[n] - 1 and pow2[n]
    let pow2_n = pow_mod(2, n, m);
    ans = (ans + pow2_n - 1 + m) % m;
    ans = (ans + pow2_n) % m;

    println!("{}", ans);
}
