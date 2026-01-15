"""Project Euler Problem 313: Sliding game.

In a sliding game a counter may slide horizontally or vertically into an empty
space. The objective of the game is to move the red counter from the top left
corner of a grid to the bottom right corner; the space always starts in the
bottom right corner. For example, the following sequence of pictures show how
the game can be completed in five moves on a 2 by 2 grid.

Let S(m, n) represent the minimum number of moves to complete the game on an
m by n grid. For example, it can be verified that S(5, 4) = 25.

There are exactly 5482 grids for which S(m, n) = p^2, where p < 100 is prime.

How many grids does S(m, n) = p^2, where p < 10^6 is prime?
"""

from __future__ import annotations

from math import isqrt
from typing import List

MAX_PRIME_LIMIT: int = 1_000_000
VERIFICATION_LIMIT: int = 100
VERIFICATION_COUNT: int = 5_482


def sieve_primes(limit: int) -> List[int]:
    """Return all primes strictly less than ``limit`` via a standard sieve."""

    if limit <= 2:
        return []

    sieve = bytearray(b"\x01") * limit
    sieve[0:2] = b"\x00\x00"

    upper = isqrt(limit - 1)
    for number in range(2, upper + 1):
        if sieve[number]:
            step = number
            start = number * number
            sieve[start:limit:step] = b"\x00" * (((limit - start - 1) // step) + 1)

    return [index for index in range(2, limit) if sieve[index]]


def count_prime_square_grids(max_prime: int) -> int:
    """Return number of (m, n) grids with S(m, n) == p^2 for primes p < max."""

    total = 0

    for prime in sieve_primes(max_prime):
        if prime == 2:
            continue

        square = prime * prime
        t_value = (square + 13) // 2
        a_min = t_value // 4 + 1
        a_max = (t_value - 2) // 3

        if a_max >= a_min:
            total += 2 * (a_max - a_min + 1)

    return total


def solve(max_prime: int = MAX_PRIME_LIMIT) -> int:
    """Compute the Project Euler 313 count for primes p < ``max_prime``."""

    if max_prime >= VERIFICATION_LIMIT:
        verification = count_prime_square_grids(VERIFICATION_LIMIT)
        if verification != VERIFICATION_COUNT:
            message = (
                f"Verification failed: expected {VERIFICATION_COUNT}, "
                f"found {verification}"
            )
            raise AssertionError(message)

    return count_prime_square_grids(max_prime)


if __name__ == "__main__":
    print(solve())
