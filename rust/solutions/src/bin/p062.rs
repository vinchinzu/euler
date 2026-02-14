// Project Euler 62: Cubic permutations
// Find the smallest cube whose digit-sorted signature has exactly 5 cube permutations.

use std::collections::HashMap;

fn digit_signature(mut n: u64) -> String {
    let mut digits = Vec::new();
    while n > 0 {
        digits.push((n % 10) as u8);
        n /= 10;
    }
    digits.sort();
    digits.iter().map(|&d| (b'0' + d) as char).collect()
}

fn main() {
    let mut map: HashMap<String, (u64, u32)> = HashMap::new(); // sig -> (smallest_cube, count)

    for n in 1u64.. {
        let cube = n * n * n;
        let sig = digit_signature(cube);
        let entry = map.entry(sig).or_insert((cube, 0));
        entry.1 += 1;
        if entry.1 == 5 {
            println!("{}", entry.0);
            return;
        }
    }
}
