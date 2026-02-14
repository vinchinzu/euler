// Project Euler 583 - Heron Envelopes
//
// Find the sum of perimeters of Heron envelopes with perimeter <= 10^7.
// A Heron envelope is a pentagon ABCDE = rectangle ABDE + isosceles triangle BCD,
// with all sides and diagonals integral.

use euler_utils::gcd_i64;

const LIMIT: i64 = 10_000_000;

fn isqrt(n: i64) -> i64 {
    if n < 0 { return 0; }
    let mut r = (n as f64).sqrt() as i64;
    while r * r > n { r -= 1; }
    while (r + 1) * (r + 1) <= n { r += 1; }
    r
}

fn main() {
    let n = LIMIT as usize;

    // Sieve for smallest prime factor
    let mut ff = vec![0u32; n + 1];
    for i in 2..=n {
        if ff[i] == 0 {
            ff[i] = i as u32;
            let mut j = i * i;
            while j <= n {
                if ff[j] == 0 { ff[j] = i as u32; }
                j += i;
            }
        }
    }

    // Compute number of divisors using SPF
    let mut num_factors = vec![0i32; n + 1];
    num_factors[1] = 1;
    for i in 2..=n {
        let p = ff[i] as i32;
        let mut ii = i as i32;
        let mut mult = 1;
        while ii % p == 0 {
            ii /= p;
            mult += 2;
        }
        num_factors[i] = num_factors[ii as usize] * mult;
    }

    // For each a, allocate storage for other legs
    let mut start_indices = vec![0usize; n + 2];
    for i in 1..=n {
        start_indices[i + 1] = start_indices[i] + (num_factors[i] / 2) as usize;
    }
    let total_size = start_indices[n + 1];
    let mut end_indices = start_indices.clone();
    let mut other_legs = vec![0i32; total_size];

    // Generate Pythagorean triples with c <= 2*N
    let m_limit = isqrt(2 * LIMIT);
    for m in 2..=m_limit {
        for nn in 1..m {
            if (m + nn) % 2 == 1 && gcd_i64(m, nn) == 1 {
                let a0 = (m * m - nn * nn) as i32;
                let b0 = (2 * m * nn) as i32;
                let c0 = (m * m + nn * nn) as i32;

                if c0 as i64 > 2 * LIMIT { break; }

                let mut k = 1i32;
                while (k as i64) * (c0 as i64) <= 2 * LIMIT {
                    let a = k * a0;
                    let b = k * b0;
                    let c = k * c0;

                    if (c as usize) <= n {
                        let au = a as usize;
                        let bu = b as usize;
                        // Store both directions for both orderings
                        if au <= n && end_indices[au] < start_indices[au + 1] {
                            other_legs[end_indices[au]] = b;
                            end_indices[au] += 1;
                        }
                        if bu <= n && end_indices[bu] < start_indices[bu + 1] {
                            other_legs[end_indices[bu]] = a;
                            end_indices[bu] += 1;
                        }
                        if bu <= n && end_indices[bu] < start_indices[bu + 1] {
                            other_legs[end_indices[bu]] = a;
                            end_indices[bu] += 1;
                        }
                        if au <= n && end_indices[au] < start_indices[au + 1] {
                            other_legs[end_indices[au]] = b;
                            end_indices[au] += 1;
                        }
                    }

                    k += 1;
                }
            }
        }
    }

    let mut ans: i64 = 0;

    // Process each even a
    let mut a = 2;
    while a <= n {
        let start = start_indices[a];
        let end = end_indices[a];
        if end > start {
            // Sort other legs
            other_legs[start..end].sort_unstable();

            // Find valid (b1, b2, b3) where 2*b1 + b2 = b3
            for i in start..end {
                let b1 = other_legs[i] as i64;
                let mut j = start;
                let mut kk = start + 1;

                while kk < end {
                    let b2 = other_legs[j] as i64;
                    let b3 = other_legs[kk] as i64;

                    if b2 > 2 * b1 { break; }

                    let target = 2 * b1 + b2;
                    if target < b3 {
                        j += 1;
                    } else if target > b3 {
                        kk += 1;
                    } else {
                        // Found valid combination
                        let perim = a as i64 + 2 * b1
                            + isqrt((a as i64) * (a as i64) + b2 * b2);
                        if b2 % 2 == 0 && perim <= LIMIT {
                            ans += perim;
                        }
                        j += 1;
                        kk += 1;
                    }
                }
            }
        }
        a += 2;
    }

    println!("{}", ans);
}
