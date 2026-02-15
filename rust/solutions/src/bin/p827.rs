// Project Euler 827 - Pythagorean Triple Occurrence

const MOD: i64 = 409120391;

fn pow_mod(mut base: i64, mut exp: i64, m: i64) -> i64 {
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

fn init_primes() -> (Vec<i32>, Vec<i32>, Vec<i32>, Vec<f64>, Vec<f64>, f64) {
    let mut is_p = vec![true; 501];
    is_p[0] = false; is_p[1] = false;
    for i in 2..=500 {
        if i * i > 500 { break; }
        if is_p[i] {
            let mut j = i * i;
            while j <= 500 { is_p[j] = false; j += i; }
        }
    }
    let mut primes = Vec::new();
    let mut p1mod4 = Vec::new();
    let mut p3mod4 = Vec::new();
    let mut log_p1 = Vec::new();
    let mut log_p3 = Vec::new();
    for i in 2..=500 {
        if is_p[i] {
            primes.push(i as i32);
            if i % 4 == 1 {
                log_p1.push((i as f64).ln());
                p1mod4.push(i as i32);
            } else if i % 4 == 3 {
                log_p3.push((i as f64).ln());
                p3mod4.push(i as i32);
            }
        }
    }
    (primes, p1mod4, p3mod4, log_p1, log_p3, 2.0f64.ln())
}

fn get_odd_divisors(mut n: i64) -> Vec<i64> {
    while n % 2 == 0 { n /= 2; }
    get_all_divisors(n)
}

fn get_all_divisors(n: i64) -> Vec<i64> {
    if n == 1 { return vec![1]; }
    let mut ps = Vec::new();
    let mut es = Vec::new();
    let mut tmp = n;
    let mut i = 2i64;
    while i * i <= tmp {
        if tmp % i == 0 {
            let mut e = 0;
            while tmp % i == 0 { tmp /= i; e += 1; }
            ps.push(i);
            es.push(e);
        }
        i += 1;
    }
    if tmp > 1 { ps.push(tmp); es.push(1); }

    let mut divs = vec![1i64];
    for (idx, &p) in ps.iter().enumerate() {
        let cur = divs.len();
        let mut pk = 1i64;
        for _ in 0..es[idx] {
            pk *= p;
            for k in 0..cur {
                divs.push(divs[k] * pk);
            }
        }
    }
    divs.sort();
    divs
}

#[derive(Clone)]
struct Factorization {
    factors: Vec<i32>,
}

fn ordered_factorizations(n: i64, min_val: i32) -> Vec<Factorization> {
    let mut results = Vec::new();
    if n == 1 {
        results.push(Factorization { factors: Vec::new() });
        return results;
    }
    if n < min_val as i64 { return results; }

    let all_divs = get_all_divisors(n);
    for &d in &all_divs {
        if d < min_val as i64 { continue; }
        if d == n {
            results.push(Factorization { factors: vec![n as i32] });
        } else if n % d == 0 && d * d <= n {
            let sub = ordered_factorizations(n / d, d as i32);
            for mut s in sub {
                s.factors.insert(0, d as i32);
                results.push(s);
            }
        }
    }
    results
}

fn min_number_for_shape(n: i64, log_primes: &[f64], mod_primes: &[i32], budget: f64)
    -> Option<(f64, i64)>
{
    if n == 1 { return Some((0.0, 1)); }
    if n < 3 || n % 2 == 0 { return None; }

    let facts = ordered_factorizations(n, 3);
    let mut best_log = budget;
    let mut best_mod: i64 = 0;
    let mut found = false;

    for f in &facts {
        let k = f.factors.len();
        if k > mod_primes.len() { continue; }

        let mut exps: Vec<i32> = f.factors.iter().map(|&x| (x - 1) / 2).collect();
        exps.sort_by(|a, b| b.cmp(a));

        let log_val: f64 = exps.iter().enumerate().map(|(i, &e)| e as f64 * log_primes[i]).sum();

        if log_val < best_log {
            best_log = log_val;
            let mut mod_val = 1i128;
            for (i, &e) in exps.iter().enumerate() {
                mod_val = mod_val * pow_mod(mod_primes[i] as i64, e as i64, MOD) as i128 % MOD as i128;
            }
            best_mod = mod_val as i64;
            found = true;
        }
    }

    if found { Some((best_log, best_mod)) } else { None }
}

fn main() {
    let (_primes, p1mod4, p3mod4, log_p1, log_p3, log2) = init_primes();

    let mut total: i64 = 0;

    for k in 1..=18 {
        let mut t: i64 = 1;
        for _ in 0..k { t *= 10; }
        let target = 2 * t + 2;

        let mut best_log = 1e30f64;
        let mut best_mod: i64 = 0;

        let odd_divs = get_odd_divisors(target);

        for &a_val in &odd_divs {
            let d_val = target / a_val;
            let dm1 = d_val - 1;

            let (log_a, mod_a) = if a_val == 1 {
                (0.0, 1i64)
            } else {
                match min_number_for_shape(a_val, &log_p1, &p1mod4, best_log) {
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
                if let Some((log_b, mod_b)) = min_number_for_shape(dm1, &log_p3, &p3mod4, remaining) {
                    let tl = log_a + log_b;
                    if tl < best_log {
                        best_log = tl;
                        best_mod = (mod_a as i128 * mod_b as i128 % MOD as i128) as i64;
                    }
                }
            }

            // Case 2: even m
            if dm1 >= 1 && dm1 % 2 == 1 {
                let c_divs = get_all_divisors(dm1);
                for &c in &c_divs {
                    let b = dm1 / c;
                    let a0 = (c + 1) / 2;
                    let log2_part = a0 as f64 * log2;

                    if log_a + log2_part >= best_log { continue; }

                    let remaining = best_log - log_a - log2_part;
                    let mod2 = pow_mod(2, a0, MOD);

                    if b == 1 {
                        let tl = log_a + log2_part;
                        if tl < best_log {
                            best_log = tl;
                            best_mod = (mod_a as i128 * mod2 as i128 % MOD as i128) as i64;
                        }
                    } else if b % 2 == 1 {
                        if let Some((log_b, mod_b)) = min_number_for_shape(b, &log_p3, &p3mod4, remaining) {
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
