// Project Euler Problem 832: Mex Sequence
// Compute M(10^18) mod 10^9+7

use num::BigInt;
use num_traits::{One, Zero};

const MOD: i64 = 1_000_000_007;

fn m(n: i64, a: i64, b: i64, c: i64, i: i32) -> i64 {
    if n <= 0 {
        return 0;
    }

    let power = 4i64.pow(i as u32);

    if power < n {
        // M(power) + M'(n - power)
        let m1 = m(power, a, b, c, i);
        let m2 = m(n - power, a * 4, b * 4, c * 4, i + 1);
        return (m1 + m2) % MOD;
    } else if power == n {
        // Sum of the entire block
        // Use BigInt to avoid overflow
        let a_big = BigInt::from(a);
        let b_big = BigInt::from(b);
        let c_big = BigInt::from(c);
        let p_big = BigInt::from(power);
        let mod_big = BigInt::from(MOD);

        // sum_a = (a + power - 1) * (a + power) / 2 - (a - 1) * a / 2
        let sum_a = (&a_big + &p_big - 1) * (&a_big + &p_big) / 2 - (&a_big - 1) * &a_big / 2;
        let sum_b = (&b_big + &p_big - 1) * (&b_big + &p_big) / 2 - (&b_big - 1) * &b_big / 2;
        let sum_c = (&c_big + &p_big - 1) * (&c_big + &p_big) / 2 - (&c_big - 1) * &c_big / 2;

        let total: BigInt = (sum_a + sum_b + sum_c) % mod_big;
        total.to_string().parse::<i64>().unwrap()
    } else {
        let s_power = power / 4;

        let sum_1 = m(n.min(s_power), a, b, c, i - 1);
        let sum_2 = m(
            (n - s_power).min(s_power).max(0),
            a + s_power,
            b + 2 * s_power,
            c + 3 * s_power,
            i - 1,
        );
        let sum_3 = m(
            (n - 2 * s_power).min(s_power).max(0),
            a + 2 * s_power,
            b + 3 * s_power,
            c + s_power,
            i - 1,
        );
        let sum_4 = m(
            (n - 3 * s_power).min(s_power).max(0),
            a + 3 * s_power,
            b + s_power,
            c + 2 * s_power,
            i - 1,
        );

        (sum_1 + sum_2 + sum_3 + sum_4) % MOD
    }
}

fn compute_m(n: i64) -> i64 {
    m(n, 1, 2, 3, 0)
}

fn main() {
    // Test: M(10) = 642
    assert_eq!(compute_m(10), 642);

    // Test: M(1000) = 5432148
    assert_eq!(compute_m(1000), 5432148);

    println!("{}", compute_m(10i64.pow(18)));
}
