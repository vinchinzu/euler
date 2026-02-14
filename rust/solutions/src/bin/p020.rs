// Project Euler 20: Sum of digits of 100!
use num::BigUint;

fn main() {
    let mut factorial = BigUint::from(1u32);
    for i in 2..=100u32 {
        factorial *= i;
    }
    let digit_sum: u32 = factorial.to_string().bytes().map(|b| (b - b'0') as u32).sum();
    println!("{digit_sum}");
}
