// Project Euler 768 - Chandelier Balance
// Meet-in-the-middle on ring positions with polynomial exponentiation.

use std::collections::HashMap;

fn pow_mod(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut result = 1i64;
    base = ((base % m) + m) % m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result as i128 * base as i128 % m as i128) as i64;
        }
        base = (base as i128 * base as i128 % m as i128) as i64;
        exp >>= 1;
    }
    result
}

fn find_generator(p: i64) -> i64 {
    let phi = p - 1;
    let mut factors = Vec::new();
    let mut temp = phi;
    let mut i = 2i64;
    while i * i <= temp {
        if temp % i == 0 {
            factors.push(i);
            while temp % i == 0 { temp /= i; }
        }
        i += 1;
    }
    if temp > 1 { factors.push(temp); }

    for g in 2..p {
        if factors.iter().all(|&f| pow_mod(g, phi / f, p) != 1) {
            return g;
        }
    }
    -1
}

fn rad(mut n: i32) -> i32 {
    let mut result = 1;
    let mut p = 2;
    while p * p <= n {
        if n % p == 0 {
            result *= p;
            while n % p == 0 { n /= p; }
        }
        p += 1;
    }
    if n > 1 { result *= n; }
    result
}

fn find_prime(l: i64) -> i64 {
    let mut p = 2_000_000_001i64;
    loop {
        if p % l == 1 {
            let mut is_prime = true;
            let mut i = 2i64;
            while i * i <= p {
                if p % i == 0 { is_prime = false; break; }
                i += 1;
            }
            if is_prime { return p; }
        }
        p += 2;
    }
}

const KK: usize = 20;
const NN: i32 = 360;

fn poly_mul(a: &[i64; KK + 1], b: &[i64; KK + 1]) -> [i64; KK + 1] {
    let mut temp = [0i64; KK + 1];
    for i in 0..=KK {
        if a[i] == 0 { continue; }
        for j in 0..=KK {
            if i + j > KK { break; }
            temp[i + j] += a[i] * b[j];
        }
    }
    temp
}

fn poly_pow(base_poly: &[i64; KK + 1], mut exp: i32) -> [i64; KK + 1] {
    let mut result = [0i64; KK + 1];
    result[0] = 1;
    let mut b = *base_poly;
    while exp > 0 {
        if exp & 1 == 1 {
            result = poly_mul(&result, &b);
        }
        b = poly_mul(&b, &b);
        exp >>= 1;
    }
    result
}

fn main() {
    let l = rad(NN) as usize; // 30
    let half = l / 2; // 15

    let p = find_prime(l as i64);
    let g = find_generator(p);
    let w = pow_mod(g, (p - 1) / l as i64, p);

    let mut ws = vec![0i64; l];
    ws[0] = 1;
    for i in 1..l {
        ws[i] = (ws[i - 1] as i128 * w as i128 % p as i128) as i64;
    }

    // Meet in the middle: first half
    let mut first_half_map: HashMap<i64, Vec<i64>> = HashMap::new();

    for subset in 0..(1u32 << half) {
        let mut weight: i64 = 0;
        let mut bit_count = 0usize;
        for i in 0..half {
            if subset & (1 << i) != 0 {
                weight = (weight + ws[i]) % p;
                bit_count += 1;
            }
        }
        if bit_count > KK { continue; }

        let entry = first_half_map.entry(weight).or_insert_with(|| vec![0i64; KK + 1]);
        entry[bit_count] += 1;
    }

    // Second half
    let second_half = l - half;
    let mut num_balanced = [0i64; KK + 1];

    for subset in 0..(1u32 << second_half) {
        let mut weight: i64 = 0;
        let mut bit_count = 0usize;
        for i in 0..second_half {
            if subset & (1 << i) != 0 {
                weight = (weight + ws[half + i]) % p;
                bit_count += 1;
            }
        }
        if bit_count > KK { continue; }

        let target = (p - weight) % p;
        if let Some(counts) = first_half_map.get(&target) {
            for bc1 in 0..=KK - bit_count {
                if counts[bc1] > 0 {
                    num_balanced[bc1 + bit_count] += counts[bc1];
                }
            }
        }
    }

    let exponent = NN / l as i32;
    let result = poly_pow(&num_balanced, exponent);

    println!("{}", result[KK]);
}
