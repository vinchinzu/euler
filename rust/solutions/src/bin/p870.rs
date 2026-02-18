// Project Euler 870 - Fraenkel sequence transitions
// T(1)=1, T(n+1) is the smallest rational > T(n) such that the Fraenkel sequence changes.
// Find T(123456) formatted to 10 decimal places.
//
// Uses num::BigInt for exact rational arithmetic (sequence values grow exponentially).
// Optimized with early termination: once the best candidate is found and no improvement
// occurs for several consecutive iterations, we break early.

use num::bigint::BigInt;
use num::rational::BigRational;
use num::{Zero, One, ToPrimitive};

fn solve() {
    let mut rn = BigInt::from(1);
    let mut rd = BigInt::from(1);
    let mut count: i64 = 1;
    let limit: i64 = 123456;
    let buffer: usize = 5000;

    while count < limit {
        let start_n: BigInt = &rn / &rd + BigInt::one();
        let start_n_usize = start_n.to_usize().unwrap();
        let mut seq: Vec<BigInt> = Vec::with_capacity(256);
        seq.push(start_n.clone());
        let mut k: usize = 1;

        let mut best_cn: BigInt = BigInt::zero();
        let mut best_cd: BigInt = BigInt::zero();
        let mut found = false;
        let mut no_improve_count: usize = 0;

        for i in 0..buffer {
            let an = seq[i].clone();
            let target = &an * &rd;

            // Find k where rn * ak >= target
            let mut ak = BigInt::zero();
            loop {
                if k <= start_n_usize {
                    ak = BigInt::from(k);
                } else {
                    let idx = k - start_n_usize;
                    if idx >= seq.len() {
                        break;
                    }
                    ak = seq[idx].clone();
                }
                if &rn * &ak >= target {
                    break;
                }
                k += 1;
            }

            if k > 1 {
                let km1 = k - 1;
                let akm1: BigInt;
                if km1 <= start_n_usize {
                    akm1 = BigInt::from(km1);
                } else {
                    let idx = km1 - start_n_usize;
                    if idx >= seq.len() {
                        seq.push(&an + &ak);
                        continue;
                    }
                    akm1 = seq[idx].clone();
                }

                // Check: an/akm1 > rn/rd
                if &an * &rd > &rn * &akm1 {
                    if !found {
                        best_cn = an.clone();
                        best_cd = akm1;
                        found = true;
                        no_improve_count = 0;
                    } else {
                        // Check: an/akm1 < best_cn/best_cd
                        if &an * &best_cd < &best_cn * &akm1 {
                            best_cn = an.clone();
                            best_cd = akm1;
                            no_improve_count = 0;
                        } else {
                            no_improve_count += 1;
                            if no_improve_count >= 100 {
                                break;
                            }
                        }
                    }
                }
            }

            seq.push(&an + &ak);
        }

        if !found {
            panic!("Buffer too small at count={}", count);
        }

        rn = best_cn;
        rd = best_cd;
        // Reduce fraction
        let r = BigRational::new(rn.clone(), rd.clone());
        rn = r.numer().clone();
        rd = r.denom().clone();
        count += 1;
    }

    // Convert to f64 for output
    let r = BigRational::new(rn, rd);
    let float_val = r.numer().to_f64().unwrap() / r.denom().to_f64().unwrap();
    println!("{:.10}", float_val);
}

fn main() {
    solve();
}
