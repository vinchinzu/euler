// Project Euler 543 - Prime-Sum Numbers
//
// S(n) uses prime counting function pi(n) on Fibonacci numbers.
// Sieve up to F_44 = 701408733, then compute S(F_k) for k=3..44.

fn main() {
    let max_fib: usize = 701_408_733;

    // Compute Fibonacci numbers
    let mut fibs = vec![0i64; 45];
    fibs[0] = 0;
    fibs[1] = 1;
    for i in 2..=44 {
        fibs[i] = fibs[i - 1] + fibs[i - 2];
    }

    // Bit sieve
    let sieve_size = max_fib + 11;
    let num_bytes = (sieve_size + 7) / 8;
    let mut sieve_bits = vec![0xFFu8; num_bytes];

    // Clear 0 and 1
    sieve_bits[0] &= !1u8; // bit 0
    sieve_bits[0] &= !2u8; // bit 1

    let sq = (sieve_size as f64).sqrt() as usize + 1;
    for i in 2..=sq {
        if (sieve_bits[i >> 3] >> (i & 7)) & 1 == 1 {
            let mut j = i * i;
            while j < sieve_size {
                sieve_bits[j >> 3] &= !(1u8 << (j & 7));
                j += i;
            }
        }
    }

    // Count primes <= n using popcount on the sieve
    let count_primes = |n: usize| -> i64 {
        if n < 2 { return 0; }
        let n = n.min(sieve_size - 1);
        let mut count: i64 = 0;
        let full_bytes = n / 8;
        for i in 0..full_bytes {
            count += sieve_bits[i].count_ones() as i64;
        }
        let rem = (n % 8) + 1;
        let mask = (1u16 << rem) as u8 - 1;
        count += (sieve_bits[full_bytes] & mask).count_ones() as i64;
        count
    };

    let triangular = |n: i64| -> i64 { n * (n + 1) / 2 };

    let compute_s = |n: i64| -> i64 {
        let nu = n as usize;
        let mut result = count_primes(nu);
        if n >= 4 {
            result += n / 2 - 1;
            result += count_primes((n - 2) as usize) - 1;
            let half = n / 2;
            if half >= 3 {
                result += (n + 1) * (half - 2) - 2 * (triangular(half) - 3);
            }
        }
        result
    };

    let mut ans: i64 = 0;
    for k in 3..=44 {
        ans += compute_s(fibs[k]);
    }

    println!("{ans}");
}
