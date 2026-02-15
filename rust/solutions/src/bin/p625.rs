// Project Euler 625 - Gcd sum
// Sublinear totient summatory function + hyperbolic summation

const NN: i64 = 100_000_000_000;
const MOD: i64 = 998_244_353;

fn md(a: i64) -> i64 { ((a % MOD) + MOD) % MOD }

fn power(mut base: i64, mut exp: i64) -> i64 {
    let mut result = 1i64;
    base = md(base);
    while exp > 0 {
        if exp & 1 == 1 { result = result * base % MOD; }
        base = base * base % MOD;
        exp >>= 1;
    }
    result
}

fn main() {
    let inv2 = power(2, MOD - 2);

    let mut sieve_limit = (NN as f64).powf(2.0 / 3.0) as i64 + 100;
    if sieve_limit < 22_000_000 { sieve_limit = 22_000_000; }
    let slen = sieve_limit as usize + 1;

    let mut phi = vec![0i32; slen];
    for i in 0..slen { phi[i] = i as i32; }
    for i in 2..slen {
        if phi[i] == i as i32 {
            for j in (i..slen).step_by(i) {
                phi[j] -= phi[j] / i as i32;
            }
        }
    }

    let mut prefix = vec![0i64; slen];
    for i in 1..slen {
        prefix[i] = (prefix[i - 1] + phi[i] as i64) % MOD;
    }

    let big_limit = (NN / sieve_limit + 2) as usize;
    let mut big = vec![0i64; big_limit + 1];

    for t in (1..=big_limit).rev() {
        let n = NN / t as i64;
        if (n as usize) < slen {
            big[t] = prefix[n as usize];
            continue;
        }
        let mut result = (n % MOD) * ((n + 1) % MOD) % MOD * inv2 % MOD;

        let mut d = 2i64;
        while d <= n {
            let q = n / d;
            let d2 = n / q;
            let count = md(d2 - d + 1);
            let sq = if (q as usize) < slen { prefix[q as usize] } else { big[(NN / q) as usize] };
            result = md(result - count * sq % MOD);
            d = d2 + 1;
        }
        big[t] = result;
    }

    let mut ans = 0i64;
    let mut k = 1i64;
    while k <= NN {
        let q = NN / k;
        let k2 = NN / q;
        let kmod = k % MOD;
        let k2mod = k2 % MOD;
        let range_sum = md(kmod + k2mod) * md(k2mod - kmod + 1) % MOD * inv2 % MOD;
        let sn = if (q as usize) < slen { prefix[q as usize] } else { big[(NN / q) as usize] };
        ans = md(ans + range_sum * sn % MOD);
        k = k2 + 1;
    }

    println!("{}", ans);
}
