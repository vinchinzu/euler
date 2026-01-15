"""Project Euler Problem 333: Special partitions.

In this problem we consider partitions of positive integers into parts of the
form 2**i * 3**j (i, j ≥ 0). A partition is called "special" if no part divides
any other part in the partition. For a prime q define P(q) as the number of
special partitions of q. The task is to compute the sum of primes q < 1_000_000
for which P(q) = 1.
"""

from __future__ import annotations

from math import isqrt
from typing import Dict, List, Tuple

LIMIT: int = 1_000_000


def generate_terms(limit: int) -> List[Tuple[int, int, int]]:
    """Return (value, exp2, exp3) for all 2**exp2 * 3**exp3 <= limit, excluding 1."""

    terms: List[Tuple[int, int, int]] = []
    value2 = 1
    exp2 = 0
    while value2 <= limit:
        value3 = value2
        exp3 = 0
        while value3 <= limit:
            if value3 > 1:
                terms.append((value3, exp2, exp3))
            value3 *= 3
            exp3 += 1
        value2 *= 2
        exp2 += 1

    terms.sort(key=lambda item: (item[1], -item[2]))  # exp2 asc, exp3 desc
    return terms


def enumerate_special_sums(limit: int) -> List[int]:
    """Enumerate sums obtainable by special partitions up to ``limit``."""

    terms = generate_terms(limit)
    if not terms:
        return [0] * (limit + 1)

    values = [term[0] for term in terms]
    exp2 = [term[1] for term in terms]
    exp3 = [term[2] for term in terms]
    n = len(terms)

    predecessors: List[List[int]] = [[] for _ in range(n)]
    for j in range(n):
        exp2_j = exp2[j]
        exp3_j = exp3[j]
        for i in range(j):
            if exp2[i] < exp2_j and exp3[i] > exp3_j:
                predecessors[j].append(i)

    counts = [0] * (limit + 1)
    dp: List[Dict[int, int]] = [{} for _ in range(n)]

    for idx in range(n):
        value = values[idx]
        current: Dict[int, int] = {}
        if value <= limit:
            current[value] = 1

        for pred in predecessors[idx]:
            for sum_value, ways in dp[pred].items():
                new_sum = sum_value + value
                if new_sum > limit:
                    continue
                current[new_sum] = current.get(new_sum, 0) + ways

        dp[idx] = current
        for sum_value, ways in current.items():
            counts[sum_value] += ways

    return counts


def sieve_primes(limit: int) -> List[int]:
    """Return primes strictly less than ``limit``."""

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


def solve(limit: int = LIMIT) -> int:
    """Return sum of primes q < limit for which there is exactly one special partition."""

    counts = enumerate_special_sums(limit)
    total = 0
    for prime in sieve_primes(limit):
        if counts[prime] == 1:
            total += prime
    return total


if __name__ == "__main__":
    print(solve())
