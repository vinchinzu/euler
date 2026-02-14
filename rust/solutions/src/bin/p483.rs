// Project Euler 483 - Repeated permutation
// Average f^2(P) over all permutations of {1..350}

use std::collections::HashMap;

fn gcd_ll(mut a: i64, mut b: i64) -> i64 {
    while b != 0 { let t = b; b = a % b; a = t; }
    a
}

fn lcm_ll(a: i64, b: i64) -> i64 {
    a / gcd_ll(a, b) * b
}

fn main() {
    let n = 350;

    // Sieve primes up to n
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=n {
        if is_prime[i] {
            let mut j = i * i;
            while j <= n { is_prime[j] = false; j += i; }
        }
    }
    let primes: Vec<usize> = (2..=n).filter(|&i| is_prime[i]).collect();

    // Largest prime factor
    let mut largest_pf = vec![0usize; n + 1];
    for &p in &primes {
        let mut i = p;
        while i <= n { largest_pf[i] = p; i += p; }
    }

    // Factorials as f64
    let mut ffact = vec![0.0f64; n + 1];
    ffact[0] = 1.0;
    for i in 1..=n { ffact[i] = ffact[i - 1] * i as f64; }

    // Memoized recursion
    type CacheKey = (usize, usize, usize, i64);
    let mut cache: HashMap<CacheKey, f64> = HashMap::new();

    fn sum_f2(
        max_index: usize, min_k: usize, n: usize, lcm: i64,
        primes: &[usize], largest_pf: &[usize], ffact: &[f64],
        cache: &mut HashMap<CacheKey, f64>,
    ) -> f64 {
        let key = (max_index, min_k, n, lcm);
        if let Some(&v) = cache.get(&key) { return v; }

        let mut relevant_lcm = lcm;
        let mut scale = 1i64;
        let mut result = lcm as f64 * lcm as f64 / ffact[n];

        // Iterate over prime indices from max_index down to 0
        // We use wrapping arithmetic since max_index could be 0
        let mut index = max_index as isize;
        while index >= 0 {
            let idx = index as usize;
            let p = primes[idx];
            let start_k = if idx == max_index { min_k } else { 1 };
            let mut k = start_k;
            while k * p <= n {
                if largest_pf[k] <= p {
                    let cycle_len = k * p;
                    let new_lcm = lcm_ll(relevant_lcm, cycle_len as i64);
                    let mut pow_cl = cycle_len as f64;
                    let mut mult = 1;
                    while mult * cycle_len <= n {
                        let sub = sum_f2(idx, k + 1, n - mult * cycle_len, new_lcm,
                                        primes, largest_pf, ffact, cache);
                        result += sub * scale as f64 * scale as f64 / pow_cl / ffact[mult];
                        pow_cl *= cycle_len as f64;
                        mult += 1;
                    }
                }
                k += 1;
            }
            while relevant_lcm % p as i64 == 0 {
                relevant_lcm /= p as i64;
                scale *= p as i64;
            }
            index -= 1;
        }

        cache.insert(key, result);
        result
    }

    let ans = sum_f2(primes.len() - 1, 1, n, 1, &primes, &largest_pf, &ffact, &mut cache);

    // Format like C: "%.9e" without '+' in exponent
    let s = format!("{:.9e}", ans);
    let s = s.replace("e+", "e").replace("e0", "e").replace("E+", "E").replace("E0", "E");
    // More carefully: just remove '+' signs
    let s = format!("{:.9e}", ans);
    let s: String = s.chars().filter(|&c| c != '+').collect();
    println!("{}", s);
}
