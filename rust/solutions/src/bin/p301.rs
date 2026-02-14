fn main() {
    let (mut a, mut b) = (1i64, 2i64);
    for _ in 0..29 {
        let t = a + b;
        a = b;
        b = t;
    }
    println!("{}", b);
}
