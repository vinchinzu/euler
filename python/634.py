"""Project Euler Problem 634: Numbers of the form a²b³.

Find the number of integers x ≤ N that can be expressed as x = a²b³ for a,b > 1.

If x is not a perfect square, then it can be written uniquely in the form
x = a²b³ for some square-free b, so we can go through all square-free b and
count ⌊N / b³⌋ - 1 (subtracting one to skip a = 1) without worrying about
duplicates.

If x is a perfect square, then it can be written in that form as long as a is
divisible by some prime cube, but is not a prime cube itself. This can be
computed by inclusion exclusion.
"""

from __future__ import annotations

from math import isqrt

from sympy import isprime, primerange


def mobius_sieve(limit: int) -> list[int]:
    """Möbius function sieve."""
    mobius = [1] * (limit + 1)
    is_prime = [True] * (limit + 1)
    for i in range(2, limit + 1):
        if is_prime[i]:
            for j in range(i, limit + 1, i):
                is_prime[j] = False
                mobius[j] *= -1
            for j in range(i * i, limit + 1, i * i):
                mobius[j] = 0
    return mobius


def solve() -> int:
    """Solve Problem 634."""
    N = 9 * 10**18
    L = int(N ** (1 / 3))

    mobius = mobius_sieve(L)
    primes = list(primerange(2, L + 1))

    ans = 0
    # Count non-square numbers
    b = 2
    while b * b * b <= N:
        if mobius[b] != 0:
            ans += isqrt(N // (b * b * b)) - 1
        b += 1

    # Subtract perfect squares that are also a²b³
    i = 2
    while pow(i, 6) <= N:
        ans -= mobius[i] * (isqrt(N) // (i * i * i))
        if isprime(i):
            ans -= 1
        i += 1

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
