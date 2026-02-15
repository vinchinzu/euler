// Project Euler 801 - x^y = y^x (mod n)
// Sieve primes in [A, A+B], factor p-1, compute multiplicative function
fn mod_pow(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
    let mut result: i64 = 1;
    base = base.rem_euclid(modulus);
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result as i128 * base as i128 % modulus as i128) as i64;
        }
        base = (base as i128 * base as i128 % modulus as i128) as i64;
        exp >>= 1;
    }
    result
}

fn main() {
    let a: i64 = 10_000_000_000_000_000; // 10^16
    let b: usize = 1_000_000;
    let m: i64 = 993353399;
    let l = ((a + b as i64) as f64).sqrt() as usize + 1;

    // Sieve primes up to l
    let mut is_prime = vec![true; l + 1];
    is_prime[0] = false;
    if l >= 1 { is_prime[1] = false; }
    let sq = (l as f64).sqrt() as usize;
    for i in 2..=sq {
        if is_prime[i] {
            let mut j = i * i;
            while j <= l {
                is_prime[j] = false;
                j += i;
            }
        }
    }
    let primes: Vec<usize> = (2..=l).filter(|&i| is_prime[i]).collect();

    // For each index in [0, B], store small prime factors
    let mut factor_count = vec![0u16; b + 1];
    let mut factor_lists: Vec<Vec<usize>> = vec![Vec::new(); b + 1];

    for &p in &primes {
        let rem = (a % p as i64) as usize;
        let start = if rem == 0 { 0 } else { p - rem };
        let mut i = start;
        while i <= b {
            if (p as i64) < a + i as i64 {
                factor_count[i] += 1;
                factor_lists[i].push(p);
            }
            i += p;
        }
    }

    let mut ans: i64 = 0;

    for i in 1..=b {
        // A+i is prime if it has no small factors
        if factor_count[i] == 0 {
            let n = a + i as i64 - 1;

            // Factorize n using factors of index i-1
            let mut temp = n;
            let mut res: i64 = 1;

            for &p in &factor_lists[i - 1] {
                let p64 = p as i64;
                if temp % p64 != 0 { continue; }
                let mut e: i64 = 0;
                while temp % p64 == 0 {
                    temp /= p64;
                    e += 1;
                }
                let term = ((mod_pow(p64, 3 * e, m) + mod_pow(p64, 3 * e - 1, m))
                    % m - mod_pow(p64, 2 * e - 1, m) + m) % m;
                res = (res as i128 * term as i128 % m as i128) as i64;
            }

            if temp > 1 {
                let p = temp;
                let term = ((mod_pow(p, 3, m) + mod_pow(p, 2, m)) % m
                    - mod_pow(p, 1, m) + m) % m;
                res = (res as i128 * term as i128 % m as i128) as i64;
            }

            ans = (ans + mod_pow(n, 2, m) + res) % m;
        }
    }

    println!("{}", ans);
}
