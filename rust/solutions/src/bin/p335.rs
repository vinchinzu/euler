// Project Euler 335: Gathering the beans
use euler_utils::{mod_pow, mod_mul, mod_inv};

fn main() {
    let big_n: u64 = 1_000_000_000_000_000_000; // 10^18
    let mut big_m: u64 = 1;
    for _ in 0..9 { big_m *= 7; } // 7^9

    let inv3 = mod_inv(3, big_m).unwrap();
    let inv2 = mod_inv(2, big_m).unwrap();

    // s1 = (4^{N+1} - 1) / 3 mod M
    let s1 = mod_mul((mod_pow(4, big_n + 1, big_m) + big_m - 1) % big_m, inv3, big_m);

    // s2 = 2 * (2^{N+1} - 1) mod M
    let s2 = mod_mul(2, (mod_pow(2, big_n + 1, big_m) + big_m - 1) % big_m, big_m);

    // s3 = (3^{N+1} - 1) / 2 mod M
    let s3 = mod_mul((mod_pow(3, big_n + 1, big_m) + big_m - 1) % big_m, inv2, big_m);

    let ans = (s1 + s2 + big_m - s3) % big_m;
    println!("{ans}");
}
