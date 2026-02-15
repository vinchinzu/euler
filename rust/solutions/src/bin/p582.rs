// Project Euler 582 - Nearly Isosceles 120 Degree Triangles
//
// Count integer-sided triangles with a 120-degree angle, sides a <= b <= c,
// b - a <= 100, and c <= 10^100.
//
// By the law of cosines: c^2 = a^2 + ab + b^2.
// Substituting d = b - a, x = 2c, y = 2a + d gives the Pell equation:
//   x^2 - 3*y^2 = d^2
// with constraints: x even (c = x/2 integer), y > d (a = (y-d)/2 > 0),
// and c >= b (x >= y + d, always satisfied for non-trivial solutions).
//
// Key fix: the equation x^2 - 3*y^2 = d^2 has MULTIPLE solution families.
// Starting from (d, 0) only generates one family. We must find ALL fundamental
// solutions (x0, y0) with x0 > 2*y0 (or y0 = 0), then generate each chain
// using the recurrence x' = 2x + 3y, y' = x + 2y.

use num::bigint::BigInt;
use num::Zero;

fn isqrt(n: i64) -> i64 {
    if n <= 0 {
        return 0;
    }
    let mut r = (n as f64).sqrt() as i64;
    while r * r > n {
        r -= 1;
    }
    while (r + 1) * (r + 1) <= n {
        r += 1;
    }
    r
}

/// Find all fundamental solutions to x^2 - 3*y^2 = d^2 with x > 0, y >= 0.
/// A fundamental solution has x > 2*y (inverse recurrence would give y < 0).
fn find_fundamental_solutions(d: i64) -> Vec<(i64, i64)> {
    let d2 = d * d;
    let mut solutions = Vec::new();
    // y ranges from 0 to d (since x > 2*y implies d^2 = x^2-3y^2 > 4y^2-3y^2 = y^2, so y < d)
    for y in 0..=d {
        let x2 = d2 + 3 * y * y;
        let x = isqrt(x2);
        if x * x == x2 && x > 0 {
            // Check fundamental: x > 2*y (or y == 0)
            if x > 2 * y || y == 0 {
                solutions.push((x, y));
            }
        }
    }
    solutions
}

fn main() {
    let k = 100;
    // N = 10^100
    let n_limit: BigInt = BigInt::from(10).pow(100);
    let two_n: BigInt = &n_limit * 2;

    let big_zero = BigInt::zero();
    let big_two = BigInt::from(2);
    let big_three = BigInt::from(3);

    let mut ans = 0i64;

    for d in 1..=k {
        let d_big = BigInt::from(d);
        let fund_sols = find_fundamental_solutions(d);

        for &(x0, y0) in &fund_sols {
            let mut x = BigInt::from(x0);
            let mut y = BigInt::from(y0);

            // Generate solutions until x > 2*N
            while x <= two_n {
                // Check if x is even (ensures c = x/2 is integer and y-d is even for a integer)
                if &x % &big_two == big_zero {
                    // Check y > d (which means a = (y-d)/2 >= 1)
                    if y > d_big {
                        ans += 1;
                    }
                }

                // Next solution: x' = 2x + 3y, y' = x + 2y
                let new_x = &x * &big_two + &y * &big_three;
                let new_y = &x + &y * &big_two;
                x = new_x;
                y = new_y;
            }
        }
    }

    println!("{}", ans);
}
