// Project Euler 036: Double-base Palindromes
// Sum of all numbers < 1_000_000 palindromic in both base 10 and base 2.

fn main() {
    let sum: u64 = (1..1_000_000u64)
        .filter(|&n| is_palindrome_10(n) && is_palindrome_2(n))
        .sum();

    println!("{sum}");
}

fn is_palindrome_10(n: u64) -> bool {
    let s: Vec<u8> = n.to_string().into_bytes();
    s.iter().eq(s.iter().rev())
}

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
