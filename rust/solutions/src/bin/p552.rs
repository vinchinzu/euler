// Project Euler 552 - Chinese Remainder Theorem / Garner's Algorithm
//
// For each prime p_i, check if any partial CRT reconstruction A_n (n < i)
// is divisible by p_i.

use euler_utils::primes_up_to;

#[derive(Clone, Copy)]
struct Item {
    garner: u32,
    prime: u32,
}

#[inline(always)]
fn barrett(x: u64, p: u64, m: u64) -> u64 {
    let q = ((x as u128 * m as u128) >> 64) as u64;
    let mut r = x - q * p;
    if r >= p {
        r -= p;
    }
    r
}

// Since prod * prime is never divisible by prime p (prod != 0 and prime < p),
// r is strictly less than p, so the conditional subtraction is unnecessary.
#[inline(always)]
fn barrett_prod(x: u64, p: u64, m: u64) -> u64 {
    let q = ((x as u128 * m as u128) >> 64) as u64;
    x - q * p
}

fn power(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result: u64 = 1;
    base %= modulus;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % modulus;
        }
        base = base * base % modulus;
        exp >>= 1;
    }
    result
}

fn main() {
    let n = 300_000;
    let primes = primes_up_to(n);
    let l = primes.len();
    let mut items: Vec<Item> = primes
        .into_iter()
        .map(|p| Item {
            garner: 0,
            prime: p as u32,
        })
        .collect();

    let mut ans: u64 = 0;

    for i in 0..l {
        let p = items[i].prime as u64;
        let m = ((1u128 << 64) / p as u128) as u64;

        let mut prod: u64 = 1;
        let mut a: u64 = 0;
        let mut good = false;

        for item in &items[..i] {
            a = barrett(a + prod * item.garner as u64, p, m);
            prod = barrett_prod(prod * item.prime as u64, p, m);
            if a == 0 {
                good = true;
            }
        }

        // Compute garner[i]
        if prod != 0 {
            let need = (i as u64 + 1 + p - a) % p;
            let inv = power(prod, p - 2, p);
            items[i].garner = (need * inv % p) as u32;
        } else {
            items[i].garner = 0;
        }

        if good {
            ans += p;
        }
    }

    println!("{ans}");
}
