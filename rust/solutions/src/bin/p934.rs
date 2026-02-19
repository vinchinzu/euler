// Project Euler Problem 934 - Unlucky Prime
// U(N) = sum_{n=1}^N u(n) where u(n) is smallest prime p with n%p not divisible by 7.
// Uses sieving over prime products with CRT-like residue expansion.

fn main() {
    // Sieve primes up to 1000
    let limit = 1000usize;
    let mut is_prime_sieve = vec![true; limit + 1];
    is_prime_sieve[0] = false;
    is_prime_sieve[1] = false;
    {
        let mut i = 2;
        while i * i <= limit {
            if is_prime_sieve[i] {
                let mut j = i * i;
                while j <= limit {
                    is_prime_sieve[j] = false;
                    j += i;
                }
            }
            i += 1;
        }
    }
    let primes: Vec<i64> = (2..=limit).filter(|&i| is_prime_sieve[i]).map(|i| i as i64).collect();

    let n: i64 = 100_000_000_000_000_000; // 10^17
    let mut total: i64 = 0;

    // k=1: P1=2. Term 2 * S1. S1=N
    total += 2 * n;
    // k=2: P2=3. Term (3-2) * S2. S2=N/2
    total += 1 * (n / 2);
    // k=3: P3=5. Term (5-3) * S3. S3=N/6
    total += 2 * (n / 6);
    // k=4: P4=7. Term (7-5) * S4. S4=N/30
    total += 2 * (n / 30);
    // k=5: P5=11. Term (11-7) * S5. S5=N/210
    total += 4 * (n / 210);

    let m = n / 210;
    let m128 = m as i128;

    // Dynamic residue expansion
    // Use i128 for mod_val to avoid overflow when products of primes exceed i64 range
    let mut residues: Vec<i64> = vec![0];
    let mut mod_val: i128 = 1;

    // Cap residue array to avoid OOM
    const MAX_RESIDUES: usize = 50_000_000;

    // Start from index 4 (primes[4] = 11)
    let mut idx = 4usize;
    while idx + 1 < primes.len() {
        let p = primes[idx];
        let p128 = p as i128;
        let next_p = primes[idx + 1];
        let diff = next_p - p;

        let next_mod = mod_val * p128;

        // Compute valid digits: s in 0..(p-1)/7
        let limit_s = (p - 1) / 7;

        // Compute inv30 mod p using Fermat's little theorem
        let inv30 = pow_mod(30 % p, p - 2, p);

        let mut valid_digits: Vec<i64> = Vec::with_capacity((limit_s + 1) as usize);
        for s in 0..=limit_s {
            valid_digits.push(s * inv30 % p);
        }

        // inv_mod = mod_val^(-1) mod p (mod_val % p fits in i64)
        let mod_val_mod_p = ((mod_val % p128) as i64 + p) % p;
        let inv_mod = pow_mod(mod_val_mod_p, p - 2, p);

        let is_next_mod_large = next_mod > m128;

        let nres = residues.len();
        let nvalid = valid_digits.len();

        // Check if allocation would be too large
        if nres.checked_mul(nvalid).unwrap_or(usize::MAX) > MAX_RESIDUES {
            break;
        }

        let mut new_residues: Vec<i64> = Vec::with_capacity(nres * nvalid);

        for &r in &residues {
            let r_mod_p = ((r % p) + p) % p;
            for &d in &valid_digits {
                let k_val = ((d - r_mod_p + p) % p * inv_mod) % p;
                // Use i128 to avoid overflow in k_val * mod_val
                let x = r as i128 + k_val as i128 * mod_val;
                if is_next_mod_large {
                    if x <= m128 {
                        new_residues.push(x as i64);
                    }
                } else {
                    new_residues.push(x as i64);
                }
            }
        }

        residues = new_residues;
        mod_val = next_mod;

        // Calculate count
        let count: i64 = if mod_val > m128 {
            residues.len() as i64
        } else {
            let mod_val_i64 = mod_val as i64; // safe: mod_val <= m which fits i64
            let full_cycles = m / mod_val_i64;
            let rem = m % mod_val_i64;
            let partial = residues.iter().filter(|&&r| r <= rem).count() as i64;
            full_cycles * residues.len() as i64 + partial
        };
        let count = count - 1; // Exclude 0

        if count == 0 {
            break;
        }

        total += diff * count;
        idx += 1;
    }

    println!("{}", total);
}

fn pow_mod(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut result: i64 = 1;
    base %= m;
    if base < 0 { base += m; }
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result as i128 * base as i128 % m as i128) as i64;
        }
        base = (base as i128 * base as i128 % m as i128) as i64;
        exp >>= 1;
    }
    result
}
