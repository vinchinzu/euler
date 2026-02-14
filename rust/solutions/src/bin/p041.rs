// Project Euler 41: Pandigital prime
// Find the largest n-digit pandigital prime (using digits 1..n).
use euler_utils::is_prime;

fn prev_permutation(arr: &mut [u8]) -> bool {
    let n = arr.len();
    if n < 2 {
        return false;
    }
    let mut i = n - 2;
    while arr[i] <= arr[i + 1] {
        if i == 0 {
            return false;
        }
        i -= 1;
    }
    let mut j = n - 1;
    while arr[j] >= arr[i] {
        j -= 1;
    }
    arr.swap(i, j);
    arr[i + 1..].reverse();
    true
}

fn digits_to_number(digits: &[u8]) -> u64 {
    digits.iter().fold(0u64, |acc, &d| acc * 10 + d as u64)
}

fn main() {
    for n in (1u8..=7).rev() {
        let mut digits: Vec<u8> = (1..=n).rev().collect();
        loop {
            let num = digits_to_number(&digits);
            if is_prime(num) {
                println!("{num}");
                return;
            }
            if !prev_permutation(&mut digits) {
                break;
            }
        }
    }
}
