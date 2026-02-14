// Project Euler 033: Digit Cancelling Fractions
// Find denominator of the product of the four non-trivial digit-cancelling fractions.

use euler_utils::gcd;

fn main() {
    let mut num_prod: u64 = 1;
    let mut den_prod: u64 = 1;

    for n in 10u64..=98 {
        for d in (n + 1)..=99 {
            if n % 10 == 0 && d % 10 == 0 {
                continue;
            }
            let (n1, n2) = (n / 10, n % 10);
            let (d1, d2) = (d / 10, d % 10);

            // Check if cancelling a shared digit gives the same fraction
            let (sn, sd) = if n1 == d2 && d1 != 0 {
                (n2, d1)
            } else if n2 == d1 && d2 != 0 {
                (n1, d2)
            } else {
                continue;
            };

            // n/d == sn/sd  <=>  n * sd == d * sn
            if n * sd == d * sn {
                num_prod *= n;
                den_prod *= d;
            }
        }
    }

    let g = gcd(num_prod, den_prod);
    println!("{}", den_prod / g);
}
