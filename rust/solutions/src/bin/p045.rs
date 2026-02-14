// Project Euler 45: Triangular, pentagonal, and hexagonal
// Find the next triangle number after 40755 that is also pentagonal and hexagonal.

fn is_pentagonal(x: u64) -> bool {
    let disc = 1 + 24 * x;
    let s = (disc as f64).sqrt().round() as u64;
    s * s == disc && (1 + s) % 6 == 0
}

fn is_hexagonal(x: u64) -> bool {
    let disc = 1 + 8 * x;
    let s = (disc as f64).sqrt().round() as u64;
    s * s == disc && (1 + s) % 4 == 0
}

fn main() {
    let mut n: u64 = 286;
    loop {
        let t = n * (n + 1) / 2;
        if t > 40755 && is_pentagonal(t) && is_hexagonal(t) {
            println!("{t}");
            return;
        }
        n += 1;
    }
}
