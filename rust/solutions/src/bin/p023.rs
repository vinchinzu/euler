// Project Euler 023: Non-abundant Sums
// Sum of all positive integers that cannot be written as the sum of two abundant numbers.

fn main() {
    const LIMIT: usize = 28_123;

    // Sieve proper divisor sums
    let mut d = vec![0u32; LIMIT + 1];
    for i in 1..=LIMIT {
        for j in (2 * i..=LIMIT).step_by(i) {
            d[j] += i as u32;
        }
    }

    let abundants: Vec<usize> = (12..=LIMIT)
        .filter(|&i| d[i] as usize > i)
        .collect();

    let mut can_be_sum = vec![false; LIMIT + 1];
    for (i, &a) in abundants.iter().enumerate() {
        for &b in &abundants[i..] {
            let s = a + b;
            if s > LIMIT {
                break;
            }
            can_be_sum[s] = true;
        }
    }

    let total: u64 = (1..=LIMIT)
        .filter(|&i| !can_be_sum[i])
        .map(|i| i as u64)
        .sum();

    println!("{total}");
}
