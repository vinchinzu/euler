// Project Euler 869 - Prime Bit Guessing Game
// E(N) = expected score when optimally guessing binary of random prime <= N.
// Process bit-by-bit from LSB, stable partition groups by bit value.

const LIMIT: usize = 100_000_000;

fn main() {
    // Sieve for odd numbers
    let half = LIMIT / 2 + 1;
    let mut sieve = vec![0u8; half];
    sieve[0] = 1; // 1 not prime

    let lim = (LIMIT as f64).sqrt() as usize;
    for i in 1..half {
        let p = 2 * i + 1;
        if p > lim { break; }
        if sieve[i] == 0 {
            let mut j = p as u64 * p as u64;
            while j <= LIMIT as u64 {
                sieve[((j - 1) / 2) as usize] = 1;
                j += 2 * p as u64;
            }
        }
    }

    // Collect primes
    let mut primes: Vec<i32> = Vec::with_capacity(6_000_000);
    primes.push(2);
    for i in 1..half {
        let p = 2 * i + 1;
        if p > LIMIT { break; }
        if sieve[i] == 0 { primes.push(p as i32); }
    }
    let nprimes = primes.len();

    let bitlen = |x: i32| -> i32 { 32 - x.leading_zeros() as i32 };
    let max_bits = bitlen(LIMIT as i32);

    let mut arr: Vec<i32> = primes.clone();
    let mut tmp = vec![0i32; nprimes];

    let mut gs: Vec<usize> = vec![0];
    let mut ge: Vec<usize> = vec![nprimes];

    let mut ngs: Vec<usize> = Vec::new();
    let mut nge: Vec<usize> = Vec::new();

    let mut total_score = 0.0f64;

    for level in 0..max_bits {
        ngs.clear();
        nge.clear();

        for g in 0..gs.len() {
            let s = gs[g];
            let e = ge[g];

            let mut c0 = 0i32;
            let mut c1 = 0i32;
            let mut cont0 = 0usize;
            let mut cont1 = 0usize;

            for i in s..e {
                let p = arr[i];
                let bit = (p >> level) & 1;
                if bit == 0 { c0 += 1; } else { c1 += 1; }
                if bitlen(p) > level + 1 {
                    if bit == 0 { cont0 += 1; } else { cont1 += 1; }
                }
            }

            total_score += c0.max(c1) as f64;

            let mut pos0 = s;
            let mut pos1 = s + cont0;

            for i in s..e {
                let p = arr[i];
                if bitlen(p) <= level + 1 { continue; }
                let bit = (p >> level) & 1;
                if bit == 0 { tmp[pos0] = p; pos0 += 1; }
                else { tmp[pos1] = p; pos1 += 1; }
            }

            if cont0 > 0 {
                ngs.push(s);
                nge.push(s + cont0);
            }
            if cont1 > 0 {
                ngs.push(s + cont0);
                nge.push(s + cont0 + cont1);
            }
        }

        // Copy tmp to arr for active regions
        for g in 0..ngs.len() {
            arr[ngs[g]..nge[g]].copy_from_slice(&tmp[ngs[g]..nge[g]]);
        }

        std::mem::swap(&mut gs, &mut ngs);
        std::mem::swap(&mut ge, &mut nge);

        if gs.is_empty() { break; }
    }

    println!("{:.8}", total_score / nprimes as f64);
}
