// Project Euler 340: Crazy Function
// Closed form for sum of F(n), n=0..b.

const MOD: i64 = 1_000_000_000;

fn tr(n: i64) -> i64 {
    if n <= 0 { return 0; }
    let nm = n % MOD;
    let np1m = (n + 1) % MOD;
    if n % 2 == 0 {
        (nm / 2 % MOD) * (np1m % MOD) % MOD
    } else {
        (nm % MOD) * ((np1m / 2) % MOD) % MOD
    }
}

fn main() {
    let mut a: i64 = 1;
    for _ in 0..7 { a *= 21; }
    let mut b: i64 = 1;
    for _ in 0..21 { b *= 7; }
    let mut c: i64 = 1;
    for _ in 0..7 { c *= 12; }

    let bma = b % a;
    let bda = b / a;

    let mut ans: i128 = 0;
    ans += tr(b) as i128;
    ans += ((b + 1) % MOD) as i128 * (4 % MOD) as i128 % MOD as i128
        * (((a - c) % MOD + MOD) % MOD) as i128 % MOD as i128;
    ans += tr(bda) as i128 * ((bma + 1) % MOD) as i128 % MOD as i128
        * (((4 * a - 3 * c) % MOD + MOD) % MOD) as i128 % MOD as i128;
    ans += tr(bda - 1) as i128
        * (((a - bma - 1) % MOD + MOD) % MOD) as i128 % MOD as i128
        * (((4 * a - 3 * c) % MOD + MOD) % MOD) as i128 % MOD as i128;
    ans = ans % MOD as i128;
    if ans < 0 { ans += MOD as i128; }

    println!("{}", ans as i64);
}
