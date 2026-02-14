// Project Euler 97: Large non-Mersenne prime
// Last 10 digits of 28433 * 2^7830457 + 1.

use euler_utils::mod_pow;

fn main() {
    let m: u64 = 10_000_000_000; // 10^10
    let power = mod_pow(2, 7830457, m);
    let result = ((28433u128 * power as u128 + 1) % m as u128) as u64;
    println!("{result}");
}
