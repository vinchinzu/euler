// Project Euler Problem 937 - Equiproduct Partition
//
// Key result: k! is in A iff parity is even, where
//   parity = TM(v_2(k!)) + sum_{inert prime q <= k} TM(v_q(k!))  (mod 2)
// TM(v) = popcount(v) mod 2. Inert primes: p % 8 in {5, 7}.
//
// Optimizations (all single-threaded):
// 1. Bitset sieve (8x less memory, better cache)
// 2. Primes > N/2: mark directly during scan
// 3. p=2: trailing_zeros for fast v_2
// 4. Small primes: power-layer v_p sieve with reusable buffer
// 5. Medium primes: precomputed TM flip table (v always +1)

fn main() {
    const MOD: u64 = 1_000_000_007;
    const N: usize = 100_000_000;

    // Bitset sieve
    let num_bytes = (N + 1 + 7) / 8;
    let mut sieve = vec![0xFFu8; num_bytes];
    sieve[0] &= !0b11;
    let sqrt_n = ((N as f64).sqrt() as usize) + 1;
    for i in 2..=sqrt_n {
        if sieve[i >> 3] & (1u8 << (i & 7)) != 0 {
            let mut j = i * i;
            while j <= N {
                sieve[j >> 3] &= !(1u8 << (j & 7));
                j += i;
            }
        }
    }

    #[inline(always)]
    fn is_prime_bit(sieve: &[u8], n: usize) -> bool {
        sieve[n >> 3] & (1u8 << (n & 7)) != 0
    }

    let mut diff = vec![0u8; N + 2];

    // Layer 1: Primes p > N/2: single multiple (p), always flips TM
    let half_n = N / 2;
    for p in (half_n + 1)..=N {
        if is_prime_bit(&sieve, p) {
            let r = p & 7;
            if r == 5 || r == 7 {
                unsafe { *diff.get_unchecked_mut(p) ^= 1; }
            }
        }
    }

    // Collect primes
    let threshold = sqrt_n;
    let mut small_primes: Vec<usize> = Vec::new();
    let mut medium_primes: Vec<usize> = Vec::new();

    for p in 3..=half_n {
        if is_prime_bit(&sieve, p) {
            let r = p & 7;
            if r == 5 || r == 7 {
                if p <= threshold {
                    small_primes.push(p);
                } else {
                    medium_primes.push(p);
                }
            }
        }
    }

    // Process p=2 using trailing_zeros
    {
        let mut v: u32 = 0;
        let mut m = 2usize;
        while m <= N {
            let e = (m as u32).trailing_zeros();
            let old_tm = v.count_ones() & 1;
            v += e;
            let new_tm = v.count_ones() & 1;
            if old_tm != new_tm {
                unsafe { *diff.get_unchecked_mut(m) ^= 1; }
            }
            m += 2;
        }
    }

    // Process small inert primes with pre-allocated reusable buffer
    {
        let max_buf = N / small_primes.first().copied().unwrap_or(5);
        let mut vp_buf = vec![0u8; max_buf];

        for &p in &small_primes {
            let num_mult = N / p;

            // Initialize: every multiple of p has v_p >= 1
            for i in 0..num_mult {
                unsafe { *vp_buf.get_unchecked_mut(i) = 1; }
            }

            // Add contributions from powers p^j for j >= 2
            let mut power = p * p;
            while power <= N {
                let mut m = power;
                while m <= N {
                    let idx = m / p - 1;
                    unsafe { *vp_buf.get_unchecked_mut(idx) += 1; }
                    m += power;
                }
                power = match power.checked_mul(p) {
                    Some(v) if v <= N => v,
                    _ => break,
                };
            }

            // Sweep through multiples computing cumulative TM flips
            let mut v: u32 = 0;
            for c in 0..num_mult {
                let e = unsafe { *vp_buf.get_unchecked(c) } as u32;
                let old_tm = v.count_ones() & 1;
                v += e;
                let new_tm = v.count_ones() & 1;
                if old_tm != new_tm {
                    let m = (c + 1) * p;
                    unsafe { *diff.get_unchecked_mut(m) ^= 1; }
                }
            }
        }
    }

    // Precompute TM flip table for medium primes (v increments by 1 each step)
    let max_mult = if threshold > 0 { N / (threshold + 1) + 2 } else { N + 1 };
    let tm_flips: Vec<bool> = {
        let mut flips = vec![false; max_mult + 1];
        let mut v: u32 = 0;
        for c in 1..=max_mult {
            let old_tm = v.count_ones() & 1;
            v += 1;
            let new_tm = v.count_ones() & 1;
            flips[c] = old_tm != new_tm;
        }
        flips
    };

    // Process medium primes sequentially
    for &p in &medium_primes {
        let num_mult = N / p;
        let mut m = p;
        for c in 1..=num_mult {
            if unsafe { *tm_flips.get_unchecked(c) } {
                unsafe { *diff.get_unchecked_mut(m) ^= 1; }
            }
            m += p;
        }
    }

    // Compute running parity and accumulate sum
    let mut parity: u64 = 0;
    let mut factorial: u64 = 1;
    let mut total_sum: u64 = 0;

    for k in 1..=N {
        factorial = factorial % MOD * (k as u64 % MOD) % MOD;
        parity ^= unsafe { *diff.get_unchecked(k) } as u64;
        total_sum = (total_sum + factorial * (1 - parity)) % MOD;
    }

    println!("{}", total_sum);
}
