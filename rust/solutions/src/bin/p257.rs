// Project Euler 257: Angular Bisectors
use euler_utils::gcd;

fn perim1(m: i64, n: i64) -> i64 { 2*m*m + n*n + 3*m*n }
fn perim2(m: i64, n: i64) -> i64 { 3*m*m + n*n + 4*m*n }

fn main() {
    let big_n: i64 = 100_000_000;
    let l = ((big_n as f64 / 3.0).sqrt()) as i64;
    let mut ans: i64 = 0;

    // r = 2 case
    for m in 1..=l {
        if perim1(m, m) > big_n { break; }
        for n in (m + 1)..(2 * m) {
            if perim1(m, n) > big_n { break; }
            if n % 2 != 0 && gcd(m as u64, (n % m) as u64) == 1 {
                ans += big_n / perim1(m, n);
            }
        }
    }

    // r = 3 case (even)
    for m in 1..=l {
        if perim2(m, m) > big_n { break; }
        let mut n = m + 1;
        if n % 2 == 0 { n += 1; }
        while n < 3 * m {
            if perim2(m, n) > big_n { break; }
            if n % 3 != 0 && gcd(m as u64, (n % m) as u64) == 1 {
                ans += big_n / perim2(m, n);
            }
            n += 2;
        }
    }

    // r = 3 case (odd m, n)
    let mut m = 1i64;
    while m <= 2 * l {
        if perim2(m, m) > 2 * big_n { break; }
        let mut n = m + 2;
        while n < 3 * m {
            if perim2(m, n) > 2 * big_n { break; }
            if n % 3 != 0 && gcd(m as u64, (n % m) as u64) == 1 {
                ans += 2 * big_n / perim2(m, n);
            }
            n += 2;
        }
        m += 2;
    }

    // r = 4 case (equilateral triangles)
    ans += big_n / 3;

    println!("{ans}");
}
