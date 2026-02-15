// Project Euler 876
// For k=1..18, a=6^k, b=10^k.
// Generate coprime divisor pairs (y,z), compute c = (y+z)*(a/y + b/z),
// numSteps via Euclidean algo. Track min numSteps per c.

use std::collections::HashMap;

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 { let t = b; b = a % b; a = t; }
    a
}

fn main() {
    let mut ans: i64 = 0;

    for k in 1..=18 {
        let mut a = 1i64;
        let mut b = 1i64;
        for _ in 0..k { a *= 6; b *= 10; }

        // Generate divisors of a = 2^k * 3^k
        let mut a_divs = Vec::new();
        let mut pw2 = 1i64;
        for _ in 0..=k {
            let mut pw3 = 1i64;
            for _ in 0..=k {
                a_divs.push(pw2 * pw3);
                pw3 *= 3;
            }
            pw2 *= 2;
        }

        // Generate divisors of b = 2^k * 5^k
        let mut b_divs = Vec::new();
        pw2 = 1;
        for _ in 0..=k {
            let mut pw5 = 1i64;
            for _ in 0..=k {
                b_divs.push(pw2 * pw5);
                pw5 *= 5;
            }
            pw2 *= 2;
        }

        let mut min_steps: HashMap<i64, i32> = HashMap::new();

        for &y in &a_divs {
            for &z in &b_divs {
                if gcd(y, z) != 1 { continue; }
                let c = (y + z) * (a / y + b / z);

                // Compute numSteps
                let (mut ly, mut lz) = (y, z);
                let mut num_steps = 0i32;
                let mut side = 0;
                loop {
                    if side == 0 {
                        if ly == 0 { break; }
                        num_steps += (lz / ly) as i32;
                        lz %= ly;
                        side = 1;
                    } else {
                        if lz == 0 { break; }
                        num_steps += (ly / lz) as i32;
                        ly %= lz;
                        side = 0;
                    }
                }

                let entry = min_steps.entry(c).or_insert(num_steps);
                if num_steps < *entry { *entry = num_steps; }
            }
        }

        for (&c, &num_steps) in &min_steps {
            ans += num_steps as i64;
            if c < 2 * (a + b) {
                ans += num_steps as i64 - 1;
            }
        }
    }

    println!("{}", ans);
}
