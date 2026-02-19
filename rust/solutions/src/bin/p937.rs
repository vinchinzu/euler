// Project Euler Problem 937 - Equiproduct Partition
//
// Let theta = sqrt(-2). T = {a + b*theta : a,b integers, a>0 or (a=0,b>0)}.
// Partition T into A and B with 1 in A, p(A,z) = p(B,z) for all z in T.
// G(n) = sum of k! in F_n intersect A, where F_n = {1!, ..., n!}.
//
// Key result: k! is in A iff the "parity" is even, where
//   parity = TM(v_2(k!)) + sum_{inert prime q <= k} TM(v_q(k!))  (mod 2)
// and TM(v) = popcount(v) mod 2 (Thue-Morse sequence).
// Inert primes in Z[sqrt(-2)] are those with p % 8 in {5, 7}.
// Split primes (p % 8 in {1, 3}) do not contribute.
//
// Approach: sieve-based. For each relevant prime p, iterate over multiples
// and track cumulative v_p. Use a difference array for parity flip events.

fn main() {
    const MOD: u64 = 1_000_000_007;
    const N: usize = 100_000_000;

    // Sieve primes
    let mut is_prime = vec![true; N + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let sqrt_n = ((N as f64).sqrt() as usize) + 1;
    for i in 2..=sqrt_n {
        if is_prime[i] {
            let mut j = i * i;
            while j <= N {
                is_prime[j] = false;
                j += i;
            }
        }
    }

    // Collect relevant primes: p=2 and inert primes (p%8 in {5,7})
    let mut relevant_primes: Vec<usize> = vec![2];
    for p in 3..=N {
        if is_prime[p] {
            let r = p & 7;
            if r == 5 || r == 7 {
                relevant_primes.push(p);
            }
        }
    }

    // Difference array for parity flips.
    // diff[k] ^= 1 means: parity flips at position k.
    let mut diff = vec![0u8; N + 2];

    for &p in &relevant_primes {
        // For prime p, process multiples: p, 2p, 3p, ..., floor(N/p)*p.
        // v_p(k!) increments by v_p(k) at each multiple k of p.
        // v_p(k) = number of times p divides k.
        // Instead of computing v_p(k) by trial division, use the structure:
        // - Multiples of p but not p^2: v_p(k) = 1 (most common)
        // - Multiples of p^2 but not p^3: v_p(k) = 2
        // - etc.
        // Process by layers: for j = 1, 2, ..., iterate over multiples of p^j.
        // Each multiple of p^j contributes +1 to v_p(k) beyond what p^{j-1} gave.
        // So total v_p(k) = #{j >= 1 : p^j | k}.
        //
        // Cumulative: v_p(m!) = sum_{k=1..m} v_p(k) = sum_{j=1..} floor(m/p^j).
        // But we need v_p at every multiple of p, which changes incrementally.
        //
        // Simpler: iterate multiples of p. For each m = c*p:
        //   v_p(m) = 1 + v_p(c). So we can compute v_p(m) recursively.
        //   But that requires knowing v_p(c), which means factoring c by p.
        //
        // Fastest: precompute v_p(k) for all multiples of p using a sieve approach.
        // For each power j: mark all multiples of p^j. v_p(k) += 1 for each such j.
        //
        // For the difference array approach, we just need to know WHEN v_p(k!) changes parity.
        // v_p(k!) = v_p((k-1)!) + v_p(k). The TM parity of v_p(k!) changes iff
        // popcount(v_p(k!)) != popcount(v_p((k-1)!)).
        //
        // Since v_p(k) >= 1 for every multiple of p, v_p(k!) is non-decreasing and
        // increases at each multiple of p. The AMOUNT of increase = v_p(k).

        let mut v: u32 = 0; // cumulative v_p(k!)

        // Process multiples of p.
        // At m = c*p where gcd(c, p) = ... c might still be divisible by p.
        // v_p(m) = v_p(c*p) = 1 + v_p(c). Need to compute v_p(c).
        // But c ranges over 1..N/p. We can compute v_p(c) on the fly.
        // For c: keep dividing by p.

        let mut m = p;
        while m <= N {
            // Compute v_p(m): count how many times p divides m
            let mut e: u32 = 1; // at least 1 since m is a multiple of p
            let mut t = m / p;
            while t % p == 0 {
                e += 1;
                t /= p;
            }

            let old_tm = v.count_ones() & 1;
            v += e;
            let new_tm = v.count_ones() & 1;

            if old_tm != new_tm {
                // SAFETY: m <= N < diff.len()
                unsafe {
                    *diff.get_unchecked_mut(m) ^= 1;
                }
            }

            m += p;
        }
    }

    // Compute running parity and accumulate sum
    let mut parity: u32 = 0;
    let mut factorial: u64 = 1;
    let mut total_sum: u64 = 0;

    for k in 1..=N {
        factorial = factorial % MOD * (k as u64 % MOD) % MOD;
        // SAFETY: k <= N < diff.len()
        parity ^= unsafe { *diff.get_unchecked(k) } as u32;

        if parity == 0 {
            total_sum = (total_sum + factorial) % MOD;
        }
    }

    println!("{}", total_sum);
}
