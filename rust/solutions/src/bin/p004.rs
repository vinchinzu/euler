// Project Euler 4: Largest palindrome from product of two 3-digit numbers

fn is_palindrome(n: u64) -> bool {
    let s = n.to_string();
    s == s.chars().rev().collect::<String>()
}

fn main() {
    let mut best = 0u64;

    for x in (100..=999).rev() {
        for y in (100..=x).rev() {
            let p = x * y;
            if p <= best {
                break;
            }
            if is_palindrome(p) {
                best = p;
            }
        }
    }

    println!("{best}");
}
