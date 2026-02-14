// Project Euler 291: Panaitopol Primes
//
// Count primes p < 5*10^15 of the form 2y^2 + 2y + 1.
// Sieve approach: for each prime q ≡ 1 (mod 4), find roots of f(y) ≡ 0 (mod q).

use euler_utils::mod_pow;

const N: i64 = 5_000_000_000_000_000;
const LIMIT: usize = 50_000_000;

fn main() {
    // Step 1: Sieve primes up to sqrt(N) ~ 70.7M
    let sqrt_n = (N as f64).sqrt() as usize + 2;
    let mut sieve_small = vec![true; sqrt_n + 1];
    sieve_small[0] = false;
    sieve_small[1] = false;
    {
        let mut i = 2;
        while i * i <= sqrt_n {
            if sieve_small[i] {
                let mut j = i * i;
                while j <= sqrt_n {
                    sieve_small[j] = false;
                    j += i;
                }
            }
            i += 1;
        }
    }

    // Step 2: is_prime_arr[y] means f(y) = 2y^2+2y+1 is prime
    let mut is_prime_arr = vec![true; LIMIT];
    is_prime_arr[0] = false; // f(0) = 1

    let candidates: &[u64] = &[
        2, 3, 5, 6, 7, 10, 11, 13, 14, 15, 17, 19, 21, 22, 23,
        26, 29, 31, 33, 34, 37, 38, 41, 42, 43, 46, 47, 51, 53,
    ];

    let mut p = 5usize;
    while p <= sqrt_n {
        if sieve_small[p] {
            let pu = p as u64;
            let exp = (pu - 1) >> 2;
            let mut r: u64 = 0;

            for &a in candidates {
                if a >= pu {
                    continue;
                }
                let t = mod_pow(a, exp, pu);
                if (t as u128 * t as u128) % pu as u128 == (pu - 1) as u128 {
                    r = t;
                    break;
                }
            }
            if r == 0 {
                let mut a = 54u64;
                while a < pu {
                    let t = mod_pow(a, exp, pu);
                    if (t as u128 * t as u128) % pu as u128 == (pu - 1) as u128 {
                        r = t;
                        break;
                    }
                    a += 1;
                }
            }

            if r != 0 {
                let inv2 = (pu + 1) >> 1;
                let y1 = ((r as u128 + pu as u128 - 1) * inv2 as u128 % pu as u128) as u64;
                let y2 = ((pu as u128 * 2 - r as u128 - 1) * inv2 as u128 % pu as u128) as u64;

                let roots = if y1 != y2 { vec![y1, y2] } else { vec![y1] };

                for yr in roots {
                    let mut start = yr as usize;
                    if start == 0 {
                        start = p;
                    } else if start < LIMIT {
                        let fval = 2 * (start as i64) * (start as i64) + 2 * (start as i64) + 1;
                        if fval == p as i64 {
                            start += p;
                        }
                    }
                    if start < LIMIT {
                        let mut i = start;
                        while i < LIMIT {
                            is_prime_arr[i] = false;
                            i += p;
                        }
                    }
                }
            }
        }
        p += 4;
    }

    let count: i64 = is_prime_arr.iter().filter(|&&x| x).count() as i64;
    println!("{}", count);
}
