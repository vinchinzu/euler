"""Project Euler Problem 234: Semidivisible numbers.

An integer n is semi-divisible if either the largest prime ≤ √n or the
smallest prime ≥ √n is divisible by n, but not both. Find the number of
integers not exceeding N that are not semi-divisible.
"""

from __future__ import annotations

from math import isqrt
from typing import List


def sieve(limit: int) -> List[int]:
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


def sq(n: int) -> int:
    """Return n squared."""
    return n * n


def round_up(n: float, k: int) -> int:
    """Round n up to nearest multiple of k."""
    return ((int(n) + k - 1) // k) * k


def round_down(n: float, k: int) -> int:
    """Round n down to nearest multiple of k."""
    return (int(n) // k) * k


def solve() -> int:
    """Solve Problem 234."""
    N = 999966663333
    primes_list = sieve(4 * isqrt(N))
    ans = 0

    for i in range(len(primes_list) - 1):
        prev_p = primes_list[i - 1] if i > 0 else 0
        p = primes_list[i]
        next_p = primes_list[i + 1]

        min_val = round_up(max(4, sq(prev_p)), p)
        max_val = round_down(min(N, sq(next_p)), p)

        if min_val > max_val:
            continue

        count = (max_val - min_val) // p + 1
        ans += (max_val + min_val) * count // 2

        # Subtract special cases
        for q in [prev_p, p, next_p]:
            if p * q <= N:
                ans -= p * q

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
