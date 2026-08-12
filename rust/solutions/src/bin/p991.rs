// Problem 991: Fruit Salad
// Parameterize positive solutions to a/(b+c)+b/(c+a)+c/(a+c)=4,
// then sum multiples of primitive solutions with a+b+c <= 10^7.

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a.abs()
}

fn isqrt(n: i64) -> i64 {
    if n <= 0 {
        return 0;
    }
    let mut x = (n as f64).sqrt() as i64;
    while (x + 1) * (x + 1) <= n {
        x += 1;
    }
    while x * x > n {
        x -= 1;
    }
    x
}

/// Sums of all primitive positive solutions with sum <= limit.
fn primitive_solutions(limit: i64) -> Vec<i64> {
    let mut sums = Vec::new();

    // Plus branch: s = 6m^2 - n^2 + mn > 4m^2, so m <= sqrt(limit/4).
    let m_max = isqrt(limit / 4) + 2;
    for m in 1..=m_max {
        let n_min = isqrt(3 * m * m) + 1;
        let n_max = 2 * m - 1;
        for n in n_min..=n_max {
            if gcd(m, n) != 1 {
                continue;
            }
            let a = 4 * m * m - n * n;
            let c = n * n - 3 * m * m;
            let b = 5 * m * m - n * n + m * n;
            let s = a + b + c;
            if a <= 0 || b <= 0 || c <= 0 {
                continue;
            }
            if s <= limit {
                sums.push(s);
            }
        }
    }

    // Minus branch: k = 2m - n, s = k(5m - k).
    let alpha = 2.0 + 3.0_f64.sqrt();
    let beta = (5.0 + 21.0_f64.sqrt()) / 2.0;

    let mut k: i64 = 1;
    loop {
        let mut low = (alpha * k as f64) as i64 + 1;
        while (2 * low - k) * (2 * low - k) <= 3 * low * low {
            low += 1;
        }

        let mut high_pos = (beta * k as f64) as i64;
        while high_pos > 0 && !(-high_pos * high_pos + 5 * high_pos * k - k * k > 0) {
            high_pos -= 1;
        }

        let high_sum = (limit + k * k) / (5 * k);
        let high = high_pos.min(high_sum);

        if low > high_sum {
            break;
        }

        for m in low..=high {
            if gcd(m, k) != 1 {
                continue;
            }
            let n = 2 * m - k;
            let a = 4 * m * m - n * n;
            let c = n * n - 3 * m * m;
            let b = 5 * m * m - n * n - m * n;
            let s = a + b + c;
            if a <= 0 || b <= 0 || c <= 0 {
                continue;
            }
            if s <= limit {
                sums.push(s);
            }
        }
        k += 1;
    }

    sums
}

fn solve(limit: i64) -> i64 {
    let primitive = primitive_solutions(limit);
    let mut total: i64 = 0;
    for s in primitive {
        let count = limit / s;
        total += s * count * (count + 1) / 2;
    }
    total
}

fn main() {
    const LIMIT: i64 = 10_000_000;
    println!("{}", solve(LIMIT));
}
