// Project Euler 040: Champernowne's Constant
// Product of digits d_1, d_10, d_100, ..., d_1000000 of 0.123456789101112...

fn main() {
    let positions = [1, 10, 100, 1_000, 10_000, 100_000, 1_000_000];
    let mut result = [0u8; 7];
    let mut next = 0usize;

    let mut length = 0usize;
    let mut number = 1u64;

    while next < positions.len() {
        let num_str = number.to_string();
        let num_len = num_str.len();

        while next < positions.len() && length + num_len >= positions[next] {
            let offset = positions[next] - length - 1;
            result[next] = num_str.as_bytes()[offset] - b'0';
            next += 1;
        }

        length += num_len;
        number += 1;
    }

    let product: u64 = result.iter().map(|&d| d as u64).product();
    println!("{product}");
}
