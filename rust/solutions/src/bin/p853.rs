// Project Euler 853 - Pisano Period

fn fib_mod(k: u64, m: u64) -> (u64, u64) {
    if m == 1 { return (0, 0); }
    if k == 0 { return (0, 1 % m); }
    let (a, b) = fib_mod(k >> 1, m);
    let two_b = (b << 1) % m;
    let temp = (two_b + m + m - a) % m;
    let c = ((a as u128 * temp as u128) % m as u128) as u64;
    let d = ((a as u128 * a as u128 + b as u128 * b as u128) % m as u128) as u64;
    if k & 1 == 1 {
        (d, (c + d) % m)
    } else {
        (c, d)
    }
}

fn fib_exact(k: u64) -> (u128, u128) {
    if k == 0 { return (0, 1); }
    let (a, b) = fib_exact(k >> 1);
    let c = a * (2 * b - a);
    let d = a * a + b * b;
    if k & 1 == 1 { (d, c + d) } else { (c, d) }
}

fn factorize128(mut n: u128) -> Vec<(u128, u32)> {
    let mut result = Vec::new();
    let mut d: u128 = 2;
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

fn gen_divisors(factors: &[(u128, u32)], idx: usize, current: u128, limit: u64, result: &mut Vec<u64>) {
    if current > limit as u128 { return; }
    if idx == factors.len() {
        result.push(current as u64);
        return;
    }
    let (p, e) = factors[idx];
    let mut pe: u128 = 1;
    for _ in 0..=e {
        gen_divisors(factors, idx + 1, current * pe, limit, result);
        if pe * p > limit as u128 { break; }
        pe *= p;
    }
}

const DIVISORS_120: [u64; 15] = [1, 2, 3, 4, 5, 6, 8, 10, 12, 15, 20, 24, 30, 40, 60];

fn main() {
    let k: u64 = 120;
    let n: u64 = 1_000_000_000;

    let (fk, _) = fib_exact(k);
    let factors = factorize128(fk);

    let mut divisors = Vec::new();
    gen_divisors(&factors, 0, 1, n, &mut divisors);

    let mut total: u64 = 0;
    for &div in &divisors {
        if div <= 1 { continue; }

        let (fk_val, fk1_val) = fib_mod(k, div);
        if fk_val != 0 || fk1_val != 1 { continue; }

        let mut exact = true;
        for &d in &DIVISORS_120 {
            let (fd, fd1) = fib_mod(d, div);
            if fd == 0 && fd1 == 1 {
                exact = false;
                break;
            }
        }

        if exact { total += div; }
    }

    println!("{}", total);
}
