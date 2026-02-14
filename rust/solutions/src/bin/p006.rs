// Project Euler 6: Difference between sum-of-squares and square-of-sum for 1..100

fn main() {
    let sum_sq: u64 = (1..=100u64).map(|x| x * x).sum();
    let sq_sum: u64 = {
        let s: u64 = (1..=100).sum();
        s * s
    };
    println!("{}", sq_sum - sum_sq);
}
