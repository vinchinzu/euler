// Project Euler 211 - Divisor Square Sum
//
// Sum of n < 64e6 with sigma_2(n) a perfect square.
// sigma_2 is multiplicative: sigma_2(p^k) = (p^{2(k+1)}-1)/(p^2-1).
// u16 SPF for p <= sqrt(N), then sequential fill + residue-filtered integer square test.

const LIMIT: usize = 64_000_000;

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

// 4095 = 5*7*9*13, coprime to 256. Combined pass rate ~1.4%.
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
    // 0 = prime (or 0/1). Nonzero = smallest prime factor <= sqrt(LIMIT-1) < 2^16.
    let mut spf = vec![0u16; LIMIT];

    unsafe {
        let ptr = spf.as_mut_ptr();
        // Evens > 2 have spf 2.
        let mut j = 4usize;
        while j < LIMIT {
            *ptr.add(j) = 2;
            j += 2;
        }
        // Odd primes p <= sqrt: mark odd multiples from p^2.
        let mut i = 3usize;
        while i * i < LIMIT {
            if *ptr.add(i) == 0 {
                let step = i << 1;
                let mut k = i * i;
                while k < LIMIT {
                    if *ptr.add(k) == 0 {
                        *ptr.add(k) = i as u16;
                    }
                    k += step;
                }
            }
            i += 2;
        }
    }

    let mut sig2 = vec![0u64; LIMIT];
    sig2[1] = 1;
    let mut ans: u64 = 1; // n = 1

    // SAFETY: i in 2..LIMIT; m = i / p^e < i; arrays have length LIMIT.
    unsafe {
        for i in 2..LIMIT {
            let p = *spf.get_unchecked(i) as usize;
            let s = if p == 0 {
                // i is prime (any n < LIMIT with no prime factor <= sqrt is prime).
                1 + (i as u64) * (i as u64)
            } else if p == 2 {
                let e = i.trailing_zeros() as usize;
                *sig2.get_unchecked(i >> e) * *GEOM2.get_unchecked(e - 1)
            } else {
                let p2 = (p as u64) * (p as u64);
                let mut m = i / p;
                let mut g = 1 + p2;
                while m % p == 0 {
                    m /= p;
                    g = g * p2 + 1;
                }
                *sig2.get_unchecked(m) * g
            };
            *sig2.get_unchecked_mut(i) = s;
            // 1+p^2 is never square for prime p, so skip p == 0.
            if p != 0 && is_square(s) {
                ans += i as u64;
            }
        }
    }

    println!("{}", ans);
}
