// Project Euler 98: Anagramic squares
// Find the largest square formed by anagram word pairs mapped to digits.

use std::collections::HashMap;

fn is_square(n: u64) -> bool {
    if n == 0 { return true; }
    let r = (n as f64).sqrt() as u64;
    for candidate in [r.saturating_sub(1), r, r + 1] {
        if candidate * candidate == n {
            return true;
        }
    }
    false
}

fn signature(s: &str) -> Vec<u8> {
    let mut chars: Vec<u8> = s.bytes().collect();
    chars.sort();
    chars
}

fn main() {
    let data = include_str!("../../../../data/words.txt");

    // Parse words
    let words: Vec<&str> = data.split(',')
        .map(|w| w.trim().trim_matches('"'))
        .filter(|w| !w.is_empty())
        .collect();

    // Group by sorted signature to find anagram pairs
    let mut sig_groups: HashMap<Vec<u8>, Vec<&str>> = HashMap::new();
    for &w in &words {
        sig_groups.entry(signature(w)).or_default().push(w);
    }

    // Precompute squares by digit count
    let max_len = words.iter().map(|w| w.len()).max().unwrap_or(0);
    let mut squares_by_len: Vec<Vec<u64>> = vec![Vec::new(); max_len + 1];
    for d in 1..=max_len {
        let lo = if d == 1 { 1u64 } else { 10u64.pow(d as u32 - 1) };
        let hi = 10u64.pow(d as u32) - 1;
        let start = (lo as f64).sqrt().ceil() as u64;
        let end = (hi as f64).sqrt() as u64;
        for n in start..=end {
            squares_by_len[d].push(n * n);
        }
    }

    let mut max_square: u64 = 0;

    for group in sig_groups.values() {
        if group.len() < 2 { continue; }

        for i in 0..group.len() {
            for j in (i + 1)..group.len() {
                let w1 = group[i].as_bytes();
                let w2 = group[j].as_bytes();
                let wlen = w1.len();

                for &sq1 in &squares_by_len[wlen] {
                    // Build letter -> digit mapping from w1 -> sq1
                    let s1 = format!("{:0>width$}", sq1, width = wlen);
                    let s1_bytes = s1.as_bytes();

                    let mut mapping = [255u8; 26]; // letter -> digit
                    let mut reverse_map = [255u8; 10]; // digit -> letter
                    let mut valid = true;

                    for k in 0..wlen {
                        let letter = (w1[k] - b'A') as usize;
                        let digit = s1_bytes[k] - b'0';

                        if mapping[letter] != 255 {
                            if mapping[letter] != digit { valid = false; break; }
                        } else {
                            if reverse_map[digit as usize] != 255 {
                                if reverse_map[digit as usize] != w1[k] { valid = false; break; }
                            }
                            mapping[letter] = digit;
                            reverse_map[digit as usize] = w1[k];
                        }
                    }
                    if !valid { continue; }

                    // Apply mapping to w2
                    let mut sq2: u64 = 0;
                    let mut valid2 = true;
                    for k in 0..wlen {
                        let letter = (w2[k] - b'A') as usize;
                        if mapping[letter] == 255 { valid2 = false; break; }
                        sq2 = sq2 * 10 + mapping[letter] as u64;
                    }
                    if !valid2 { continue; }

                    // Check leading zero
                    if wlen > 1 && mapping[(w2[0] - b'A') as usize] == 0 { continue; }

                    if is_square(sq2) {
                        max_square = max_square.max(sq1).max(sq2);
                    }
                }
            }
        }
    }

    println!("{max_square}");
}
