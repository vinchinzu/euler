// Project Euler 48: Self powers
// Last 10 digits of 1^1 + 2^2 + ... + 1000^1000.
use euler_utils::mod_pow;

fn main() {
    let m: u64 = 10_000_000_000;
    let total: u64 = (1..=1000u64).map(|n| mod_pow(n, n, m)).sum::<u64>() % m;
    println!("{total}");
}
