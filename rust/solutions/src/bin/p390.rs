// Project Euler 390: Triangles with Non Rational Sides and Integral Area
fn isqrt128(n: u128) -> u64 {
    if n == 0 { return 0; }
    let mut x = (n as f64).sqrt() as u64;
    for _ in 0..5 {
        if x == 0 { break; }
        let x128 = x as u128;
        let next = (x128 + n / x128) / 2;
        if next >= x128 && next - x128 <= 1 { break; }
        if x128 >= next && x128 - next <= 1 { break; }
        x = next as u64;
    }
    while (x as u128) * (x as u128) > n { x -= 1; }
    while ((x + 1) as u128) * ((x + 1) as u128) <= n { x += 1; }
    x
}

fn main() {
    let big_n: u64 = 20_000_000_000;
    let mut ans: u128 = 0;

    let mut a: u64 = 2;
    while (a as u128) * (a as u128) + 1 <= big_n as u128 {
        let a2 = a as u128 * a as u128;
        let upper_bound = big_n as u128 / (a2 + 1);
        let mut t: u64 = 2;
        while t as u128 <= upper_bound {
            let t128 = t as u128;
            let s = a2 * t128 * t128 - a2 + t128 * t128;
            let v = isqrt128(s);
            if v as u128 * v as u128 == s {
                let b = a as u128 * t128 + v as u128;
                let n_val = a as u128 * b + t128;
                if n_val > big_n as u128 { break; }
                ans += n_val / 2;
            }
            t += 2;
        }
        a += 2;
    }

    println!("{ans}");
}
