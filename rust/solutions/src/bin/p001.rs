// Project Euler 1: Sum of multiples of 3 or 5 below 1000

fn main() {
    let total: u64 = (1..1000).filter(|i| i % 3 == 0 || i % 5 == 0).sum();
    println!("{total}");
}
