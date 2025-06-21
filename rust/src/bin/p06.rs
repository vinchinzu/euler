fn main() {
    let sum_sq: i32 = (1..=100).map(|x| x * x).sum();
    let sum: i32 = (1..=100).sum();
    let diff = sum * sum - sum_sq;
    println!("{}", diff);
}
