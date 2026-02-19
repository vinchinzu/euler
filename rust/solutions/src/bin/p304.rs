// Project Euler 304: Primonacci
//
// Sum of Fibonacci(a(n)) mod 1234567891011 for n=1..100000,
// where a(1) is smallest prime > 10^14, a(n+1) is next prime after a(n).
// Uses segmented sieve + fast doubling Fibonacci.

use euler_utils::{mod_mul, primes_up_to};

const MOD: u64 = 1_234_567_891_011;
const START: u64 = 100_000_000_000_000; // 10^14
const COUNT: usize = 100_000;

fn fib_pair(n: u64, m: u64) -> (u64, u64) {
    if n == 0 {
        return (0, 1);
    }
    let (a, b) = fib_pair(n >> 1, m);
    let c = mod_mul(a, ((2 * b as u128 + m as u128 - a as u128) % m as u128) as u64, m);
    let d = (mod_mul(a, a, m) + mod_mul(b, b, m)) % m;
    if n & 1 == 1 {
        (d, (c + d) % m)
    } else {
        (c, d)
    }
}

fn fib_mod(n: u64, m: u64) -> u64 {
    fib_pair(n, m).0
}

fn main() {
    // Small primes for segmented sieve
    let small_limit = 10_000_100;
    let small_primes: Vec<u64> = primes_up_to(small_limit).into_iter().map(|p| p as u64).collect();

    // Segmented sieve
    let first = START + 1;
    let delta = (COUNT as f64 * (START as f64).ln() * 1.5) as u64;
    let high = first + delta;
    let size = (high - first) as usize;

    let mut sieve = vec![true; size];
    for &p in &small_primes {
        if p * p >= high {
            break;
        }
        let start_off = if p * p >= first {
            (p * p - first) as usize
        } else {
            let rem = first % p;
            if rem == 0 { 0 } else { (p - rem) as usize }
        };
        let mut j = start_off;
        while j < size {
            sieve[j] = false;
            j += p as usize;
        }
    }

    let mut total: u64 = 0;
    let mut found = 0;
    for i in 0..size {
        if sieve[i] {
            total = (total + fib_mod(first + i as u64, MOD)) % MOD;
            found += 1;
            if found == COUNT {
                break;
            }
        }
    }

    println!("{}", total);
}
