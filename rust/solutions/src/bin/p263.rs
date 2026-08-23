// Project Euler 263: An engineers' dream come true
//
// Find the sum of the first 4 engineers' paradise numbers n where:
// - n-9, n-3, n+3, n+9 are consecutive primes (sexy prime quadruplet)
// - n-8, n-4, n, n+4, n+8 are practical numbers
//
// n = 840*i ± 20. Local SPRP (2,7,61), small-prime wheel, rayon chunks.

use rayon::prelude::*;

/// Deterministic SPRP witnesses for n < 4_759_123_141.
const MR_WITNESSES: [u64; 3] = [2, 7, 61];

/// Wheel of primes that commonly kill a sexy quadruplet.
const WHEEL_PS: [u64; 5] = [11, 13, 17, 19, 23];
const WHEEL_MOD: u64 = 11 * 13 * 17 * 19 * 23; // 1_062_347

#[inline(always)]
fn mul_mod(a: u64, b: u64, m: u64) -> u64 {
    // Candidates stay below ~2.2e9, so a*b fits in u64.
    a.wrapping_mul(b) % m
}

#[inline(always)]
fn pow_mod(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut r = 1u64;
    while exp > 0 {
        if exp & 1 == 1 {
            r = mul_mod(r, base, m);
        }
        base = mul_mod(base, base, m);
        exp >>= 1;
    }
    r
}

#[inline(always)]
fn sprp(n: u64, a: u64) -> bool {
    if n % a == 0 {
        return n == a;
    }
    let mut d = n - 1;
    let r = d.trailing_zeros();
    d >>= r;
    let mut x = pow_mod(a, d, n);
    if x == 1 || x == n - 1 {
        return true;
    }
    for _ in 1..r {
        x = mul_mod(x, x, n);
        if x == n - 1 {
            return true;
        }
    }
    false
}

/// Primality for n >= 20. Tiny primes 2,3,5,7 are already excluded on the
/// sexy-quad residues; still test 11..53 before SPRP.
#[inline(always)]
fn is_prime_large(n: u64) -> bool {
    if n % 11 == 0
        || n % 13 == 0
        || n % 17 == 0
        || n % 19 == 0
        || n % 23 == 0
        || n % 29 == 0
        || n % 31 == 0
        || n % 37 == 0
        || n % 41 == 0
        || n % 43 == 0
        || n % 47 == 0
        || n % 53 == 0
    {
        return false;
    }
    sprp(n, MR_WITNESSES[0]) && sprp(n, MR_WITNESSES[1]) && sprp(n, MR_WITNESSES[2])
}

fn sieve_primes(limit: u32) -> Vec<u32> {
    let n = limit as usize;
    let mut mark = vec![true; n + 1];
    mark[0] = false;
    mark[1] = false;
    let mut p = 2usize;
    while p * p <= n {
        if mark[p] {
            let mut m = p * p;
            while m <= n {
                mark[m] = false;
                m += p;
            }
        }
        p += 1;
    }
    (2..=n).filter(|&i| mark[i]).map(|i| i as u32).collect()
}

fn mod_inv(a: u64, m: u64) -> u64 {
    let mut t = 0i64;
    let mut newt = 1i64;
    let mut r = m as i64;
    let mut newr = (a % m) as i64;
    while newr != 0 {
        let q = r / newr;
        (t, newt) = (newt, t - q * newt);
        (r, newr) = (newr, r - q * newr);
    }
    if t < 0 {
        t += m as i64;
    }
    t as u64
}

/// Bit0 = n=840i+20 may have a prime quadruplet; bit1 = n=840i-20.
fn build_wheel() -> Vec<u8> {
    let w = WHEEL_MOD as usize;
    let mut ok = vec![3u8; w];
    // Offsets from 840*i that must be prime for each sign.
    let plus_off: [i64; 4] = [11, 17, 23, 29];
    let minus_off: [i64; 4] = [-29, -23, -17, -11];
    for &p in &WHEEL_PS {
        let inv = mod_inv(840 % p, p);
        for &off in &plus_off {
            let offm = ((off % p as i64) + p as i64) as u64 % p;
            let i0 = ((p - offm) % p) * inv % p;
            let mut i = i0;
            while i < WHEEL_MOD {
                ok[i as usize] &= !1;
                i += p;
            }
        }
        for &off in &minus_off {
            let offm = ((off % p as i64) + p as i64) as u64 % p;
            let i0 = ((p - offm) % p) * inv % p;
            let mut i = i0;
            while i < WHEEL_MOD {
                ok[i as usize] &= !2;
                i += p;
            }
        }
    }
    ok
}

#[inline]
fn is_practical(n: u64, primes: &[u32]) -> bool {
    if n & 1 == 1 {
        return n == 1;
    }
    let mut tmp = n;
    let tz = tmp.trailing_zeros();
    tmp >>= tz;
    let mut sigma = (2u64 << tz) - 1;

    for &p in primes.iter().skip(1) {
        let p = p as u64;
        if p * p > tmp {
            break;
        }
        if tmp % p == 0 {
            if p > sigma + 1 {
                return false;
            }
            let mut pw = 1u64;
            while tmp % p == 0 {
                tmp /= p;
                pw = pw.wrapping_mul(p);
            }
            sigma *= (pw * p - 1) / (p - 1);
        }
    }
    tmp <= 1 || tmp <= sigma + 1
}

#[inline]
fn is_paradise(n: u64, primes: &[u32]) -> bool {
    if !is_prime_large(n - 9)
        || !is_prime_large(n - 3)
        || !is_prime_large(n + 3)
        || !is_prime_large(n + 9)
    {
        return false;
    }

    // Of the six in-between odd slots, four are always divisible by 3 or 5.
    if n % 840 == 20 {
        if is_prime_large(n - 7) || is_prime_large(n - 1) {
            return false;
        }
    } else if is_prime_large(n + 1) || is_prime_large(n + 7) {
        return false;
    }

    is_practical(n - 8, primes)
        && is_practical(n - 4, primes)
        && is_practical(n, primes)
        && is_practical(n + 4, primes)
        && is_practical(n + 8, primes)
}

fn hits_for_i(i: u64, primes: &[u32], wheel: &[u8]) -> impl Iterator<Item = u64> {
    let bits = wheel[(i % WHEEL_MOD) as usize];
    let n_minus = 840 * i - 20;
    let n_plus = 840 * i + 20;
    let a = (bits & 2) != 0 && n_minus >= 20 && is_paradise(n_minus, primes);
    let b = (bits & 1) != 0 && is_paradise(n_plus, primes);
    [a.then_some(n_minus), b.then_some(n_plus)]
        .into_iter()
        .flatten()
}

fn main() {
    let primes = sieve_primes(50_000);
    let wheel = build_wheel();

    const CHUNK: u64 = 16_384;
    let mut found: Vec<u64> = Vec::with_capacity(4);
    let mut start = 1u64;

    while found.len() < 4 {
        let end = start + CHUNK;
        let mut hits: Vec<u64> = (start..end)
            .into_par_iter()
            .flat_map_iter(|i| hits_for_i(i, &primes, &wheel))
            .collect();
        if !hits.is_empty() {
            hits.sort_unstable();
            found.extend(hits);
        }
        start = end;
    }

    found.sort_unstable();
    found.truncate(4);
    println!("{}", found.iter().sum::<u64>());
}
