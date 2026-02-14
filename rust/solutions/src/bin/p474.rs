// Project Euler 474: Last digits of divisors
//
// Count divisors of N! whose last 5 digits equal K=65432.
// N=10^6, M=10^16+61.

use euler_utils::{mod_mul, mod_inv, euler_phi};

const NN: usize = 1_000_000;
const K_VAL: u64 = 65432;
const M_VAL: u64 = 10_000_000_000_000_061;
const B: u64 = 10;

fn num_factors_in_factorial(n: usize, p: usize) -> u64 {
    let mut count = 0u64;
    let mut power = p as u64;
    while power <= n as u64 {
        count += n as u64 / power;
        if power > n as u64 / p as u64 {
            break;
        }
        power *= p as u64;
    }
    count
}

fn gcd_u64(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn iceil_pow(n: u64, base: u64) -> u64 {
    let mut result = 1u64;
    while result < n {
        result *= base;
    }
    result
}

fn lcm_u64(a: u64, b: u64) -> u64 {
    a / gcd_u64(a, b) * b
}

fn main() {
    // Sieve
    let mut is_prime = vec![true; NN + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut i = 2;
    while i * i <= NN {
        if is_prime[i] {
            let mut j = i * i;
            while j <= NN {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }

    let mut res = 1u64;

    for p in 2..=NN {
        if is_prime[p] && gcd_u64(p as u64, B) == 1 {
            let v = num_factors_in_factorial(NN, p);
            let factor = (1 + v) % M_VAL;
            res = mod_mul(res, factor, M_VAL);
        }
    }

    let cp = iceil_pow(K_VAL, B);
    let lc = lcm_u64(K_VAL, cp);
    let r = euler_phi(lc / K_VAL);
    let inv_r = mod_inv(r, M_VAL).unwrap();
    let ans = mod_mul(res % M_VAL, inv_r, M_VAL);

    println!("{}", ans);
}
