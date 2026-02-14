use euler_utils::is_prime;

fn fn_cr(a: f64, b: i32) -> f64 {
    if b < 0 || a < b as f64 { return 0.0; }
    let mut result = 1.0;
    for i in 0..b {
        result *= (a - i as f64) / (i as f64 + 1.0);
    }
    result
}

fn ffact(n: i32) -> f64 {
    let mut r = 1.0;
    for i in 2..=n {
        r *= i as f64;
    }
    r
}

fn main() {
    let n = 100i32;
    let k = 22i32;
    let mut num_primes = 0i32;
    for i in 2..=n as u64 {
        if is_prime(i) { num_primes += 1; }
    }

    let mut ans = 0.0f64;
    for kk in 0..=num_primes {
        let parity = if kk % 2 == 0 { 1.0 } else { -1.0 };
        ans -= parity * fn_cr(kk as f64, num_primes - k) * fn_cr(num_primes as f64, kk) * ffact(n - kk);
    }
    ans /= ffact(n);

    println!("{:.12}", ans);
}
