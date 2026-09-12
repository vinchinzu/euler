// Project Euler 495 - Writing n! as product of k distinct integers
// Inclusion-exclusion over partitions of K=30.

use rayon::prelude::*;

const N: usize = 10000;
const K: usize = 30;
const MOD: u64 = 1_000_000_007;

fn vp_factorial(n: usize, p: usize) -> usize {
    let mut count = 0;
    let mut pw = p as u64;
    while pw <= n as u64 {
        count += n / pw as usize;
        pw *= p as u64;
    }
    count
}

fn gen_partitions(min_val: u8, remaining: u8, cur: &mut Vec<u8>, out: &mut Vec<Vec<u8>>) {
    if remaining == 0 {
        out.push(cur.clone());
        return;
    }
    for c in min_val..=remaining {
        cur.push(c);
        gen_partitions(c, remaining - c, cur, out);
        cur.pop();
    }
}

fn main() {
    let mut is_prime = vec![true; N + 1];
    is_prime[0] = false;
    if N >= 1 {
        is_prime[1] = false;
    }
    let mut i = 2;
    while i * i <= N {
        if is_prime[i] {
            let mut j = i * i;
            while j <= N {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    let primes: Vec<usize> = (2..=N).filter(|&i| is_prime[i]).collect();
    let nprimes = primes.len();

    let exponents: Vec<usize> = primes.iter().map(|&p| vp_factorial(N, p)).collect();

    let mut inv_val = vec![0u64; K + 1];
    inv_val[1] = 1;
    for i in 2..=K {
        inv_val[i] = (MOD - MOD / i as u64) % MOD * inv_val[(MOD % i as u64) as usize] % MOD;
    }

    let mut fact = 1u64;
    for i in 1..=K {
        fact = fact * i as u64 % MOD;
    }
    let mut inv_fact = vec![0u64; K + 1];
    {
        let mut base = fact;
        let mut exp = MOD - 2;
        let mut result = 1u64;
        while exp > 0 {
            if exp & 1 == 1 {
                result = result * base % MOD;
            }
            base = base * base % MOD;
            exp >>= 1;
        }
        inv_fact[K] = result;
    }
    for i in (0..K).rev() {
        inv_fact[i] = inv_fact[i + 1] * (i + 1) as u64 % MOD;
    }

    let mut partitions = Vec::with_capacity(6000);
    let mut cur = Vec::with_capacity(K);
    gen_partitions(1, K as u8, &mut cur, &mut partitions);

    let ans = partitions
        .par_iter()
        .fold(
            || (vec![0u64; N + 1], 0u64),
            |(mut dp, acc), part| {
                dp.fill(0);
                dp[0] = 1;
                for &coeff in part.iter() {
                    let c = coeff as usize;
                    for e in c..=N {
                        let s = dp[e] + dp[e - c];
                        dp[e] = if s >= MOD { s - MOD } else { s };
                    }
                }
                let mut res = 1u64;
                for i in 0..nprimes {
                    res = res * dp[exponents[i]] % MOD;
                }
                for &c in part.iter() {
                    if c % 2 == 0 {
                        res = res * (MOD - 1) % MOD;
                    }
                    res = res * inv_val[c as usize] % MOD;
                }
                let depth = part.len();
                let mut i = 0;
                while i < depth {
                    let mut j = i;
                    while j < depth && part[j] == part[i] {
                        j += 1;
                    }
                    res = res * inv_fact[j - i] % MOD;
                    i = j;
                }
                (dp, acc + res)
            },
        )
        .map(|(_, acc)| acc)
        .reduce(|| 0, |a, b| a + b)
        % MOD;

    println!("{}", ans);
}
