// Project Euler 545 - Faulhaber's Formulas
//
// D(k) = product of primes p where (p-1)|k (von Staudt-Clausen theorem).
// Find the 100000th k where D(k) = 20010.
// 20010 = 2*3*5*23*29, so required primes have (p-1) in {1,2,4,22,28}.
// lcm(1,2,4,22,28) = 308. k must be a multiple of 308.

use rayon::prelude::*;

fn is_good_prime(p: i64) -> bool {
    p == 2 || p == 3 || p == 5 || p == 23 || p == 29
}

fn main() {
    let target = 100_000;
    let base = 308;
    let l: usize = 5_000_000;
    let max_p = l as i64 * 308 + 2;

    // Sieve small primes up to sqrt(max_p)
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
    let sprimes: Vec<usize> = (2..=sqrt_max).filter(|&i| small_sieve[i]).collect();

    // Segmented sieve to find primes up to max_p, parallelized by segment
    let seg_size: usize = 1 << 20;
    let n_segments = ((max_p as usize - 2) + seg_size - 1) / seg_size;

    // Each segment produces a list of forbidden divisors
    let forbidden_lists: Vec<Vec<usize>> = (0..n_segments).into_par_iter().map(|seg_idx| {
        let lo = 2i64 + seg_idx as i64 * seg_size as i64;
        let hi = (lo + seg_size as i64 - 1).min(max_p - 1);
        let len = (hi - lo + 1) as usize;

        let mut seg = vec![true; len];

        for &p in &sprimes {
            let p64 = p as i64;
            let mut start = ((lo + p64 - 1) / p64) * p64;
            if start == p64 { start += p64; }
            if start < lo { start = lo; }
            let mut j = start;
            while j <= hi {
                seg[(j - lo) as usize] = false;
                j += p64;
            }
        }

        let mut local_forbidden = Vec::new();
        for idx in 0..len {
            if !seg[idx] { continue; }
            let p = lo + idx as i64;
            if is_good_prime(p) { continue; }

            let pm1 = p - 1;
            let mut ga = pm1;
            let mut gb: i64 = 308;
            while gb != 0 {
                let t = gb;
                gb = ga % gb;
                ga = t;
            }
            let d = pm1 / ga;
            if d >= 2 && d <= l as i64 {
                local_forbidden.push(d as usize);
            }
        }
        local_forbidden
    }).collect();

    let mut forbidden = vec![false; l + 1];
    for list in &forbidden_lists {
        for &d in list {
            forbidden[d] = true;
        }
    }

    // Sieve valid array
    let mut valid = vec![true; l + 1];
    for d in 2..=l {
        if forbidden[d] {
            let mut j = d;
            while j <= l {
                valid[j] = false;
                j += d;
            }
        }
    }

    let mut count = 0;
    for m in 1..=l {
        if valid[m] {
            count += 1;
            if count == target {
                println!("{}", m as i64 * base);
                return;
            }
        }
    }
    eprintln!("Need larger L! Found {} valid values out of {}", count, l);
}
