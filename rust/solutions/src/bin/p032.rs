// Project Euler 032: Pandigital Products
// Sum of all products whose multiplicand/multiplier/product is 1-9 pandigital.

use std::collections::HashSet;

fn main() {
    let mut digits = [1, 2, 3, 4, 5, 6, 7, 8, 9u32];
    let mut products = HashSet::new();

    loop {
        // Split 1: 1-digit * 4-digit = 4-digit
        let a = digits[0];
        let b = digits[1] * 1000 + digits[2] * 100 + digits[3] * 10 + digits[4];
        let c = digits[5] * 1000 + digits[6] * 100 + digits[7] * 10 + digits[8];
        if a * b == c {
            products.insert(c);
        }

        // Split 2: 2-digit * 3-digit = 4-digit
        let a2 = digits[0] * 10 + digits[1];
        let b2 = digits[2] * 100 + digits[3] * 10 + digits[4];
        if a2 * b2 == c {
            products.insert(c);
        }

        if !next_permutation(&mut digits) {
            break;
        }
    }

    let sum: u32 = products.into_iter().sum();
    println!("{sum}");
}

fn next_permutation(arr: &mut [u32]) -> bool {
    let n = arr.len();
    if n < 2 {
        return false;
    }
    let mut i = n - 2;
    while arr[i] >= arr[i + 1] {
        if i == 0 {
            return false;
        }
        i -= 1;
    }
    let mut j = n - 1;
    while arr[j] <= arr[i] {
        j -= 1;
    }
    arr.swap(i, j);
    arr[i + 1..].reverse();
    true
}
