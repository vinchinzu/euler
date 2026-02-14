// Project Euler 162 - Hexadecimal numbers
// Count hex numbers up to 16 digits containing at least one 0, 1, and A.
// Uses inclusion-exclusion with u128 arithmetic.

fn power(base: u128, exp: u32) -> u128 {
    let mut result = 1u128;
    for _ in 0..exp {
        result *= base;
    }
    result
}

fn sum_series(first_count: u128, base: u128, max_k: u32) -> u128 {
    if first_count == 0 || max_k == 0 { return 0; }
    if base == 1 { return first_count * max_k as u128; }
    first_count * (power(base, max_k) - 1) / (base - 1)
}

fn main() {
    let max_digits = 16u32;

    let total = sum_series(15, 16, max_digits);
    let missing_0 = sum_series(15, 15, max_digits);
    let missing_1 = sum_series(14, 15, max_digits);
    let missing_a = sum_series(14, 15, max_digits);
    let missing_0_1 = sum_series(14, 14, max_digits);
    let missing_0_a = sum_series(14, 14, max_digits);
    let missing_1_a = sum_series(13, 14, max_digits);
    let missing_all = sum_series(13, 13, max_digits);

    let missing_at_least_one = missing_0 + missing_1 + missing_a
        - missing_0_1 - missing_0_a - missing_1_a
        + missing_all;

    let result = total - missing_at_least_one;

    // Print as uppercase hex
    println!("{:X}", result);
}
