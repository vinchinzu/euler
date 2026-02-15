// Project Euler 622 - Riffle Shuffles
// Find sum of even n with order(2, n-1) = 60, i.e. n-1 | 2^60-1

const PRIMES: [i64; 11] = [3, 5, 7, 11, 13, 31, 41, 61, 151, 331, 1321];
const EXPONENTS: [i32; 11] = [1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1];

fn powmod(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut result = 1i64;
    base = ((base % m) + m) % m;
    while exp > 0 {
        if exp & 1 == 1 { result = (result as i128 * base as i128 % m as i128) as i64; }
        base = (base as i128 * base as i128 % m as i128) as i64;
        exp >>= 1;
    }
    result
}

fn order2(n: i64) -> i64 {
    if n <= 1 { return 1; }
    let mut phi = n;
    let mut temp = n;
    for &p in &PRIMES {
        if p * p > temp { break; }
        if temp % p == 0 {
            phi = phi / p * (p - 1);
            while temp % p == 0 { temp /= p; }
        }
    }
    if temp > 1 { phi = phi / temp * (temp - 1); }

    let mut result = phi;
    let mut temp = phi;
    let mut factors = Vec::new();
    let mut p = 2i64;
    while p * p <= temp {
        if temp % p == 0 {
            let mut e = 0;
            while temp % p == 0 { e += 1; temp /= p; }
            factors.push((p, e));
        }
        p += 1;
    }
    if temp > 1 { factors.push((temp, 1)); }

    for &(f, e) in &factors {
        for _ in 0..e {
            if powmod(2, result / f, n) == 1 {
                result /= f;
            } else {
                break;
            }
        }
    }
    result
}

fn main() {
    let mut ans: i64 = 0;

    fn enumerate(idx: usize, d: i64, ans: &mut i64) {
        if idx == 11 {
            if d > 1 && order2(d) == 60 {
                *ans += d + 1;
            }
            return;
        }
        let mut pp = 1i64;
        for _ in 0..=EXPONENTS[idx] {
            enumerate(idx + 1, d * pp, ans);
            pp *= PRIMES[idx];
        }
    }

    enumerate(0, 1, &mut ans);
    println!("{}", ans);
}
