// Project Euler 348: Sum of a Square and a Cube
// Find palindromic numbers expressible as a^2 + b^3 in exactly 4 ways.
//
// Instead of checking each palindrome for representations, enumerate all
// (a,b) pairs and count how many times each palindrome appears.

use std::collections::HashMap;

#[inline]
fn is_palindrome(n: u64) -> bool {
    if n < 10 { return true; }
    if n % 10 == 0 { return false; }
    let mut rev = 0u64;
    let mut m = n;
    while m > 0 {
        rev = rev * 10 + m % 10;
        m /= 10;
    }
    rev == n
}

fn main() {
    const MAX_VAL: u64 = 1_000_000_000;

    // Count representations for each palindrome n = a^2 + b^3
    let mut counts: HashMap<u64, u32> = HashMap::with_capacity(50_000);

    let mut b: u64 = 2;
    while b * b * b < MAX_VAL {
        let b3 = b * b * b;
        let mut a: u64 = 2;
        loop {
            let n = a * a + b3;
            if n >= MAX_VAL { break; }
            if is_palindrome(n) {
                *counts.entry(n).or_insert(0) += 1;
            }
            a += 1;
        }
        b += 1;
    }

    // Collect palindromes with exactly 4 representations
    let mut results: Vec<u64> = counts.into_iter()
        .filter(|&(_, c)| c == 4)
        .map(|(n, _)| n)
        .collect();
    results.sort();

    let total: u64 = results.iter().take(5).sum();
    println!("{}", total);
}
