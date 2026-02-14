// Project Euler 108: Diophantine Reciprocals I
// Find smallest n with > 1000 solutions to 1/x + 1/y = 1/n.
// Number of solutions = (d(n^2) + 1) / 2 where d = divisor count.

const PRIMES: [i64; 13] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41];

fn search(idx: usize, limit_exp: i32, current_n: i64, divisor_count: i64, target: i64, best: &mut i64) {
    if divisor_count > target {
        if current_n < *best {
            *best = current_n;
        }
        return;
    }
    if idx >= PRIMES.len() {
        return;
    }

    let p = PRIMES[idx];
    let mut value = current_n * p;
    let mut exp = 1;

    while exp <= limit_exp {
        if value >= *best {
            break;
        }
        let new_div = divisor_count * (2 * exp as i64 + 1);
        search(idx + 1, exp, value, new_div, target, best);
        exp += 1;
        value *= p;
    }
}

fn main() {
    let threshold = 1000i64;
    let target_divisors = 2 * threshold - 1;
    let mut best = 1i64 << 60;
    search(0, 20, 1, 1, target_divisors, &mut best);
    println!("{best}");
}
