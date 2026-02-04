"""Project Euler Problem 740: Secret Santa.

In a variant of Secret Santa, each of N players has two slips of paper, each
with their name, mixed in a hat. Each player selects two slips of paper at
random, potentially multiple times until they don't select a piece of paper
with their own name. Find the probability that the last person is unable to
finish this procedure because at least of the remaining two slips of paper
has their own name.

We use dynamic programming, where the state is the number of remaining
players ns[k] who still have k slips of paper with their name still in the
hat, for k=0,1,2. Due to symmetry, the answer is equivalent if we select
the next player randomly each turn. So for a random player, we iterate over
all possible types of the two slips of paper that player draws.
"""

from __future__ import annotations

from functools import lru_cache
from typing import Tuple


def npr(n: int, r: int) -> int:
    """Permutations P(n, r)."""
    result = 1
    for i in range(r):
        result *= n - i
    return result


@lru_cache(maxsize=None)
def f(ns: Tuple[int, ...]) -> float:
    """Compute probability recursively."""
    ns_list = list(ns)
    if any(n < 0 for n in ns_list):
        return 0.0

    sum_ns = sum(ns_list)
    if sum_ns == 1:
        return 1.0 if ns_list[0] == 0 else 0.0

    slips = [
        2 * ns_list[0] + ns_list[1],
        ns_list[1],
        2 * ns_list[2],
        1,
    ]

    result = 0.0
    for p in range(3):
        for s1 in range(3):
            for s2 in range(4 if s1 == 2 else 3):
                num_s1 = slips[s1] - (s1 == p and p or 0)
                num_s2 = slips[s2] - (s2 == p and p or 0)
                if s1 == s2:
                    num_s2 -= 2 if s1 == 2 else 1

                if num_s1 <= 0 or num_s2 <= 0:
                    continue

                new_ns = list(ns_list)
                new_ns[p] -= 1
                if s1 != 0:
                    new_ns[s1] -= 1
                    new_ns[0 if s2 == 3 else s1 - 1] += 1
                if s2 != 0 and s2 != 3:
                    new_ns[s2] -= 1
                    new_ns[s2 - 1] += 1

                result += (
                    f(tuple(new_ns))
                    * num_s1
                    * num_s2
                    / npr(2 * sum_ns - p, 2)
                    * ns_list[p]
                )

    return result / sum_ns


def solve() -> float:
    """Solve Problem 740."""
    n = 100
    return f((0, 0, n))


def main() -> int:
    """Main entry point."""
    result = solve()
    print(f"{result:.10f}")
    return int(result * 1e10)


if __name__ == "__main__":
    main()
