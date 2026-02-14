// Project Euler 582 - Nearly Isosceles 120 Degree Triangles
//
// Count integer-sided triangles with 120 degree angle and b-a <= 100, c <= 10^100.
// Uses Pell equation x^2 - 3*y^2 = d^2 with big integer arithmetic.

use num::bigint::BigInt;
use num::Zero;

fn main() {
    let k = 100;
    // N = 10^100
    let n_limit: BigInt = BigInt::from(10).pow(100);
    let two_n: BigInt = &n_limit * 2;

    let mut ans = 0i64;

    for d in 1..=k {
        let d_big = BigInt::from(d);
        let mut x = BigInt::from(d);
        let mut y = BigInt::zero();

        // Generate solutions until x > 2*N
        while x <= two_n {
            // Check if x is even
            if &x % 2 == BigInt::zero() {
                // Check y > d (which means a > 0)
                if y > d_big {
                    ans += 1;
                }
            }

            // Next solution: x' = 2x + 3y, y' = x + 2y
            let new_x = &x * 2 + &y * 3;
            let new_y = &x + &y * 2;
            x = new_x;
            y = new_y;
        }
    }

    println!("{}", ans);
}
