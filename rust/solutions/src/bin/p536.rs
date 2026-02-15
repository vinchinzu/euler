// Project Euler 536 - Modulo Power Identity
//
// Find sum of all m <= N such that a^{m+4} = a (mod m) for all a.
// Condition: m squarefree, lambda(m) | m+3.
//
// Recursively build squarefree m by multiplying primes, tracking
// carmichael = lcm(p_i - 1). CRT optimization when search space is small.

use euler_utils::primes::sieve_smallest_factor;

const N: u64 = 1_000_000_000_000;
const SPF_LIMIT: usize = 100_000_001;

fn sieve_primes(limit: usize) -> Vec<u64> {
    let mut is_p = vec![true; limit + 1];
    is_p[0] = false;
    if limit >= 1 { is_p[1] = false; }
    let mut i = 2;
    while i * i <= limit {
        if is_p[i] {
            let mut j = i * i;
            while j <= limit {
                is_p[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    (2..=limit).filter(|&i| is_p[i]).map(|i| i as u64).collect()
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 { let t = b; b = a % b; a = t; }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

fn mod_inv(a: i64, m: i64) -> i64 {
    if m == 1 { return 0; }
    let (mut t, mut new_t) = (0i64, 1i64);
    let (mut r, mut new_r) = (m, a);
    while new_r != 0 {
        let q = r / new_r;
        let tmp_t = new_t;
        new_t = t - q * new_t;
        t = tmp_t;
        let tmp_r = new_r;
        new_r = r - q * new_r;
        r = tmp_r;
    }
    if t < 0 { t += m; }
    t
}

fn imod(a: i64, m: i64) -> i64 {
    ((a % m) + m) % m
}

/// Check if m*r satisfies conditions where r > 1 is the remaining factor
fn good(m: u64, mut r: u64, max_p: u64, spf: &[u32]) -> bool {
    let m3 = m + 3;
    while r > 1 {
        let p = if (r as usize) < spf.len() {
            spf[r as usize] as u64
        } else {
            r // r itself must be prime
        };
        if m3 % (p - 1) != 0 { return false; }
        if p >= max_p { return false; }
        r /= p;
        if r % p == 0 { return false; } // not squarefree
    }
    true
}

fn helper(max_index: usize, m: u64, carmichael: u64, primes: &[u64], spf: &[u32], ans: &mut u64) {
    let g = gcd(m, carmichael);
    if 3 % g != 0 { return; }
    if (m + 3) % carmichael == 0 {
        *ans += m;
    }

    // CRT optimization: enumerate multiples when search space is small
    let nm = N / m;
    if nm < SPF_LIMIT as u64 && nm / carmichael < (1u64 << max_index.min(63)) {
        let modv = carmichael / g;
        if modv > 0 {
            let mg = (m / g) as i64;
            let modv_i = modv as i64;
            let inv = mod_inv(mg, modv_i);
            let neg3g = -3i64 / g as i64;
            let r_start = imod(neg3g * inv, modv_i) as u64;
            let mut r = r_start;
            while m.checked_mul(r).map_or(false, |v| v <= N) {
                if r > 1 {
                    let max_p = if max_index < primes.len() { primes[max_index] } else { N + 1 };
                    if good(m * r, r, max_p, spf) {
                        *ans += m * r;
                    }
                }
                r += modv;
            }
        }
        return;
    }

    // Recursive case: try adding primes (from index 0..max_index)
    for index in 0..max_index {
        let p = primes[index];
        if m > N / p { break; } // m*p > N
        helper(index, m * p, lcm(carmichael, p - 1), primes, spf, ans);
    }
}

fn main() {
    let sqrt_n = (N as f64).sqrt() as usize + 1;
    let primes = sieve_primes(sqrt_n);

    // Build SPF table
    let spf = sieve_smallest_factor(SPF_LIMIT);

    let mut ans = 0u64;
    helper(primes.len(), 1, 1, &primes, &spf, &mut ans);
    println!("{}", ans);
}
