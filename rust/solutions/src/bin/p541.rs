// Project Euler 541 - Divisibility of Harmonic Number Denominators
//
// Find the largest n such that H_n has denominator not divisible by P=137,
// using p-adic approach with Lagrange interpolation and BFS over base-P digits.

use std::collections::HashMap;

const P: i64 = 137;

fn pow_mod(mut base: i128, mut exp: i64, modulus: i64) -> i64 {
    if modulus <= 0 { return 0; }
    let m = modulus as i128;
    let mut result: i128 = 1;
    base = ((base % m) + m) % m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % m;
        }
        base = base * base % m;
        exp >>= 1;
    }
    result as i64
}

fn mod_inv(a: i64, m: i64) -> i64 {
    if m == 1 { return 0; }
    let mut t: i64 = 0;
    let mut new_t: i64 = 1;
    let mut r: i64 = m;
    let mut new_r: i64 = ((a % m) + m) % m;
    while new_r != 0 {
        let q = r / new_r;
        let tmp = new_t;
        new_t = t - q * new_t;
        t = tmp;
        let tmp = new_r;
        new_r = r - q * new_r;
        r = tmp;
    }
    if t < 0 { t += m; }
    t
}

fn mod_pos(a: i64, m: i64) -> i64 {
    ((a % m) + m) % m
}

fn pow_int(base: i64, exp: i32) -> i64 {
    let mut result: i64 = 1;
    for _ in 0..exp {
        result *= base;
    }
    result
}

fn sum_powers(limit: i64, exp: i32, mod_val: i64) -> i64 {
    if limit == 0 { return 0; }
    let n = (exp + 2) as i64;
    let mut sum_pows: i64 = 0;
    let mut result: i64 = 0;

    for j in 1..=n {
        sum_pows = mod_pos(sum_pows + pow_mod(j as i128, exp as i64, mod_val), mod_val);
        let mut res = sum_pows;
        if (exp as i64 + j) % 2 != 0 {
            res = mod_pos(-res, mod_val);
        }
        let mut denom: i64 = 1;
        for k in 1..j {
            denom = ((denom as i128) * (k as i128) % (mod_val as i128)) as i64;
        }
        for k in 1..=(n - j) {
            denom = ((denom as i128) * (k as i128) % (mod_val as i128)) as i64;
        }
        let inv_denom = mod_inv(denom, mod_val);
        res = ((res as i128) * (inv_denom as i128) % (mod_val as i128)) as i64;

        for k in 1..=n {
            if k != j {
                res = ((res as i128) * (mod_pos(limit - k, mod_val) as i128) % (mod_val as i128)) as i64;
            }
        }
        result = mod_pos(result + res, mod_val);
    }
    result
}

fn h(n: i64, e: i32, cache: &mut HashMap<(i64, i32), i64>) -> i64 {
    if n == 0 { return 0; }
    if let Some(&v) = cache.get(&(n, e)) {
        return v;
    }

    let h_val = h(n / P, e + 1, cache);
    if h_val == -1 || h_val % P != 0 {
        cache.insert((n, e), -1);
        return -1;
    }

    let mut h_val = h_val / P;
    let pe = pow_int(P, e);

    for r in 1..P {
        let l = if (n % P + r) < P { n / P } else { n / P + 1 };
        let r_inv = mod_inv(r, pe);
        for k in 0..e {
            let pr_inv = ((P as i128) * (r_inv as i128) % (pe as i128)) as i64;
            let term_base = pow_mod(pr_inv as i128, k as i64, pe);
            let term = ((r_inv as i128) * (term_base as i128) % (pe as i128)) as i64;
            let sp = sum_powers(l, k, pe);
            let term = ((term as i128) * (sp as i128) % (pe as i128)) as i64;
            h_val = mod_pos(h_val - term, pe);
        }
    }

    let result = mod_pos(h_val, pe);
    cache.insert((n, e), result);
    result
}

fn main() {
    let mut cache: HashMap<(i64, i32), i64> = HashMap::new();

    let mut queue: Vec<i64> = Vec::with_capacity(200000);
    queue.push(0);
    let mut head = 0;
    let mut ans: i64 = 0;

    while head < queue.len() {
        let n = queue[head];
        head += 1;
        if h(n * P, 0, &mut cache) == 0 {
            let start = if n == 0 { 1 } else { 0 };
            for i in start..P {
                if queue.len() < 200000 {
                    queue.push(n * P + i);
                }
            }
        }
        ans = n;
    }

    println!("{ans}");
}
