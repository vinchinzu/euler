"""Project Euler Problem 239: Twenty-two Foolish Primes.

Find the probability that a random permutation of disks from 1 to N inclusive
will result in exactly K prime disks not being in their natural positions.
"""

from __future__ import annotations

from math import factorial
from typing import List


def sieve(limit: int) -> List[int]:
    """Generate all primes up to limit."""
    if limit < 2:
        return []
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, int(limit**0.5) + 1):
        if is_prime[i]:
            for j in range(i * i, limit + 1, i):
                is_prime[j] = False
    return [i for i in range(limit + 1) if is_prime[i]]


def parity(n: int) -> int:
    """Return 1 if n is even, -1 if odd."""
    return 1 if n % 2 == 0 else -1


def fn_cr(a: float, b: int) -> float:
    """Compute C(a, b) as float."""
    if b < 0 or a < b:
        return 0.0
    result = 1.0
    for i in range(b):
        result *= (a - i) / (i + 1)
    return result


def ffactorial(n: int) -> float:
    """Return n! as float."""
    return float(factorial(n))


def solve() -> float:
    """Solve Problem 239."""
    N = 100
    K = 22

    num_primes = len(sieve(N))
    ans = 0.0

    for k in range(num_primes + 1):
        ans -= (
            parity(k)
            * fn_cr(k, num_primes - K)
            * fn_cr(num_primes, k)
            * ffactorial(N - k)
        )

    ans /= ffactorial(N)
    return ans


def main() -> float:
    """Main entry point."""
    result = solve()
    print(f"{result:.12f}")
    return result


if __name__ == "__main__":
    main()
