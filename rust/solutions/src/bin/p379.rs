// Project Euler 379 - Least common multiple count
// Uses Mobius sieve + hyperbola method for D(n) and T(m).

use rayon::prelude::*;

fn isqrt(n: i64) -> i64 {
    if n <= 0 {
        return 0;
    }
    let mut x = (n as f64).sqrt() as i64;
    while x * x > n {
        x -= 1;
    }
    while (x + 1) * (x + 1) <= n {
        x += 1;
    }
    x
}

fn icbrt(n: i64) -> i64 {
    if n <= 0 {
        return 0;
    }
    let mut x = (n as f64).cbrt() as i64;
    while x > 0 && x * x * x > n {
        x -= 1;
    }
    while (x + 1) * (x + 1) * (x + 1) <= n {
        x += 1;
    }
    x
}

/// D(n) = sum_{k=1}^{n} floor(n/k) in O(sqrt(n)) time
fn d_func(n: i64) -> i64 {
    if n <= 0 {
        return 0;
    }
    let sq = isqrt(n);
    let mut s: i64 = 0;
    for k in 1..=sq {
        s += n / k;
    }
    2 * s - sq * sq
}

/// T(m) = number of ordered triples (a,b,c) with a*b*c <= m
/// Uses inner parallelism when cbrt(m) is large enough to justify the overhead.
fn t_func(m: i64) -> i64 {
    if m <= 0 {
        return 0;
    }

    let cbrt_m = icbrt(m);

    // Only use inner parallelism when cbrt(m) is large enough to justify overhead
    let total_first: i64 = if cbrt_m >= 500 {
        (1..=cbrt_m).into_par_iter()
            .map(|a| d_func(m / a))
            .sum()
    } else {
        (1..=cbrt_m).map(|a| d_func(m / a)).sum()
    };

    // Second part is O(sqrt(m)) hyperbola steps -- fast, keep sequential
    let mut total_second: i64 = 0;
    let mut a = cbrt_m + 1;
    while a <= m {
        let v = m / a;
        let a_max = m / v;
        total_second += d_func(v) * (a_max - a + 1);
        a = a_max + 1;
    }

    total_first + total_second
}

fn main() {
    let n_big: i64 = 1_000_000_000_000;
    let l = isqrt(n_big) as usize;

    // Sieve Mobius function using linear sieve
    let mut mobius = vec![0i8; l + 1];
    let mut is_p = vec![true; l + 1];
    let mut primes = Vec::with_capacity(l / 2 + 1);
    mobius[1] = 1;

    for i in 2..=l {
        if is_p[i] {
            primes.push(i);
            mobius[i] = -1;
        }
        for &p in &primes {
            let ip = i as u64 * p as u64;
            if ip > l as u64 {
                break;
            }
            is_p[ip as usize] = false;
            if i % p == 0 {
                mobius[ip as usize] = 0;
                break;
            }
            mobius[ip as usize] = -mobius[i];
        }
    }

    // Parallel summation over all squarefree d values.
    // The cost of t_func(n/d²) decreases sharply with d, so we use
    // fine-grained work stealing. Rayon's par_iter with reversed order
    // places expensive items first where work-stealing is most effective.
    let mut nonzero_d: Vec<usize> = (1..=l).filter(|&d| mobius[d] != 0).collect();
    nonzero_d.reverse(); // Expensive items (small d) first for better work-stealing
    let ans_parallel: i64 = nonzero_d.par_iter()
        .map(|&d| mobius[d] as i64 * t_func(n_big / ((d as i64) * (d as i64))))
        .sum();
    let mut ans = ans_parallel;
    ans += n_big;
    ans /= 2;

    println!("{}", ans);
}
