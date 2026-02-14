// Project Euler Problem 125: Palindromic sums.
//
// Find sum of all numbers < 10^8 that are palindromic and expressible
// as sum of consecutive squares.

const LIMIT: u64 = 100_000_000;

fn is_palindrome(n: u64) -> bool {
    let s = n.to_string();
    let bytes = s.as_bytes();
    let len = bytes.len();
    for i in 0..len / 2 {
        if bytes[i] != bytes[len - 1 - i] {
            return false;
        }
    }
    true
}

fn main() {
    let nbytes = ((LIMIT + 7) / 8) as usize;
    let mut seen = vec![0u8; nbytes];

    let i_upper = ((LIMIT as f64 / 2.0).sqrt() as u64) + 1;

    for i in 1..=i_upper {
        let mut sum = i * i;
        let mut j = i + 1;
        loop {
            sum += j * j;
            if sum >= LIMIT {
                break;
            }
            if is_palindrome(sum) {
                let idx = sum as usize;
                seen[idx / 8] |= 1 << (idx % 8);
            }
            j += 1;
        }
    }

    let mut total: u64 = 0;
    for n in 1..LIMIT {
        let idx = n as usize;
        if seen[idx / 8] & (1 << (idx % 8)) != 0 {
            total += n;
        }
    }

    println!("{}", total);
}
