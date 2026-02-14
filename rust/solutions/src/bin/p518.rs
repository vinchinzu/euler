// Project Euler 518 - Prime Triples and Geometric Sequences
// Find all triples (a,b,c) of primes < 10^8 forming a geometric sequence.
// a+1 = k*p^2, b+1 = k*p*q, c+1 = k*q^2 with gcd(p,q)=1, p<q.

use euler_utils::gcd;

const N: usize = 100_000_000;

fn main() {
    // Bit sieve
    let mut sieve = vec![0u8; N / 8 + 1];
    let set_composite = |sieve: &mut Vec<u8>, i: usize| {
        sieve[i >> 3] |= 1 << (i & 7);
    };
    let is_prime = |sieve: &[u8], i: usize| -> bool {
        if i < 2 { return false; }
        (sieve[i >> 3] & (1 << (i & 7))) == 0
    };

    set_composite(&mut sieve, 0);
    set_composite(&mut sieve, 1);
    let sq = (N as f64).sqrt() as usize + 1;
    for i in 2..=sq {
        if is_prime(&sieve, i) {
            let mut j = i * i;
            while j < N {
                set_composite(&mut sieve, j);
                j += i;
            }
        }
    }

    fn isqrt_int(n: i64) -> i64 {
        let mut r = (n as f64).sqrt() as i64;
        while r * r > n { r -= 1; }
        while (r + 1) * (r + 1) <= n { r += 1; }
        r
    }

    let mut ans: i64 = 0;
    let k_max = N / 4;

    for k in 1..=k_max {
        let q_max = isqrt_int(N as i64 / k as i64);

        for q in 2..=q_max {
            let c = k as i64 * q * q - 1;
            if c < 2 || c >= N as i64 { continue; }
            if !is_prime(&sieve, c as usize) { continue; }

            for p in 1..q {
                if gcd(q as u64, p as u64) != 1 { continue; }
                let a = k as i64 * p * p - 1;
                if a < 2 { continue; }
                if !is_prime(&sieve, a as usize) { continue; }
                let b = k as i64 * p * q - 1;
                if b < 2 || b >= N as i64 { continue; }
                if !is_prime(&sieve, b as usize) { continue; }
                ans += a + b + c;
            }
        }
    }

    println!("{}", ans);
}
