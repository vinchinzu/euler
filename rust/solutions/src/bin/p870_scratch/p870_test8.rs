// Compute T(123456) using the correct algorithm with BigInt for exact arithmetic.

use num::bigint::BigInt;
use num::integer::Integer;
use num::{Zero, One, ToPrimitive};

fn main() {
    let limit: usize = 123456;
    let mut rn = BigInt::one();
    let mut rd = BigInt::one();

    for ti in 1..=limit {
        if ti == limit {
            let r_float = rn.to_f64().unwrap() / rd.to_f64().unwrap();
            println!("{:.10}", r_float);
            break;
        }

        if ti <= 30 || ti % 10000 == 0 {
            let r_float = rn.to_f64().unwrap_or(f64::INFINITY) / rd.to_f64().unwrap_or(1.0);
            eprintln!("T({}) = {:.10}", ti, r_float);
        }

        // Compute losing sequence for r = rn/rd + eps
        let mut p: Vec<BigInt> = vec![BigInt::zero(), BigInt::one()]; // 1-indexed
        let mut j_lo = 1usize;

        let mut best_n = BigInt::zero();
        let mut best_d = BigInt::zero();
        let mut best_set = false;
        let mut no_improve = 0usize;
        let max_no_improve = 50;

        loop {
            let k = p.len() - 1;
            let pk = p[k].clone();
            let target = &pk * &rd;

            let mut m = j_lo;
            while m < p.len() {
                if &rn * &p[m] >= target {
                    break;
                }
                m += 1;
            }
            if m >= p.len() { break; }

            let next = &pk + &p[m];
            p.push(next);
            j_lo = m;

            // Check transition candidate: pk / p[m-1]
            if m >= 2 {
                let cand_d = &p[m - 1];
                // Check pk / cand_d > rn / rd
                if &pk * &rd > &rn * cand_d {
                    if !best_set || &pk * &best_d < &best_n * cand_d {
                        best_n = pk;
                        best_d = cand_d.clone();
                        best_set = true;
                        no_improve = 0;
                    } else {
                        no_improve += 1;
                        if no_improve >= max_no_improve {
                            break;
                        }
                    }
                }
            }
        }

        if !best_set {
            eprintln!("ERROR: Could not find transition at step {}", ti);
            break;
        }

        let g = best_n.gcd(&best_d);
        rn = best_n / &g;
        rd = best_d / g;
    }
}
