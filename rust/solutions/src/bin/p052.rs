// Project Euler 52: Permuted multiples
// Find the smallest x such that 2x, 3x, 4x, 5x, 6x all contain the same digits as x.

fn sorted_digits(n: u64) -> Vec<u8> {
    let mut d: Vec<u8> = n.to_string().bytes().collect();
    d.sort_unstable();
    d
}

fn main() {
    let mut x = 1u64;
    loop {
        let base = sorted_digits(x);
        if (2..=6).all(|m| sorted_digits(x * m) == base) {
            println!("{x}");
            return;
        }
        x += 1;
    }
}
