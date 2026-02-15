// Project Euler 616 - Creative numbers
// Sum of all creative numbers n <= 10^12

use std::collections::HashSet;

fn main() {
    let n: i64 = 1_000_000_000_000;
    let limit = 1_000_000usize;

    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut i = 2;
    while i * i <= limit { if is_prime[i] { let mut j = i*i; while j <= limit { is_prime[j] = false; j += i; } } i += 1; }

    let mut creative = HashSet::new();

    let mut a = 2i64;
    while a * a <= n {
        let mut power = a * a;
        let mut b = 2;
        while power <= n {
            let a_prime = (a as usize) <= limit && is_prime[a as usize];
            let b_prime = b <= limit && is_prime[b];
            if !(a_prime && b_prime) {
                creative.insert(power);
            }
            if power > n / a { break; }
            power *= a;
            b += 1;
        }
        a += 1;
    }

    let sum: i64 = creative.iter().filter(|&&v| v != 16).sum();
    println!("{}", sum);
}
