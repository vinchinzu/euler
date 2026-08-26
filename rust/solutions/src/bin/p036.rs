// Project Euler 036: Double-base Palindromes
// Sum of all numbers < 1_000_000 palindromic in both base 10 and base 2.

fn is_palindrome_2(n: u64) -> bool {
    if n == 0 {
        return true;
    }
    let bits = 64 - n.leading_zeros();
    for i in 0..bits / 2 {
        if ((n >> i) & 1) != ((n >> (bits - 1 - i)) & 1) {
            return false;
        }
    }
    true
}

fn make_palindrome(mut half: u64, odd: bool) -> u64 {
    let mut n = half;
    if odd {
        half /= 10;
    }
    while half > 0 {
        n = n * 10 + half % 10;
        half /= 10;
    }
    n
}

fn main() {
    let mut sum = 0u64;
    // Even-length palindromes: abccba from half 1..999
    for half in 1..1000u64 {
        let n = make_palindrome(half, false);
        if n < 1_000_000 && is_palindrome_2(n) {
            sum += n;
        }
    }
    // Odd-length palindromes: abcba from half 1..999 (includes 1-digit)
    for half in 1..1000u64 {
        let n = make_palindrome(half, true);
        if n < 1_000_000 && is_palindrome_2(n) {
            sum += n;
        }
    }
    println!("{sum}");
}
