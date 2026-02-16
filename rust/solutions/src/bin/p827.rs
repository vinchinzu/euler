// Project Euler 827 - Pythagorean Triple Occurrence
// Q(n) = smallest number occurring in exactly n Pythagorean triples.
// Find sum_{k=1}^{18} Q(10^k) mod 409120391.

use std::collections::HashMap;

const MOD: i64 = 409120391;

fn pow_mod(base: i64, mut exp: i64, m: i64) -> i64 {
    let mut result = 1i128;
    let mut b = (base % m) as i128;
    let m128 = m as i128;
    while exp > 0 {
        if exp & 1 == 1 { result = result * b % m128; }
        b = b * b % m128;
        exp >>= 1;
    }
    result as i64
}

struct PrimeData {
    primes: Vec<i32>,
    p1mod4: Vec<i32>,
    p3mod4: Vec<i32>,
    log_p1: Vec<f64>,
    log_p3: Vec<f64>,
    log2: f64,
}

fn init_primes() -> PrimeData {
    let mut is_p = vec![true; 501];
    is_p[0] = false; is_p[1] = false;
    for i in 2..=500usize {
        if i * i > 500 { break; }
        if is_p[i] {
            let mut j = i * i;
            while j <= 500 { is_p[j] = false; j += i; }
        }
    }
    let mut pd = PrimeData {
        primes: Vec::new(), p1mod4: Vec::new(), p3mod4: Vec::new(),
        log_p1: Vec::new(), log_p3: Vec::new(), log2: 2.0f64.ln(),
    };
    for i in 2..=500usize {
        if is_p[i] {
            pd.primes.push(i as i32);
            if i % 4 == 1 {
                pd.log_p1.push((i as f64).ln());
                pd.p1mod4.push(i as i32);
            } else if i % 4 == 3 {
                pd.log_p3.push((i as f64).ln());
                pd.p3mod4.push(i as i32);
            }
        }
    }
    pd
}

// Factorize using small primes first, then trial division for remainder
fn factorize(n: i64, small_primes: &[i32]) -> Vec<(i64, i32)> {
    if n <= 1 { return vec![]; }
    let mut factors = Vec::new();
    let mut tmp = n;

    for &p in small_primes {
        let p64 = p as i64;
        if p64 * p64 > tmp { break; }
        if tmp % p64 == 0 {
            let mut e = 0i32;
            while tmp % p64 == 0 { tmp /= p64; e += 1; }
            factors.push((p64, e));
        }
    }

    // Continue trial division beyond small primes if needed
    if tmp > 1 {
        let start = small_primes.last().map(|&p| p as i64 + 2).unwrap_or(2);
        let mut i = if start % 2 == 0 { start + 1 } else { start };
        while i * i <= tmp {
            if tmp % i == 0 {
                let mut e = 0i32;
                while tmp % i == 0 { tmp /= i; e += 1; }
                factors.push((i, e));
            }
            i += 2;
        }
        if tmp > 1 { factors.push((tmp, 1)); }
    }

    factors
}

fn divisors_from_factors(factors: &[(i64, i32)]) -> Vec<i64> {
    let mut divs = vec![1i64];
    for &(p, e) in factors {
        let cur = divs.len();
        let mut pk = 1i64;
        for _ in 0..e {
            pk *= p;
            for k in 0..cur {
                divs.push(divs[k] * pk);
            }
        }
    }
    divs.sort_unstable();
    divs
}

struct Caches {
    div_cache: HashMap<i64, Vec<i64>>,
    fact_cache: HashMap<(i64, i64), Vec<Vec<i64>>>,
}

impl Caches {
    fn new() -> Self {
        Caches {
            div_cache: HashMap::new(),
            fact_cache: HashMap::new(),
        }
    }

    fn get_all_divisors(&mut self, n: i64, primes: &[i32]) -> Vec<i64> {
        if let Some(cached) = self.div_cache.get(&n) {
            return cached.clone();
        }
        let result = if n == 1 {
            vec![1]
        } else {
            let factors = factorize(n, primes);
            divisors_from_factors(&factors)
        };
        self.div_cache.insert(n, result.clone());
        result
    }

    fn get_odd_divisors(&mut self, mut n: i64, primes: &[i32]) -> Vec<i64> {
        while n % 2 == 0 { n /= 2; }
        self.get_all_divisors(n, primes)
    }

