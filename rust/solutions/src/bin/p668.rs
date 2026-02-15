// Project Euler 668 - Square Root Smooth Numbers
// Lucy DP for prime counting, then count n <= N with largest prime factor <= sqrt(n).

fn main() {
    let n: i64 = 10_000_000_000;
    let l = {
        let mut l = (n as f64).sqrt() as i64;
        while l * l > n { l -= 1; }
        l as usize
    };
    let mut small_vals = vec![0i64; l + 2];
    let mut large_vals = vec![0i64; l + 2];
    for i in 1..=l { small_vals[i] = i as i64 - 1; large_vals[i] = n / i as i64 - 1; }
    for p in 2..=l {
        if small_vals[p] == small_vals[p - 1] { continue; }
        let pi_pm1 = small_vals[p - 1];
        let p2 = (p * p) as i64;
        for i in 1..=l {
            if n / i as i64 < p2 { break; }
            let v = n / i as i64;
            let v_div_p = v / p as i64;
            let sub = if v_div_p <= l as i64 { small_vals[v_div_p as usize] } else { large_vals[(n / v_div_p) as usize] };
            large_vals[i] -= sub - pi_pm1;
        }
        for i in (p2 as usize..=l).rev() {
            small_vals[i] -= small_vals[i / p] - pi_pm1;
        }
    }
    let pi = |v: i64| -> i64 {
        if v <= 0 { 0 } else if v <= l as i64 { small_vals[v as usize] } else { large_vals[(n / v) as usize] }
    };
    let mut ans = n;
    // Sieve primes up to n/l
    let prime_limit = (n / l as i64) as usize;
    let mut is_composite = vec![false; prime_limit + 1];
    let mut sieve_primes = Vec::new();
    for i in 2..=prime_limit {
        if !is_composite[i] {
            sieve_primes.push(i as i64);
            if (i as u64) * (i as u64) <= prime_limit as u64 {
                for j in (i*i..=prime_limit).step_by(i) { is_composite[j] = true; }
            }
        }
    }
    for &p in &sieve_primes { ans -= p; }
    for d in 1..l {
        let count = pi(n / d as i64) - pi(n / (d as i64 + 1));
        ans -= d as i64 * count;
    }
    println!("{}", ans);
}
