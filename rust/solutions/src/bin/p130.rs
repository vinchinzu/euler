// Project Euler Problem 130: Composites with prime repunit property.
//
// Sum the first 25 composite n (coprime to 10) where A(n) | (n-1).
// A(n) = least k such that repunit R(k) divisible by n.

use euler_utils::is_prime;

fn repunit_period(n: u64) -> u64 {
    let mut remainder = 1 % n;
    let mut length = 1u64;
    while remainder != 0 {
        remainder = (remainder * 10 + 1) % n;
        length += 1;
    }
    length
}

fn is_composite_coprime10(n: u64) -> bool {
    if n < 4 {
        return false;
    }
    if n % 2 == 0 || n % 5 == 0 {
        return false;
    }
    !is_prime(n)
}

fn main() {
    let mut found = 0u32;
    let mut total: u64 = 0;
    let mut n = 2u64;

    while found < 25 {
        if n % 2 != 0 && n % 5 != 0 && is_composite_coprime10(n) {
            let period = repunit_period(n);
            if (n - 1) % period == 0 {
                total += n;
                found += 1;
            }
        }
        n += 1;
    }

    println!("{}", total);
}
