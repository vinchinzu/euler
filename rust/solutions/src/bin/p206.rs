// Project Euler 206: Concealed Square
fn matches(mut sq: i64) -> bool {
    let digits = [0, 9, 8, 7, 6, 5, 4, 3, 2, 1];
    for &d in &digits {
        if sq % 10 != d { return false; }
        sq /= 100;
    }
    sq == 0
}

fn main() {
    let hi = 1_929_394_959_697_989_990i64;
    let lo = 1_020_304_050_607_080_900i64;

    let mut n = (hi as f64).sqrt() as i64;
    while n * n > hi { n -= 1; }

    // Align to end in 30 or 70
    let r = n % 100;
    if r > 70 { n -= r - 70; }
    else if r > 30 { n -= r - 30; }
    else { n -= r + 30; }

    let lo_n = {
        let mut v = (lo as f64).sqrt() as i64;
        while v * v < lo { v += 1; }
        v
    };

    while n >= lo_n {
        if matches(n * n) {
            println!("{n}");
            return;
        }
        if n % 100 == 70 { n -= 40; }
        else { n -= 60; }
    }
    println!("0");
}
