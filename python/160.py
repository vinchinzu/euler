"""Project Euler Problem 160: Factorial trailing digits."""

from typing import Tuple


MOD = 100_000
MOD2 = 32  # 2^5
MOD5 = 3125  # 5^5
N = 1_000_000_000_000


def extended_gcd(a: int, b: int) -> Tuple[int, int, int]:
    """Extended Euclidean Algorithm."""
    if a == 0:
        return (b, 0, 1)
    g, x1, y1 = extended_gcd(b % a, a)
    x = y1 - (b // a) * x1
    y = x1
    return (g, x, y)


def mod_inverse(a: int, m: int) -> int:
    """Compute modular inverse of a modulo m."""
    g, x, y = extended_gcd(a, m)
    if g != 1:
        raise ValueError("Modular inverse does not exist")
    return x % m


def mod_pow(base: int, exp: int, mod: int) -> int:
    """Binary exponentiation for modular power."""
    result = 1
    base = base % mod
    while exp > 0:
        if exp % 2 == 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp //= 2
    return result


def prime_exponent(p: int, n: int) -> int:
    """Compute the number of times prime p divides n! using de Polignac's formula."""
    exp = 0
    power = p
    while power <= n:
        exp += n // power
        if power > n // p:  # Avoid overflow
            break
        power *= p
    return exp


def factorial_p_free(n: int, p: int, pk: int) -> int:
    """Recursive helper to compute (n! / p^e_p) mod p^k."""
    if n == 0:
        return 1

    # Product of numbers in [1, pk) not divisible by p
    prod_cycle = 1
    for i in range(1, pk):
        if i % p != 0:
            prod_cycle = (prod_cycle * i) % pk

    res = mod_pow(prod_cycle, n // pk, pk)

    # Product of remaining terms
    for i in range(1, (n % pk) + 1):
        if i % p != 0:
            res = (res * i) % pk

    # Recursively handle the terms divisible by p
    res = (res * factorial_p_free(n // p, p, pk)) % pk

    return res


def chinese_remainder_theorem(a1: int, m1: int, a2: int, m2: int) -> int:
    """Solve system of congruences using Chinese Remainder Theorem."""
    inv = mod_inverse(m1, m2)
    u = (inv * (a2 - a1) % m2 + m2) % m2
    return (a1 + m1 * u) % (m1 * m2)


def factorial_nonzero_last_five(n: int) -> int:
    """Compute the last five non-zero digits of n!."""
    if n < 0 or not isinstance(n, int):
        raise ValueError("n must be non-negative integer")

    exp2 = prime_exponent(2, n)
    exp5 = prime_exponent(5, n)

    res_mod_32 = 0

    # Calculate (n! / 5^exp5) mod 3125
    term_five_free = factorial_p_free(n, 5, 3125)

    # We need to compute n! / (2^exp2 * 5^exp5) mod 3125
    # This is (n!/5^exp5) * inv(2^exp2) mod 3125
    term_two_inv = mod_inverse(mod_pow(2, exp2, 3125), 3125)

    # The product of all numbers up to n, with 2s and 5s removed
    m_mod_3125 = (term_five_free * term_two_inv) % 3125

    # Now, multiply by the excess twos
    res_mod_3125 = (m_mod_3125 * mod_pow(2, exp2 - exp5, 3125)) % 3125

    return chinese_remainder_theorem(res_mod_32, 32, res_mod_3125, 3125)


def main() -> int:
    """Main function."""
    result = factorial_nonzero_last_five(N)
    return result


if __name__ == "__main__":
    print(f"{main():05d}")
