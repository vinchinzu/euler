// Project Euler 711 - Binary Blackboard
//
// Count starting values n <= 2^N for which Eric can guarantee even popcount
// at sum 2n. N = 12345678, mod = 10^9+7.

const MOD: i64 = 1_000_000_007;

fn pow_mod(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut result: i64 = 1;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % m;
        }
        base = base * base % m;
        exp >>= 1;
    }
    result
}

fn main() {
    let n: i64 = 12345678;
    let m = MOD;

    let mut ans: i64 = 0;
    let mut pow2_term: i64 = 8;
    
    const BATCH: i32 = 30;
    let mut batch_count = 0;

    let mut i = 2i64;
    while i < n {
        ans = 2 * ans + pow2_term - 1;
        pow2_term = pow2_term * 8 % m;
        i += 2;
        batch_count += 1;
        
        if batch_count == BATCH {
            ans %= m;
            batch_count = 0;
        }
    }
    if batch_count > 0 {
        ans %= m;
    }

    let mut pow2_odd: i64 = 2;
    batch_count = 0;
    
    i = 1;
    while i < n {
        ans += pow2_odd - 1;
        pow2_odd = pow2_odd * 4 % m;
        i += 2;
        batch_count += 1;
        
        if batch_count == BATCH {
            ans %= m;
            batch_count = 0;
        }
    }
    if batch_count > 0 {
        ans %= m;
    }

    let pow2_n = pow_mod(2, n, m);
    ans = (ans + pow2_n - 1 + m) % m;
    ans = (ans + pow2_n) % m;

    println!("{}", ans);
}
