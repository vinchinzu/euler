"""Project Euler Problem 733: Ascending subsequences.

For the given sequence, find the sum of the terms in all increasing subsequences
of length K.

We use two sets of binary index trees. The first set maintains for each i the
number of sequences of length i ending at a given term. The second set
maintains the sum of those sequences. It is easy to update the first set by
summing the number of sequences ending at any term less than the current term
a. Similarly, we can update the second set by taking count * a + sum, for all
sequences ending at a term less than a.
"""

from __future__ import annotations

from typing import List


class BIT:
    """Binary Indexed Tree (Fenwick Tree)."""

    def __init__(self, size: int, mod: int) -> None:
        """Initialize BIT with given size and modulus."""
        self.size = size
        self.mod = mod
        self.tree = [0] * (size + 1)

    def add(self, index: int, value: int) -> None:
        """Add value at index."""
        while index <= self.size:
            self.tree[index] = (self.tree[index] + value) % self.mod
            index += index & -index

    def sum(self, index: int) -> int:
        """Get prefix sum up to index."""
        result = 0
        while index > 0:
            result = (result + self.tree[index]) % self.mod
            index -= index & -index
        return result


def generate_sequence(n: int) -> List[int]:
    """Generate sequence: a_0 = 153, a_{n+1} = 153 * a_n mod L."""
    L = 10000019
    sequence: List[int] = []
    a = 153
    for _ in range(n):
        sequence.append(a)
        a = (a * 153) % L
    return sequence


def solve() -> int:
    """Solve Problem 733."""
    N = 1000000
    K = 4
    L = 10000019
    M = 10**9 + 7

    sequence = generate_sequence(N)

    # BIT arrays for counts and sums
    counts: List[BIT] = [BIT(L, M) for _ in range(K + 1)]
    sums: List[BIT] = [BIT(L, M) for _ in range(K + 1)]

    for a in sequence:
        # Length 1 sequences
        counts[1].add(a + 1, 1)
        sums[1].add(a + 1, a)

        # Length 2 to K sequences
        for i in range(2, K + 1):
            count = counts[i - 1].sum(a)
            counts[i].add(a + 1, count)
            sum_val = (count * a + sums[i - 1].sum(a)) % M
            sums[i].add(a + 1, sum_val)

    ans = sums[K].sum(L)
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
