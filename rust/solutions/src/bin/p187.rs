// Project Euler 187: Semiprimes.
// Count n < 10^8 that are the product of exactly two primes.

const LIMIT: u64 = 100_000_000;
const SIEVE_LIMIT: usize = 50_000_000; // LIMIT/2

fn main() {
    let n_odd = SIEVE_LIMIT / 2 + 1;
    let mut is_comp = vec![0u64; (n_odd + 63) / 64];
    is_comp[0] |= 1; // 1

    let mut p = 3usize;
    while p * p <= SIEVE_LIMIT {
        let i = p >> 1;
        if (is_comp[i >> 6] >> (i & 63)) & 1 == 0 {
            let mut q = p * p;
            let step = p * 2;
            while q <= SIEVE_LIMIT {
                let j = q >> 1;
                is_comp[j >> 6] |= 1u64 << (j & 63);
                q += step;
            }
        }
        p += 2;
    }

    let mut primes: Vec<u32> = Vec::with_capacity(3_000_000);
    primes.push(2);
    let mut n = 3usize;
    while n <= SIEVE_LIMIT {
        let i = n >> 1;
        if (is_comp[i >> 6] >> (i & 63)) & 1 == 0 {
            primes.push(n as u32);
        }
        n += 2;
    }

    let mut count: u64 = 0;
    for (i, &pp) in primes.iter().enumerate() {
        let p = pp as u64;
        if p * p >= LIMIT {
            break;
        }
        let max_q = (LIMIT - 1) / p;
        let hi = primes.partition_point(|&q| q as u64 <= max_q);
        if hi > i {
            count += (hi - i) as u64;
        }
    }
    println!("{}", count);
}
