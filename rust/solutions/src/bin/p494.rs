// Project Euler 494: Collatz prefix families

use std::collections::HashSet;

const NSTEPS: usize = 90;
const L: u64 = 100_000;

fn fibonacci(n: usize) -> u64 {
    if n == 0 { return 0; }
    let (mut a, mut b) = (1u64, 1u64);
    for _ in 2..n {
        let c = a + b;
        a = b;
        b = c;
    }
    if n == 1 { a } else { b }
}

fn is_power_of_2(n: u64) -> bool {
    n > 0 && (n & (n - 1)) == 0
}

fn helper_func(n: u64, num_steps: usize, n_even: bool, m3_39: u64) -> u64 {
    if num_steps == 0 { return 1; }
    let mut result = helper_func((n * 2) % m3_39, num_steps - 1, true, m3_39);
    if n_even && n % 3 == 1 {
        result += helper_func((n - 1) / 3, num_steps - 1, false, m3_39);
    }
    result
}

fn main() {
    let m3_39: u64 = 3u64.pow(39);

    let ans_base = fibonacci(NSTEPS);
    let mut ans = ans_base;

    let mut specials: Vec<(u64, Vec<u64>)> = Vec::new();
    let mut special_set: HashSet<u64> = HashSet::new();

    for start in 1..L {
        let mut collatz: Vec<u64> = Vec::new();
        let mut n = start;
        let mut r = 1.0f64;
        let mut is_special = false;

        for _ in 0..NSTEPS {
            collatz.push(n);
            if n % 2 == 0 {
                n /= 2;
                r /= 2.0;
            } else {
                n = 3 * n + 1;
                r *= 3.0;
            }
            if is_power_of_2(n) { break; }
            if n > start && r < 1.0 { is_special = true; }
        }

        if is_special {
            special_set.insert(start);
            specials.push((start, collatz));
        }
    }

    for (start, collatz) in &specials {
        let is_redundant = collatz[1..].iter().any(|v| special_set.contains(v));
        if !is_redundant {
            let num_steps = NSTEPS - collatz.len();
            let paths = helper_func(*start, num_steps, false, m3_39);
            ans += paths;
        }
    }

    println!("{}", ans);
}
