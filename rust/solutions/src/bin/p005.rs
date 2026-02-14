// Project Euler 5: Smallest number divisible by all 1..20
use euler_utils::lcm;

fn main() {
    let result = (1u64..=20).fold(1u64, |acc, n| lcm(acc, n));
    println!("{result}");
}
