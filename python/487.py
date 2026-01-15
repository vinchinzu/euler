"""Project Euler Problem 487: Sums of power sums.

Let f_k(n) be the sum of the k'th powers of the first n positive integers, and
let S_k(n) = Σ_{i=1}^n f_k(i). Find Σ S_K(N) (mod p) for all primes p with
L ≤ p ≤ H.
"""

from __future__ import annotations

from math import isqrt
from typing import List


def is_probable_prime(n: int) -> bool:
    """Check if n is prime."""
    if n < 2:
        return False
    for i in range(2, isqrt(n) + 1):
        if n % i == 0:
            return False
    return True


def sum_powers(n: int, k: int, mod: int) -> int:
    """Sum of k-th powers from 1 to n modulo mod."""
    result = 0
    for i in range(1, n + 1):
        result = (result + pow(i, k, mod)) % mod
    return result


def solve() -> int:
    """Solve Problem 487."""
    N = 10**12
    K = 10_000
    L = 2 * 10**9
    H = 2 * 10**9 + 2000

    ans = 0
    for p in range(L, H):
        if is_probable_prime(p):
            term = ((N + 1) % p * sum_powers(N, K, p) - sum_powers(N, K + 1, p)) % p
            ans = (ans + term) % p

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
