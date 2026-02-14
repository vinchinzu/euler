// Project Euler 294: Sum of digits - experience #23
// N=11^12 digits, divisible by 23, digit sum 23, modulo 10^9.

const K: usize = 23;
const M: i64 = 1_000_000_000;
const B: usize = 10;

type Table = [[i64; K + 1]; K];

fn pow_mod(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
    let mut result = 1i64;
    base %= modulus;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % modulus;
        }
        base = base * base % modulus;
        exp >>= 1;
    }
    result
}

fn combine(f1: &Table, f2: &Table, n: i64) -> Table {
    let mult = pow_mod(B as i64, n, K as i64);
    let mut out = [[0i64; K + 1]; K];
    for r1 in 0..K {
        for s1 in 0..=K {
            if f1[r1][s1] == 0 { continue; }
            for r2 in 0..K {
                let r = ((r1 as i64 * mult + r2 as i64) % K as i64) as usize;
                for s2 in 0..=(K - s1) {
                    if f2[r2][s2] == 0 { continue; }
                    out[r][s1 + s2] = (out[r][s1 + s2] + f1[r1][s1] * f2[r2][s2]) % M;
                }
            }
        }
    }
    out
}

fn main() {
    // N = 11^12
    let mut n: i64 = 1;
    for _ in 0..12 { n *= 11; }

    // Single digit table
    let mut single = [[0i64; K + 1]; K];
    for d in 0..B {
        single[d % K][d] = 1;
    }

    // Build via binary decomposition
    let mut bits = Vec::new();
    {
        let mut tmp = n;
        while tmp > 0 {
            bits.push((tmp & 1) as usize);
            tmp >>= 1;
        }
    }

    let mut result: Table = [[0; K + 1]; K];
    result[0][0] = 1;
    let mut result_width: i64 = 0;

    let mut power = single;
    let mut power_width: i64 = 1;

    for i in 0..bits.len() {
        if bits[i] == 1 {
            let temp = combine(&result, &power, power_width);
            result = temp;
            result_width += power_width;
        }
        if i + 1 < bits.len() {
            let temp = combine(&power, &power, power_width);
            power = temp;
            power_width *= 2;
        }
    }

    // Suppress unused variable warning
    let _ = result_width;

    println!("{}", result[0][K]);
}
