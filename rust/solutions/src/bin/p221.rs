// Project Euler 221: Alexandrian Integers
//
// A = a(a+d)(a+(a^2+1)/d) for each d | a^2+1 with d ≤ a.
// Factor a^2+1 by sieving p=2 and p≡1 (mod 4): p | a^2+1 iff a ≡ ±sqrt(-1) (mod p).
// After primes ≤ L the cofactor is 1 or a prime > L. min A > 4a^3+a so a ≤ 77806
// covers the 150000th (1.884e15).

const N_TARGET: usize = 150_000;
const L: usize = 78_000;
const MAX_OMEGA: usize = 8;
const AMAX: u128 = 3_000_000_000_000_000; // > 150000th; drops a^4-scale values

#[inline(always)]
fn mod_pow(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut r = 1u64;
    while exp > 0 {
        if exp & 1 == 1 {
            r = r * base % m;
        }
        base = base * base % m;
        exp >>= 1;
    }
    r
}

/// sqrt(-1) mod p for odd prime p ≡ 1 (mod 4).
#[inline(always)]
fn sqrt_neg1(p: u64) -> u64 {
    if p & 7 == 5 {
        return mod_pow(2, (p - 1) / 4, p);
    }
    let mut z = 3u64;
    while mod_pow(z, (p - 1) / 2, p) != p - 1 {
        z += 1;
    }
    mod_pow(z, (p - 1) / 4, p)
}

fn main() {
    let mut is_prime = vec![true; L + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut primes = Vec::with_capacity(8000);
    for i in 2..=L {
        if is_prime[i] {
            primes.push(i as u32);
            let mut j = i * i;
            while j <= L {
                is_prime[j] = false;
                j += i;
            }
        }
    }

    // fac[a*MAX_OMEGA..] = distinct primes p ≤ L dividing a^2+1
    let mut nfac = vec![0u8; L + 1];
    let mut fac = vec![0u32; (L + 1) * MAX_OMEGA];

    for &p in &primes {
        if p == 2 {
            let mut a = 1;
            while a <= L {
                let k = nfac[a] as usize;
                fac[a * MAX_OMEGA + k] = 2;
                nfac[a] += 1;
                a += 2;
            }
            continue;
        }
        if p & 3 != 1 {
            continue;
        }
        let pu = p as usize;
        let r = sqrt_neg1(p as u64) as usize;
        let mut a = r;
        while a <= L {
            let k = nfac[a] as usize;
            fac[a * MAX_OMEGA + k] = p;
            nfac[a] += 1;
            a += pu;
        }
        a = pu - r;
        while a <= L {
            let k = nfac[a] as usize;
            fac[a * MAX_OMEGA + k] = p;
            nfac[a] += 1;
            a += pu;
        }
    }

    let mut alex = Vec::with_capacity(200_000);
    let mut divs = [0i64; 256];

    for a in 1..=L {
        let a64 = a as i64;
        let n = a64 * a64 + 1;
        let nf = nfac[a] as usize;
        let base = a * MAX_OMEGA;

        divs[0] = 1;
        let mut nd = 1usize;
        let mut rem = n;
        for k in 0..nf {
            // SAFETY: nf = nfac[a] ≤ MAX_OMEGA, base = a * MAX_OMEGA
            let p = unsafe { *fac.get_unchecked(base + k) } as i64;
            let sz = nd;
            let mut mul = 1i64;
            while rem % p == 0 {
                rem /= p;
                mul *= p;
                for i in 0..sz {
                    let v = unsafe { *divs.get_unchecked(i) } * mul;
                    if v <= a64 {
                        unsafe { *divs.get_unchecked_mut(nd) = v };
                        nd += 1;
                    }
                }
            }
        }

        for i in 0..nd {
            let d = unsafe { *divs.get_unchecked(i) };
            let val = a64 as u128 * (a64 + d) as u128 * (a64 + n / d) as u128;
            if val <= AMAX {
                alex.push(val as i64);
            }
        }
    }

    // No duplicates among A ≤ AMAX for this L (complement of 4a^3 bound).
    let nth = *alex.select_nth_unstable(N_TARGET - 1).1;
    println!("{}", nth);
}
