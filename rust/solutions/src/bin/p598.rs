// Project Euler 598 - Split Divisibilities
//
// Find the number of pairs (a,b) with a<=b such that a*b = 100!
// and tau(a) = tau(b).
// Meet-in-the-middle approach splitting primes into two halves.

use std::collections::HashMap;
use euler_utils::gcd_i32;

fn main() {
    let n_fact = 100;
    let primes = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
        89, 97,
    ];
    let num_primes = primes.len();

    // Compute exponents for N!
    let mut es = [0i32; 25];
    for i in 0..num_primes {
        let p = primes[i];
        let mut e = 0;
        let mut n = n_fact;
        while n > 0 {
            n /= p;
            e += n;
        }
        es[i] = e as i32;
    }

    // Build map from the "right" side (large primes)
    let mut hash_map: HashMap<(i32, i32), i64> = HashMap::new();
    hash_map.insert((1, 1), 1);

    let mut index = num_primes as i32 - 1;

    while index >= 0 {
        let e = es[index as usize];

        // Check if brute force product for remaining primes is smaller than current map
        let mut prod: i64 = 1;
        let mut overflow = false;
        for i in 0..index as usize {
            prod *= (es[i] + 1) as i64;
            if prod > 10_000_000 {
                overflow = true;
                break;
            }
        }

        let map_size = hash_map.len() as i64;

        if !overflow && prod < map_size {
            break;
        }

        // Expand map with current prime
        let entries: Vec<((i32, i32), i64)> = hash_map.drain().collect();

        for &((rnum, rden), cnt) in &entries {
            for f in 0..=e {
                let new_x = rnum * (f + 1);
                let new_y = rden * (e - f + 1);
                let g = gcd_i32(new_x, new_y);
                *hash_map.entry((new_x / g, new_y / g)).or_insert(0) += cnt;
            }
        }

        index -= 1;
    }

    // Brute force the "left" side
    let n_left = (index + 1) as usize;

    let mut total_bf: i64 = 1;
    for i in 0..n_left {
        total_bf *= (es[i] + 1) as i64;
    }

    let mut result: i64 = 0;

    for combo in 0..total_bf {
        let mut x: i32 = 1;
        let mut y: i32 = 1;
        let mut tmp = combo;
        for i in 0..n_left {
            let f = (tmp % (es[i] + 1) as i64) as i32;
            tmp /= (es[i] + 1) as i64;
            x *= f + 1;
            y *= es[i] - f + 1;
        }
        // Need map ratio y/x
        let g = gcd_i32(x, y);
        let target_num = y / g;
        let target_den = x / g;
        if let Some(&cnt) = hash_map.get(&(target_num, target_den)) {
            result += cnt;
        }
    }

    result /= 2;

    println!("{}", result);
}
