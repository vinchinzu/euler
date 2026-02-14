// Project Euler 550 - Divisor Game
//
// Compute Sprague-Grundy nimbers for the divisor game, then use
// Walsh-Hadamard XOR convolution to count losing positions for K rounds.

const N: usize = 10_000_000;
const K: i64 = 1_000_000_000_000;
const L: usize = 64;
const M: i64 = 987_654_321;

fn power(mut base: i64, mut exp: i64) -> i64 {
    let mut result: i64 = 1;
    base = ((base % M) + M) % M;
    while exp > 0 {
        if exp & 1 == 1 { result = (result as i128 * base as i128 % M as i128) as i64; }
        base = (base as i128 * base as i128 % M as i128) as i64;
        exp >>= 1;
    }
    result
}

fn mod_inverse(a: i64, m: i64) -> i64 {
    fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
        if a == 0 { return (b, 0, 1); }
        let (g, x1, y1) = extended_gcd(b % a, a);
        (g, y1 - (b / a) * x1, x1)
    }
    let (_, x, _) = extended_gcd(a % m, m);
    ((x % m) + m) % m
}

fn main() {
    // Sieve
    let mut is_prime = vec![false; N + 1];
    for i in 2..=N { is_prime[i] = true; }
    {
        let mut i = 2i64;
        while i * i <= N as i64 {
            if is_prime[i as usize] {
                let mut j = i * i;
                while j <= N as i64 { is_prime[j as usize] = false; j += i; }
            }
            i += 1;
        }
    }

    let mut primes: Vec<usize> = Vec::new();
    let mut num_primes = vec![0usize; N + 1];
    let mut cnt = 0;
    for i in 0..=N {
        if i >= 2 && is_prime[i] {
            primes.push(i);
            cnt += 1;
        }
        num_primes[i] = cnt;
    }

    let mut nimbers = vec![0u8; N + 1];
    let mut counts = [0i64; L];

    // Get sorted divisors of n
    fn get_divisors(n: usize) -> Vec<usize> {
        let mut divs = Vec::new();
        let mut i = 1;
        while i * i <= n {
            if n % i == 0 {
                divs.push(i);
                if i != n / i { divs.push(n / i); }
            }
            i += 1;
        }
        divs.sort_unstable();
        divs
    }

    // Recursive helper
    fn helper(
        min_idx: usize, n: usize,
        primes: &[usize], num_primes: &[usize],
        nimbers: &mut [u8], counts: &mut [i64; L],
    ) {
        if n > 1 {
            let divs = get_divisors(n);
            let mut used = [false; L];
            for i in 1..divs.len() - 1 {
                for j in i..divs.len() - 1 {
                    let xor_val = (nimbers[divs[i]] ^ nimbers[divs[j]]) as usize;
                    if xor_val < L { used[xor_val] = true; }
                }
            }
            let mut nim = 0u8;
            while (nim as usize) < L && used[nim as usize] { nim += 1; }
            nimbers[n] = nim;
            counts[nim as usize] += 1;
        }

        for idx in min_idx..primes.len() {
            let p = primes[idx];
            if (n as u64) * (p as u64) > N as u64 { break; }

            let mut new_n = n;
            while (new_n as u64) * (p as u64) <= N as u64 {
                new_n *= p;
                helper(idx + 1, new_n, primes, num_primes, nimbers, counts);
            }

            // Tail: remaining primes only multiply once
            if (n as u64) * (p as u64) * (p as u64) > N as u64 {
                if idx > 0 {
                    let prev_p = primes[idx - 1];
                    let add = num_primes[N / n] as i64 - num_primes[prev_p] as i64 - 1;
                    if add > 0 {
                        counts[nimbers[n * p] as usize] += add;
                    }
                }
                return;
            }
        }
    }

    helper(0, 1, &primes, &num_primes, &mut nimbers, &mut counts);

    let mut arr = [0i64; L];
    for i in 0..L { arr[i] = counts[i] % M; }

    // Forward FWHT
    let mut len = 1;
    while len < L {
        let mut i = 0;
        while i < L {
            for j in 0..len {
                let u = arr[i + j];
                let v = arr[i + j + len];
                arr[i + j] = (u + v) % M;
                arr[i + j + len] = ((u - v) % M + M) % M;
            }
            i += len << 1;
        }
        len <<= 1;
    }

    // Raise to K-th power
    for i in 0..L {
        arr[i] = power(arr[i], K);
    }

    // Inverse FWHT
    len = 1;
    while len < L {
        let mut i = 0;
        while i < L {
            for j in 0..len {
                let u = arr[i + j];
                let v = arr[i + j + len];
                arr[i + j] = (u + v) % M;
                arr[i + j + len] = ((u - v) % M + M) % M;
            }
            i += len << 1;
        }
        len <<= 1;
    }

    let inv_n = mod_inverse(L as i64, M);
    for i in 0..L {
        arr[i] = (arr[i] as i128 * inv_n as i128 % M as i128) as i64;
    }

    let mut ans: i64 = 0;
    for i in 1..L {
        ans = (ans + arr[i]) % M;
    }

    println!("{ans}");
}
