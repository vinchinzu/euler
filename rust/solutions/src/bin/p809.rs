// Project Euler 809 - Rational Recurrence Relation
// Find fixed point of 2^x mod M, then subtract 3

fn mod_pow(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
    let mut result: i64 = 1;
    base = base.rem_euclid(modulus);
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result as i128 * base as i128 % modulus as i128) as i64;
        }
        base = (base as i128 * base as i128 % modulus as i128) as i64;
        exp >>= 1;
    }
    result
}

fn main() {
    let m: i64 = 1_000_000_000_000_000; // 10^15

    // Find fixed point of 2^x mod M
    let mut b: i64 = 0;
    loop {
        let next_b = mod_pow(2, b, m);
        if next_b == b { break; }
        b = next_b;
    }

    println!("{}", b - 3);
}
