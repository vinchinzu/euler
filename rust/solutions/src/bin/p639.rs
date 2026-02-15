// Project Euler 639 - Summing a multiplicative function
// Powerful number iteration with Lagrange interpolation for power sums

const N_VAL: i64 = 1_000_000_000_000;
const K_VAL: usize = 50;
const MOD: i64 = 1_000_000_007;

fn powmod(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut result = 1i64;
    base = ((base % m) + m) % m;
    while exp > 0 {
        if exp & 1 == 1 { result = (result as i128 * base as i128 % m as i128) as i64; }
        base = (base as i128 * base as i128 % m as i128) as i64;
        exp >>= 1;
    }
    result
}

fn main() {
    let l = {
        let mut x = (N_VAL as f64).sqrt() as i64;
        while x * x > N_VAL { x -= 1; }
        while (x + 1) * (x + 1) <= N_VAL { x += 1; }
        x as usize
    };

    let mut is_prime = vec![false; l + 1];
    if l >= 2 { is_prime[2] = true; }
    for i in (3..=l).step_by(2) { is_prime[i] = true; }
    let mut i = 3;
    while i * i <= l { if is_prime[i] { let mut j = i*i; while j <= l { is_prime[j] = false; j += 2*i; } } i += 2; }

    let primes: Vec<usize> = (2..=l).filter(|&i| is_prime[i]).collect();

    // Lagrange setup
    let max_deg = K_VAL + 2;
    let mut fact = vec![1i64; max_deg + 1];
    for i in 1..=max_deg { fact[i] = fact[i-1] * i as i64 % MOD; }
    let mut inv_fact = vec![1i64; max_deg + 1];
    inv_fact[max_deg] = powmod(fact[max_deg], MOD - 2, MOD);
    for i in (0..max_deg).rev() { inv_fact[i] = inv_fact[i+1] * (i as i64 + 1) % MOD; }

    let mut nth_pows = vec![1i64; l + 1];
    let mut sum_powers = vec![0i64; l + 1];
    let mut sum_coeffs = vec![0i64; l + 1];

    let sum_kth_powers = |n: i64, k: usize, sp: &[i64]| -> i64 {
        if n <= l as i64 { return sp[n as usize]; }
        let pts = k + 2;
        let mut prefix = vec![1i64; pts + 1];
        for j in 0..pts { prefix[j+1] = (prefix[j] as i128 * ((n - j as i64) % MOD + MOD) as i128 % MOD as i128) as i64; }
        let mut suffix = vec![1i64; pts + 1];
        for j in (0..pts).rev() { suffix[j] = (suffix[j+1] as i128 * ((n - j as i64) % MOD + MOD) as i128 % MOD as i128) as i64; }
        let mut result = 0i64;
        let m = k + 1;
        for i in 0..pts {
            let numer = (prefix[i] as i128 * suffix[i+1] as i128 % MOD as i128 * sp[i] as i128 % MOD as i128) as i64;
            let mut denom = (inv_fact[i] as i128 * inv_fact[m - i] as i128 % MOD as i128) as i64;
            if (m - i) % 2 == 1 { denom = MOD - denom; }
            result = (result + (numer as i128 * denom as i128 % MOD as i128) as i64) % MOD;
        }
        result
    };

    let mut ans = 0i64;

    for k in 1..=K_VAL {
        sum_powers[0] = 0;
        sum_coeffs[0] = 0;
        for i in 1..=l {
            nth_pows[i] = (nth_pows[i] as i128 * i as i128 % MOD as i128) as i64;
            sum_powers[i] = (sum_powers[i-1] + nth_pows[i]) % MOD;
            let coeff = if is_prime[i] {
                (nth_pows[i] as i128 * (1 - nth_pows[i] as i128 + MOD as i128) % MOD as i128) as i64
            } else { 0 };
            sum_coeffs[i] = (sum_coeffs[i-1] + coeff) % MOD;
        }

        struct Entry { min_idx: usize, d: i64, mult: i64, prev_e: i32 }
        let mut stack: Vec<Entry> = vec![Entry { min_idx: 0, d: 1, mult: 1, prev_e: 0 }];

        while let Some(e) = stack.pop() {
            let n = N_VAL / e.d;

            if e.prev_e != 2 {
                let sp = sum_kth_powers(n, k, &sum_powers);
                ans = (ans + (sp as i128 * e.mult as i128 % MOD as i128) as i64) % MOD;
            }

            let mut lim = (n as f64).cbrt() as i64;
            while lim > 0 && lim * lim * lim > n { lim -= 1; }
            while (lim + 1) * (lim + 1) * (lim + 1) <= n { lim += 1; }

            // Individual primes (exp 2)
            for i in e.min_idx..primes.len() {
                let p = primes[i] as i64;
                let pp = p * p;
                let threshold = if lim > 0 { n / lim } else { n };
                if pp > threshold { break; }
                let q = n / pp;
                let sp_q = sum_kth_powers(q, k, &sum_powers);
                let coeff = (nth_pows[p as usize] as i128 * (1 - nth_pows[p as usize] as i128 + MOD as i128) % MOD as i128) as i64;
                ans = (ans + (sp_q as i128 * e.mult as i128 % MOD as i128 * coeff as i128 % MOD as i128) as i64) % MOD;
            }

            // Ranges of primes
            let p_min = if e.min_idx < primes.len() { primes[e.min_idx] as i64 } else { l as i64 + 1 };
            for q in 1..lim {
                let mut high = ((n / q) as f64).sqrt() as i64;
                while high > 0 && high * high > n / q { high -= 1; }
                while (high + 1) * (high + 1) <= n / q { high += 1; }
                let low_sq = n / (q + 1);
                let mut low = (low_sq as f64).sqrt() as i64;
                while low > 0 && low * low > low_sq { low -= 1; }
                while (low + 1) * (low + 1) <= low_sq { low += 1; }
                if low < p_min - 1 { low = p_min - 1; }
                if high > l as i64 { high = l as i64; }
                if high >= low && high >= 0 && low >= 0 {
                    let coeff_sum = (sum_coeffs[high as usize] - sum_coeffs[low as usize] + MOD) % MOD;
                    ans = (ans + (sum_powers[q as usize] as i128 * e.mult as i128 % MOD as i128 * coeff_sum as i128 % MOD as i128) as i64) % MOD;
                } else { break; }
            }

            // Higher prime powers (exp >= 3)
            for i in e.min_idx..primes.len() {
                let p = primes[i] as i64;
                if e.d > N_VAL / p / p / p { break; }
                let mut new_d = e.d * p;
                let new_mult = (e.mult as i128 * nth_pows[p as usize] as i128 % MOD as i128
                    * (1 - nth_pows[p as usize] as i128 + MOD as i128) % MOD as i128) as i64;
                let mut ee = 1;
                while new_d <= N_VAL / p {
                    new_d *= p;
                    ee += 1;
                    stack.push(Entry { min_idx: i + 1, d: new_d, mult: new_mult, prev_e: ee });
                }
            }
        }
    }

    println!("{}", ans % MOD);
}
