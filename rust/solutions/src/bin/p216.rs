// Project Euler 216 - Investigating the Primality of 2n^2-1
//
// Count primes of the form 2n^2 - 1 for 2 <= n <= 50,000,000.
// Sieve approach using Tonelli-Shanks for square root mod p.

fn mulmod(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128) * (b as u128) % (m as u128)) as u64
}

fn powmod(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut result = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = mulmod(result, base, m);
        }
        base = mulmod(base, base, m);
        exp >>= 1;
    }
    result
}

fn sqrt_mod(n: u64, p: u64) -> u64 {
    if p % 4 == 3 {
        return powmod(n, (p + 1) / 4, p);
    }
    let mut q = p - 1;
    let mut s = 0u32;
    while q % 2 == 0 {
        q /= 2;
        s += 1;
    }
    let mut z = 2u64;
    while powmod(z, (p - 1) / 2, p) + 1 != p {
        z += 1;
    }
    let mut m = s;
    let mut c = powmod(z, q, p);
    let mut t = powmod(n, q, p);
    let mut r = powmod(n, (q + 1) / 2, p);
    while t != 1 {
        let mut i = 1u32;
        let mut tmp = mulmod(t, t, p);
        while tmp != 1 {
            tmp = mulmod(tmp, tmp, p);
            i += 1;
        }
        let b = powmod(c, 1u64 << (m - i - 1), p);
        m = i;
        c = mulmod(b, b, p);
        t = mulmod(t, c, p);
        r = mulmod(r, b, p);
    }
    r
}

fn main() {
    let n_max: usize = 50_000_000;
    let l = ((2.0f64).sqrt() * n_max as f64) as usize;

    // Sieve primes up to l
    let mut is_prime = vec![true; l + 1];
    is_prime[0] = false;
    if l >= 1 {
        is_prime[1] = false;
    }
    let mut i = 2;
    while i * i <= l {
        if is_prime[i] {
            let mut j = i * i;
            while j <= l {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }

    // Sieve for 2n^2-1 primality
    let mut sieve_arr = vec![true; n_max + 1];
    sieve_arr[0] = false;
    if n_max >= 1 {
        sieve_arr[1] = false;
    }

    for p in (3..=l).filter(|&p| is_prime[p]) {
        let p64 = p as u64;
        if p64 % 8 != 1 && p64 % 8 != 7 {
            continue;
        }
        let half = (p64 + 1) / 2;
        let r = sqrt_mod(half, p64);
        let start1 = if 2 * r * r - 1 == p64 { r + p64 } else { r };
        let mut ii = start1 as usize;
        while ii <= n_max {
            sieve_arr[ii] = false;
            ii += p;
        }
        let r2 = p64 - r;
        let start2 = if 2 * r2 * r2 - 1 == p64 {
            r2 + p64
        } else {
            r2
        };
        if start2 > 0 {
            let mut ii = start2 as usize;
            while ii <= n_max {
                sieve_arr[ii] = false;
                ii += p;
            }
        }
    }

    let ans: usize = (2..=n_max).filter(|&i| sieve_arr[i]).count();
    println!("{}", ans);
}
