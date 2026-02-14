// Project Euler 160 - Factorial trailing digits
//
// Compute last 5 non-zero digits of (10^12)!
// Uses CRT: compute mod 32 and mod 3125 separately.

fn mod_pow(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut result = 1i64;
    base = ((base % m) + m) % m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % m;
        }
        base = base * base % m;
        exp >>= 1;
    }
    result
}

fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        return (b, 0, 1);
    }
    let (g, x1, y1) = extended_gcd(b % a, a);
    (g, y1 - (b / a) * x1, x1)
}

fn mod_inverse(a: i64, m: i64) -> i64 {
    let (_, x, _) = extended_gcd(a, m);
    ((x % m) + m) % m
}

fn prime_exponent(p: i64, n: i64) -> i64 {
    let mut exp = 0i64;
    let mut power = p;
    while power <= n {
        exp += n / power;
        if power > n / p {
            break;
        }
        power *= p;
    }
    exp
}

/// Compute (n! / p^{v_p(n!)}) mod p^k recursively
fn factorial_p_free(n: i64, p: i64, pk: i64) -> i64 {
    if n == 0 {
        return 1;
    }

    // Product of numbers in [1, pk) not divisible by p
    let mut prod_cycle = 1i64;
    for i in 1..pk {
        if i % p != 0 {
            prod_cycle = prod_cycle * i % pk;
        }
    }

    let mut res = mod_pow(prod_cycle, n / pk, pk);

    // Remaining terms
    let remainder = n % pk;
    for i in 1..=remainder {
        if i % p != 0 {
            res = res * i % pk;
        }
    }

    // Recurse for terms divisible by p
    res = res * factorial_p_free(n / p, p, pk) % pk;
    res
}

fn crt(a1: i64, m1: i64, a2: i64, m2: i64) -> i64 {
    let inv = mod_inverse(m1, m2);
    let u = ((a2 - a1) % m2 + m2) % m2;
    let u = u * inv % m2;
    (a1 + m1 * u) % (m1 * m2)
}

fn main() {
    let n: i64 = 1_000_000_000_000; // 10^12
    let mod2: i64 = 32; // 2^5
    let mod5: i64 = 3125; // 5^5

    let exp2 = prime_exponent(2, n);
    let exp5 = prime_exponent(5, n);

    // Compute mod 3125
    let term_five_free = factorial_p_free(n, 5, 3125);
    let term_two_inv = mod_inverse(mod_pow(2, exp2, 3125), 3125);
    let m_mod_3125 = term_five_free * term_two_inv % 3125;
    let res_mod_3125 = m_mod_3125 * mod_pow(2, exp2 - exp5, 3125) % 3125;

    let res_mod_32: i64 = 0;

    let result = crt(res_mod_32, mod2, res_mod_3125, mod5);

    println!("{:05}", result);
}
