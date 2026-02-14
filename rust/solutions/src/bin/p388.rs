// Project Euler 388 - Distinct Lines through lattice points
// D(N) = sum_{d=1}^N mu(d) * (floor(N/d) + 1)^3 - M(N)
// Uses O(N^{2/3}) Mertens via Lucy DP.
// Output: first 9 digits concatenated with last 9 digits.

use std::collections::HashMap;

const N_VAL: i64 = 10_000_000_000;

fn main() {
    let cbrt_n = {
        let mut c = (N_VAL as f64).cbrt() as i64;
        while (c + 1) * (c + 1) * (c + 1) <= N_VAL { c += 1; }
        while c * c * c > N_VAL { c -= 1; }
        c
    };
    let sqrt_n = {
        let mut s = (N_VAL as f64).sqrt() as i64;
        while s * s > N_VAL { s -= 1; }
        while (s + 1) * (s + 1) <= N_VAL { s += 1; }
        s
    };

    let mut sieve_limit = cbrt_n * cbrt_n;
    if sieve_limit < sqrt_n + 1 { sieve_limit = sqrt_n + 1; }
    let sl = sieve_limit as usize;

    // Linear sieve for mu
    let mut mu = vec![0i32; sl + 1];
    let mut is_composite = vec![false; sl + 1];
    let mut primes_list = Vec::new();
    mu[1] = 1;

    for i in 2..=sl {
        if !is_composite[i] {
            primes_list.push(i);
            mu[i] = -1;
        }
        for &p in &primes_list {
            let v = p * i;
            if v > sl { break; }
            is_composite[v] = true;
            if i % p == 0 {
                mu[v] = 0;
                break;
            }
            mu[v] = -mu[i];
        }
    }

    let mut mu_prefix = vec![0i64; sl + 1];
    for i in 1..=sl {
        mu_prefix[i] = mu_prefix[i - 1] + mu[i] as i64;
    }

    let mut mc: HashMap<i64, i64> = HashMap::new();

    fn mertens(n: i64, sl: usize, mu_prefix: &[i64], mc: &mut HashMap<i64, i64>) -> i64 {
        if n <= sl as i64 { return mu_prefix[n as usize]; }
        if let Some(&v) = mc.get(&n) { return v; }

        let mut s: i64 = 0;
        let mut d: i64 = 2;
        while d <= n {
            let q = n / d;
            let d_max = n / q;
            s += (d_max - d + 1) * mertens(q, sl, mu_prefix, mc);
            d = d_max + 1;
        }
        let result = 1 - s;
        mc.insert(n, result);
        result
    }

    let m_n = mertens(N_VAL, sl, &mu_prefix, &mut mc);

    // Main sum using i128
    let mut main_sum: i128 = 0;
    let mut d_start: i64 = 1;
    while d_start <= N_VAL {
        let q = N_VAL / d_start;
        let d_max = N_VAL / q;

        let mu_range: i64 = if d_max <= sl as i64 {
            mu_prefix[d_max as usize] - mu_prefix[(d_start - 1) as usize]
        } else if d_start <= sl as i64 {
            let part1 = mu_prefix[sl] - mu_prefix[(d_start - 1) as usize];
            let part2 = mertens(d_max, sl, &mu_prefix, &mut mc) - mu_prefix[sl];
            part1 + part2
        } else {
            mertens(d_max, sl, &mu_prefix, &mut mc) - mertens(d_start - 1, sl, &mu_prefix, &mut mc)
        };

        let cube = (q + 1) as i128 * (q + 1) as i128 * (q + 1) as i128;
        main_sum += mu_range as i128 * cube;
        d_start = d_max + 1;
    }

    let result = main_sum - m_n as i128;

    // Format result
    let neg = result < 0;
    let abs_result = if neg { -result } else { result };
    let s = if neg {
        format!("-{}", abs_result)
    } else {
        format!("{}", abs_result)
    };

    if s.len() <= 18 {
        println!("{}", s);
    } else {
        let first9 = &s[..9];
        let last9 = &s[s.len()-9..];
        println!("{}{}", first9, last9);
    }
}
