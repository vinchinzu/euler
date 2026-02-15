// Project Euler 748 - Upside Down Diophantine Equation
//
// Enumerate coprime pairs (m,n) and compute Pythagorean-like triples.

const N: i64 = 10_000_000_000_000_000; // 10^16
const M: i64 = 1_000_000_000; // 10^9

fn sq(x: i64) -> i64 {
    x * x
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn main() {
    let a_const = ((6.5f64).sqrt() - 2.0) / (3.0 - (6.5f64).sqrt());
    let b_const = 2.0 / ((13.0f64).sqrt() - 3.0);

    let mut ans: i64 = 0;

    let process = |m: i64, n: i64, g: i64, ans: &mut i64| {
        let a = sq(m) + sq(n);
        let b = -2 * sq(m) + 6 * m * n + 2 * sq(n);
        let c = 3 * sq(m) + 4 * m * n - 3 * sq(n);
        let x = a * b / g;
        let y = a * c / g;
        let z = b * c / g;
        if y <= N && z <= N && y > 0 && z > 0 {
            *ans = (*ans + x + y + z) % M;
        }
    };

    // Section 1
    let mut n = 1i64;
    while 8 * sq(n) * sq(n) <= 4 * N {
        let mut m = n + 1;
        while (m as f64) < b_const * n as f64
            && (sq(m) + sq(n)) * (3 * sq(m) + 4 * m * n - 3 * sq(n)) <= 4 * N
        {
            if m as f64 > a_const * n as f64
                && gcd(m % n, n) == 1
                && (2 * m - 3 * n) % 13 != 0
            {
                let g = if (m + n) % 2 == 0 { 4 } else { 1 };
                process(m, n, g, &mut ans);
            }
            m += 1;
        }
        n += 1;
    }

    // Section 2
    n = 1;
    while 8 * sq(n) * sq(n) <= 676 * N {
        let start_m = n + (7 * n) % 13;
        let mut m = start_m;
        while (m as f64) < b_const * n as f64
            && (sq(m) + sq(n)) * (3 * sq(m) + 4 * m * n - 3 * sq(n)) <= 676 * N
        {
            if m as f64 > a_const * n as f64 && gcd(m, n) == 1 {
                let g = if (m + n) % 2 == 0 { 676 } else { 169 };
                process(m, n, g, &mut ans);
            }
            m += 13;
        }
        n += 1;
    }

    println!("{}", ans);
}
