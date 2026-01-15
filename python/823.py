"""Project Euler Problem 823: Factor Shuffle.

A list contains the numbers 2 to N (inclusive) and at each round, every
number is divided by its smallest prime factor, and the product of all
those prime factors is added to the list as a new number, with all 1s
removed from the list. Find the sum of all numbers after K rounds.

We focus on each prime factor individually, with an artificial tiebreak
value for copies of the same factor. The position of this factor
eventually becomes a cycle, after a number of rounds equal to about the
total number of factors. We shuffle for twice this number to include an
adequate buffer. If we sort all prime factors for each number in
decreasing order, then the focused factor will move towards the end of the
list after each step, and then become the largest value in a new number.
So we can compute the period by adding the number of smaller factors in
the current number, with the number of numbers with more factors.

After computing the periods, we can then shuffle the list some k more
times, for each period k, to find the position of each factor that has
period k. This gives us the positions of all factors, after which we can
sum the factor products to get the answer.
"""

from __future__ import annotations

from collections import defaultdict
from dataclasses import dataclass
from math import isqrt
from typing import Dict, List


@dataclass
class Factor:
    """Factor with value and tiebreak."""

    value: int
    tiebreak: int

    def __lt__(self, other: "Factor") -> bool:
        """Compare by value descending, then tiebreak descending."""
        if self.value != other.value:
            return self.value > other.value
        return self.tiebreak > other.tiebreak


def sieve_spf(limit: int) -> List[int]:
    """Smallest prime factor sieve."""
    spf = list(range(limit + 1))
    spf[0] = spf[1] = 0
    for i in range(2, isqrt(limit) + 1):
        if spf[i] == i:
            for j in range(i * i, limit + 1, i):
                if spf[j] == j:
                    spf[j] = i
    return spf


def prime_factorize(n: int, spf: List[int]) -> Dict[int, int]:
    """Factorize n using SPF."""
    factors: Dict[int, int] = {}
    while n > 1:
        p = spf[n]
        factors[p] = factors.get(p, 0) + 1
        n //= p
    return factors


def imod(a: int, m: int) -> int:
    """Integer mod."""
    return ((a % m) + m) % m


def shuffle_factors(all_factors: List[List[Factor]]) -> List[List[Factor]]:
    """Shuffle factors according to problem rules."""
    new_all_factors: List[List[Factor]] = []
    new_factors: List[Factor] = []
    new_all_factors.append(new_factors)

    for factors in all_factors:
        if factors:
            new_factors.append(factors.pop())
            if factors:
                new_all_factors.append(factors)

    new_factors.sort()
    return new_all_factors


def solve() -> int:
    """Solve Problem 823."""
    N = 10000
    K = 10**16
    M = 1234567891

    spf = sieve_spf(N)
    all_factors: List[List[Factor]] = []
    num_factors = 0

    for n in range(2, N + 1):
        factors_dict = prime_factorize(n, spf)
        factors: List[Factor] = []
        for p, e in factors_dict.items():
            for i in range(e):
                factors.append(Factor(p, i * N + n))
        factors.sort()
        all_factors.append(factors)
        num_factors += len(factors)

    # Shuffle for buffer
    l = 2 * num_factors
    for _ in range(l):
        all_factors = shuffle_factors(all_factors)

    # Compute periods
    factor_periods: Dict[int, List[Factor]] = defaultdict(list)
    for i, factors in enumerate(all_factors):
        for j in range(len(factors)):
            period = i + len(factors) - j
            key = imod(K - l, period)
            factor_periods[key].append(factors[j])

    if not factor_periods:
        return 0

    max_period = max(factor_periods.keys())
    final_factors: List[List[int]] = []

    for k in range(max_period + 1):
        for factor in factor_periods.get(k, []):
            # Find position of factor
            pos = -1
            for idx, factors_list in enumerate(all_factors):
                if any(f.value == factor.value and f.tiebreak == factor.tiebreak for f in factors_list):
                    pos = idx
                    break
            if pos >= 0:
                while len(final_factors) <= pos:
                    final_factors.append([])
                final_factors[pos].append(factor.value)
        all_factors = shuffle_factors(all_factors)

    ans = 0
    for factors in final_factors:
        prod = 1
        for factor in factors:
            prod = (prod * factor) % M
        ans = (ans + prod) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
