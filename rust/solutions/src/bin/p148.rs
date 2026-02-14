// Project Euler 148: Entries in Pascal's triangle not divisible by 7
fn main() {
    let n: i64 = 999_999_999;

    // Convert to base 7
    let mut digits = Vec::new();
    let mut tmp = n;
    while tmp > 0 {
        digits.push(tmp % 7);
        tmp /= 7;
    }
    digits.reverse();

    let mut result: i64 = 0;
    let mut prefix_product: i64 = 1;

    for pos in 0..digits.len() {
        let max_d = digits[pos];
        let mut free_suffix: i64 = 1;
        for _ in (pos + 1)..digits.len() {
            free_suffix *= 28;
        }

        let sum_d: i64 = (0..max_d).map(|d| d + 1).sum();
        result += prefix_product * sum_d * free_suffix;
        prefix_product *= max_d + 1;
    }
    result += prefix_product;

    println!("{}", result);
}
