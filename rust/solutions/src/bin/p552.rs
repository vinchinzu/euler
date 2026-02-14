// Project Euler 552 - Chinese Remainder Theorem / Garner's Algorithm
//
// For each prime p_i, check if any partial CRT reconstruction A_n (n < i)
// is divisible by p_i.

use euler_utils::primes_up_to;

fn power(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
    let mut result: i64 = 1;
    base %= modulus;
    while exp > 0 {
        if exp & 1 == 1 { result = result * base % modulus; }
        base = base * base % modulus;
        exp >>= 1;
    }
    result
}

fn main() {
    let n = 300_000;
    let primes_list: Vec<i64> = primes_up_to(n).into_iter().map(|p| p as i64).collect();
    let l = primes_list.len();

    let mut garner = vec![0i64; l];
    let mut ans: i64 = 0;

    for i in 0..l {
        let p = primes_list[i];
        let mut prod: i64 = 1;
        let mut a: i64 = 0;
        let mut good = false;

        for j in 0..i {
            a = (a + prod % p * (garner[j] % p)) % p;
            prod = prod % p * (primes_list[j] % p) % p;
            if a == 0 && j > 0 {
                good = true;
            }
        }

        // Compute garner[i]
        if prod % p != 0 {
            let need = ((i as i64 + 1 - a) % p + p) % p;
            let inv = power(prod % p, p - 2, p);
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
