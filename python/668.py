"""Project Euler Problem 668: Square Root Smooth Numbers.

Find the number of square root smooth numbers up to N, i.e. a number such that
all its prime factors are strictly less than its square root.

For every prime p, we need to remove p*i for all i ≤ p. This means that for
primes p ≤ √N, we need to remove p values, and for primes p > √N, we need to
remove ⌊N / p⌋ values.
"""

from __future__ import annotations

from math import isqrt

from sympy import primerange


def sum_prime_powers_div(n: int, d: int) -> int:
    """Count primes p such that ⌊n/p⌋ = d."""
    min_p = n // (d + 1) + 1
    max_p = n // d
    if min_p > max_p:
        return 0
    primes = list(primerange(min_p, max_p + 1))
    return len(primes)


def solve() -> int:
    """Solve Problem 668."""
    N = 10**10
    L = isqrt(N)

    ans = N
    # Remove primes p ≤ N/L
    primes_small = list(primerange(2, N // L + 1))
    for p in primes_small:
        ans -= p

    # Remove primes p > √N
    for d in range(1, L):
        count = sum_prime_powers_div(N, d)
        ans -= d * count

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
