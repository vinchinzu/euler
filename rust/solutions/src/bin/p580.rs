// Project Euler 580 - Squarefree Hilbert Numbers
//
// Count Hilbert squarefree numbers below N=10^16.
// A Hilbert number is any positive integer of form 4k+1 (k >= 0).
// A squarefree Hilbert number is not divisible by the square of any Hilbert
// number other than 1.
//
// Uses inclusion-exclusion with a Hilbert Mobius function:
//   answer = Sum_{i odd, i^2 <= N} hilbert_mu(i) * floor((N/i^2 + 3) / 4)
//
// The coefficient hilbert_mu(i) depends on the prime factorization of i:
//   - Let n1 = number of distinct 4k+1 primes dividing i (all must have exponent 1)
//   - Let n3_sf = number of 4k+3 primes with exponent 1
//   - Let n3_sq = number of 4k+3 primes with exponent 2 (at most 1 allowed)
//   - No 4k+3 prime may have exponent > 2
//   - No 4k+1 prime may have exponent > 1
//
//   If n3_sq == 0: hilbert_mu = (-1)^n1 * (-1)^n3_sf * (1 - n3_sf)
//   If n3_sq == 1: hilbert_mu = (-1)^n1 * (-1)^(n3_sf + n3_sq) = (-1)^(n1 + n3_sf + 1)
//
// We track 4 tables via SPF sieve:
//   t0: sum of (exponent - 1) over 4k+1 prime factors  (must be 0)
//   t1: count of distinct 4k+1 prime factors            (= n1)
//   t2: sum of (exponent - 1) over 4k+3 prime factors  (= n3_sq, must be 0 or 1)
//   t3: count of distinct 4k+3 prime factors            (= n3_sf + n3_sq)

fn main() {
    let n: i64 = 10_000_000_000_000_000; // 10^16
    let mut l = (n as f64).sqrt() as i64;
    while l * l > n {
        l -= 1;
    }
    while (l + 1) * (l + 1) <= n {
        l += 1;
    }

    let l_usize = l as usize;

    // Sieve smallest prime factor
    let mut ff = vec![0u32; l_usize + 1];
    for i in 2..=l_usize {
        if ff[i] == 0 {
            ff[i] = i as u32;
            let i64val = i as u64;
            if i64val * i64val <= l_usize as u64 {
                let mut j = i * i;
                while j <= l_usize {
                    if ff[j] == 0 {
                        ff[j] = i as u32;
                    }
                    j += i;
                }
            }
        }
    }

    // t0: sum(exp-1) for 4k+1 primes (incremented when is_square, i.e. repeated factor)
    // t1: count of distinct 4k+1 primes (incremented when NOT is_square, i.e. first occurrence)
    // t2: sum(exp-1) for 4k+3 primes
    // t3: count of distinct 4k+3 primes
    let mut t0 = vec![0i8; l_usize + 1];
    let mut t1 = vec![0i8; l_usize + 1];
    let mut t2 = vec![0i8; l_usize + 1];
    let mut t3 = vec![0i8; l_usize + 1];

    let mut i = 3usize;
    while i <= l_usize {
        let d = {
            let v = ff[i];
            if v == 0 { i as u32 } else { v }
        };
        let d_usize = d as usize;
        let prev = i / d_usize;
        t0[i] = t0[prev];
        t1[i] = t1[prev];
        t2[i] = t2[prev];
        t3[i] = t3[prev];

        let rem_type = d % 4;
        let is_square = (i as u64) % ((d as u64) * (d as u64)) == 0;

        if rem_type == 1 {
            if is_square {
                t0[i] += 1;
            } else {
                t1[i] += 1;
            }
        } else {
            if is_square {
                t2[i] += 1;
            } else {
                t3[i] += 1;
            }
        }

        i += 2;
    }

    // Compute Hilbert Mobius coefficient and accumulate
    let mut ans: i64 = 0;
    let mut i = 1usize;
    while i <= l_usize {
        // Skip if any 4k+1 prime has exponent > 1
        if t0[i] != 0 {
            i += 2;
            continue;
        }
        // Skip if sum(exp-1) for 4k+3 primes >= 2
        // (means either multiple squared primes, or one prime with exp >= 3)
        if t2[i] >= 2 {
            i += 2;
            continue;
        }

        let n1 = t1[i] as i32;
        let hilbert_mu: i32;

        if t2[i] == 0 {
            // All 4k+3 primes have exponent 1
            let s = t3[i] as i32; // n3_sf
            // hilbert_mu = (-1)^n1 * (-1)^s * (1 - s)
            let sign = if (n1 + s) % 2 == 0 { 1 } else { -1 };
            hilbert_mu = sign * (1 - s);
        } else {
            // t2[i] == 1: exactly one 4k+3 prime with exponent 2
            // n3_sf = t3[i] - 1 (since that squared prime also counted in t3)
            // Wait: t3 counts distinct primes (first occurrence), t2 counts extra.
            // For a prime p^2, t3 gets +1 (first occurrence) and t2 gets +1 (second).
            // So t3 = n3_sf + n3_sq = n3_sf + 1, and n3_sf = t3 - 1.
            // hilbert_mu = (-1)^n1 * (-1)^(n3_sf + 1)
            //            = (-1)^n1 * (-1)^(t3[i] - 1 + 1)
            //            = (-1)^n1 * (-1)^t3[i]
            //            = (-1)^(n1 + t3[i])
            let total = n1 + t3[i] as i32;
            hilbert_mu = if total % 2 == 0 { 1 } else { -1 };
        }

        if hilbert_mu == 0 {
            i += 2;
            continue;
        }

        let isq = (i as i64) * (i as i64);
        let q = n / isq;
        let count = (q + 3) / 4;

        ans += hilbert_mu as i64 * count;

        i += 2;
    }

    println!("{}", ans);
}
