// Project Euler 293: Pseudo-Fortunate Numbers
// An admissible number has consecutive prime factors starting from 2.
// Find sum of distinct pseudo-Fortunate numbers.

use euler_utils::is_prime;

const N: u64 = 1_000_000_000;
const SIEVE_SIZE: usize = 2_000_000;

fn main() {
    // Build small prime sieve
    let mut sieve = vec![true; SIEVE_SIZE];
    sieve[0] = false;
    sieve[1] = false;
    let mut i = 2;
    while i * i < SIEVE_SIZE {
        if sieve[i] {
            let mut j = i * i;
            while j < SIEVE_SIZE {
                sieve[j] = false;
                j += i;
            }
        }
        i += 1;
    }

    let is_prime_check = |n: u64| -> bool {
        if n < 2 { return false; }
        if (n as usize) < SIEVE_SIZE { return sieve[n as usize]; }
        is_prime(n)
    };

    let mut fortunates = std::collections::HashSet::new();

    let primes: [u64; 10] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];

    // Generate admissible numbers using nested loops (matching C approach)
    let mut v = 2u64;
    while v < N {
        let mut m = 2u64;
        while !is_prime_check(v + m) { m += 1; }
        fortunates.insert(m);

        let mut w = v * 3;
        while w < N {
            m = 2;
            while !is_prime_check(w + m) { m += 1; }
            fortunates.insert(m);

            let mut x = w * 5;
            while x < N {
                m = 2;
                while !is_prime_check(x + m) { m += 1; }
                fortunates.insert(m);

                let mut y = x * 7;
                while y < N {
                    m = 2;
                    while !is_prime_check(y + m) { m += 1; }
                    fortunates.insert(m);

                    let mut z = y * 11;
                    while z < N {
                        m = 2;
                        while !is_prime_check(z + m) { m += 1; }
                        fortunates.insert(m);

                        let mut a = z * 13;
                        while a < N {
                            m = 2;
                            while !is_prime_check(a + m) { m += 1; }
                            fortunates.insert(m);

                            let mut b = a * 17;
                            while b < N {
                                m = 2;
                                while !is_prime_check(b + m) { m += 1; }
                                fortunates.insert(m);

                                let mut c = b * 19;
                                while c < N {
                                    m = 2;
                                    while !is_prime_check(c + m) { m += 1; }
                                    fortunates.insert(m);

                                    let mut d = c * 23;
                                    while d < N {
                                        m = 2;
                                        while !is_prime_check(d + m) { m += 1; }
                                        fortunates.insert(m);
                                        d *= 23;
                                    }
                                    c *= 19;
                                }
                                b *= 17;
                            }
                            a *= 13;
                        }
                        z *= 11;
                    }
                    y *= 7;
                }
                x *= 5;
            }
            w *= 3;
        }
        v *= 2;
    }

    let _ = primes; // used conceptually

    let sum: u64 = fortunates.iter().sum();
    println!("{}", sum);
}
