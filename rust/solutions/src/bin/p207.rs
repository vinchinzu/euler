fn main() {
    let r: f64 = 1.0 / 12345.0;
    let mut n: i64 = 2;
    while (n as f64).log2().floor() / (n - 1) as f64 >= r {
        n += 1;
    }
    println!("{}", n * n - n);
}
