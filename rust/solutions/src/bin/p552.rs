// Project Euler 552 - Chinese Remainder Theorem / Garner's Algorithm
//
// For each prime p_i, check if any partial CRT reconstruction A_n (n < i)
// is divisible by p_i.

use euler_utils::primes_up_to;

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
    let primes_list: Vec<u64> = primes_up_to(n).into_iter().map(|p| p as u64).collect();
    let l = primes_list.len();

    let mut garner = vec![0u64; l];
    let mut ans: u64 = 0;

    for i in 0..l {
        let p = primes_list[i];
        // prod and a stay in [0, p). Products prod * garner[j] and
        // prod * primes_list[j] are < 300_000^2, so they fit in u64.
        let mut prod: u64 = 1;
        let mut a: u64 = 0;
        let mut good = false;

        for j in 0..i {
            // j < i => primes_list[j] < p and garner[j] < primes_list[j] < p,
            // so `% p` on those values is the identity.
            a = (a + prod * garner[j]) % p;
            prod = prod * primes_list[j] % p;
            if a == 0 && j > 0 {
                good = true;
            }
        }

        // Compute garner[i]
        if prod != 0 {
            let need = (i as u64 + 1 + p - a) % p;
            let inv = power(prod, p - 2, p);
            garner[i] = need * inv % p;
        } else {
            garner[i] = 0;
        }

        if good {
            ans += p;
        }
    }

    println!("{ans}");
}
