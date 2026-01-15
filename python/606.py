"""Project Euler Problem 606: Gozinta Chains.

A Gozinta chain for n is a sequence {1,...,n} such that each element divides
the next. Find the number of integers n ≤ N with exactly 252 distinct Gozinta
chains.

Only integers of the form n = p³q³ for primes p<q have 252 Gozinta chains. For
each prime p, we compute the sum of the cubes of all primes q from p exclusive
to n/p inclusive, and multiply by p³.
"""

from __future__ import annotations

from math import isqrt

from sympy import primerange


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Modular exponentiation."""
    result = 1
    base %= mod
    while exp > 0:
        if exp & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp >>= 1
    return result


def sum_prime_powers_simple(n: int, power: int, mod: int) -> dict[int, int]:
    """Simplified sum of prime powers - returns mapping for quotient values."""
    # For this problem, we need sum of p^3 for primes p <= n
    # Simplified version: compute directly
    primes = list(primerange(2, n + 1))
    result = {}
    cumulative = 0
    for p in primes:
        cumulative = (cumulative + pow_mod(p, power, mod)) % mod
        result[p] = cumulative
    return result


def solve() -> int:
    """Solve Problem 606."""
    N = 10**36
    M = 10**9
    L = int(N ** (1 / 3))

    primes_list = list(primerange(2, isqrt(L) + 1))
    sum_powers = sum_prime_powers_simple(L, 3, M)

    ans = 0
    for p in primes_list:
        if p * p * p > L:
            break
        p3 = pow_mod(p, 3, M)
        # Sum of cubes of primes q where p < q <= L/p
        q_max = L // p
        if q_max >= p:
            # Get sum for q_max and subtract sum for p
            sum_val = sum_powers.get(q_max, 0) - sum_powers.get(p, 0)
            ans = (ans + p3 * sum_val) % M
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
