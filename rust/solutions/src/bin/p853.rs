// Project Euler 853 - Pisano Period

fn fib_mod(k: i64, m: i64) -> (i64, i64) {
    if m == 1 { return (0, 0); }
    if k == 0 { return (0, 1 % m); }
    let (a, b) = fib_mod(k >> 1, m);
    let c = (a as i128 * (2 * b as i128 - a as i128 + m as i128 + m as i128) % m as i128) as i64;
    let d = ((a as i128 * a as i128 + b as i128 * b as i128) % m as i128) as i64;
    if k & 1 == 1 {
        (d, (c + d) % m)
    } else {
        (c, d)
    }
}

fn fib_exact(k: i64) -> (i128, i128) {
    if k == 0 { return (0, 1); }
    let (a, b) = fib_exact(k >> 1);
    let c = a * (2 * b - a);
    let d = a * a + b * b;
    if k & 1 == 1 { (d, c + d) } else { (c, d) }
}

fn factorize128(mut n: i128) -> Vec<(i128, i32)> {
    let mut result = Vec::new();
    let mut d: i128 = 2;
    while d * d <= n {
        if n % d == 0 {
            let mut e = 0;
            while n % d == 0 { n /= d; e += 1; }
            result.push((d, e));
        }
        d += 1;
    }
    if n > 1 { result.push((n, 1)); }
    result
}

fn gen_divisors(factors: &[(i128, i32)], idx: usize, current: i128, limit: i64, result: &mut Vec<i64>) {
    if current > limit as i128 { return; }
    if idx == factors.len() {
        result.push(current as i64);
        return;
    }
    let (p, e) = factors[idx];
    let mut pe: i128 = 1;
    for _ in 0..=e {
        gen_divisors(factors, idx + 1, current * pe, limit, result);
        if pe as i128 * p > limit as i128 { break; }
        pe *= p;
    }
}

fn main() {
    let k = 120;
    let n: i64 = 1_000_000_000;

    let (fk, _) = fib_exact(k);

    let factors = factorize128(fk);

    let mut divisors = Vec::new();
    gen_divisors(&factors, 0, 1, n, &mut divisors);

    // Proper divisors of 120 (less than 120)
    let divisors_120: Vec<i32> = {
        let mut d = Vec::new();
        for i in 1..120 {
            if 120 % i == 0 { d.push(i); }
        }
        d
    };

    let mut total: i64 = 0;
    for &div in &divisors {
        if div <= 0 { continue; }
        if div == 1 { if k == 1 { total += 1; } continue; }

        let (fk_val, fk1_val) = fib_mod(k as i64, div);
        if fk_val != 0 || fk1_val != 1 { continue; }

        let mut exact = true;
        for &d in &divisors_120 {
            let (fd, fd1) = fib_mod(d as i64, div);
            if fd == 0 && fd1 == 1 { exact = false; break; }
        }

        if exact { total += div; }
    }

    println!("{}", total);
}
