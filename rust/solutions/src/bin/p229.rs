// Project Euler 229 - Four Representations using Squares
//
// Count n <= 2*10^9 representable as a^2+b^2, a^2+2b^2, a^2+3b^2, a^2+7b^2
// simultaneously (a,b > 0 for each). Process in chunks to save memory.

fn main() {
    let n_limit: u64 = 2_000_000_000;
    let chunk: u64 = 50_000_000;
    let ks = [1u64, 2, 3, 7];

    let mut total: u64 = 0;

    let mut base = 1u64;
    while base <= n_limit {
        let hi = std::cmp::min(base + chunk - 1, n_limit);
        let len = (hi - base + 1) as usize;
        let bytes = (len + 7) / 8;

        let mut combined = vec![0xFFu8; bytes];
        // Clear excess bits in last byte
        if len % 8 != 0 {
            combined[bytes - 1] = (1u8 << (len % 8)) - 1;
        }

        for &k in &ks {
            let mut temp = vec![0u8; bytes];

            // Mark all n = a^2 + k*b^2 in [base, hi] with a,b > 0
            let mut b = 1u64;
            while k * b * b < hi {
                let kb2 = k * b * b;
                let min_n = if kb2 + 1 < base { base } else { kb2 + 1 };
                if min_n > hi {
                    break;
                }

                let diff_lo = min_n - kb2;
                let mut a_lo = (diff_lo as f64).sqrt() as u64;
                if a_lo * a_lo < diff_lo {
                    a_lo += 1;
                }
                if a_lo < 1 {
                    a_lo = 1;
                }

                let diff_hi = hi - kb2;
                let mut a_hi = (diff_hi as f64).sqrt() as u64;
                while a_hi * a_hi > diff_hi {
                    a_hi -= 1;
                }

                for a in a_lo..=a_hi {
                    let n = a * a + kb2;
                    let idx = (n - base) as usize;
                    temp[idx >> 3] |= 1 << (idx & 7);
                }

                b += 1;
            }

            // AND with combined
            for i in 0..bytes {
                combined[i] &= temp[i];
            }
        }

        // Count set bits
        for &byte in &combined {
            total += byte.count_ones() as u64;
        }

        base += chunk;
    }

    println!("{}", total);
}
