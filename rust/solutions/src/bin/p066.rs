// Project Euler 66: Diophantine equation
// Find D <= 1000 that maximizes x in Pell's equation x^2 - D*y^2 = 1.

use num::BigUint;
use num::traits::One;
use num::Zero;

fn main() {
    let mut max_x = BigUint::zero();
    let mut result_d = 0u32;

    for d in 2..=1000u32 {
        let a0 = (d as f64).sqrt() as u32;
        if a0 * a0 == d { continue; }

        // Compute CF period length
        let period = {
            let mut m = 0u32;
            let mut dd = 1u32;
            let mut a = a0;
            let mut len = 0u32;
            loop {
                m = dd * a - m;
                dd = (d - m * m) / dd;
                a = (a0 + m) / dd;
                len += 1;
                if a == 2 * a0 { break; }
            }
            len
        };

        // For even period, process 1 full period. For odd period, process 2 full periods.
        let reps = if period % 2 == 0 { 1 } else { 2 };

        let mut pk2 = BigUint::zero();
        let mut pk1 = BigUint::one();
        let mut qk2 = BigUint::one();
        let mut qk1 = BigUint::zero();

        // Process a0 term
        {
            let a_big = BigUint::from(a0);
            let cp = &a_big * &pk1 + &pk2;
            let cq = &a_big * &qk1 + &qk2;
            pk2 = pk1; pk1 = cp;
            qk2 = qk1; qk1 = cq;
        }

        for _ in 0..reps {
            let mut m = 0u32;
            let mut dd = 1u32;
            let mut a = a0;
            for _ in 0..period {
                m = dd * a - m;
                dd = (d - m * m) / dd;
                a = (a0 + m) / dd;

                let a_big = BigUint::from(a);
                let cp = &a_big * &pk1 + &pk2;
                let cq = &a_big * &qk1 + &qk2;
                pk2 = pk1; pk1 = cp;
                qk2 = qk1; qk1 = cq;
            }
        }

        if pk1 > max_x {
            max_x = pk1;
            result_d = d;
        }
    }

    println!("{result_d}");
}