    // Ordered factorizations of n into factors >= min_val
    fn ordered_factorizations(&mut self, n: i64, min_val: i64, primes: &[i32]) -> Vec<Vec<i64>> {
        if let Some(cached) = self.fact_cache.get(&(n, min_val)) {
            return cached.clone();
        }

        let mut results = Vec::new();
        if n == 1 {
            results.push(Vec::new());
            self.fact_cache.insert((n, min_val), results.clone());
            return results;
        }
        if n < min_val {
            self.fact_cache.insert((n, min_val), results.clone());
            return results;
        }

        let all_divs = self.get_all_divisors(n, primes);
        for &d in &all_divs {
            if d < min_val { continue; }
            if d == n {
                results.push(vec![n]);
            } else if (d as i128) * (d as i128) <= n as i128 {
                let sub = self.ordered_factorizations(n / d, d, primes);
                for s in &sub {
                    let mut v = Vec::with_capacity(s.len() + 1);
                    v.push(d);
                    v.extend_from_slice(s);
                    results.push(v);
                }
            }
        }

        self.fact_cache.insert((n, min_val), results.clone());
        results
    }
}

fn min_number_for_shape(
    n: i64,
    log_primes: &[f64],
    mod_primes: &[i32],
    budget: f64,
    caches: &mut Caches,
    primes: &[i32],
) -> Option<(f64, i64)> {
    if n == 1 { return Some((0.0, 1)); }
    if n < 3 || n % 2 == 0 { return None; }

    let facts = caches.ordered_factorizations(n, 3, primes);
    let mut best_log = budget;
    let mut best_mod: i64 = 0;
    let mut found = false;

    for f in &facts {
        let k = f.len();
        if k > mod_primes.len() { continue; }

        let mut exps: Vec<i64> = f.iter().map(|&x| (x - 1) / 2).collect();
        exps.sort_unstable_by(|a, b| b.cmp(a));

        let log_val: f64 = exps.iter().enumerate().map(|(i, &e)| e as f64 * log_primes[i]).sum();

        if log_val < best_log {
            best_log = log_val;
            let mut mod_val = 1i128;
            for (i, &e) in exps.iter().enumerate() {
                mod_val = mod_val * pow_mod(mod_primes[i] as i64, e, MOD) as i128 % MOD as i128;
            }
            best_mod = mod_val as i64;
            found = true;
        }
    }

    if found { Some((best_log, best_mod)) } else { None }
}

fn run() {
    let pd = init_primes();

    let mut total: i64 = 0;
    let mut caches = Caches::new();

    for k in 1..=18 {
        let mut t: i64 = 1;
        for _ in 0..k { t *= 10; }
        let target = 2 * t + 2;

        let mut best_log = 1e30f64;
        let mut best_mod: i64 = 0;

        let odd_divs = caches.get_odd_divisors(target, &pd.primes);

        for &a_val in &odd_divs {
            let d_val = target / a_val;
            let dm1 = d_val - 1;

            let (log_a, mod_a) = if a_val == 1 {
                (0.0, 1i64)
            } else {
                match min_number_for_shape(a_val, &pd.log_p1, &pd.p1mod4, best_log, &mut caches, &pd.primes) {
                    Some(v) => v,
                    None => continue,
                }
            };

            if log_a >= best_log { continue; }

            // Case 1: odd m, B = dm1
            if dm1 == 1 {
                if log_a < best_log { best_log = log_a; best_mod = mod_a; }
            } else if dm1 % 2 == 1 {
                let remaining = best_log - log_a;
                if let Some((log_b, mod_b)) = min_number_for_shape(dm1, &pd.log_p3, &pd.p3mod4, remaining, &mut caches, &pd.primes) {
                    let tl = log_a + log_b;
                    if tl < best_log {
                        best_log = tl;
                        best_mod = (mod_a as i128 * mod_b as i128 % MOD as i128) as i64;
                    }
                }
            }

            // Case 2: even m
            if dm1 >= 1 && dm1 % 2 == 1 {
                let c_divs = caches.get_all_divisors(dm1, &pd.primes);
                for &c in &c_divs {
                    let b_val = dm1 / c;
                    let a0 = (c + 1) / 2;
                    let log2_part = a0 as f64 * pd.log2;

                    if log_a + log2_part >= best_log { continue; }

                    let remaining = best_log - log_a - log2_part;
                    let mod2 = pow_mod(2, a0, MOD);

                    if b_val == 1 {
                        let tl = log_a + log2_part;
                        if tl < best_log {
                            best_log = tl;
                            best_mod = (mod_a as i128 * mod2 as i128 % MOD as i128) as i64;
                        }
                    } else if b_val % 2 == 1 {
                        if let Some((log_b, mod_b)) = min_number_for_shape(b_val, &pd.log_p3, &pd.p3mod4, remaining, &mut caches, &pd.primes) {
                            let tl = log_a + log2_part + log_b;
                            if tl < best_log {
                                best_log = tl;
                                let mut bm = (mod_a as i128 * mod2 as i128 % MOD as i128) as i64;
                                bm = (bm as i128 * mod_b as i128 % MOD as i128) as i64;
                                best_mod = bm;
                            }
                        }
                    }
                }
            }
        }

        total = (total + best_mod) % MOD;
    }

    println!("{}", total);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(256 * 1024 * 1024)
        .spawn(run)
        .unwrap()
        .join()
        .unwrap();
}
