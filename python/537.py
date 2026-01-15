"""Project Euler Problem 537: Counting tuples.

Find the number of K-tuples (x_i) such that Σ_i π(x_i) = N, where π(x) is
the prime-counting function.
"""

from __future__ import annotations

from typing import List


def sieve_primes(limit: int) -> List[int]:
    """Sieve of Eratosthenes."""
    if limit < 2:
        return []
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, int(limit**0.5) + 1):
        if is_prime[i]:
            for j in range(i * i, limit + 1, i):
                is_prime[j] = False
    return [i for i in range(limit + 1) if is_prime[i]]


def polynomial_pow(coeffs: List[int], k: int, mod: int) -> List[int]:
    """Compute polynomial power using convolution."""
    # Simple implementation - for small cases
    result = [0] * (len(coeffs) * k + 1)
    result[0] = 1

    for _ in range(k):
        new_result = [0] * len(result)
        for i in range(len(coeffs)):
            for j in range(len(result) - i):
                new_result[i + j] = (new_result[i + j] + coeffs[i] * result[j]) % mod
        result = new_result

    return result


def solve() -> int:
    """Solve Problem 537."""
    N = 20000
    K = 20000
    M = 1004535809

    primes = sieve_primes(N + 100)
    f = [0] * (N + 1)
    f[0] = 1
    for i in range(1, N + 1):
        if i < len(primes):
            f[i] = primes[i] - primes[i - 1]

    # Compute F^K and get coefficient of x^N
    result_poly = polynomial_pow(f, K, M)
    return result_poly[N] if N < len(result_poly) else 0


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
