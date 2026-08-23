// Project Euler 543 - Prime-Sum Numbers
//
// S(n) uses prime counting function pi(n) on Fibonacci numbers.
// Sieve up to F_44 = 701408733, then compute S(F_k) for k=3..44.

fn main() {
    let max_fib: usize = 701_408_733;

    // Compute Fibonacci numbers
    let mut fibs = [0i64; 45];
    fibs[1] = 1;
    for i in 2..=44 {
        fibs[i] = fibs[i - 1] + fibs[i - 2];
    }

    // Bit sieve, padded to a multiple of 8 bytes for u64 prefix scans.
    // 0xAA keeps odd bits; evens (except 2) are composite.
    let sieve_size = max_fib + 11;
    let num_bytes = (((sieve_size + 7) / 8) + 7) & !7;
    let mut sieve_bits = vec![0xAAu8; num_bytes];
    sieve_bits[0] |= 1 << 2; // 2 is prime
    sieve_bits[0] &= !2u8; // 1 is not prime

    let sq = (sieve_size as f64).sqrt() as usize + 1;
    for i in (3..=sq).step_by(2) {
        if (sieve_bits[i >> 3] >> (i & 7)) & 1 == 1 {
            let step = i << 1;
            let mut j = i * i;
            while j < sieve_size {
                // SAFETY: j < sieve_size and num_bytes covers all sieve bits
                unsafe {
                    *sieve_bits.get_unchecked_mut(j >> 3) &= !(1u8 << (j & 7));
                }
                j += step;
            }
        }
    }

    // One prefix popcount of the bit sieve, then O(1) π(n) for all queries.
    let nwords = num_bytes / 8;
    let mut prefix = vec![0u32; nwords + 1];
    for w in 0..nwords {
        // SAFETY: w < nwords, buffer is exactly 8 * nwords bytes
        let word = u64::from_le_bytes(unsafe {
            *sieve_bits.as_ptr().add(w * 8).cast::<[u8; 8]>()
        });
        prefix[w + 1] = prefix[w] + word.count_ones();
    }

    let count_primes = |n: usize| -> i64 {
        if n < 2 {
            return 0;
        }
        let n = n.min(sieve_size - 1);
        let w = n / 64;
        let bit = n % 64;
        // SAFETY: n < sieve_size ≤ 8 * num_bytes, so word w is in-bounds
        let word = u64::from_le_bytes(unsafe {
            *sieve_bits.as_ptr().add(w * 8).cast::<[u8; 8]>()
        });
        let mask = if bit == 63 {
            u64::MAX
        } else {
            (1u64 << (bit + 1)) - 1
        };
        prefix[w] as i64 + (word & mask).count_ones() as i64
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
