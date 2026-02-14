// Project Euler 365: A huge binomial coefficient
//
// Compute sum(C(10^18, 10^9) mod p*q*r) for all primes 1000 < p < q < r < 5000,
// using Lucas' theorem and CRT.

use euler_utils::mod_inv;

const MAX_PRIME: usize = 5000;

fn sieve_primes() -> Vec<usize> {
    let mut sieve = vec![true; MAX_PRIME + 1];
    sieve[0] = false;
    sieve[1] = false;
    let mut i = 2;
    while i * i <= MAX_PRIME {
        if sieve[i] {
            let mut j = i * i;
            while j <= MAX_PRIME {
                sieve[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    (1001..MAX_PRIME).filter(|&i| sieve[i]).collect()
}

fn precompute_factorials(p: usize) -> (Vec<i64>, Vec<i64>) {
    let mut fact = vec![0i64; p];
    let mut inv_fact = vec![0i64; p];
    fact[0] = 1;
    for i in 1..p {
        fact[i] = fact[i - 1] * i as i64 % p as i64;
    }
    inv_fact[p - 1] = mod_inv(fact[p - 1] as u64, p as u64).unwrap() as i64;
    for i in (0..p - 1).rev() {
        inv_fact[i] = inv_fact[i + 1] * (i + 1) as i64 % p as i64;
    }
    (fact, inv_fact)
}

fn binomial_small(n: usize, k: usize, p: usize, fact: &[i64], inv_fact: &[i64]) -> i64 {
    if k > n || n >= p {
        return 0;
    }
    fact[n] * inv_fact[k] % p as i64 * inv_fact[n - k] % p as i64
}

fn lucas_mod(mut n: i64, mut k: i64, p: usize, fact: &[i64], inv_fact: &[i64]) -> i64 {
    if k < 0 || k > n { return 0; }
    if k == 0 { return 1; }
    let mut result = 1i64;
    while n > 0 || k > 0 {
        let nd = (n % p as i64) as usize;
        let kd = (k % p as i64) as usize;
        if kd > nd { return 0; }
        let c = binomial_small(nd, kd, p, fact, inv_fact);
        result = result * c % p as i64;
        n /= p as i64;
        k /= p as i64;
    }
    result
}

fn crt_three(a1: i64, m1: i64, a2: i64, m2: i64, a3: i64, m3: i64) -> i64 {
    let m12 = m1 as i128 * m2 as i128;
    let m = m12 * m3 as i128;

    let inv1 = mod_inv(m1 as u64, m2 as u64).unwrap() as i128;
    let x12 = ((a1 as i128 + m1 as i128 * (((a2 as i128 - a1 as i128) * inv1 % m2 as i128 + m2 as i128) % m2 as i128)) % m12 + m12) % m12;

    let inv12 = mod_inv((m12 % m3 as i128) as u64, m3 as u64).unwrap() as i128;
    let x = ((x12 + m12 * (((a3 as i128 - x12 % m3 as i128) * inv12 % m3 as i128 + m3 as i128) % m3 as i128)) % m + m) % m;

    x as i64
}

fn main() {
    let primes = sieve_primes();
    let np = primes.len();

    let n_val: i64 = 1_000_000_000_000_000_000; // 10^18
    let k_val: i64 = 1_000_000_000; // 10^9

    // Precompute factorials and Lucas values for each prime
    let mut lucas_vals = vec![0i64; np];
    let mut facts: Vec<Vec<i64>> = Vec::with_capacity(np);
    let mut inv_facts: Vec<Vec<i64>> = Vec::with_capacity(np);

    for i in 0..np {
        let (f, inv_f) = precompute_factorials(primes[i]);
        lucas_vals[i] = lucas_mod(n_val, k_val, primes[i], &f, &inv_f);
        facts.push(f);
        inv_facts.push(inv_f);
    }

    let mut total_sum: i64 = 0;
    for i in 0..np.saturating_sub(2) {
        let ap = lucas_vals[i];
        let p = primes[i] as i64;
        for j in i + 1..np.saturating_sub(1) {
            let aq = lucas_vals[j];
            let q = primes[j] as i64;
            for ki in j + 1..np {
                let ar = lucas_vals[ki];
                let r = primes[ki] as i64;
                total_sum += crt_three(ap, p, aq, q, ar, r);
            }
        }
    }

    println!("{}", total_sum);
}
