fn main() {
    let mut n: u64 = 600851475143;
    let mut factor = 2u64;
    let mut last_factor = 1u64;
    while factor * factor <= n {
        if n % factor == 0 {
            n /= factor;
            last_factor = factor;
            while n % factor == 0 {
                n /= factor;
            }
        }
        factor += 1;
    }
    if n > 1 {
        last_factor = n;
    }
    println!("{}", last_factor);
}
