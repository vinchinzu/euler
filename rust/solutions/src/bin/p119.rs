// Project Euler Problem 119: Digit Power Sum
// Find a(30) where a(n) is the sorted sequence of numbers that are
// a power of their digit sum.

fn digit_sum(mut n: u64) -> u64 {
    let mut s = 0u64;
    while n > 0 {
        s += n % 10;
        n /= 10;
    }
    s
}

fn main() {
    let limit: u64 = 1_000_000_000_000_000_000; // 10^18
    let mut results: Vec<u64> = Vec::new();

    for s in 2u64..=200 {
        let mut power = s * s;
        for _k in 2..=60 {
            if power >= 10 && digit_sum(power) == s {
                results.push(power);
            }
            if power > limit / s {
                break;
            }
            power *= s;
        }
    }

    results.sort();
    results.dedup();

    println!("{}", results[29]); // a(30), 0-indexed
}
