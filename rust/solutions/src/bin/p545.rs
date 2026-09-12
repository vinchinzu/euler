// Project Euler 545 - Faulhaber's Formulas
//
// D(k) = product of primes p where (p-1)|k (von Staudt-Clausen theorem).
// Find the 100000th k where D(k) = 20010.
// 20010 = 2*3*5*23*29, so required primes have (p-1) in {1,2,4,22,28}.
// lcm(1,2,4,22,28) = 308. k must be a multiple of 308.
//
// Forbidden d = (p-1)/gcd(p-1, 308) for bad primes p. Equivalently, for each
// g|308, sieve the AP p = g*d+1 (d = 2..=L) instead of all integers to 1.54e9.

use rayon::prelude::*;

const DIVS308: [u64; 12] = [1, 2, 4, 7, 11, 14, 22, 28, 44, 77, 154, 308];

#[inline]
fn is_good_prime(p: u64) -> bool {
    p == 2 || p == 3 || p == 5 || p == 23 || p == 29
}

fn gcd_u64(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn modinv(a: u64, m: u64) -> u64 {
    let mut t: i64 = 0;
    let mut newt: i64 = 1;
    let mut r = m as i64;
    let mut newr = (a % m) as i64;
    while newr != 0 {
        let q = r / newr;
        (t, newt) = (newt, t - q * newt);
        (r, newr) = (newr, r - q * newr);
    }
    if t < 0 {
        t += m as i64;
    }
    t as u64
}

fn main() {
    let target = 100_000;
    let base = 308i64;
    let l: usize = 5_000_000;
    let lu = l as u64;
    let max_p = lu * 308 + 1;

    let sqrt_max: usize = 40_000;
    let mut small_sieve = vec![true; sqrt_max + 1];
    small_sieve[0] = false;
    small_sieve[1] = false;
    let mut i = 2;
    while i * i <= sqrt_max {
        if small_sieve[i] {
            let mut j = i * i;
            while j <= sqrt_max {
                small_sieve[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    let sprimes: Vec<u64> = (2..=sqrt_max)
        .filter(|&i| small_sieve[i])
        .map(|i| i as u64)
        .collect();

    // For each g|308, sieve primality of g*d+1 for d=1..=L and mark forbidden d
    let forbidden_parts: Vec<Vec<u8>> = DIVS308
        .into_par_iter()
        .map(|g| {
            let mut is_comp = vec![0u8; l + 1];
            let max_n = g * lu + 1;
            for &q in &sprimes {
                if q * q > max_n {
                    break;
                }
                let gmod = g % q;
                if gmod == 0 {
                    continue;
                }
                let inv = modinv(gmod, q);
                let d0 = (q - inv) % q;
                let nmin = q * q;
                let d_min = (nmin - 1 + g - 1) / g;
                let rem = d_min % q;
                let mut d = if rem == d0 {
                    d_min
                } else {
                    d_min + (d0 + q - rem) % q
                };
                while d <= lu {
                    is_comp[d as usize] = 1;
                    d += q;
                }
            }

            let mut local = vec![0u8; l + 1];
            for d in 1..=l {
                if is_comp[d] != 0 {
                    continue;
                }
                let p = g * d as u64 + 1;
                if p < 2 || p > max_p {
                    continue;
                }
                if is_good_prime(p) {
                    continue;
                }
                let g308 = gcd_u64(p - 1, 308);
                let dd = (p - 1) / g308;
                if dd >= 2 && dd <= lu {
                    local[dd as usize] = 1;
                }
            }
            local
        })
        .collect();

    let mut forbidden = vec![0u8; l + 1];
    for part in &forbidden_parts {
        for d in 2..=l {
            forbidden[d] |= part[d];
        }
    }

    let mut valid = vec![1u8; l + 1];
    for d in 2..=l {
        if forbidden[d] != 0 {
            let mut j = d;
            while j <= l {
                valid[j] = 0;
                j += d;
            }
        }
    }

    let mut count = 0;
    for m in 1..=l {
        if valid[m] != 0 {
            count += 1;
            if count == target {
                println!("{}", m as i64 * base);
                return;
            }
        }
    }
    eprintln!("Need larger L! Found {} valid values out of {}", count, l);
}
