fn main() {
    let mut n = 20u64;
    loop {
        if (1..=20).all(|i| n % i == 0) {
            println!("{}", n);
            break;
        }
        n += 20; // step by 20
    }
}
