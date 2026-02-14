// Project Euler 558 - Irrational Base
//
// Uses BigUint for big integer arithmetic (tribonacci-like sequence grows
// beyond 64-bit range).

use num::BigUint;

const N_VAL: u64 = 5_000_000;
const L_VAL: usize = 200;
const TOTAL: usize = 2 * L_VAL;

fn main() {
    let mut a: Vec<BigUint> = Vec::with_capacity(TOTAL);
    for i in 0..TOTAL {
        if i < 3 {
            a.push(BigUint::from((i + 1) as u64));
        } else {
            let val = &a[i - 1] + &a[i - 3];
            a.push(val);
        }
    }

    let mut ans: i64 = 0;

    for j in 1..=N_VAL {
        let mut target = &a[L_VAL] * BigUint::from(j) * BigUint::from(j);
        let mut count = 0i64;

        for i in (0..TOTAL).rev() {
            if target >= a[i] {
                target -= &a[i];
                count += 1;
            }
        }
        ans += count;
    }

    println!("{ans}");
}
