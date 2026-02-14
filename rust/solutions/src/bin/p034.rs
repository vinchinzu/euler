// Project Euler 034: Digit Factorials
// Sum of all numbers equal to the sum of the factorials of their digits.

fn main() {
    let mut fact = [1u64; 10];
    for i in 1..10 {
        fact[i] = fact[i - 1] * i as u64;
    }

    let upper = 7 * fact[9]; // 2_540_160

    let total: u64 = (10..=upper)
        .filter(|&n| {
            let mut sum = 0u64;
            let mut tmp = n;
            while tmp > 0 {
                sum += fact[(tmp % 10) as usize];
                tmp /= 10;
            }
            sum == n
        })
        .sum();

    println!("{total}");
}
