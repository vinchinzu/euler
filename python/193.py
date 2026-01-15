"""Project Euler Problem 193: Squarefree Numbers."""

from typing import List
import math

LIMIT = 1 << 50


def mobius(limit: int) -> List[int]:
    """Möbius function up to n via sieve."""
    mu: List[int] = [0] * (limit + 1)
    mu[1] = 1
    primes: List[int] = []
    composite = [False] * (limit + 1)

    for i in range(2, limit + 1):
        if not composite[i]:
            primes.append(i)
            mu[i] = -1
        for p in primes:
            if i * p > limit:
                break
            composite[i * p] = True
            if i % p == 0:
                mu[i * p] = 0
                break
            else:
                mu[i * p] = -mu[i]

    return mu


def squarefree_count(n: int, mu: List[int]) -> int:
    """Count squarefree numbers ≤ n using Möbius function."""
    limit = int(math.sqrt(n))
    total = 0
    for d in range(1, limit + 1):
        if mu[d] == 0:
            continue
        total += mu[d] * (n // (d * d))
    return total


def main() -> int:
    """Main function."""
    sqrt_limit = int(math.sqrt(LIMIT))
    mu = mobius(sqrt_limit)
    return squarefree_count(LIMIT, mu)


if __name__ == "__main__":
    print(main())
