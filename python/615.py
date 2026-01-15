"""Project Euler Problem 615: The millionth number with at least one million
prime factors.

Find the Nth number with at least N prime factors.

We find all numbers with at least N prime factors below a given upper bound,
and gradually increase the upper bound until at least N numbers are found.
"""

from __future__ import annotations

from dataclasses import dataclass
from math import log

from sympy import primerange


@dataclass
class Result:
    """Result with log value and modular value."""

    log_val: float
    mod: int


def solve() -> int:
    """Solve Problem 615."""
    N = 1_000_000
    M = 123454321
    primes = list(primerange(2, N + 2))

    limit = N * log(2)
    while True:
        results: list[Result] = []
        helper(0, 0, 0.0, 1, limit, results, primes, M, N)
        if len(results) >= N:
            results.sort(key=lambda r: r.log_val)
            return results[N - 1].mod
        limit += 1


def helper(
    min_index: int,
    num_primes: int,
    log_val: float,
    mod: int,
    limit: float,
    results: list[Result],
    primes: list[int],
    M: int,
    N: int,
) -> None:
    """Recursive helper to find numbers with at least N prime factors."""
    if num_primes >= N:
        results.append(Result(log_val, mod))
    for index in range(min_index, len(primes)):
        p = primes[index]
        log_p = log(p)
        if log_val + max(N - num_primes, 1) * log_p > limit:
            break
        new_mod = mod * p % M
        e = 1
        while log_val + e * log_p < limit:
            helper(
                index + 1,
                num_primes + e,
                log_val + e * log_p,
                new_mod,
                limit,
                results,
                primes,
                M,
                N,
            )
            e += 1
            new_mod = new_mod * p % M


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
