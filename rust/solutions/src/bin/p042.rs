// Project Euler 42: Coded triangle numbers
// Count words whose letter-value sum is a triangle number.

fn is_triangle(n: u32) -> bool {
    if n == 0 {
        return true;
    }
    // n = k*(k+1)/2 => k^2 + k - 2n = 0 => disc = 1 + 8n
    let disc = 1 + 8 * n as u64;
    let s = (disc as f64).sqrt() as u64;
    for m in s.saturating_sub(1)..=s + 1 {
        if m * m == disc && (m + 1) % 2 == 0 {
            return true;
        }
    }
    false
}

fn word_value(word: &str) -> u32 {
    word.bytes()
        .filter(|b| b.is_ascii_uppercase())
        .map(|b| (b - b'A' + 1) as u32)
        .sum()
}

fn main() {
    let data = include_str!("../../../../data/words.txt");
    let count = data
        .split(',')
        .map(|w| w.trim_matches('"'))
        .filter(|w| is_triangle(word_value(w)))
        .count();
    println!("{count}");
}
