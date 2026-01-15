"""Project Euler Problem 524: First Sort II.

Consider a sorting algorithm where we repeatedly take the first pair of
adjacent elements not in order, and move the smaller element to the
beginning of the list. For a given n, let Q(n, k) be the index of the
lexicographically first permutation of n elements that requires k steps
to be sorted with this algorithm. Find the minimum Q(n, N) over all n.
"""

from __future__ import annotations

from typing import List, Optional


def factorial(n: int) -> int:
    """Factorial."""
    if n <= 1:
        return 1
    result = 1
    for i in range(2, n + 1):
        result *= i
    return result


def ilog2(n: int) -> int:
    """Integer log base 2."""
    result = 0
    while n > 1:
        n >>= 1
        result += 1
    return result


def solve() -> int:
    """Solve Problem 524."""
    N = 12**12
    L = ilog2(N) + 1

    # Extract ranks from binary representation
    ranks: List[int] = []
    for i in range(L + 1):
        if (N & (1 << i)) != 0:
            ranks.append(i)

    remaining = list(range(L))
    ans: Optional[int] = None

    def helper(
        rem: List[int], rks: List[int], order_index: int
    ) -> None:
        """Recursive helper."""
        nonlocal ans
        if ans is not None:
            return

        if not rem:
            if not rks:
                ans = order_index
            return

        # Pruning: check if remaining elements can satisfy ranks
        for i in range(min(len(rem), len(rks))):
            if rem[i] > rks[i] + i:
                return

        for i in range(len(rem)):
            if ans is not None:
                break
            el = rem.pop(i)
            rank = el - i
            bit_index = -1
            if rank in rks:
                bit_index = rks.index(rank)
                if rank != L - len(rem):
                    rks.pop(bit_index)

            new_order = order_index + i * factorial(len(rem))
            helper(rem, rks, new_order)

            # Restore
            if rank != L - len(rem) and bit_index >= 0:
                rks.insert(bit_index, rank)
            rem.insert(i, el)

    helper(remaining, ranks, 1)
    return ans if ans is not None else 0


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
