// Project Euler 348: Sum of a Square and a Cube
// Find palindromic numbers expressible as a^2 + b^3 in exactly 4 ways.

fn is_palindrome(n: i64) -> bool {
    let s = n.to_string();
    let b = s.as_bytes();
    let len = b.len();
    for i in 0..len / 2 {
        if b[i] != b[len - 1 - i] { return false; }
    }
    true
}

fn count_representations(n: i64) -> i32 {
    let mut count = 0;
    let mut b: i64 = 2;
    while b * b * b < n {
        let remainder = n - b * b * b;
        if remainder > 1 {
            let a = (remainder as f64).sqrt().round() as i64;
            for aa in (a - 1).max(2)..=a + 1 {
                if aa > 1 && aa * aa == remainder {
                    count += 1;
                    break;
                }
            }
        }
        b += 1;
    }
    count
}

fn main() {
    const MAX_VAL: i64 = 1_000_000_000;
    let mut results: Vec<i64> = Vec::new();

    // Generate palindromes in order of length
    for length in 1..=10 {
        if results.len() >= 5 { break; }
        if length == 1 {
            for d in 1..=9i64 {
                if d >= 28 {
                    if count_representations(d) == 4 {
                        results.push(d);
                        if results.len() >= 5 { break; }
                    }
                }
            }
            continue;
        }
        let half_len = (length + 1) / 2;
        let start = 10i64.pow(half_len as u32 - 1);
        let end = 10i64.pow(half_len as u32);

        for half in start..end {
            if results.len() >= 5 { break; }
            let hs = half.to_string();
            let hb = hs.as_bytes();
            let mut ps = hs.clone();
            if length % 2 == 0 {
                let rev: String = hb.iter().rev().map(|&b| b as char).collect();
                ps.push_str(&rev);
            } else {
                let rev: String = hb[..hb.len() - 1].iter().rev().map(|&b| b as char).collect();
                ps.push_str(&rev);
            }
            let palindrome: i64 = ps.parse().unwrap();
            if palindrome > MAX_VAL { break; }
            if palindrome < 28 { continue; }
            if count_representations(palindrome) == 4 {
                results.push(palindrome);
            }
        }
    }

    let total: i64 = results.iter().take(5).sum();
    println!("{}", total);
}
