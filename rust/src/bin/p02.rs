fn main() {
    let mut sum = 0u32;
    let mut a = 1u32;
    let mut b = 2u32;
    while b <= 4_000_000 {
        if b % 2 == 0 {
            sum += b;
        }
        let temp = a + b;
        a = b;
        b = temp;
    }
    println!("{}", sum);
}
