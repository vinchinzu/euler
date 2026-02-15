// Project Euler 647 - Linear Transformations of Polygonal Numbers
// For positive odd k, find all (A,B) with A,B positive integers such that
// A * X_n + B is always a k-gonal number. Sum A+B over all such pairs.

fn isqrt(n: i64) -> i64 {
    let mut x = (n as f64).sqrt() as i64;
    while x > 0 && x * x > n { x -= 1; }
    while (x + 1) * (x + 1) <= n { x += 1; }
    x
}

fn main() {
    let n: i64 = 1_000_000_000_000; // 10^12
    let l = isqrt(n) as i32;

    let mut ans: i64 = 0;

    // odd sqrt_a only
    let mut sqrt_a = 1i32;
    while sqrt_a <= l {
        let a = sqrt_a as i64 * sqrt_a as i64;
        let d_max = (sqrt_a - 1) / 2;
        if d_max == 0 { sqrt_a += 2; continue; }

        // Iterate over all odd divisors of d_max
        let mut d = 1i32;
        while (d as i64) * (d as i64) <= d_max as i64 {
            if d_max % d == 0 {
                // Process divisor d
                if d % 2 == 1 {
                    let b = ((a - 1) / (8 * d as i64)) * ((d as i64 - 2) * (d as i64 - 2));
                    if b >= 1 && b <= n {
                        ans += a + b;
                    }
                }
                // Process complementary divisor
                let d2 = d_max / d;
                if d2 != d && d2 % 2 == 1 {
                    let b = ((a - 1) / (8 * d2 as i64)) * ((d2 as i64 - 2) * (d2 as i64 - 2));
                    if b >= 1 && b <= n {
                        ans += a + b;
                    }
                }
            }
            d += 1;
        }
        sqrt_a += 2;
    }

    println!("{}", ans);
}
