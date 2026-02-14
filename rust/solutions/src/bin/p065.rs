// Project Euler 65: Convergents of e
// Sum of digits of the numerator of the 100th convergent of e.

use num::BigUint;
use num::traits::One;
use num::Zero;

fn main() {
    let num_terms = 100;

    // CF coefficients for e: [2; 1,2,1, 1,4,1, 1,6,1, ...]
    let mut a = vec![0u32; num_terms];
    a[0] = 2;
    for k in 1..num_terms {
        if (k + 1) % 3 == 0 {
            a[k] = 2 * ((k + 1) / 3) as u32;
        } else {
            a[k] = 1;
        }
    }

    // Convergent recurrence: p_k = a_k * p_{k-1} + p_{k-2}
    let mut p_prev2 = BigUint::zero();
    let mut p_prev1 = BigUint::one();

    for k in 0..num_terms {
        let ak = BigUint::from(a[k]);
        let p_curr = &ak * &p_prev1 + &p_prev2;
        p_prev2 = p_prev1;
        p_prev1 = p_curr;
    }

    let digit_sum: u32 = p_prev1.to_string().chars().map(|c| c.to_digit(10).unwrap()).sum();
    println!("{digit_sum}");
}
