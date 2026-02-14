// Project Euler 112: Bouncy Numbers
// Find n where proportion of bouncy numbers is exactly 99%.

fn is_bouncy(n: u32) -> bool {
    if n < 100 {
        return false;
    }
    let mut digits = [0u8; 10];
    let mut nd = 0;
    let mut tmp = n;
    while tmp > 0 {
        digits[nd] = (tmp % 10) as u8;
        nd += 1;
        tmp /= 10;
    }
    digits[..nd].reverse();

    let mut inc = true;
    let mut dec = true;
    for i in 0..nd - 1 {
        if digits[i] > digits[i + 1] {
            inc = false;
        }
        if digits[i] < digits[i + 1] {
            dec = false;
        }
    }
    !inc && !dec
}

fn main() {
    let mut bouncy_count: u64 = 0;
    let mut n: u64 = 0;
    loop {
        n += 1;
        if is_bouncy(n as u32) {
            bouncy_count += 1;
        }
        if n % 100 == 0 && bouncy_count * 100 == 99 * n {
            println!("{n}");
            return;
        }
    }
}
