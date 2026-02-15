// Project Euler 646 - Bounded Divisors
// Sum of lambda(d)*d over divisors d of N! where L <= d <= H.
// Meet-in-the-middle with binary search.

const MOD: i64 = 1_000_000_007;

fn pow_mod(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut result = 1i64;
    base = ((base % m) + m) % m;
    while exp > 0 {
        if exp & 1 == 1 { result = result * base % m; }
        base = base * base % m;
        exp >>= 1;
    }
    result
}

fn num_factors_in_factorial(n: i32, p: i32) -> i32 {
    let mut count = 0;
    let mut power = p as i64;
    while power <= n as i64 {
        count += n as i64 / power;
        power *= p as i64;
    }
    count as i32
}

struct Divisor {
    log_val: f64,
    lio: i64,
}

fn gen_divisors(primes: &[i32], exponents: &[i32], start: usize, end: usize) -> Vec<Divisor> {
    let mut total: usize = 1;
    for i in start..end { total *= (exponents[i] + 1) as usize; }
    let mut divs = Vec::with_capacity(total);
    divs.push(Divisor { log_val: 0.0, lio: 1 });
    for fi in start..end {
        let p = primes[fi];
        let e_max = exponents[fi];
        let log_p = (p as f64).ln();
        let old_count = divs.len();
        for e in 1..=e_max {
            let neg_p_e = pow_mod(MOD - p as i64, e as i64, MOD);
            let log_pe = e as f64 * log_p;
            for j in 0..old_count {
                divs.push(Divisor {
                    log_val: divs[j].log_val + log_pe,
                    lio: divs[j].lio * neg_p_e % MOD,
                });
            }
        }
    }
    divs
}

fn main() {
    let n = 70;
    let log_l = 20.0f64 * (10.0f64).ln();
    let log_h = 60.0f64 * (10.0f64).ln();

    let primes = [2,3,5,7,11,13,17,19,23,29,31,37,41,43,47,53,59,61,67];
    let num_primes = primes.len();
    let mut exponents = vec![0i32; num_primes];
    for i in 0..num_primes {
        exponents[i] = num_factors_in_factorial(n, primes[i]);
    }

    let mut total_factors: i64 = 1;
    for i in 0..num_primes { total_factors *= (exponents[i] + 1) as i64; }

    let mut half_index = 0;
    let mut nf: i64 = 1;
    while nf * nf < total_factors {
        nf *= (exponents[half_index] + 1) as i64;
        half_index += 1;
    }

    let mut left = gen_divisors(&primes, &exponents, 0, half_index);
    let right = gen_divisors(&primes, &exponents, half_index, num_primes);

    left.sort_by(|a, b| a.log_val.partial_cmp(&b.log_val).unwrap());

    let mut prefix = vec![0i64; left.len() + 1];
    for i in 0..left.len() {
        prefix[i + 1] = (prefix[i] + left[i].lio) % MOD;
    }

    let left_logs: Vec<f64> = left.iter().map(|d| d.log_val).collect();

    let eps_val = 1e-14f64;
    let mut ans = 0i64;

    for ri in 0..right.len() {
        let log_r = right[ri].log_val;
        let lio_r = right[ri].lio;
        let lo = log_l - log_r - eps_val;
        let hi = log_h - log_r + eps_val;

        // Binary search for lower bound
        let a = left_logs.partition_point(|&x| x <= lo);
        // Binary search for upper bound
        let b = left_logs.partition_point(|&x| x <= hi);

        let range_sum = (prefix[b] - prefix[a] + MOD) % MOD;
        ans = (ans + lio_r * range_sum) % MOD;
    }

    ans = ((ans % MOD) + MOD) % MOD;
    println!("{}", ans);
}
