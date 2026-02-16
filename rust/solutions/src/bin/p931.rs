// Project Euler 931 - Totient Graph
//
// For positive integer n, build graph on divisors: edge a->b when b|a and a/b is prime,
// weight = phi(a) - phi(b). t(n) = total weight. T(N) = sum_{n=1}^N t(n).
// Find T(10^12) mod 715827883.
//
// Key derivation:
// T(N) = sum_{m=1}^N g(m)*floor(N/m) where g(m) = sum_{p|m,p prime} (phi(m)-phi(m/p)).
// Using Dirichlet convolution: T(N) = sum_{n=1}^N h(n) where h = g*1.
//
// After simplification: T(N) = S_main + D - C_ge2 where:
// S_main = sum_{p prime, p<=N} (p-2) * f(floor(N/p))   with f(x) = x*(x+1)/2
// D = sum_{p prime, p<=sqrt(N)} p * f(floor(N/p^2))
// C_ge2 = sum_{a>=2} sum_{p: p^a<=N} [f(floor(N/p^a)) - p*f(floor(N/p^{a+1}))]
//
// S_main is computed via Lucy DP (prime sum/count at quotient values).

const MOD: u64 = 715827883;
const LIMIT: u64 = 1_000_000_000_000;

fn main() {
    let sqrt_n = (LIMIT as f64).sqrt() as u64 + 1;

    // Collect all quotient values floor(N/k)
    let mut quotients: Vec<u64> = Vec::new();
    {
        let mut k = 1u64;
        while k <= LIMIT {
            let q = LIMIT / k;
            quotients.push(q);
            k = LIMIT / q + 1;
        }
    }
    quotients.sort_unstable();
    quotients.dedup();

    let num_q = quotients.len();
    let sq = sqrt_n as usize + 2;
    let mut idx_small = vec![0usize; sq];
    let mut idx_large = vec![0usize; sq];

    for (i, &q) in quotients.iter().enumerate() {
        if q < sq as u64 {
            idx_small[q as usize] = i;
        } else {
            idx_large[(LIMIT / q) as usize] = i;
        }
    }

    let get_idx = |q: u64| -> usize {
        if q < sq as u64 {
            idx_small[q as usize]
        } else {
            idx_large[(LIMIT / q) as usize]
        }
    };

    // Lucy DP: compute prime sum and prime count at all quotient values
    let inv2 = (MOD + 1) / 2;

    let mut s0: Vec<u64> = vec![0; num_q]; // prime count
    let mut s1: Vec<u64> = vec![0; num_q]; // prime sum

    for (i, &q) in quotients.iter().enumerate() {
        let qm = q % MOD;
        s1[i] = (qm * ((qm + 1) % MOD) % MOD * inv2 % MOD + MOD - 1) % MOD;
        s0[i] = (qm + MOD - 1) % MOD;
    }

    // Sieve primes up to sqrt(N)
    let sieve_limit = sqrt_n as usize + 1;
    let mut is_prime = vec![true; sieve_limit + 1];
    is_prime[0] = false;
    if sieve_limit >= 1 { is_prime[1] = false; }
    {
        let mut i = 2;
        while i * i <= sieve_limit {
            if is_prime[i] {
                let mut j = i * i;
                while j <= sieve_limit {
                    is_prime[j] = false;
                    j += i;
                }
            }
            i += 1;
        }
    }
    let primes: Vec<u64> = (2..=sieve_limit).filter(|&i| is_prime[i]).map(|i| i as u64).collect();

    // Lucy DP sieve step
    for &p in &primes {
        let p2 = p * p;
        let pm = p % MOD;
        for j in (0..num_q).rev() {
            let q = quotients[j];
            if q < p2 { break; }
            let idx_qp = get_idx(q / p);
            let idx_pm1 = if p > 1 { get_idx(p - 1) } else { 0 };

            let sub1 = (s1[idx_qp] + MOD - s1[idx_pm1]) % MOD;
            s1[j] = (s1[j] + MOD - pm * sub1 % MOD) % MOD;

            let sub0 = (s0[idx_qp] + MOD - s0[idx_pm1]) % MOD;
            s0[j] = (s0[j] + MOD - sub0) % MOD;
        }
    }

    // f(x) = x*(x+1)/2 mod MOD
    let f = |x: u64| -> u64 {
        let xm = x % MOD;
        xm * ((xm + 1) % MOD) % MOD * inv2 % MOD
    };

    // S_main = sum_{p prime, p<=N} (p-2) * f(floor(N/p))
    // Group by v = floor(N/p)
    let mut s_main: u64 = 0;
    {
        let mut k = 1u64;
        while k <= LIMIT {
            let v = LIMIT / k;
            let k_end = LIMIT / v;

            let hi = k_end;
            let lo = if v + 1 <= LIMIT { LIMIT / (v + 1) } else { 0 };

            let (sp_hi, cp_hi) = (s1[get_idx(hi)], s0[get_idx(hi)]);
            let (sp_lo, cp_lo) = if lo == 0 { (0, 0) } else { (s1[get_idx(lo)], s0[get_idx(lo)]) };

            let sp_range = (sp_hi + MOD - sp_lo) % MOD;
            let cp_range = (cp_hi + MOD - cp_lo) % MOD;
            let sum_p_minus_2 = (sp_range + 2 * MOD - 2 * cp_range % MOD) % MOD;

            s_main = (s_main + f(v) * sum_p_minus_2 % MOD) % MOD;

            k = k_end + 1;
        }
    }

    // D = sum_{p prime, p<=sqrt(N)} p * f(floor(N/p^2))
    let mut d_sum: u64 = 0;
    for &p in &primes {
        if p * p > LIMIT { break; }
        d_sum = (d_sum + (p % MOD) * f(LIMIT / (p * p)) % MOD) % MOD;
    }

    // C_ge2 = sum_{a>=2} sum_{p: p^a<=N} [f(floor(N/p^a)) - p * f(floor(N/p^{a+1}))]
    let mut c_ge2: u64 = 0;
    for &p in &primes {
        let mut pa = p * p;
        if pa > LIMIT { break; }
        loop {
            let q1 = LIMIT / pa;
            let q2 = q1 / p;
            let term = (f(q1) + MOD - (p % MOD) * f(q2) % MOD) % MOD;
            c_ge2 = (c_ge2 + term) % MOD;
            if pa > LIMIT / p { break; }
            pa *= p;
            if pa > LIMIT { break; }
        }
    }

    println!("{}", (s_main + d_sum + MOD - c_ge2) % MOD);
}
