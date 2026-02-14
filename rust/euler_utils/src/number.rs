use num_integer::Integer;

/// Greatest common divisor (unsigned).
pub fn gcd(a: u64, b: u64) -> u64 {
    a.gcd(&b)
}

/// Greatest common divisor (signed i64).
pub fn gcd_i64(a: i64, b: i64) -> i64 {
    a.gcd(&b)
}

/// Greatest common divisor (signed i32).
pub fn gcd_i32(a: i32, b: i32) -> i32 {
    a.gcd(&b)
}

/// Least common multiple.
pub fn lcm(a: u64, b: u64) -> u64 {
    a.lcm(&b)
}

/// Euler's totient function φ(n).
pub fn euler_phi(mut n: u64) -> u64 {
    let mut result = n;
    let mut p = 2u64;
    while p * p <= n {
        if n % p == 0 {
            while n % p == 0 {
                n /= p;
            }
            result -= result / p;
        }
        p += 1;
    }
    if n > 1 {
        result -= result / n;
    }
    result
}

/// Return all divisors of n in sorted order.
pub fn divisors(n: u64) -> Vec<u64> {
    if n == 0 {
        return vec![];
    }
    let mut small = Vec::new();
    let mut large = Vec::new();
    let mut i = 1u64;
    while i * i <= n {
        if n % i == 0 {
            small.push(i);
            if i != n / i {
                large.push(n / i);
            }
        }
        i += 1;
    }
    large.reverse();
    small.extend(large);
    small
}

/// Count of divisors of n.
pub fn divisor_count(mut n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut count = 1u64;
    let mut p = 2u64;
    while p * p <= n {
        let mut exp = 0u64;
        while n % p == 0 {
            n /= p;
            exp += 1;
        }
        count *= exp + 1;
        p += 1;
    }
    if n > 1 {
        count *= 2;
    }
    count
}

/// Sum of divisors of n.
pub fn divisor_sum(mut n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut sum = 1u64;
    let mut p = 2u64;
    while p * p <= n {
        if n % p == 0 {
            let mut pk = 1u64;
            let mut s = 0u64;
            while n % p == 0 {
                n /= p;
                pk *= p;
                s += pk;
            }
            s += 1;
            sum *= s;
        }
        p += 1;
    }
    if n > 1 {
        sum *= n + 1;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(gcd(7, 13), 1);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(4, 6), 12);
        assert_eq!(lcm(3, 7), 21);
    }

    #[test]
    fn test_euler_phi() {
        assert_eq!(euler_phi(1), 1);
        assert_eq!(euler_phi(10), 4);
        assert_eq!(euler_phi(12), 4);
        assert_eq!(euler_phi(7), 6);
    }

    #[test]
    fn test_divisors() {
        assert_eq!(divisors(12), vec![1, 2, 3, 4, 6, 12]);
        assert_eq!(divisors(7), vec![1, 7]);
    }

    #[test]
    fn test_divisor_count() {
        assert_eq!(divisor_count(12), 6);
        assert_eq!(divisor_count(1), 1);
        assert_eq!(divisor_count(7), 2);
    }

    #[test]
    fn test_divisor_sum() {
        assert_eq!(divisor_sum(12), 28); // 1+2+3+4+6+12
        assert_eq!(divisor_sum(1), 1);
    }
}
