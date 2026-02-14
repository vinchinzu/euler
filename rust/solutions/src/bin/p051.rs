// Project Euler 51: Prime digit replacements
// Find the smallest prime that is part of an 8-prime family by replacing some digits.
use euler_utils::sieve;

fn is_prime_large(n: u32, sieve_arr: &[bool]) -> bool {
    if (n as usize) < sieve_arr.len() {
        return sieve_arr[n as usize];
    }
    if n % 2 == 0 {
        return false;
    }
    let mut i = 3u32;
    while (i as u64) * (i as u64) <= n as u64 {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

fn main() {
    let sieve_limit = 200_000;
    let is_prime = sieve(sieve_limit);

    let primes: Vec<u32> = (2..sieve_limit).filter(|&i| is_prime[i]).map(|i| i as u32).collect();

    for &p in &primes {
        let s: Vec<u8> = p.to_string().bytes().map(|b| b - b'0').collect();
        let num_digits = s.len();
        if num_digits < 2 {
            continue;
        }

        // Count digit frequencies and record positions
        let mut positions: Vec<Vec<usize>> = vec![vec![]; 10];
        for (i, &d) in s.iter().enumerate() {
            positions[d as usize].push(i);
        }

        for d in 0..=9u8 {
            if positions[d as usize].len() < 3 {
                continue;
            }
            let pos = &positions[d as usize];

            let mut family_count = 0;
            let mut min_prime = 0u32;

            for rep in 0..=9u8 {
                // Skip leading zero
                if pos[0] == 0 && rep == 0 {
                    continue;
                }
                let mut candidate = s.clone();
                for &pi in pos {
                    candidate[pi] = rep;
                }
                let num: u32 = candidate.iter().fold(0u32, |acc, &c| acc * 10 + c as u32);
                if is_prime_large(num, &is_prime) {
                    family_count += 1;
                    if min_prime == 0 || num < min_prime {
                        min_prime = num;
                    }
                }
            }

            if family_count == 8 {
                println!("{min_prime}");
                return;
            }
        }
    }
}
