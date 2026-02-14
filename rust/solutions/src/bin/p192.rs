// Project Euler 192: Best Approximations
// For each non-square n in [2, 100000], find the best rational approximation
// to sqrt(n) with denominator <= 10^12. Sum the denominators.
// Uses continued fraction expansion and semiconvergent comparison.

fn isqrt(n: i64) -> i64 {
    let mut r = (n as f64).sqrt() as i64;
    while r * r > n { r -= 1; }
    while (r + 1) * (r + 1) <= n { r += 1; }
    r
}

fn main() {
    let big_n = 100_000i64;
    let k = 1_000_000_000_000i64;
    let mut total: i64 = 0;

    for n in 2..=big_n {
        let a0 = isqrt(n);
        if a0 * a0 == n { continue; }

        let mut m: i64 = 0;
        let mut d: i64 = 1;
        let mut a = a0;
        let (mut a_prev, mut b_prev) = (1i64, 0i64);
        let (mut a_curr, mut b_curr) = (a0, 1i64);

        loop {
            m = d * a - m;
            d = (n - m * m) / d;
            a = (a0 + m) / d;

            let a_next = a * a_curr + a_prev;
            let b_next = a * b_curr + b_prev;

            if b_next > k {
                let h = (k - b_prev) / b_curr;
                let den1 = b_curr;
                let den2 = b_prev + h * b_curr;

                if h > a / 2 {
                    total += den2;
                } else if h < (a + 1) / 2 {
                    total += den1;
                } else {
                    let num1 = a_curr;
                    let num2 = a_prev + h * a_curr;

                    let cross1 = num1 as i128 * den2 as i128;
                    let cross2 = num2 as i128 * den1 as i128;
                    let bot = den1 as i128 * den2 as i128;

                    let c1sq = cross1 * cross1;
                    let c2sq = cross2 * cross2;
                    let diff_sq = {
                        let d = c1sq - c2sq;
                        d * d
                    };

                    let cdiff = cross1 - cross2;
                    let rhs = 4 * n as i128 * cdiff * cdiff * bot * bot;

                    let cond1 = diff_sq > rhs;
                    let cond2 = cross1 > cross2;
                    if cond1 ^ cond2 {
                        total += den1;
                    } else {
                        total += den2;
                    }
                }
                break;
            }

            a_prev = a_curr; a_curr = a_next;
            b_prev = b_curr; b_curr = b_next;
        }
    }

    println!("{}", total);
}
