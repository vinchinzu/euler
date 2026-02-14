// Project Euler 16: Sum of digits of 2^1000
use num::BigUint;

fn main() {
    let val = BigUint::from(2u32).pow(1000);
    let digit_sum: u32 = val.to_string().bytes().map(|b| (b - b'0') as u32).sum();
    println!("{digit_sum}");
}
