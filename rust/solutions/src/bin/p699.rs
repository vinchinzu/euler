// Project Euler 699 - Triffle Numbers
// Find sum of n <= N where denom of sigma(n)/n in lowest terms is a power of 3.
// DFS from 3-smooth starting numbers.

fn sigma_pe(p: i64, e: i32) -> i64 {
    let mut result = 1i64;
    let mut pw = 1i64;
    for _ in 0..e {
        pw *= p;
        result += pw;
    }
    result
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 { let t = b; b = a % b; a = t; }
    a
}

fn is_pow3(mut n: i64) -> bool {
    if n <= 0 { return false; }
    while n % 3 == 0 { n /= 3; }
    n == 1
}

struct Factor { p: i64, e: i32 }

fn factorize(mut n: i64) -> Vec<Factor> {
    let mut factors = Vec::new();
    if n <= 1 { return factors; }
    let mut d = 2i64;
    while d * d <= n {
        if n % d == 0 {
            let mut e = 0;
            while n % d == 0 { e += 1; n /= d; }
            factors.push(Factor { p: d, e });
        }
        d += 1;
    }
    if n > 1 { factors.push(Factor { p: n, e: 1 }); }
    factors
}

const N_LIMIT: i64 = 100_000_000_000_000;

fn dfs(
    n: i64, num: i64, den: i64,
    primes: &[i64], exps: &[i32], np: usize,
    answer: &mut i64,
) {
    if den > 1 && is_pow3(den) {
        *answer += n;
    }

    let nf = factorize(num);

    for factor in &nf {
        let p = factor.p;
        // Check if p already divides n
        let already = primes[..np].iter().any(|&pp| pp == p);
        if already { continue; }

        let mut pw = 1i64;
        for e in 1.. {
            if pw > N_LIMIT / p { break; }
            pw *= p;
            if n > N_LIMIT / pw { break; }
            let new_n = n * pw;
            let sp = sigma_pe(p, e);
            let new_num_raw = num * sp;
            let new_den_raw = den * pw;
            let g = gcd(new_num_raw.abs(), new_den_raw.abs());
            let new_num = new_num_raw / g;
            let new_den = new_den_raw / g;

            let mut new_primes = primes[..np].to_vec();
            let mut new_exps = exps[..np].to_vec();
            new_primes.push(p);
            new_exps.push(e);
            let new_np = np + 1;

            dfs(new_n, new_num, new_den, &new_primes, &new_exps, new_np, answer);
        }
    }
}

fn main() {
    let mut answer = 0i64;

    let mut pw2 = 1i64;
    let mut a = 0;
    while pw2 <= N_LIMIT {
        let mut pw3 = 3i64;
        let mut b = 1;
        while pw2 <= N_LIMIT / pw3 {
            let n = pw2 * pw3;
            let s2 = sigma_pe(2, a);
            let s3 = sigma_pe(3, b);
            let num_raw = s2 * s3;
            let den_raw = n;
            let g = gcd(num_raw, den_raw);
            let num = num_raw / g;
            let den = den_raw / g;

            let mut primes = Vec::new();
            let mut exps = Vec::new();
            if a > 0 { primes.push(2); exps.push(a); }
            primes.push(3); exps.push(b);
            let np = primes.len();

            dfs(n, num, den, &primes, &exps, np, &mut answer);

            if pw3 > N_LIMIT / 3 { break; }
            pw3 *= 3;
            b += 1;
        }
        if pw2 > N_LIMIT / 2 { break; }
        pw2 *= 2;
        a += 1;
    }

    println!("{}", answer);
}
