// Project Euler 261: Pivotal Square Sums
// Pell equation approach: for each m, solve x^2 - D*y^2 = m^2*(m+1).

const LIMIT: i64 = 10_000_000_000; // 10^10

fn is_square_64(n: i64) -> bool {
    if n < 0 { return false; }
    let r = isqrt64(n);
    r * r == n
}

fn isqrt64(n: i64) -> i64 {
    if n <= 0 { return 0; }
    let mut r = (n as f64).sqrt() as i64;
    while r > 0 && r * r > n { r -= 1; }
    while (r + 1) * (r + 1) <= n { r += 1; }
    r
}

#[derive(Clone)]
struct Factorization {
    primes: Vec<i64>,
    exps: Vec<i32>,
}

fn factorize(mut n: i64) -> Factorization {
    let mut f = Factorization { primes: Vec::new(), exps: Vec::new() };
    let mut d = 2i64;
    while d * d <= n {
        if n % d == 0 {
            let mut e = 0;
            while n % d == 0 { e += 1; n /= d; }
            f.primes.push(d);
            f.exps.push(e);
        }
        d += 1;
    }
    if n > 1 {
        f.primes.push(n);
        f.exps.push(1);
    }
    f
}

fn ipow(base: i64, exp: i32) -> i64 {
    let mut result = 1i64;
    for _ in 0..exp { result *= base; }
    result
}

fn main() {
    let max_l = isqrt64(LIMIT / 2);
    let mut pivots = std::collections::HashSet::new();

    for m in 1..=max_l {
        let d_val: i128 = m as i128 * (m as i128 + 1);

        let fm = factorize(m);
        let fm1 = factorize(m + 1);

        // Combined factorization of m*(m+1)
        let mut combined_p = Vec::new();
        let mut combined_e = Vec::new();
        for i in 0..fm.primes.len() {
            combined_p.push(fm.primes[i]);
            combined_e.push(fm.exps[i]);
        }
        for i in 0..fm1.primes.len() {
            let mut found = false;
            for j in 0..combined_p.len() {
                if combined_p[j] == fm1.primes[i] {
                    combined_e[j] += fm1.exps[i];
                    found = true;
                    break;
                }
            }
            if !found {
                combined_p.push(fm1.primes[i]);
                combined_e.push(fm1.exps[i]);
            }
        }

        // sD = squarefree part of D
        let mut s_d = 1i64;
        for i in 0..combined_p.len() {
            if combined_e[i] % 2 == 1 { s_d *= combined_p[i]; }
        }

        // sqrt(D/sD)
        let mut sqrt_d_over_sd = 1i64;
        for i in 0..combined_p.len() {
            sqrt_d_over_sd *= ipow(combined_p[i], combined_e[i] / 2);
        }

        // sm = product of p^ceil(e/2) for factors of m
        let mut sm = 1i64;
        for i in 0..fm.primes.len() {
            sm *= ipow(fm.primes[i], (fm.exps[i] + 1) / 2);
        }

        // Find base solutions
        let mut base_x: Vec<i128> = Vec::new();
        let mut base_y: Vec<i128> = Vec::new();

        let mut y = 0i64;
        while y <= m {
            let res = m + y * y;
            if res % s_d != 0 { y += sm; continue; }
            let quotient = res / s_d;
            if !is_square_64(quotient) { y += sm; continue; }
            let x = s_d as i128 * sqrt_d_over_sd as i128 * isqrt64(quotient) as i128;
            base_x.push(x);
            base_y.push(y as i128);
            y += sm;
        }

        // Fundamental solution: (2m+1, 2)
        let xf: i128 = 2 * m as i128 + 1;
        let yf: i128 = 2;

        for b in 0..base_x.len() {
            // Direct chain
            let mut x = base_x[b];
            let mut y = base_y[b];
            loop {
                if y + m as i128 > 2 * LIMIT as i128 { break; }
                if x % m as i128 == 0 {
                    let val = x / m as i128 - m as i128 - 1;
                    if val >= 0 && val % 2 == 0 {
                        let ym = y + m as i128;
                        if ym >= 0 && ym % 2 == 0 {
                            let k = ym / 2;
                            let n_val = val / 2;
                            if n_val >= k && k > 0 && k <= LIMIT as i128 {
                                pivots.insert(k as i64);
                            }
                        }
                    }
                }
                let nx = xf * x + d_val * yf * y;
                let ny = xf * y + yf * x;
                x = nx;
                y = ny;
            }

            // Conjugate chain
            if base_y[b] > 0 {
                x = xf * base_x[b] - d_val * yf * base_y[b];
                y = yf * base_x[b] - xf * base_y[b];
                while y < 0 {
                    let nx = xf * x + d_val * yf * y;
                    let ny = xf * y + yf * x;
                    x = nx;
                    y = ny;
                }
                loop {
                    if y + m as i128 > 2 * LIMIT as i128 { break; }
                    if x % m as i128 == 0 {
                        let val = x / m as i128 - m as i128 - 1;
                        if val >= 0 && val % 2 == 0 {
                            let ym = y + m as i128;
                            if ym >= 0 && ym % 2 == 0 {
                                let k = ym / 2;
                                let n_val = val / 2;
                                if n_val >= k && k > 0 && k <= LIMIT as i128 {
                                    pivots.insert(k as i64);
                                }
                            }
                        }
                    }
                    let nx = xf * x + d_val * yf * y;
                    let ny = xf * y + yf * x;
                    x = nx;
                    y = ny;
                }
            }
        }
    }

    let ans: i64 = pivots.iter().sum();
    println!("{}", ans);
}
