// Project Euler 263: An engineers' dream come true
//
// Find the sum of the first 4 engineers' paradise numbers n where:
// - n-9, n-3, n+3, n+9 are consecutive primes (sexy prime quadruplet)
// - n-8, n-4, n, n+4, n+8 are practical numbers

use euler_utils::{mod_pow, mod_mul};

fn miller_rabin_witness(n: i64, a: i64) -> bool {
    if n % a == 0 {
        return n == a;
    }
    let mut d = n - 1;
    let mut r = 0;
    while d % 2 == 0 {
        d /= 2;
        r += 1;
    }
    let mut x = mod_pow(a as u64, d as u64, n as u64) as i64;
    if x == 1 || x == n - 1 {
        return true;
    }
    for _ in 0..r - 1 {
        x = mod_mul(x as u64, x as u64, n as u64) as i64;
        if x == n - 1 {
            return true;
        }
    }
    false
}

fn isprime(n: i64) -> bool {
    if n < 2 { return false; }
    if n < 4 { return true; }
    if n % 2 == 0 || n % 3 == 0 { return false; }
    if n % 5 == 0 { return n == 5; }
    if n % 7 == 0 { return n == 7; }
    miller_rabin_witness(n, 2) && miller_rabin_witness(n, 3)
        && miller_rabin_witness(n, 5) && miller_rabin_witness(n, 7)
}

fn is_practical(n: i64) -> bool {
    if n <= 1 { return true; }
    if n % 2 != 0 { return false; }

    let mut tmp = n;
    let mut pw: i64 = 1;
    while tmp % 2 == 0 {
        tmp /= 2;
        pw *= 2;
    }
    let mut sigma = 2 * pw - 1;

    let mut p = 3i64;
    while p * p <= tmp {
        if tmp % p == 0 {
            if p > sigma + 1 { return false; }
            pw = 1;
            while tmp % p == 0 {
                tmp /= p;
                pw *= p;
            }
            sigma *= (pw * p - 1) / (p - 1);
        }
        p += 2;
    }
    if tmp > 1 {
        if tmp > sigma + 1 { return false; }
    }
    true
}

fn main() {
    let mut found = 0;
    let mut total: i64 = 0;
    let target = 4;

    let mut i: i64 = 1;
    while found < target {
        for sign in &[-1i64, 1i64] {
            if found >= target { break; }
            let n = 840 * i + sign * 20;
            if n < 20 { continue; }

            if !isprime(n - 9) || !isprime(n - 3) || !isprime(n + 3) || !isprime(n + 9) {
                continue;
            }

            // Check consecutive: no primes between n-9 and n-3, etc.
            let mut consecutive = true;
            {
                let mut k = n - 7;
                while k < n - 3 {
                    if isprime(k) { consecutive = false; break; }
                    k += 2;
                }
            }
            if !consecutive { continue; }

            {
                let mut k = n - 1;
                while k < n + 3 {
                    if isprime(k) { consecutive = false; break; }
                    k += 2;
                }
            }
            if !consecutive { continue; }

            {
                let mut k = n + 5;
                while k < n + 9 {
                    if isprime(k) { consecutive = false; break; }
                    k += 2;
                }
            }
            if !consecutive { continue; }

            if !is_practical(n - 8) || !is_practical(n - 4) || !is_practical(n)
                || !is_practical(n + 4) || !is_practical(n + 8)
            {
                continue;
            }

            total += n;
            found += 1;
        }
        i += 1;
    }

    println!("{}", total);
}
