// Project Euler 211 - Divisor Square Sum
//
// Sum of n < 64e6 with sigma_2(n) a perfect square.
// Odd-only SPF + sigma2 (half the arrays), then 2-adic extension in parallel.

use rayon::prelude::*;

const LIMIT: usize = 64_000_000;
const ODD_N: usize = (LIMIT + 1) / 2;

/// sigma_2(2^{k+1}) = (4^{k+2}-1)/3, indexed by k = trailing_zeros-1.
const fn geom2_table() -> [u64; 26] {
    let mut a = [0u64; 26];
    let mut g = 1u64;
    let mut k = 0;
    while k < 26 {
        g = 4 * g + 1;
        a[k] = g;
        k += 1;
    }
    a
}
const GEOM2: [u64; 26] = geom2_table();

const fn qr256() -> [u8; 256] {
    let mut t = [0u8; 256];
    let mut i = 0u32;
    while i < 256 {
        t[((i * i) & 255) as usize] = 1;
        i += 1;
    }
    t
}
const QR256: [u8; 256] = qr256();

const fn qr4095() -> [u8; 4095] {
    let mut t = [0u8; 4095];
    let mut i = 0u32;
    while i < 4095 {
        t[(((i as u64) * (i as u64)) % 4095) as usize] = 1;
        i += 1;
    }
    t
}
const QR4095: [u8; 4095] = qr4095();

#[inline(always)]
fn is_square(n: u64) -> bool {
    if QR256[(n & 255) as usize] == 0 {
        return false;
    }
    if QR4095[(n % 4095) as usize] == 0 {
        return false;
    }
    let r = n.isqrt();
    r * r == n
}

fn main() {
    // spf[i] = smallest prime factor of odd (2i+1); 0 if prime (or 1).
    let mut spf = vec![0u16; ODD_N];

    unsafe {
        let ptr = spf.as_mut_ptr();
        let mut i = 1usize;
        while {
            let n = 2 * i + 1;
            n * n < LIMIT
        } {
            if *ptr.add(i) == 0 {
                let p = 2 * i + 1;
                let step = p << 1;
                let mut k = p * p;
                while k < LIMIT {
                    let idx = k >> 1;
                    if *ptr.add(idx) == 0 {
                        *ptr.add(idx) = p as u16;
                    }
                    k += step;
                }
            }
            i += 1;
        }
    }

    let mut sig2 = vec![0u64; ODD_N];
    sig2[0] = 1;

    unsafe {
        for i in 1..ODD_N {
            let n = 2 * i + 1;
            if n >= LIMIT {
                break;
            }
            let p = *spf.get_unchecked(i) as usize;
            let s = if p == 0 {
                1 + (n as u64) * (n as u64)
            } else {
                let p2 = (p as u64) * (p as u64);
                let mut m = n / p;
                let mut g = 1 + p2;
                while m % p == 0 {
                    m /= p;
                    g = g * p2 + 1;
                }
                *sig2.get_unchecked(m >> 1) * g
            };
            *sig2.get_unchecked_mut(i) = s;
        }
    }

    const CHUNK: usize = 1 << 16;
    let ans: u64 = (0..ODD_N)
        .into_par_iter()
        .with_min_len(CHUNK)
        .map(|i| {
            let n = 2 * i + 1;
            if n >= LIMIT {
                return 0u64;
            }
            let s = unsafe { *sig2.get_unchecked(i) };
            let mut local = 0u64;
            if is_square(s) {
                local += n as u64;
            }
            // 2^e * odd, e >= 1
            let mut v = n << 1;
            let mut e = 1usize;
            while v < LIMIT {
                if is_square(s * unsafe { *GEOM2.get_unchecked(e - 1) }) {
                    local += v as u64;
                }
                v <<= 1;
                e += 1;
            }
            local
        })
        .sum();

    println!("{}", ans);
}
