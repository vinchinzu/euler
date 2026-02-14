// Project Euler 511 - Sequences with Divisibility Constraints
// Cyclic convolution mod K using binary exponentiation.

const K: usize = 4321;
const MOD: i64 = 1_000_000_000;

fn poly_multiply_cyclic(p1: &[i64; K], p2: &[i64; K]) -> Box<[i64; K]> {
    let mut result = Box::new([0i64; K]);
    for i in 0..K {
        if p1[i] == 0 { continue; }
        for j in 0..K {
            let idx = (i + j) % K;
            result[idx] = (result[idx] + (p1[i] as i128 * p2[j] as i128 % MOD as i128) as i64) % MOD;
        }
    }
    result
}

fn find_divisors(n: i64) -> Vec<i64> {
    let mut divs = Vec::new();
    let mut i = 1i64;
    while i * i <= n {
        if n % i == 0 {
            divs.push(i);
            if i * i != n { divs.push(n / i); }
        }
        i += 1;
    }
    divs
}

fn imod(a: i64, m: i64) -> usize {
    ((a % m + m) % m) as usize
}

fn num_transitions(n: i64, divisors: &[i64]) -> Box<[i64; K]> {
    if n == 1 {
        let mut result = Box::new([0i64; K]);
        for &d in divisors {
            let idx = imod(d, K as i64);
            result[idx] = (result[idx] + 1) % MOD;
        }
        return result;
    }

    let half = num_transitions(n / 2, divisors);
    let mut result = poly_multiply_cyclic(&half, &half);

    if n % 2 == 1 {
        let one = num_transitions(1, divisors);
        let temp = result;
        result = poly_multiply_cyclic(&temp, &one);
    }

    result
}

fn main() {
    let n: i64 = 1234567898765;
    let divisors = find_divisors(n);

    let transitions = num_transitions(n, &divisors);

    let idx = imod(-n, K as i64);
    println!("{}", transitions[idx]);
}
