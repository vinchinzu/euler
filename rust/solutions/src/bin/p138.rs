// Project Euler 138: Special isosceles triangles
fn main() {
    let mut fib = vec![0i64; 76];
    fib[1] = 1;
    for i in 2..=75 {
        fib[i] = fib[i - 1] + fib[i - 2];
    }
    let total: i64 = (1..=12).map(|j| fib[6 * j + 3] / 2).sum();
    println!("{total}");
}
