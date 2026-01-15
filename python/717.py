"""Project Euler Problem 717: Summation of a Modular Formula.

Let g(p) = ⌊2^(2^p) / p⌋ (mod 2^p) (mod p). Find the sum of g(p) for all
odd primes p ≤ N.

Let k ∈ [0, p-1] such that k ≡ 2^(2^p - p) (mod p). To avoid negative
exponents, we instead compute 2^(2^p + p - 2) (mod p), which is equivalent by
Fermat's Little Theorem.

We can see that g(p) ≡ ⌊k 2^p / p⌋ (mod 2^p) (mod p).

But ⌊k 2^p / p⌋ ∈ [0, 2^p - 1], so the (mod 2^p) is unnecessary:
g(p) ≡ ⌊k 2^p / p⌋ (mod p).

This means that we need to compute the numerator (mod p²).
"""

from __future__ import annotations

from math import isqrt


def sieve(limit: int) -> list[int]:
    """Generate all primes up to limit."""
    if limit < 2:
        return []
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, isqrt(limit) + 1):
        if is_prime[i]:
            for j in range(i * i, limit + 1, i):
                is_prime[j] = False
    return [i for i in range(limit + 1) if is_prime[i]]


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Fast exponentiation modulo mod."""
    result = 1
    base = base % mod
    while exp > 0:
        if exp & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp >>= 1
    return result


def g(p: int) -> int:
    """Compute g(p)."""
    # Compute 2^(2^p + p - 2) mod p
    exp_mod_p_minus_1 = pow_mod(2, p, p - 1)
    exp_mod_p = pow_mod(2, exp_mod_p_minus_1 + p - 2, p)

    # Compute 2^p mod p^2
    p_squared = p * p
    two_to_p_mod_p2 = pow_mod(2, p, p_squared)

    # Compute k * 2^p mod p^2
    numerator = (exp_mod_p * two_to_p_mod_p2) % p_squared

    # Compute floor(k * 2^p / p) mod p
    return (numerator // p) % p


def solve() -> int:
    """Solve Problem 717."""
    n = 10**7
    primes = sieve(n)
    # Filter odd primes (skip 2)
    odd_primes = [p for p in primes if p > 2 and p <= n]
    ans = 0
    for p in odd_primes:
        ans += g(p)
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
