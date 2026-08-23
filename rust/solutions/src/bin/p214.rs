// Project Euler 214: Totient Chains
// Sum of primes p < 40,000,000 whose totient chain has length 25.
// Linear sieve for phi (one write per n) + even-only u8 chain (phi(n) even for n>=3).

const LIMIT: usize = 40_000_000;
const TARGET: u8 = 25;

fn main() {
    let mut phi = vec![0u32; LIMIT];
    let mut primes: Vec<u32> = Vec::with_capacity(2_500_000);

    // SAFETY: every index is in 1..LIMIT; composites are written at i*p with i*p < LIMIT.
    unsafe {
        let phi_p = phi.as_mut_ptr();
        *phi_p.add(1) = 1;

        primes.push(2);
        *phi_p.add(2) = 1;
        if LIMIT > 4 {
            *phi_p.add(4) = 2;
        }

        for i in 3..LIMIT {
            if i & 1 == 0 {
                let phi_i = *phi_p.add(i);
                if i < LIMIT / 2 {
                    *phi_p.add(i * 2) = phi_i << 1;
                }
                continue;
            }

            let mut phi_i = *phi_p.add(i);
            if phi_i == 0 {
                primes.push(i as u32);
                phi_i = i as u32 - 1;
                *phi_p.add(i) = phi_i;
            }

            if i < LIMIT / 2 {
                *phi_p.add(i * 2) = phi_i;
            }

            let pr = primes.as_ptr();
            let np = primes.len();
            let mut j = 1;
            while j < np {
                let p = *pr.add(j);
                let ip = i as u64 * p as u64;
                if ip >= LIMIT as u64 {
                    break;
                }
                if (i as u32) % p == 0 {
                    *phi_p.add(ip as usize) = phi_i * p;
                    break;
                }
                *phi_p.add(ip as usize) = phi_i * (p - 1);
                j += 1;
            }
        }
    }

    // chain[n] for even n (and 1) lives at index n/2. phi(n) is even for n >= 3,
    // so even n only ever look up even (or 1) predecessors.
    let mut chain = vec![0u8; LIMIT / 2];
    chain[0] = 1;

    // SAFETY: i even in 2..LIMIT, phi[i] < i so phi[i]>>1 < LIMIT/2; chain has LIMIT/2 slots.
    unsafe {
        let phi_p = phi.as_ptr();
        let ch = chain.as_mut_ptr();
        let mut i = 2usize;
        while i < LIMIT {
            #[cfg(target_arch = "x86_64")]
            if i + 64 < LIMIT {
                let nxt = (*phi_p.add(i + 64) as usize) >> 1;
                core::arch::x86_64::_mm_prefetch::<{ core::arch::x86_64::_MM_HINT_T0 }>(
                    ch.add(nxt) as *const i8,
                );
            }
            let ph = *phi_p.add(i) as usize;
            *ch.add(i >> 1) = *ch.add(ph >> 1) + 1;
            i += 2;
        }
    }

    let mut ans: i64 = 0;
    for &p in &primes {
        // chain(p) = chain(p-1) + 1; p-1 is even for p>2
        // SAFETY: p < LIMIT so (p-1)>>1 < LIMIT/2
        unsafe {
            if *chain.get_unchecked((p as usize - 1) >> 1) + 1 == TARGET {
                ans += p as i64;
            }
        }
    }

    println!("{ans}");
}
