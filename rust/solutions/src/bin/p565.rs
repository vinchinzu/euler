// Project Euler 565 - Divisor Sum Divisibility
//
// Find sum of all integers n <= N=10^11 such that sigma(n) % 2017 == 0.
// Uses inclusion-exclusion over prime power bases.

use euler_utils::primes_up_to;

const MAXN: i64 = 100_000_000_000;
const KK: i64 = 2017;

struct Base {
    p: i64,
    #[allow(dead_code)]
    e: i32,
    pe: i64,
}

fn tr(n: i64) -> i128 {
    let nn = n as i128;
    nn * (nn + 1) / 2
}

fn mod_inv_i64(a: i64, m: i64) -> i64 {
    let mut t: i64 = 0;
    let mut new_t: i64 = 1;
    let mut r: i64 = m;
    let mut new_r: i64 = a % m;
    while new_r != 0 {
        let q = r / new_r;
        let tmp = new_t;
        new_t = t - q * new_t;
        t = tmp;
        let tmp = new_r;
        new_r = r - q * new_r;
        r = tmp;
    }
    if t < 0 {
        t += m;
    }
    t
}

fn main() {
    let nn = MAXN;
    let sieve_limit = (nn as f64).sqrt() as usize + 1;
    let primes = primes_up_to(sieve_limit);

    let sieve_size = (nn / KK + 1) as usize;
    let mut sieve2 = vec![true; sieve_size];

    for &p in &primes {
        let p = p as i64;
        if p == KK {
            continue;
        }
        let inv = mod_inv_i64(KK, p) as usize;
        let mut i = inv;
        while i < sieve_size {
            let val = i as i64 * KK - 1;
            if p != val {
                sieve2[i] = false;
            }
            i += p as usize;
        }
    }

    let mut bases: Vec<Base> = Vec::with_capacity(3_000_000);

    for ii in 1..sieve_size {
        if sieve2[ii] {
            let p = ii as i64 * KK - 1;
            bases.push(Base { p, e: 1, pe: p });
        }
    }

    for &p in &primes {
        let p = p as i64;
        let mut sum_div: i64 = 1 + p;
        let mut e = 2;
        let mut pe = p * p;
        while pe <= nn {
            sum_div = sum_div * p + 1;
            if sum_div % KK == 0 {
                bases.push(Base { p, e, pe });
            }
            e += 1;
            if pe > nn / p {
                break;
            }
            pe *= p;
        }
    }

    bases.sort_by_key(|b| b.pe);

    let mut ans: i128 = 0;

    fn helper(min_index: usize, parity: i128, n: i64, bases: &[Base], nn: i64, ans: &mut i128) {
        if n > 1 {
            let q = nn / n;
            let t = tr(q);
            *ans += parity * n as i128 * t;
        }
        for i in min_index..bases.len() {
            let pe = bases[i].pe;
            if pe > nn / n {
                break;
            }
            let npe = n * pe;
            helper(i + 1, -parity, npe, bases, nn, ans);

            let pe1 = pe * bases[i].p;
            if pe1 <= nn / n {
                helper(i + 1, parity, n * pe1, bases, nn, ans);
            }
        }
    }

    helper(0, -1, 1, &bases, nn, &mut ans);

    println!("{}", ans as i64);
}
