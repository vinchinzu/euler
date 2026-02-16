// Project Euler Problem 937 - Factorial Partitions
// G(n) = sum of k! for k=1..n where k%3 != 2, mod 10^9+7

fn main() {
    const MOD: u64 = 1_000_000_007;
    let n: u64 = 100_000_000; // 10^8

    let mut total_sum: u64 = 0;
    let mut current_factorial: u64 = 1;

    for k in 1..=n {
        current_factorial = current_factorial % MOD * (k % MOD) % MOD;
        if k % 3 != 2 {
            total_sum = (total_sum + current_factorial) % MOD;
        }
    }

    println!("{}", total_sum);
}
