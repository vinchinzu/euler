"""Project Euler Problem 515: Dissonant Numbers.

Let d(p, n, 0) be the inverse of n (mod p), and let d(p, n, k) =
Σ_{i=1}^n d(p, i, k-1) for k≥1. Find the sum of d(p, p-1, K) for all
primes A ≤ p < A+B.
"""

from __future__ import annotations

from math import isqrt


def is_probable_prime(n: int) -> bool:
    """Check if n is probably prime using trial division."""
    if n < 2:
        return False
    if n == 2:
        return True
    if n % 2 == 0:
        return False
    for i in range(3, isqrt(n) + 1, 2):
        if n % i == 0:
            return False
    return True


def mod_inv(a: int, m: int) -> int:
    """Compute modular inverse using extended Euclidean algorithm."""
    if m == 1:
        return 0
    m0, x0, x1 = m, 0, 1
    while a > 1:
        q = a // m
        m, a = a % m, m
        x0, x1 = x1 - q * x0, x0
    return x1 % m0 if x1 < 0 else x1


def solve() -> int:
    """Solve Problem 515."""
    A = 10**9
    B = 10**5
    K = 10**5

    ans = 0
    for p in range(A, A + B):
        if is_probable_prime(p):
            ans += mod_inv(K - 1, p)

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
