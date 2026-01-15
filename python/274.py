"""Project Euler Problem 274: Divisibility Multipliers.

Find the sum of the divisibility multipliers of all primes up to N that
are relatively prime to 10.
"""

from __future__ import annotations

from math import gcd, isqrt


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


def mod_inverse(a: int, m: int) -> int:
    """Modular inverse."""
    g, x, _ = ext_gcd(a, m)
    if g != 1:
        return 0
    return (x % m + m) % m


def ext_gcd(a: int, b: int) -> tuple[int, int, int]:
    """Extended Euclidean algorithm."""
    if b == 0:
        return (a, 1, 0)
    g, x1, y1 = ext_gcd(b, a % b)
    return (g, y1, x1 - (a // b) * y1)


def solve() -> int:
    """Solve Problem 274."""
    N = 10**7
    B = 10

    primes_list = sieve(N)
    ans = 0

    for p in primes_list:
        if gcd(p, B) == 1:
            ans += mod_inverse(B, p)

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
