// Project Euler 443: GCD sequence
use euler_utils::gcd;

fn factorize(n: i64) -> Vec<i64> {
    let mut n = n.abs();
    let mut factors = Vec::new();
    let mut d = 2i64;
    while d * d <= n {
        if n % d == 0 {
            factors.push(d);
            while n % d == 0 { n /= d; }
        }
        d += 1;
    }
    if n > 1 { factors.push(n); }
    factors
}

fn main() {
    let n_limit: i64 = 1_000_000_000_000_000; // 10^15
    let l = 1000;

    let mut ans: i64 = 13;
    let mut n: i64 = 4;

    while n < n_limit {
        let mut found = false;
        for d in 0..l {
            let g = gcd((ans + d) as u64, (n + d + 1) as u64) as i64;
            if g > 1 {
                ans += d;
                n += d + 1;
                ans += gcd(n as u64, ans as u64) as i64;
                found = true;
                break;
            }
        }

        if !found {
            let diff = ans - (n + 1);
            if diff == 0 {
                n += 1;
                ans += 1;
            } else {
                let factors = factorize(diff);
                let mut next_val = ans + n_limit - n - 1;
                for &p in &factors {
                    let candidate = ((ans / p) + 1) * p;
                    if candidate < next_val {
                        next_val = candidate;
                    }
                }
                let jump = next_val - ans;
                n += jump + 1;
                ans = next_val + gcd(n as u64, next_val as u64) as i64;
            }
        }
    }

    println!("{}", ans);
}
