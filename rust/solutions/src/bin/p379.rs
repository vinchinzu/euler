// Project Euler 379 - Least common multiple count
// Uses Mobius sieve + hyperbola method for D(n) and T(m).

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
fn t_func(m: i64) -> i64 {
    if m <= 0 {
        return 0;
    }

    let cbrt_m = icbrt(m);
    let mut total: i64 = 0;

    for a in 1..=cbrt_m {
        total += d_func(m / a);
    }

    let mut a = cbrt_m + 1;
    while a <= m {
        let v = m / a;
        let a_max = m / v;
        total += d_func(v) * (a_max - a + 1);
        a = a_max + 1;
    }

    total
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

    let mut ans: i64 = 0;
    for d in 1..=l {
        if mobius[d] != 0 {
            ans += mobius[d] as i64 * t_func(n_big / ((d as i64) * (d as i64)));
        }
    }
    ans += n_big;
    ans /= 2;

    println!("{}", ans);
}
