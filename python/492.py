"""Project Euler Problem 492: Exploding sequence.

Define the sequence a_1 = 1 and a_{n+1} = 6(a_n)² + 10(a_n) + 3. Find Σ_p
a_N (mod p) for all primes X ≤ p ≤ X+Y.
"""

from __future__ import annotations

from math import isqrt
from typing import List


def sieve_primes(limit: int) -> List[int]:
    """Sieve of Eratosthenes."""
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, isqrt(limit) + 1):
        if is_prime[i]:
            for j in range(i * i, limit + 1, i):
                is_prime[j] = False
    return [i for i in range(limit + 1) if is_prime[i]]


def matrix_multiply_2x2(
    a: List[int], b: List[int], mod: int
) -> List[int]:
    """Multiply two 2x2 matrices."""
    return [
        (a[0] * b[0] + a[1] * b[2]) % mod,
        (a[0] * b[1] + a[1] * b[3]) % mod,
        (a[2] * b[0] + a[3] * b[2]) % mod,
        (a[2] * b[1] + a[3] * b[3]) % mod,
    ]


def pow_2x2(matrix: List[int], exp: int, mod: int) -> List[int]:
    """Matrix exponentiation."""
    result = [1, 0, 0, 1]
    base = matrix[:]
    while exp > 0:
        if exp % 2 == 1:
            result = matrix_multiply_2x2(result, base, mod)
        base = matrix_multiply_2x2(base, base, mod)
        exp //= 2
    return result


def solve() -> int:
    """Solve Problem 492."""
    N = 10**15
    X = 10**9
    Y = 10**7

    # Sieve primes in range [X, X+Y]
    is_prime = [True] * (Y + 1)
    small_primes = sieve_primes(isqrt(X + Y))
    for p in small_primes:
        start = (-X) % p
        for i in range(start, Y + 1, p):
            is_prime[i] = False

    A = [0, 1, -1, 11]
    ans = 0

    for i in range(Y + 1):
        if is_prime[i]:
            p = X + i
            # Check period
            period = p - 1 if pow_2x2(A, p - 1, p) == [1, 0, 0, 1] else p + 1
            exp = pow(2, N - 1, period)
            vec = pow_2x2(A, exp, p)
            x_n = (vec[0] * 2 + vec[1] * 11) % p
            a_n = ((x_n - 5) * pow(6, p - 2, p)) % p
            ans = (ans + a_n) % p

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
