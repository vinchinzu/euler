"""Project Euler Problem 524: First Sort II.

Consider a sorting algorithm where we repeatedly take the first pair of
adjacent elements not in order, and move the smaller element to the
beginning of the list. For a given n, let Q(n, k) be the index of the
lexicographically first permutation of n elements that requires k steps
to be sorted with this algorithm. Find the minimum Q(n, N) over all n.
"""

from __future__ import annotations

from math import factorial
from typing import List


def ilog2(n: int) -> int:
    """Integer log base 2."""
    return n.bit_length() - 1


def solve() -> int:
    """Solve Problem 524."""
    N = 12**12
    L = ilog2(N) + 1

    ranks: List[int] = []
    for i in range(L + 1):
        if (N & (1 << i)) != 0:
            ranks.append(i)

    ans = [0]

    def helper(remaining: List[int], rks: List[int], order_index: int) -> None:
        if not remaining:
            if not rks:
                ans[0] = order_index
            return
        # Pruning
        for i in range(min(len(remaining), len(rks))):
            if remaining[i] > rks[i] + i:
                return
        for i in range(len(remaining)):
            if ans[0] != 0:
                break
            el = remaining.pop(i)
            rank = el - i
            bit_index = -1
            if rank in rks:
                bit_index = rks.index(rank)
            if rank != L - len(remaining):
                if bit_index < 0:
                    # This rank isn't valid, skip this element
                    remaining.insert(i, el)
                    continue
                rks.pop(bit_index)
            helper(remaining, rks, order_index + i * factorial(len(remaining)))
            if rank != L - len(remaining):
                rks.insert(bit_index, rank)
            remaining.insert(i, el)

    helper(list(range(L + 1)), ranks, 1)
    return ans[0]


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
