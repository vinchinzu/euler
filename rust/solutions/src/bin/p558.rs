// Project Euler 558 - Irrational Base
//
// Uses BigUint for big integer arithmetic (tribonacci-like sequence grows
// beyond 64-bit range).

use num::BigUint;
use rayon::prelude::*;

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

    let a_l = &a[L_VAL];

    let ans: i64 = (1..=N_VAL).into_par_iter().map(|j| {
        let j_big = BigUint::from(j);
        let mut target = a_l * &j_big * &j_big;
        let mut count = 0i64;

        for i in (0..TOTAL).rev() {
            if target >= a[i] {
                target -= &a[i];
                count += 1;
            }
        }
        count
    }).sum();

    println!("{ans}");
}
