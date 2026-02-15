// Project Euler 417: Reciprocal cycles II
//
// Compute sum of L(n) for n=3..10^8, where L(n) is the length of the
// repeating cycle in the decimal expansion of 1/n.

use euler_utils::mod_pow;
const NMAX: usize = 100_000_001;

fn main() {
    let n: usize = 100_000_000;

    // Sieve smallest prime factors
    let mut spf = vec![0u32; NMAX];
    {
        let mut i = 2usize;
        while i * i <= n {
            if spf[i] == 0 {
                let mut j = i * i;
                while j <= n {
                    if spf[j] == 0 {
                        spf[j] = i as u32;
                    }
                    j += i;
                }
            }
            i += 1;
        }
    }

    // Compute ord_10(p) for all primes p != 2, 5
    let mut ord10 = vec![0u32; NMAX];
    for p in 3..=n {
        if spf[p] != 0 || p == 2 || p == 5 {
            continue;
        }

        // Factor p-1 using spf
        let mut result = (p - 1) as u32;
        let mut temp = p - 1;
        while temp > 1 {
            let q = if spf[temp] == 0 { temp as u32 } else { spf[temp] };

            while temp % q as usize == 0 {
                temp /= q as usize;
            }
            while result % q == 0 {
                // Use 64-bit for intermediate products
                let pow = pow_mod_32(10, result / q, p as u32);
                if pow == 1 {
                    result /= q;
                } else {
                    break;
                }
            }
        }
        ord10[p] = result;
    }

    // Compute sum of L(n) for n = 3..N
    let mut total: i64 = 0;
    for nn in 3..=n {
        let mut temp = nn;
        while temp % 2 == 0 { temp /= 2; }
        while temp % 5 == 0 { temp /= 5; }
        if temp <= 1 { continue; }

        // If temp is prime, use cached order
        if spf[temp] == 0 {
            total += ord10[temp] as i64;
            continue;
        }

        // Factor temp, compute LCM of L(p^e)
        let mut result: i64 = 0;
        let mut t = temp;
        while t > 1 {
            let p = if spf[t] == 0 { t as u32 } else { spf[t] };

            let mut e = 0u32;
            while t % p as usize == 0 {
                t /= p as usize;
                e += 1;
            }

            let lp = ord10[p as usize] as i64;
            if lp == 0 { continue; }

            let mut lpe = lp;
            if e >= 2 {
                let pp = p as u64 * p as u64;
                if mod_pow(10, lp as u64, pp) == 1 {
                    let mut e0 = 2u32;
                    let mut ppow = pp;
                    for i in 3..=e {
                        ppow *= p as u64;
                        if mod_pow(10, lp as u64, ppow) == 1 {
                            e0 = i;
                        } else {
                            break;
                        }
                    }
                    for _ in e0..e {
                        lpe *= p as i64;
                    }
                } else {
                    for _ in 1..e {
                        lpe *= p as i64;
                    }
                }
            }
            result = lcm_i64(result, lpe);
        }
        total += result;
    }

    println!("{}", total);
}

fn pow_mod_32(mut base: u32, mut exp: u32, modv: u32) -> u32 {
    let mut result = 1u64;
    let mut b = base as u64 % modv as u64;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * b % modv as u64;
        }
        b = b * b % modv as u64;
        exp >>= 1;
    }
    result as u32
}

fn gcd_i64(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm_i64(a: i64, b: i64) -> i64 {
    if a == 0 { return b; }
    if b == 0 { return a; }
    a / gcd_i64(a, b) * b
}
