/// Sieve of Eratosthenes returning a boolean vec where `result[i]` is true if `i` is prime.
pub fn sieve(limit: usize) -> Vec<bool> {
    let mut is_prime = vec![true; limit + 1];
    if limit >= 1 {
        is_prime[0] = false;
    }
    if limit >= 1 {
        is_prime[1] = false;
    }
    let mut i = 2;
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
    is_prime
}

/// Sieve returning smallest prime factor for each number up to `limit`.
pub fn sieve_smallest_factor(limit: usize) -> Vec<u32> {
    let mut spf = vec![0u32; limit + 1];
    for i in 2..=limit {
        if spf[i] == 0 {
            // i is prime
            let mut j = i;
            while j <= limit {
                if spf[j] == 0 {
                    spf[j] = i as u32;
                }
                j += i;
            }
        }
    }
    spf
}

/// Return a Vec of all primes up to `limit`.
pub fn primes_up_to(limit: usize) -> Vec<usize> {
    let is_p = sieve(limit);
    (2..=limit).filter(|&i| is_p[i]).collect()
}

/// Deterministic primality test for small numbers (< sieve limit).
pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n < 4 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5u64;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

/// Miller-Rabin primality test, deterministic for n < 3,317,044,064,679,887,385,961,981.
pub fn miller_rabin(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 || n == 5 || n == 7 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 || n % 5 == 0 {
        return false;
    }

    // Write n-1 as 2^r * d
    let mut d = n - 1;
    let mut r = 0u32;
    while d % 2 == 0 {
        d /= 2;
        r += 1;
    }

    // Witnesses sufficient for n < 3.3e24
    let witnesses = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

    'outer: for &a in &witnesses {
        if a >= n {
            continue;
        }
        let mut x = mod_pow_u64(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 0..r - 1 {
            x = mod_mul_u64(x, x, n);
            if x == n - 1 {
                continue 'outer;
            }
        }
        return false;
    }
    true
}

/// Modular exponentiation using u128 to avoid overflow.
fn mod_pow_u64(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result = 1u64;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = ((result as u128 * base as u128) % modulus as u128) as u64;
        }
        exp /= 2;
        base = ((base as u128 * base as u128) % modulus as u128) as u64;
    }
    result
}

/// Modular multiplication using u128.
fn mod_mul_u64(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sieve_small() {
        let s = sieve(20);
        let primes: Vec<usize> = (0..=20).filter(|&i| s[i]).collect();
        assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19]);
    }

    #[test]
    fn test_primes_up_to() {
        assert_eq!(primes_up_to(10), vec![2, 3, 5, 7]);
    }

    #[test]
    fn test_smallest_factor() {
        let spf = sieve_smallest_factor(20);
        assert_eq!(spf[12], 2);
        assert_eq!(spf[15], 3);
        assert_eq!(spf[17], 17);
    }

    #[test]
    fn test_is_prime() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
        assert!(is_prime(97));
        assert!(!is_prime(100));
    }

    #[test]
    fn test_miller_rabin() {
        assert!(miller_rabin(2));
        assert!(miller_rabin(104729));
        assert!(!miller_rabin(104730));
        assert!(miller_rabin(999999937));
    }
}
