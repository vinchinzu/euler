// Project Euler 56: Powerful digit sum
// Maximum digit sum of a^b for a,b < 100.
use num::BigUint;

fn digit_sum(n: &BigUint) -> u32 {
    n.to_string().bytes().map(|b| (b - b'0') as u32).sum()
}

fn main() {
    let mut max_sum = 0u32;
    for a in 2u32..100 {
        let base = BigUint::from(a);
        let mut power = BigUint::from(1u32);
        for _b in 1..100 {
            power *= &base;
            let s = digit_sum(&power);
            if s > max_sum {
                max_sum = s;
            }
        }
    }
    println!("{max_sum}");
}
