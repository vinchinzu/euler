// Project Euler 642 - Sum of largest prime factors
// Lucy hedgehog for sum of primes, then recursive enumeration

const MOD: i64 = 1_000_000_000;

fn isqrt(n: i64) -> i64 {
    let mut x = (n as f64).sqrt() as i64;
    while x > 0 && x * x > n { x -= 1; }
    while (x + 1) * (x + 1) <= n { x += 1; }
    x
}

fn main() {
    let n_global: i64 = 201_820_182_018;
    let r = isqrt(n_global) as usize;

    // Lucy hedgehog: small[k] = sum of primes <= k, big[i] = sum of primes <= N/i
    let mut small = vec![0i64; r + 1];
    let mut big = vec![0i64; r + 1];

    let inv2 = 500_000_000i64; // inv(2) mod 10^9

    for k in 2..=r {
        small[k] = ((k as i64) * (k as i64 + 1) / 2 - 1) % MOD;
    }
    for i in 1..=r {
        let v = n_global / i as i64;
        let vm = v % MOD;
        let vp1m = (v + 1) % MOD;
        big[i] = ((vm * vp1m % MOD) * inv2 % MOD - 1 + MOD) % MOD;
    }

    for p in 2..=r {
        if small[p] == small[p - 1] { continue; }
        let sp = small[p - 1];
        let p2 = (p as i64) * (p as i64);
        let pi = p as i64;

        for i in 1..=r {
            if n_global / (i as i64) < p2 { break; }
            let d = n_global / (i as i64) / pi;
            if d <= r as i64 {
                big[i] = (big[i] - pi % MOD * ((small[d as usize] - sp + MOD) % MOD) % MOD + MOD) % MOD;
            } else {
                let idx = (n_global / d) as usize;
                big[i] = (big[i] - pi % MOD * ((big[idx] - sp + MOD) % MOD) % MOD + MOD) % MOD;
            }
        }

        for k in (p2 as usize..=r).rev() {
            small[k] = (small[k] - pi % MOD * ((small[k / p] - sp + MOD) % MOD) % MOD + MOD) % MOD;
        }
    }

    let get_sum = |v: i64| -> i64 {
        if v <= r as i64 { small[v as usize] }
        else { big[(n_global / v) as usize] }
    };

    // Sieve primes up to R
    let mut is_p = vec![true; r + 1];
    is_p[0] = false;
    if r >= 1 { is_p[1] = false; }
    let sq = isqrt(r as i64) as usize;
    for i in 2..=sq {
        if is_p[i] {
            let mut j = i * i;
            while j <= r { is_p[j] = false; j += i; }
        }
    }
    let primes: Vec<i64> = (2..=r).filter(|&i| is_p[i]).map(|i| i as i64).collect();

    // Recursive DFS
    let mut ans = 0i64;

    struct Frame { min_index: usize, n: i64 }
    let mut stack = vec![Frame { min_index: 0, n: 1 }];

    while let Some(f) = stack.pop() {
        let max_p = n_global / f.n;
        if f.min_index >= primes.len() { continue; }
        let min_p = primes[f.min_index];
        if min_p > max_p { continue; }

        let contrib = (get_sum(max_p) - get_sum(min_p - 1) + MOD) % MOD;
        ans = (ans + contrib) % MOD;

        for index in f.min_index..primes.len() {
            let p = primes[index];
            if f.n * p > n_global / p { break; } // n * p * p > N
            stack.push(Frame { min_index: index, n: f.n * p });
        }
    }

    println!("{}", ans);
}
