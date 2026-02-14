// Project Euler 437 - Fibonacci primitive roots
//
// Sum of primes p <= 10^8 that have a Fibonacci primitive root.

const N: usize = 100_000_000;

fn fib_pair(n: u64, m: u64) -> (u64, u64) {
    if n == 0 {
        return (0, 1);
    }
    let (a, b) = fib_pair(n >> 1, m);
    let two_b = (b << 1) % m;
    let c = a as u128 * ((two_b + m - a) % m) as u128 % m as u128;
    let d = (a as u128 * a as u128 + b as u128 * b as u128) % m as u128;
    if n & 1 == 1 {
        (d as u64, (c + d) as u64 % m)
    } else {
        (c as u64, d as u64)
    }
}

fn main() {
    // Linear sieve for smallest prime factor
    let mut spf = vec![0u32; N + 1];
    let mut primes = Vec::with_capacity(N / 10);

    for i in 2..=N {
        if spf[i] == 0 {
            spf[i] = i as u32;
            primes.push(i as u32);
        }
        for &p in &primes {
            let v = i as u64 * p as u64;
            if v > N as u64 || p > spf[i] {
                break;
            }
            spf[v as usize] = p;
        }
    }

    let mut sum: u64 = 0;
    for &p in &primes {
        let p64 = p as u64;
        if p == 5 {
            sum += 5;
            continue;
        }
        let mod10 = p % 10;
        if mod10 != 1 && mod10 != 9 {
            continue;
        }

        let mut n = p - 1;
        let mut good = true;
        while n > 1 {
            let q = spf[n as usize];
            while n % q == 0 {
                n /= q;
            }
            let (f, g) = fib_pair((p64 - 1) / q as u64, p64);
            if f == 0 && g == 1 {
                good = false;
                break;
            }
        }
        if good {
            sum += p64;
        }
    }

    println!("{}", sum);
}
