// Project Euler 57: Square root convergents
// Count how many of the first 1000 expansions of sqrt(2) have a numerator with
// more digits than the denominator.
use num::BigInt;

fn num_digits(n: &BigInt) -> usize {
    n.to_string().trim_start_matches('-').len()
}

fn main() {
    let two = BigInt::from(2);

    // Track n and d where the fraction is (n + d) / d = 1 + n/d
    // Recurrence: new_n = n + 2*d, new_d = n + d
    let mut n = BigInt::from(1);
    let mut d = BigInt::from(1);
    let mut count = 0;

    for _ in 1..=1000 {
        let new_n = &n + &two * &d;
        let new_d = &n + &d;
        // The actual fraction is (new_n + new_d) / new_d ... but the C code
        // counts when numerator digits > denominator digits using cnlen > cdlen
        // where cn = n + 2*d, cd = n + d. So the "numerator" of the convergent
        // is n + 2*d and denominator is n + d. Actually the C stores the continued
        // fraction as (n, d) where the convergent is n/d = (n + 2d)/(n + d).
        // It checks cnlen > cdlen.
        if num_digits(&new_n) > num_digits(&new_d) {
            count += 1;
        }
        n = new_n;
        d = new_d;
    }

    println!("{count}");
}
