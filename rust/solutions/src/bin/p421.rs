// Project Euler 421 - Prime factors of n^15+1
// For each prime p <= K, contribution is p * (number of n in [1,N] with p | n^15+1).

use euler_utils::{mod_pow, gcd};

fn main() {
    let n: u64 = 100_000_000_000; // 10^11
    let k: usize = 100_000_000;   // 10^8
    let r: u64 = 15;

    // Sieve
    let mut is_composite = vec![false; k + 1];
    is_composite[0] = true;
    if k >= 1 {
        is_composite[1] = true;
    }
    let mut i = 2usize;
    while i * i <= k {
        if !is_composite[i] {
            let mut j = i * i;
            while j <= k {
                is_composite[j] = true;
                j += i;
            }
        }
        i += 1;
    }

    let mut ans: i64 = 0;

    for p in 2..=k as u64 {
        if is_composite[p as usize] {
            continue;
        }

        let g_val = gcd(p - 1, r);

        // Find primitive g_val-th root of unity mod p
        let mut nth_root: u64 = 1;
        for g in 1..p {
            nth_root = mod_pow(g, (p - 1) / g_val, p);
            let mut rr = nth_root;
            let mut e = 1u64;
            while rr != 1 {
                rr = (rr as u128 * nth_root as u128 % p as u128) as u64;
                e += 1;
            }
            if e == g_val {
                break;
            }
        }

        let mut root = 1u64;
        for _ in 0..g_val {
            ans += p as i64 * ((n + root) / p) as i64;
            root = (root as u128 * nth_root as u128 % p as u128) as u64;
        }
    }

    println!("{}", ans);
}
