// Project Euler 133 - Repunit non-factors
// Sum of primes < 100000 that never divide R(10^n) for any n.
// A prime p never divides R(10^n) iff ord_p(10) is not of the form 2^a * 5^b.

use euler_utils::{sieve, mod_pow};

const PLIMIT: usize = 100_000;

fn mult_order_10(p: u64) -> u64 {
    let phi = p - 1;

    // Factor phi
    let mut temp = phi;
    let mut factors = Vec::new();
    let mut f = 2u64;
    while f * f <= temp {
        if temp % f == 0 {
            factors.push(f);
            while temp % f == 0 {
                temp /= f;
            }
        }
        f += 1;
    }
    if temp > 1 {
        factors.push(temp);
    }

    let mut order = phi;
    for &fac in &factors {
        while order % fac == 0 && mod_pow(10, order / fac, p) == 1 {
            order /= fac;
        }
    }
    order
}

fn is_2_5_smooth(mut n: u64) -> bool {
    if n == 0 {
        return false;
    }
    while n % 2 == 0 {
        n /= 2;
    }
    while n % 5 == 0 {
        n /= 5;
    }
    n == 1
}

fn main() {
    let is_prime = sieve(PLIMIT);

    // 2, 3, 5 never divide R(10^n)
    let mut total: u64 = 2 + 3 + 5;

    for p in 7..PLIMIT {
        if !is_prime[p] {
            continue;
        }
        let order = mult_order_10(p as u64);
        if !is_2_5_smooth(order) {
            total += p as u64;
        }
    }

    println!("{}", total);
}
