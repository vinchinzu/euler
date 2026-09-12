// Project Euler 443: GCD sequence
// g(4) = 13, g(n) = g(n-1) + gcd(n, g(n-1)). Find g(10^15).
//
// While gcd stays 1, g(n)+d and n+1+d share factors with the constant
// diff = g(n)-(n+1). Jump to the next multiple of a prime factor of diff.
use euler_utils::{factor, gcd};

fn main() {
    let n_limit: u64 = 1_000_000_000_000_000; // 10^15

    let mut ans: u64 = 13;
    let mut n: u64 = 4;

    while n < n_limit {
        let np1 = n + 1;
        let g0 = gcd(ans, np1);
        if g0 > 1 {
            n = np1;
            ans += g0;
            continue;
        }

        let diff = ans - np1;
        if diff == 0 {
            n = np1;
            ans += 1;
            continue;
        }

        // Next n' > n with gcd(g(n)+(n'-n), n'+1) > 1 is the soonest
        // multiple of a prime factor of diff after `ans`.
        let factors = factor(diff);
        let mut next_val = ans + n_limit - n - 1;
        for &(p, _) in &factors {
            let candidate = (ans / p + 1) * p;
            if candidate < next_val {
                next_val = candidate;
            }
        }
        n += next_val - ans + 1;
        ans = next_val + gcd(n, next_val);
    }

    println!("{}", ans);
}
