// Project Euler Problem 953
// Factorisation Nim: S(10^14) mod 10^9+7
// Translated from C solution.

const N_VAL: i64 = 100_000_000_000_000;
const MOD: i64 = 1_000_000_007;
const LIMIT_PRIME: usize = 22_000_000;
const SMALL_M_LIMIT: usize = 100_000;

fn s2_contribution(k: i64) -> i64 {
    if k > N_VAL {
        return 0;
    }
    let quot = N_VAL / k;
    let mut m = (quot as f64).sqrt() as i64;
    // Adjust sqrt
    while (m + 1) * (m + 1) <= quot {
        m += 1;
    }
    while m * m > quot {
        m -= 1;
    }
    if m == 0 {
        return 0;
    }

    let v = m as i128 * (m as i128 + 1) * (2 * m as i128 + 1) / 6;
    let s2 = (v % MOD as i128) as i64;
    (k % MOD * s2 % MOD + MOD) % MOD
}

fn is_prime_large(p: i64, is_prime_sieve: &[bool]) -> bool {
    if p <= LIMIT_PRIME as i64 {
        return is_prime_sieve[p as usize];
    }
    if p % 2 == 0 {
        return false;
    }
    let mut d: i64 = 3;
    while d * d <= p {
        if p % d == 0 {
            return false;
        }
        d += 2;
    }
    true
}

fn dfs(
    idx: i32,
    current_m: i64,
    current_g: i32,
    q: i32,
    limit_m: i64,
    primes_small: &[i32],
    is_prime_sieve: &[bool],
    total_sum: &mut i64,
) {
    let p = current_g ^ q;

    if p > q {
        if is_prime_large(p as i64, is_prime_sieve) {
            let k = current_m as i128 * q as i128 * p as i128;
            if k <= N_VAL as i128 {
                *total_sum = (*total_sum + s2_contribution(k as i64)) % MOD;
            }
        }
    }

    let mut i = idx;
    while i >= 0 {
        let next_p = primes_small[i as usize];
        let next_m = current_m.checked_mul(next_p as i64);
        if let Some(nm) = next_m {
            if nm <= limit_m {
                dfs(
                    i - 1,
                    nm,
                    current_g ^ next_p,
                    q,
                    limit_m,
                    primes_small,
                    is_prime_sieve,
                    total_sum,
                );
            }
        }
        i -= 1;
    }
}

fn main() {
    // Sieve of Eratosthenes
    let mut is_prime_sieve = vec![true; LIMIT_PRIME + 1];
    is_prime_sieve[0] = false;
    is_prime_sieve[1] = false;
    {
        let mut i = 2;
        while i * i <= LIMIT_PRIME {
            if is_prime_sieve[i] {
                let mut j = i * i;
                while j <= LIMIT_PRIME {
                    is_prime_sieve[j] = false;
                    j += i;
                }
            }
            i += 1;
        }
    }

    let primes: Vec<i32> = (2..=LIMIT_PRIME)
        .filter(|&i| is_prime_sieve[i])
        .map(|i| i as i32)
        .collect();

    // Precompute small m arrays using linear sieve
    let mut lp = vec![0i32; SMALL_M_LIMIT + 1];
    let mut g_arr = vec![0i32; SMALL_M_LIMIT + 1];
    let mut max_p_arr = vec![0i32; SMALL_M_LIMIT + 1];
    let mut sq = vec![true; SMALL_M_LIMIT + 1];
    let mut pr: Vec<i32> = Vec::with_capacity(10000);

    for i in 2..=SMALL_M_LIMIT {
        if lp[i] == 0 {
            lp[i] = i as i32;
            pr.push(i as i32);
            g_arr[i] = i as i32;
            max_p_arr[i] = i as i32;
        }
        for pi in 0..pr.len() {
            let p = pr[pi];
            if p > lp[i] || (i as i64) * (p as i64) > SMALL_M_LIMIT as i64 {
                break;
            }
            let ip = i * p as usize;
            lp[ip] = p;
            max_p_arr[ip] = max_p_arr[i];
            if p == lp[i] {
                sq[ip] = false;
                g_arr[ip] = g_arr[i] ^ p;
            } else {
                sq[ip] = sq[i];
                g_arr[ip] = g_arr[i] ^ p;
            }
        }
    }

    let mut total_sum: i64 = 0;

    // k=1 case
    total_sum = (total_sum + s2_contribution(1)) % MOD;

    let max_q = ((N_VAL / 2) as f64).sqrt() as i64;

    for qi in 0..primes.len() {
        let q = primes[qi];
        if q as i64 > max_q {
            break;
        }

        let q_sq = q as i64 * q as i64;
        let limit_m = N_VAL / q_sq;

        if limit_m == 0 {
            break;
        }

        if limit_m <= SMALL_M_LIMIT as i64 {
            // Direct iteration
            for m in 2..=limit_m as usize {
                if sq[m] && max_p_arr[m] < q {
                    let p = g_arr[m] ^ q;
                    if p > q {
                        if is_prime_large(p as i64, &is_prime_sieve) {
                            let k = m as i128 * q as i128 * p as i128;
                            if k <= N_VAL as i128 {
                                total_sum = (total_sum + s2_contribution(k as i64)) % MOD;
                            }
                        }
                    }
                }
            }
        } else {
            if 2 * q_sq > N_VAL {
                continue;
            }

            // Use a slice of primes array directly (primes < q)
            // qi is the index of q in primes, so primes[0..qi] are all < q
            if qi > 0 {
                dfs(
                    qi as i32 - 1,
                    1,
                    0,
                    q,
                    limit_m,
                    &primes[..qi],
                    &is_prime_sieve,
                    &mut total_sum,
                );
            }
        }
    }

    println!("{}", total_sum);
}
