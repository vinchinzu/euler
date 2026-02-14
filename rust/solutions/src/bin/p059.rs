// Project Euler 59: XOR decryption
// Decrypt a message encrypted with a 3-character lowercase key by XOR.

fn main() {
    let data = include_str!("../../../../data/cipher1.txt");
    let cipher: Vec<u8> = data
        .trim()
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    for a in b'a'..=b'z' {
        for b in b'a'..=b'z' {
            for c in b'a'..=b'z' {
                let key = [a, b, c];
                let text: Vec<u8> = cipher
                    .iter()
                    .enumerate()
                    .map(|(i, &v)| v ^ key[i % 3])
                    .collect();

                // Check for common English words
                let s = String::from_utf8_lossy(&text);
                if s.contains(" the ") && s.contains(" and ") {
                    let sum: u32 = text.iter().map(|&x| x as u32).sum();
                    println!("{sum}");
                    return;
                }
            }
        }
    }
}
