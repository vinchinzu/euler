// Project Euler 501 - Eight Divisors
// Count integers <= N with exactly 8 divisors.
// Forms: p*q*r (3 distinct primes), p^3*q (p!=q), p^7.
// Uses Lucy_Hedgehog prime counting for large values.

fn main() {
    let n: i64 = 1_000_000_000_000;
    let sqrt_n = {
        let mut s = (n as f64).sqrt() as i64;
        while s * s > n { s -= 1; }
        while (s + 1) * (s + 1) <= n { s += 1; }
        s as usize
    };

    // Lucy DP: S_small[v] = pi(v) for v <= sqrt_n, S_large[k] = pi(n/k) for k >= 1
    let mut s_small = vec![0i64; sqrt_n + 2];
    let mut s_large = vec![0i64; sqrt_n + 2];

    for i in 0..=sqrt_n {
        s_small[i] = i as i64 - 1;
    }
    for k in 1..=sqrt_n {
        s_large[k] = n / (k as i64) - 1;
    }

    let mut is_prime_flag = vec![true; sqrt_n + 1];
    is_prime_flag[0] = false;
    if sqrt_n >= 1 { is_prime_flag[1] = false; }

    for p in 2..=sqrt_n {
        if !is_prime_flag[p] { continue; }
        let p2 = (p as i64) * (p as i64);

        // Update S_large
        for k in 1..=sqrt_n {
            if n / (k as i64) < p2 { break; }
            let v = n / (k as i64);
            let v_over_p = v / (p as i64);
            let sub = if v_over_p <= sqrt_n as i64 {
                s_small[v_over_p as usize]
            } else {
                s_large[(n / v_over_p) as usize]
            };
            s_large[k] -= sub - s_small[p - 1];
        }

        // Update S_small
        for v in (p2 as usize..=sqrt_n).rev() {
            s_small[v] -= s_small[v / p] - s_small[p - 1];
        }

        // Mark composites
        let mut j = p * p;
        while j <= sqrt_n {
            is_prime_flag[j] = false;
            j += p;
        }
    }

    let pi = |v: i64| -> i64 {
        if v <= 0 { return 0; }
        if v <= sqrt_n as i64 { return s_small[v as usize]; }
        s_large[(n / v) as usize]
    };

    // Sieve small primes
    let limit = (n as f64).powf(2.0 / 3.0) as usize + 100;
    let sieve = euler_utils::sieve(limit);
    let small_primes: Vec<usize> = (2..=limit).filter(|&i| sieve[i]).collect();

    // Build pi_small for direct lookup
    let mut pi_small = vec![0i64; limit + 1];
    {
        let mut cnt = 0i64;
        let mut pidx = 0;
        for i in 1..=limit {
            if pidx < small_primes.len() && small_primes[pidx] == i {
                cnt += 1;
                pidx += 1;
            }
            pi_small[i] = cnt;
        }
    }

    let mut ans: i64 = 0;

    // Count p*q*r with p < q < r, all primes
    for pi_idx in 0..small_primes.len() {
        let p = small_primes[pi_idx] as i64;
        if p * p * p > n { break; }
        for qi_idx in (pi_idx + 1)..small_primes.len() {
            let q = small_primes[qi_idx] as i64;
            if p * q * q > n { break; }
            let lim = n / (p * q);
            let pi_limit = pi(lim);
            ans += pi_limit - pi_small[q as usize];
        }
    }

    // Count p^3 * q
    for pi_idx in 0..small_primes.len() {
        let p = small_primes[pi_idx] as i64;
        if p * p * p > n { break; }
        let lim = n / (p * p * p);
        let pi_limit = pi(lim);
        ans += pi_limit;
        // Subtract q == p case
        if p * p * p * p <= n {
            ans -= 1;
        }
    }

    // Count p^7
    let mut max_p7 = (n as f64).powf(1.0 / 7.0) as usize;
    loop {
        let mut test = 1.0f64;
        let mut ok = true;
        for _ in 0..7 {
            test *= (max_p7 + 1) as f64;
            if test > n as f64 { ok = false; break; }
        }
        if !ok { break; }
        max_p7 += 1;
    }
    while max_p7 > 0 {
        let mut test = 1.0f64;
        let mut ok = true;
        for _ in 0..7 {
            test *= max_p7 as f64;
            if test > n as f64 { ok = false; break; }
        }
        if ok { break; }
        max_p7 -= 1;
    }
    ans += pi_small[max_p7];

    println!("{}", ans);
}
