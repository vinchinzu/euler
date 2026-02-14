// Project Euler 030: Digit Fifth Powers
// Sum of all numbers equal to the sum of the fifth powers of their digits.

fn main() {
    let fifth: [u64; 10] = std::array::from_fn(|i| (i as u64).pow(5));
    let limit = 6 * fifth[9]; // 6 * 9^5 = 354294

    let total: u64 = (2..=limit)
        .filter(|&n| {
            let mut sum = 0u64;
            let mut tmp = n;
            while tmp > 0 {
                sum += fifth[(tmp % 10) as usize];
                tmp /= 10;
            }
            sum == n
        })
        .sum();

    println!("{total}");
}
