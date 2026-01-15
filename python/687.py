"""Project Euler Problem 687: Shuffling Cards.

Given a random shuffle of a deck of cards, call a rank "perfect" if none of
the 4 cards of that rank are next to each other. Find the probability that the
number of perfect ranks is prime.

We use DP and compute the following generalization: assume that we have already
shuffled some cards, and p ranks are still perfect (no cards of that rank have
been shuffled next to each other). Out of those p ranks, r_0 have no cards
remaining (they have all been shuffled, just not next to each other), r_1 have
one card remaining, up to r_4 which have all four cards remaining. There are u
other cards in ranks that are definitely not perfect. Finally, the top card so
far is in a particular category: i > 0 if its rank is still perfect and there
are i of that rank remaining, and 0 otherwise. Given this state, what is the
probability that the number of perfect ranks at the end is prime?

There are a few possibilities for the next card.
- Firstly, it can be one of the u cards of imperfect ranks. We simply decrement
  u and recurse.
- The next card has the same rank as the last one. This is only possible if its
  category j is equal to i, and we can choose any of the j cards. In this case
  we decrement p, decrement r_j, and add j-1 to the number of other cards u.
- The next card has a different rank. We still decrement r_j, but this time this
  rank is still perfect and we need to increment r_{j-1}. There are j * r_j
  possible cards to choose, unless i = j, in which case there are only
  j * (r_j - 1) possible cards.

Finally, if all perfect ranks are accounted for (p = r_0) and there are no
other cards (i = 0), then we add 1 if the number of perfect ranks is prime.
This sum gives the number of shuffles with a prime number of perfect ranks, and
at the end we divide by 52! to get the probability. To solve the original
problem, we initialize with r_4 = 13 and p = 13.
"""

from __future__ import annotations

from dataclasses import dataclass
from functools import lru_cache
from typing import Tuple


@dataclass(frozen=True)
class Key:
    """State key for memoization."""

    r: Tuple[int, ...]
    i: int
    u: int
    p: int


def build_primes(limit: int) -> list[bool]:
    """Build prime sieve."""
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, int(limit**0.5) + 1):
        if is_prime[i]:
            for j in range(i * i, limit + 1, i):
                is_prime[j] = False
    return is_prime


def factorial(n: int) -> float:
    """Factorial."""
    result = 1.0
    for i in range(1, n + 1):
        result *= i
    return result


def solve() -> float:
    """Solve Problem 687."""
    N = 13
    K = 4
    limit = N * K
    is_prime = build_primes(limit)

    @lru_cache(maxsize=None)
    def helper(key: Key) -> float:
        """Helper function with memoization."""
        r, i, u, p = key.r, key.i, key.u, key.p
        if p == r[0] and u == 0:
            return 1.0 if is_prime[p] else 0.0

        res = 0.0
        if u > 0:
            new_key = Key(r, 0, u - 1, p)
            res = u * helper(new_key)

        for j in range(1, len(r)):
            r_j = r[j]
            if r_j > 0:
                if j == i:
                    new_r = list(r)
                    new_r[j] = r_j - 1
                    new_key = Key(tuple(new_r), 0, u + j - 1, p - 1)
                    res += j * helper(new_key)

                new_r = list(r)
                new_r[j] = r_j - 1
                new_r[j - 1] = new_r[j - 1] + 1
                multiplier = j * (r_j - 1 if j == i else r_j)
                new_key = Key(tuple(new_r), j - 1, u, p)
                res += multiplier * helper(new_key)

        return res

    r = [0] * (K + 1)
    r[K] = N
    initial_key = Key(tuple(r), 0, 0, N)
    result = helper(initial_key)
    total_permutations = factorial(N * K)
    return result / total_permutations


def main() -> int:
    """Main entry point."""
    result = solve()
    print(f"{result:.10f}")
    return int(result * 10**10)


if __name__ == "__main__":
    main()
