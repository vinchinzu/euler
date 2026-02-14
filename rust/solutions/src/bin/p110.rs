// Project Euler Problem 110: Diophantine Reciprocals II
// Find smallest n with > 4,000,000 solutions to 1/x + 1/y = 1/n
// Uses u128 for large values.

const PRIMES: [u128; 15] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];

static mut BEST: u128 = 1u128 << 120;

fn search(idx: usize, limit_exp: u32, current_n: u128, divisor_count: i64, target: i64) {
    if divisor_count > target {
        unsafe {
            if current_n < BEST {
                BEST = current_n;
            }
        }
        return;
    }
    if idx >= PRIMES.len() {
        return;
    }

    let p = PRIMES[idx];
    let mut value = current_n * p;
    let mut exp = 1u32;

    while exp <= limit_exp {
        unsafe {
            if value >= BEST {
                break;
            }
        }
        let new_div = divisor_count * (2 * exp as i64 + 1);
        search(idx + 1, exp, value, new_div, target);
        exp += 1;
        value *= p;
    }
}

fn main() {
    let threshold: i64 = 4_000_000;
    let target_divisors = 2 * threshold - 1;

    search(0, 20, 1, 1, target_divisors);

    let result = unsafe { BEST };
    println!("{}", result);
}
