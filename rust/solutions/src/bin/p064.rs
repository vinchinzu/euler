// Project Euler 64: Odd period square roots
// Count how many continued fractions for sqrt(N), N <= 10000, have an odd period.

fn cf_period(n: u32) -> u32 {
    let a0 = (n as f64).sqrt() as u32;
    if a0 * a0 == n { return 0; }

    let mut period = 0u32;
    let mut m = 0u32;
    let mut d = 1u32;
    let mut a = a0;

    loop {
        m = d * a - m;
        d = (n - m * m) / d;
        a = (a0 + m) / d;
        period += 1;
        if a == 2 * a0 { break; }
    }
    period
}

fn main() {
    let count = (2..=10000).filter(|&n| cf_period(n) % 2 == 1).count();
    println!("{count}");
}
