// Project Euler 129: Repunit Divisibility
// Find least n with gcd(n,10)=1 such that A(n) > 1,000,000.

const TARGET: u32 = 1_000_000;

fn repunit_order(n: u32) -> u32 {
    let mut remainder = 1 % n;
    let mut k = 1u32;
    while remainder != 0 {
        remainder = (remainder * 10 + 1) % n;
        k += 1;
    }
    k
}

fn main() {
    let mut n = TARGET + 1;
    loop {
        if n % 2 != 0 && n % 5 != 0 && repunit_order(n) > TARGET {
            println!("{n}");
            return;
        }
        n += 1;
    }
}
