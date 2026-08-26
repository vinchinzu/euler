// Project Euler 136: Singleton difference.
// n = a(4d-a) has exactly one solution iff n is 4, 16, an odd prime p ≡ 3
// (mod 4), 4p for an odd prime p, or 16p for an odd prime p.

const LIMIT: usize = 50_000_000;

fn main() {
    // Odd-only bit sieve: bit i corresponds to 2*i+1. 1 = composite.
    let n_odd = LIMIT / 2;
    let mut is_comp = vec![0u64; (n_odd + 63) / 64];
    is_comp[0] |= 1; // 1 is not prime

    let mut p = 3usize;
    while p * p < LIMIT {
        let i = p >> 1;
        if (is_comp[i >> 6] >> (i & 63)) & 1 == 0 {
            let mut q = p * p;
            let step = p * 2;
            while q < LIMIT {
                let j = q >> 1;
                is_comp[j >> 6] |= 1u64 << (j & 63);
                q += step;
            }
        }
        p += 2;
    }

    let mut ans: u64 = 2; // n = 4 and n = 16
    let mut n = 3usize;
    while n < LIMIT {
        let i = n >> 1;
        if (is_comp[i >> 6] >> (i & 63)) & 1 == 0 {
            if n % 4 == 3 {
                ans += 1;
            }
            if n < LIMIT / 4 {
                ans += 1;
            }
            if n < LIMIT / 16 {
                ans += 1;
            }
        }
        n += 2;
    }
    println!("{}", ans);
}
