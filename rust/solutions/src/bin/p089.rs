// Project Euler 89: Roman numerals
// Compute characters saved by writing each numeral in minimal form.

fn roman_char_value(c: u8) -> i32 {
    match c {
        b'I' => 1,
        b'V' => 5,
        b'X' => 10,
        b'L' => 50,
        b'C' => 100,
        b'D' => 500,
        b'M' => 1000,
        _ => 0,
    }
}

fn roman_to_int(s: &str) -> i32 {
    let bytes = s.as_bytes();
    let mut total = 0;
    let mut prev = 0;
    for &b in bytes.iter().rev() {
        let val = roman_char_value(b);
        if val < prev {
            total -= val;
        } else {
            total += val;
        }
        prev = val;
    }
    total
}

fn int_to_roman_len(mut num: i32) -> usize {
    let values = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
    let symbol_lens = [1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1];
    let mut len = 0;
    for i in 0..13 {
        while num >= values[i] {
            len += symbol_lens[i];
            num -= values[i];
        }
    }
    len
}

fn main() {
    let data = include_str!("../../../../data/0089_roman.txt");
    let mut total_saved = 0usize;

    for line in data.lines() {
        let s = line.trim();
        if s.is_empty() {
            continue;
        }
        let value = roman_to_int(s);
        let minimal_len = int_to_roman_len(value);
        total_saved += s.len() - minimal_len;
    }

    println!("{total_saved}");
}
