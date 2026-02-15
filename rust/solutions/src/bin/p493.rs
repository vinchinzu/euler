fn main() {
    let n = 20;
    let k = 10;
    let c = 7;

    let mut p = 1.0f64;
    for i in 0..n {
        p *= ((c - 1) * k - i) as f64 / (c * k - i) as f64;
    }

    let ans = c as f64 * (1.0 - p);
    println!("{:.9}", ans);
}
