// Project Euler 615 - The millionth number with at least one million prime factors
//
// Numbers with Omega >= 1e6 are 2^{1e6-c} times a c-almost-prime core.
// Enumerate cores with Omega >= THRESHOLD below 3^THRESHOLD (the first
// threshold that yields >= 1e6 candidates), take the millionth, lift by
// 2^{1e6-THRESHOLD} mod 123454321.

const TARGET: usize = 1_000_000;
const MOD: u64 = 123_454_321;
const THRESHOLD: u32 = 29;
const PRIME_LIMIT: usize = 300_000;

fn pow_mod(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut r = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            r = r * base % m;
        }
        base = base * base % m;
        exp >>= 1;
    }
    r
}

fn sieve_primes(limit: usize) -> Vec<u32> {
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut i = 2usize;
    while i * i <= limit {
        if is_prime[i] {
            let mut j = i * i;
            while j <= limit {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    (2..=limit as u32).filter(|&p| is_prime[p as usize]).collect()
}

fn search(
    primes: &[u32],
    logs: &[f64],
    start_index: usize,
    product: u64,
    used: u32,
    log_product: f64,
    limit: u64,
    log_limit: f64,
    out: &mut Vec<u64>,
) {
    if used >= THRESHOLD {
        out.push(product);
    }
    let remaining = THRESHOLD.saturating_sub(used);
    for index in start_index..primes.len() {
        let p = primes[index] as u64;
        let next_product = match product.checked_mul(p) {
            Some(x) if x <= limit => x,
            _ => break,
        };
        if remaining > 0 && log_product + remaining as f64 * logs[index] > log_limit + 1e-12 {
            break;
        }
        search(
            primes,
            logs,
            index,
            next_product,
            used + 1,
            log_product + logs[index],
            limit,
            log_limit,
            out,
        );
    }
}

fn main() {
    let primes = sieve_primes(PRIME_LIMIT);
    let logs: Vec<f64> = primes.iter().map(|&p| (p as f64).ln()).collect();
    let limit = 3u64.pow(THRESHOLD);
    let log_limit = THRESHOLD as f64 * 3f64.ln();

    let mut values = Vec::with_capacity(TARGET + TARGET / 4);
    search(
        &primes,
        &logs,
        0,
        1,
        0,
        0.0,
        limit,
        log_limit,
        &mut values,
    );

    let k = TARGET - 1;
    values.select_nth_unstable(k);
    let core = values[k];
    let ans = (core % MOD) * pow_mod(2, TARGET as u64 - THRESHOLD as u64, MOD) % MOD;
    println!("{}", ans);
}
