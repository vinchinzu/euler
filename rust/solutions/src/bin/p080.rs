// Project Euler 80: Square root digital expansion
// For the first 100 natural numbers, sum the first 100 decimal digits of sqrt(n)
// for each non-perfect-square n. Uses num::BigUint for arbitrary precision.

use num::BigUint;
use num::traits::One;
use num::Zero;

fn big_sqrt(n: &BigUint) -> BigUint {
    if n.is_zero() { return BigUint::zero(); }

    // Newton's method: x_{k+1} = (x_k + n/x_k) / 2
    // Start with a power-of-2 upper bound
    let bits = n.bits();
    let mut x = BigUint::one() << ((bits + 1) / 2);

    loop {
        let q = n / &x;
        let sum = &x + &q;
        let new_x = &sum >> 1;
        if new_x >= x { break; }
        x = new_x;
    }

    // Ensure x*x <= n
    if &x * &x > *n {
        x -= BigUint::one();
    }
    x
}

fn main() {
    let mut total = 0u64;

    // We need 100 decimal digits of precision, so multiply n by 10^198
    // (since sqrt(n * 10^198) = sqrt(n) * 10^99, giving 100 digits)
    let ten = BigUint::from(10u32);
    let mut scale = BigUint::one();
    for _ in 0..198 {
        scale = &scale * &ten;
    }

    for n in 1..=100u32 {
        let rt = (n as f64).sqrt() as u32;
        if rt * rt == n { continue; }

        let big_n = BigUint::from(n) * &scale;
        let root = big_sqrt(&big_n);
        let s = root.to_string();

        // Sum first 100 digits
        let sum: u64 = s.chars().take(100).map(|c| c.to_digit(10).unwrap() as u64).sum();
        total += sum;
    }

    println!("{total}");
}
