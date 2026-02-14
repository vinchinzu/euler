// Project Euler 55: Lychrel numbers
// Count numbers below 10000 that don't become palindromes within 50 reverse-and-add iterations.
use num::BigUint;

fn is_palindrome(n: &BigUint) -> bool {
    let s = n.to_string();
    let b = s.as_bytes();
    let len = b.len();
    (0..len / 2).all(|i| b[i] == b[len - 1 - i])
}

fn reverse_digits(n: &BigUint) -> BigUint {
    let s = n.to_string();
    let rev: String = s.chars().rev().collect();
    rev.parse().unwrap()
}

fn is_lychrel(num: u32) -> bool {
    let mut current = BigUint::from(num);
    for _ in 0..50 {
        let rev = reverse_digits(&current);
        current = &current + &rev;
        if is_palindrome(&current) {
            return false;
        }
    }
    true
}

fn main() {
    let count = (1u32..10000).filter(|&n| is_lychrel(n)).count();
    println!("{count}");
}
