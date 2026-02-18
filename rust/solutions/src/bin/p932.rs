// Project Euler 932 — 2025
// 2025 = (20+25)^2. ab is a "2025-number" if ab = (a+b)^2.
// T(n) = sum of all 2025-numbers with n digits or less. T(4) = 5131.
// Find T(16).
// Expected: 72673459417881349

fn isqrt(n: u128) -> u128 {
    if n == 0 { return 0; }
    let mut x = (n as f64).sqrt() as u128;
    // Newton's method refinement
    loop {
        let x1 = (x + n / x) / 2;
        if x1 >= x { break; }
        x = x1;
    }
    // Ensure exact
    while x * x > n { x -= 1; }
    while (x + 1) * (x + 1) <= n { x += 1; }
    x
}

fn compute(max_digits: u32) -> u128 {
    let mut total: u128 = 0;
    // -1 to account for (a, b) = (0, 1) which gives 01 = 1 but a must be > 0
    // Actually the Python starts with total = -1. Let's follow the logic.
    // When b has n digits, we iterate b from 1 to 10^(N/2)-1
    // For each b, compute discriminant v^2 = 4b(1-10^n) + 10^(2n)
    // = 10^(2n) - 4b*10^n + 4b = (10^n - 2b)^2 - 4b^2 + 4b = (10^n)^2 - 4b(10^n - 1)
    // If v is a perfect square, then a = (10^n - 2b ± v) / 2

    let half_digits = max_digits / 2;
    let b_limit = 10u128.pow(half_digits); // b < 10^(N/2)

    for b in 1..b_limit {
        let n = {
            let mut digits = 0u32;
            let mut tmp = b;
            while tmp > 0 { digits += 1; tmp /= 10; }
            digits
        };
        let pow10n = 10u128.pow(n);
        let pow10_2n = pow10n * pow10n;

        // v^2 = 4*b*(1 - 10^n) + 10^(2n)
        // = 10^(2n) - 4*b*(10^n - 1)
        let term = 4 * b * (pow10n - 1);
        if pow10_2n <= term { continue; }
        let disc = pow10_2n - term;

        let v = isqrt(disc);
        if v * v != disc { continue; }

        // a1 = (10^n - 2*b + v) / 2
        let num1 = pow10n - 2 * b + v;
        if num1 % 2 == 0 {
            let a = num1 / 2;
            if a > 0 {
                let val = a * pow10n + b;
                // Verify total digits <= max_digits
                if val > 0 {
                    let total_digits = {
                        let mut d = 0u32;
                        let mut tmp = val;
                        while tmp > 0 { d += 1; tmp /= 10; }
                        d
                    };
                    if total_digits <= max_digits {
                        total += val;
                    }
                }
            }
        }

        // a2 = (10^n - 2*b - v) / 2
        if pow10n >= 2 * b + v {
            let num2 = pow10n - 2 * b - v;
            if num2 % 2 == 0 {
                let a = num2 / 2;
                if a > 0 {
                    let val = a * pow10n + b;
                    if val > 0 {
                        let total_digits = {
                            let mut d = 0u32;
                            let mut tmp = val;
                            while tmp > 0 { d += 1; tmp /= 10; }
                            d
                        };
                        if total_digits <= max_digits {
                            total += val;
                        }
                    }
                }
            }
        }
    }

    total
}

fn main() {
    debug_assert_eq!(compute(4), 5131);
    println!("{}", compute(16));
}
