// Project Euler 2: Sum of even Fibonacci numbers below 4 million

fn main() {
    let mut a: u64 = 1;
    let mut b: u64 = 2;
    let mut sum: u64 = 0;

    while b < 4_000_000 {
        if b % 2 == 0 {
            sum += b;
        }
        let temp = a + b;
        a = b;
        b = temp;
    }

    println!("{sum}");
}
